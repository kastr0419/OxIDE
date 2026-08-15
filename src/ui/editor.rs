// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

pub fn ui_editor(
    app: &mut crate::app::IdeApp,
    ui: &mut egui::Ui,
    _tx: &crossbeam_channel::Sender<crate::app::AppMessage>,
) {
    // ─ タブバー ─
    if !app.open_tabs.is_empty() {
        let mut switch_to: Option<usize> = None;
        let mut close_idx: Option<usize> = None;
        egui::ScrollArea::horizontal()
            .id_salt("tab_bar_scroll")
            .max_height(28.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, tab) in app.open_tabs.iter().enumerate() {
                        let name = tab
                            .path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("untitled");
                        let dirty = tab.is_dirty || (i == app.active_tab && app.is_dirty);
                        let label = if dirty {
                            format!("● {}", name)
                        } else {
                            name.to_string()
                        };
                        let is_active = i == app.active_tab;
                        let resp =
                            ui.selectable_label(is_active, egui::RichText::new(&label).small());
                        if resp.clicked() && !is_active {
                            switch_to = Some(i);
                        }
                        if ui
                            .small_button("×")
                            .on_hover_text("タブを閉じる（自動保存）")
                            .clicked()
                        {
                            close_idx = Some(i);
                        }
                        ui.separator();
                    }
                });
            });
        if let Some(idx) = switch_to {
            app.switch_to_tab(idx);
        }
        if let Some(idx) = close_idx {
            app.close_tab(idx);
        }
        ui.separator();
    }

    // Font metrics for gutter
    let font_id = egui::TextStyle::Monospace.resolve(ui.style());
    let font_size = font_id.size;
    let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
    let line_count = app.editor_text.lines().count().max(1);
    let num_digits = format!("{}", line_count).len();
    let gutter_width = num_digits as f32 * font_size * 0.65 + 12.0;

    // ─ エディタツールバー ─
    ui.horizontal(|ui| {
        if ui.button("📂 Open").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Rust", &["rs"])
                .add_filter("All", &["*"])
                .pick_file()
            {
                app.open_file_in_tab(path);
            }
        }
        if ui.button("💾 Save").clicked() {
            let path = app.file_path.clone().or_else(|| {
                rfd::FileDialog::new()
                    .add_filter("Rust", &["rs"])
                    .save_file()
            });
            if let Some(p) = path {
                crate::app::write_or_log(&p, &app.editor_text, &mut app.build_log);
                app.file_path = Some(p.clone());
                app.is_dirty = false;
                if let Some(tab) = app.open_tabs.get_mut(app.active_tab) {
                    tab.path = p;
                    tab.content = app.editor_text.clone();
                    tab.is_dirty = false;
                }
            }
        }
        ui.separator();
        if ui.button("📋 Copy All").clicked() {
            ui.ctx().copy_text(app.editor_text.clone());
        }
        ui.separator();
        if let Some(ref p) = app.file_path {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("untitled");
            let label = if app.is_dirty {
                format!("● {}", name)
            } else {
                name.to_string()
            };
            ui.label(egui::RichText::new(label).small());
        } else {
            ui.label(
                egui::RichText::new("untitled.rs")
                    .small()
                    .color(egui::Color32::GRAY),
            );
        }
    });
    ui.separator();

    // ─ エディタ高さ計算 ─
    // 下部のスニペットバー/ダイアグノスティクス用に160px確保し、残りをエディタに割り当てる
    let editor_height = (ui.available_height() - 160.0).max(200.0);

    // ─ 行番号ガター + テキストエディタ ─
    let mut changed = false;

    // Smart editor input preprocessing (Tab, Enter, auto-close brackets)
    let te_id = ui.make_persistent_id("main_editor");
    let editor_had_focus = ui.memory(|memory| memory.had_focus_last_frame(te_id));
    let mut text_to_insert: Option<String> = None;
    let mut close_bracket: Option<char> = None;
    let mut tab_accept_completion = false;
    let mut tab_trigger_completion = false;
    let mut keep_editor_focus = false;

    if editor_had_focus {
        ui.input_mut(|input| {
            input.events.retain(|event| {
                match event {
                    // Tab -> 補完確定 / 補完トリガー / インデント（文脈依存）
                    egui::Event::Key {
                        key: egui::Key::Tab,
                        pressed: true,
                        modifiers,
                        ..
                    } if !modifiers.any() => {
                        keep_editor_focus = true;
                        if app.show_completion && !app.lsp_completions.is_empty() {
                            // ポップアップ表示中 → 選択補完を確定
                            tab_accept_completion = true;
                        } else {
                            let word = word_before_cursor(&app.editor_text, app.cursor_char_idx);
                            if !word.is_empty() {
                                // 単語の途中 → LSP補完をトリガー
                                tab_trigger_completion = true;
                            } else {
                                // 行頭・空白後 → 4スペース挿入
                                text_to_insert = Some("    ".to_string());
                            }
                        }
                        false // consume
                    }
                    // Enter -> auto-indent
                    egui::Event::Key {
                        key: egui::Key::Enter,
                        pressed: true,
                        modifiers,
                        ..
                    } if !modifiers.any() => {
                        // compute indentation from current line
                        let chars: Vec<char> = app.editor_text.chars().collect();
                        let end = app.cursor_char_idx.min(chars.len());
                        let line_start = chars[..end]
                            .iter()
                            .rposition(|&c| c == '\n')
                            .map(|i| i + 1)
                            .unwrap_or(0);
                        let indent: String = chars[line_start..end]
                            .iter()
                            .take_while(|&&c| c == ' ' || c == '\t')
                            .collect();
                        // add extra indent if last non-whitespace before cursor is '{'
                        let extra = {
                            let last_nw = chars[..end].iter().rposition(|c| !c.is_whitespace());
                            if last_nw.is_some_and(|i| chars[i] == '{') {
                                "    "
                            } else {
                                ""
                            }
                        };
                        text_to_insert = Some(format!("\n{}{}", indent, extra));
                        false // consume
                    }
                    // Intercept text events to detect opening brackets, but let TextEdit insert the character
                    egui::Event::Text(s) => match s.as_str() {
                        "{" => {
                            close_bracket = Some('}');
                            true
                        }
                        "(" => {
                            close_bracket = Some(')');
                            true
                        }
                        "[" => {
                            close_bracket = Some(']');
                            true
                        }
                        "\"" => {
                            close_bracket = Some('\"');
                            true
                        }
                        _ => true,
                    },
                    _ => true,
                }
            });
        });
    }

    // Apply any text insertion (Tab/Enter) before showing TextEdit and update cursor state
    if let Some(ins) = text_to_insert {
        let chars: Vec<char> = app.editor_text.chars().collect();
        let end = app.cursor_char_idx.min(chars.len());
        let before: String = chars[..end].iter().collect();
        let after: String = chars[end..].iter().collect();
        app.editor_text = format!("{}{}{}", before, ins, after);
        app.cursor_char_idx = end + ins.chars().count();
        app.is_dirty = true;
        if let Some(tab) = app.open_tabs.get_mut(app.active_tab) {
            tab.content = app.editor_text.clone();
            tab.is_dirty = true;
        }
        // Update TextEdit cursor state so the cursor appears in the right place
        let mut state = egui::TextEdit::load_state(ui.ctx(), te_id).unwrap_or_default();
        let cursor = egui::text::CCursor::new(app.cursor_char_idx);
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::one(cursor)));
        egui::TextEdit::store_state(ui.ctx(), te_id, state);
    }

    egui::ScrollArea::vertical()
        .id_salt("editor_scroll")
        .max_height(editor_height)
        .show(ui, |ui| {
            let resp = ui.horizontal_top(|ui| {
                // ── 行番号ガター ──
                let total_h = (row_height * line_count as f32 + 6.0).max(ui.available_height());
                let (gutter_rect, _) =
                    ui.allocate_exact_size(egui::vec2(gutter_width, total_h), egui::Sense::hover());
                ui.painter()
                    .rect_filled(gutter_rect, 0.0, egui::Color32::from_gray(28));
                for i in 0..line_count {
                    let y = gutter_rect.min.y + 2.0 + i as f32 * row_height;
                    let line_num = i + 1;

                    // Clickable area for this line in gutter
                    let line_rect = egui::Rect::from_min_size(
                        egui::pos2(
                            gutter_rect.min.x,
                            gutter_rect.min.y + 2.0 + i as f32 * row_height,
                        ),
                        egui::vec2(gutter_width, row_height),
                    );
                    let line_resp =
                        ui.interact(line_rect, egui::Id::new(("bp", i)), egui::Sense::click());
                    if line_resp.clicked() {
                        if app.breakpoints.contains(&line_num) {
                            app.breakpoints.remove(&line_num);
                        } else {
                            app.breakpoints.insert(line_num);
                        }
                    }

                    // Draw breakpoint dot
                    if app.breakpoints.contains(&line_num) {
                        ui.painter().circle_filled(
                            egui::pos2(gutter_rect.min.x + 6.0, y + row_height * 0.5),
                            4.0,
                            egui::Color32::RED,
                        );
                    }

                    // Draw line number (existing code)
                    ui.painter().text(
                        egui::pos2(gutter_rect.max.x - 4.0, y),
                        egui::Align2::RIGHT_TOP,
                        format!("{:>w$}", line_num, w = num_digits),
                        egui::FontId::monospace(font_size),
                        if app.breakpoints.contains(&line_num) {
                            egui::Color32::from_rgb(255, 150, 150) // highlight line number in red too
                        } else {
                            egui::Color32::from_gray(110)
                        },
                    );
                }

                // ── TextEdit ──
                let te_output = egui::TextEdit::multiline(&mut app.editor_text)
                    .code_editor()
                    .desired_width(ui.available_width())
                    .desired_rows(line_count.max(30))
                    .font(egui::TextStyle::Monospace)
                    .id(te_id)
                    .show(ui);
                if keep_editor_focus {
                    te_output.response.request_focus();
                }

                // カーソル位置を更新
                let cursor_range = te_output.cursor_range;
                if let Some(cr) = cursor_range {
                    let char_idx = cr.primary.ccursor.index;
                    let before: String = app.editor_text.chars().take(char_idx).collect();
                    app.cursor_line = before.chars().filter(|&c| c == '\n').count() + 1;
                    app.cursor_col = before
                        .rfind('\n')
                        .map(|p| before[p + 1..].chars().count() + 1)
                        .unwrap_or_else(|| before.chars().count() + 1);
                    app.cursor_char_idx = char_idx;
                    // スクリーン座標を記録（補完ポップアップ位置に使用）
                    let cursor_rect = te_output.galley.pos_from_cursor(&cr.primary);
                    app.cursor_screen_pos = Some(te_output.galley_pos + cursor_rect.min.to_vec2());

                    // Auto-close bracket: if an opening bracket was typed this frame, insert or skip over the closer
                    if let Some(closer) = close_bracket {
                        let chars: Vec<char> = app.editor_text.chars().collect();
                        let idx = app.cursor_char_idx.min(chars.len());
                        // If next char is already the closer, just move cursor over it
                        if chars.get(idx) == Some(&closer) {
                            app.cursor_char_idx = idx + 1;
                            let mut state =
                                egui::TextEdit::load_state(ui.ctx(), te_id).unwrap_or_default();
                            let cursor = egui::text::CCursor::new(app.cursor_char_idx);
                            state
                                .cursor
                                .set_char_range(Some(egui::text::CCursorRange::one(cursor)));
                            egui::TextEdit::store_state(ui.ctx(), te_id, state);
                        } else {
                            // Insert the closer after the cursor, leaving cursor between pair
                            let before: String = chars[..idx].iter().collect();
                            let after: String = chars[idx..].iter().collect();
                            app.editor_text = format!("{}{}{}", before, closer, after);
                            if let Some(tab) = app.open_tabs.get_mut(app.active_tab) {
                                tab.content = app.editor_text.clone();
                                tab.is_dirty = true;
                            }
                            app.is_dirty = true;
                        }
                    }
                }

                // ── ブラケットペアハイライト ──
                if let Some((open_idx, close_idx)) =
                    bracket_pair_at_cursor(&app.editor_text, app.cursor_char_idx)
                {
                    let highlight_color = egui::Color32::from_rgb(0, 212, 212);
                    let painter = ui.painter();
                    for &idx in &[open_idx, close_idx] {
                        let cc = egui::text::CCursor::new(idx);
                        let cursor = te_output.galley.from_ccursor(cc);
                        let rel_rect = te_output.galley.pos_from_cursor(&cursor);
                        let screen_min = te_output.galley_pos + rel_rect.min.to_vec2();
                        // Width of one char ≈ font_size * 0.62 (monospace approximation)
                        let char_w = font_size * 0.62;
                        let char_rect = egui::Rect::from_min_size(
                            screen_min,
                            egui::vec2(char_w, rel_rect.height()),
                        );
                        painter.rect_stroke(
                            char_rect,
                            0.0,
                            egui::Stroke::new(1.0, highlight_color),
                            egui::StrokeKind::Inside,
                        );
                    }
                }

                // 選択テキスト取得（右クリックメニュー用）
                let copy_text = cursor_range
                    .as_ref()
                    .and_then(|cr| {
                        let a = cr.primary.ccursor.index.min(cr.secondary.ccursor.index);
                        let b = cr.primary.ccursor.index.max(cr.secondary.ccursor.index);
                        if a == b {
                            return None;
                        }
                        let chars: Vec<char> = app.editor_text.chars().collect();
                        let end = b.min(chars.len());
                        if a > end {
                            return None;
                        }
                        Some(chars[a..end].iter().collect::<String>())
                    })
                    .unwrap_or_else(|| app.editor_text.clone());

                // 右クリックコンテキストメニュー
                te_output.response.context_menu(|ui| {
                    if ui.button("📋 Copy").clicked() {
                        ui.ctx().copy_text(copy_text.clone());
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.label(
                        egui::RichText::new("Cut:       Ctrl+X")
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                    ui.label(
                        egui::RichText::new("Paste:     Ctrl+V")
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                    ui.label(
                        egui::RichText::new("Select All: Ctrl+A")
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                    ui.label(
                        egui::RichText::new("Undo:      Ctrl+Z")
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                });

                te_output.response.changed()
            });
            changed = resp.inner;
        });

    // LSP: テキスト変更通知
    if changed {
        app.is_dirty = true;
        // アクティブタブの内容を同期
        if let Some(tab) = app.open_tabs.get_mut(app.active_tab) {
            tab.content = app.editor_text.clone();
            tab.is_dirty = true;
        }
        app.doc_version += 1;
        if let Some(ref lsp) = app.lsp_client {
            let uri = app
                .file_path
                .as_ref()
                .map(|p| crate::core::lsp::file_uri(p))
                .unwrap_or_else(|| "file:///untitled.rs".to_string());
            lsp.did_change(&uri, app.doc_version, &app.editor_text);
            let line = app.cursor_line.saturating_sub(1) as u32;
            let col = app.cursor_col.saturating_sub(1) as u32;

            // Only request completion on word characters, '_' or '.' or ':'
            let _word = word_before_cursor(&app.editor_text, app.cursor_char_idx);
            let last_char = app
                .editor_text
                .chars()
                .nth(app.cursor_char_idx.saturating_sub(1));
            let should_complete =
                last_char.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '.' || c == ':');

            if should_complete {
                lsp.request_completion(&uri, line, col);
            } else {
                // Typed space / punctuation → hide completion
                app.show_completion = false;
            }
        }
    }

    // Ctrl+Space / Tab(単語途中) で補完強制表示
    let want_completion =
        ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Space)) || tab_trigger_completion;
    if want_completion {
        if let Some(ref lsp) = app.lsp_client {
            let uri = app
                .file_path
                .as_ref()
                .map(|p| crate::core::lsp::file_uri(p))
                .unwrap_or_else(|| "file:///untitled.rs".to_string());
            let line = app.cursor_line.saturating_sub(1) as u32;
            let col = app.cursor_col.saturating_sub(1) as u32;
            lsp.request_completion(&uri, line, col);
        }
        app.show_completion = true;
    }

    // Arrow key handling
    // When completion popup is visible, use Up/Down to navigate it. Otherwise, don't auto-hide completion on arrow keys.
    if app.show_completion && !app.lsp_completions.is_empty() {
        if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            app.completion_selected = app.completion_selected.saturating_add(1);
        }
        if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            app.completion_selected = app.completion_selected.saturating_sub(1);
        }
        // Bounds will be clamped once we compute filtered list in the popup rendering
    } else {
        // No popup — arrow keys should not hide anything; normal cursor movement handled by TextEdit
    }

    // ─ 補完ポップアップ（カーソル直下に表示） ─
    if app.show_completion && !app.lsp_completions.is_empty() {
        if let Some(cursor_pos) = app.cursor_screen_pos {
            // カーソルの1行下・少し左にポップアップを配置
            let popup_pos = egui::pos2(cursor_pos.x, cursor_pos.y + 18.0);
            egui::Area::new(egui::Id::new("lsp_completion_popup"))
                .fixed_pos(popup_pos)
                .order(egui::Order::Tooltip)
                .interactable(true)
                .show(ui.ctx(), |ui| {
                    egui::Frame::popup(ui.style())
                        .shadow(egui::epaint::Shadow::NONE)
                        .show(ui, |ui| {
                            ui.set_max_width(320.0);
                            egui::ScrollArea::vertical()
                                .max_height(200.0)
                                .show(ui, |ui| {
                                    // Clone current completion items for local filtering
                                    let items = app.lsp_completions.clone();

                                    // Get current word prefix for filtering
                                    let word_prefix: String = {
                                        let chars: Vec<char> = app.editor_text.chars().collect();
                                        let end = app.cursor_char_idx.min(chars.len());
                                        let start = chars[..end]
                                            .iter()
                                            .rposition(|c| !c.is_alphanumeric() && *c != '_')
                                            .map(|i| i + 1)
                                            .unwrap_or(0);
                                        chars[start..end].iter().collect()
                                    };

                                    // Filter completions by prefix (case-insensitive)
                                    let filtered_items: Vec<(
                                        usize,
                                        crate::core::lsp::CompletionItem,
                                    )> = items
                                        .into_iter()
                                        .enumerate()
                                        .filter(|(_, item)| {
                                            if word_prefix.is_empty() {
                                                true
                                            } else {
                                                item.label
                                                    .to_lowercase()
                                                    .starts_with(&word_prefix.to_lowercase())
                                            }
                                        })
                                        .collect();

                                    // Hide popup if no matches
                                    if filtered_items.is_empty() {
                                        app.show_completion = false;
                                        return;
                                    }

                                    // Clamp selection
                                    if app.completion_selected >= filtered_items.len() {
                                        app.completion_selected = 0;
                                    }

                                    let max_show = 12usize;
                                    let show_len = filtered_items.len().min(max_show);

                                    for (i, (_orig_idx, item)) in
                                        filtered_items.iter().enumerate().take(show_len)
                                    {
                                        let icon = match item.kind {
                                            crate::core::lsp::CompletionKind::Function
                                            | crate::core::lsp::CompletionKind::Method => "fn ",
                                            crate::core::lsp::CompletionKind::Struct => "st ",
                                            crate::core::lsp::CompletionKind::Enum => "en ",
                                            crate::core::lsp::CompletionKind::Module => "mo ",
                                            crate::core::lsp::CompletionKind::Snippet => "✂ ",
                                            _ => "   ",
                                        };
                                        let label = format!("{}{}", icon, item.label);
                                        let selected = i == app.completion_selected;
                                        let resp = ui.selectable_label(selected, &label);
                                        if selected {
                                            if let Some(ref detail) = item.detail {
                                                ui.label(
                                                    egui::RichText::new(detail)
                                                        .small()
                                                        .color(egui::Color32::GRAY),
                                                );
                                            }
                                        }
                                        if resp.clicked() {
                                            let insert = item
                                                .insert_text
                                                .as_ref()
                                                .unwrap_or(&item.label)
                                                .clone();
                                            // カーソル位置の手前の単語を補完テキストで置き換える
                                            let char_idx = app.cursor_char_idx;
                                            let chars: Vec<char> =
                                                app.editor_text.chars().collect();
                                            let end = char_idx.min(chars.len());
                                            // 単語の開始位置を探す（英数字とアンダースコア以外で区切る）
                                            let word_start = chars[..end]
                                                .iter()
                                                .rposition(|c| !c.is_alphanumeric() && *c != '_')
                                                .map(|i| i + 1)
                                                .unwrap_or(0);
                                            let before: String =
                                                chars[..word_start].iter().collect();
                                            let after: String = chars[end..].iter().collect();
                                            let new_cursor_idx =
                                                word_start + insert.chars().count();
                                            app.editor_text =
                                                format!("{}{}{}", before, insert, after);
                                            app.cursor_char_idx = new_cursor_idx;
                                            app.show_completion = false;
                                            // タブの内容を同期
                                            if let Some(tab) = app.open_tabs.get_mut(app.active_tab)
                                            {
                                                tab.content = app.editor_text.clone();
                                                tab.is_dirty = true;
                                            }
                                            app.is_dirty = true;
                                        }
                                    }
                                });

                            // Key handling inside popup: Escape to hide
                            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                app.show_completion = false;
                            }

                            // Enter to accept when not in the middle of typing
                            if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !changed {
                                // Recompute filtered items to find selected
                                let items = app.lsp_completions.clone();
                                let word_prefix: String = {
                                    let chars: Vec<char> = app.editor_text.chars().collect();
                                    let end = app.cursor_char_idx.min(chars.len());
                                    let start = chars[..end]
                                        .iter()
                                        .rposition(|c| !c.is_alphanumeric() && *c != '_')
                                        .map(|i| i + 1)
                                        .unwrap_or(0);
                                    chars[start..end].iter().collect()
                                };
                                let filtered_items: Vec<crate::core::lsp::CompletionItem> = items
                                    .into_iter()
                                    .filter(|item| {
                                        if word_prefix.is_empty() {
                                            true
                                        } else {
                                            item.label
                                                .to_lowercase()
                                                .starts_with(&word_prefix.to_lowercase())
                                        }
                                    })
                                    .collect();
                                if !filtered_items.is_empty() {
                                    let idx = app.completion_selected.min(filtered_items.len() - 1);
                                    let item = &filtered_items[idx];
                                    let insert =
                                        item.insert_text.as_ref().unwrap_or(&item.label).clone();
                                    let char_idx = app.cursor_char_idx;
                                    let chars: Vec<char> = app.editor_text.chars().collect();
                                    let end = char_idx.min(chars.len());
                                    let word_start = chars[..end]
                                        .iter()
                                        .rposition(|c| !c.is_alphanumeric() && *c != '_')
                                        .map(|i| i + 1)
                                        .unwrap_or(0);
                                    let before: String = chars[..word_start].iter().collect();
                                    let after: String = chars[end..].iter().collect();
                                    let new_cursor_idx = word_start + insert.chars().count();
                                    app.editor_text = format!("{}{}{}", before, insert, after);
                                    app.cursor_char_idx = new_cursor_idx;
                                    app.show_completion = false;
                                    if let Some(tab) = app.open_tabs.get_mut(app.active_tab) {
                                        tab.content = app.editor_text.clone();
                                        tab.is_dirty = true;
                                    }
                                    app.is_dirty = true;
                                }
                            }
                        });
                });
        }
    }

    // Tab で補完確定（ポップアップ表示中）
    if tab_accept_completion && !app.lsp_completions.is_empty() {
        let word_prefix: String = {
            let chars: Vec<char> = app.editor_text.chars().collect();
            let end = app.cursor_char_idx.min(chars.len());
            let start = chars[..end]
                .iter()
                .rposition(|c| !c.is_alphanumeric() && *c != '_')
                .map(|i| i + 1)
                .unwrap_or(0);
            chars[start..end].iter().collect()
        };
        let filtered: Vec<crate::core::lsp::CompletionItem> = app
            .lsp_completions
            .clone()
            .into_iter()
            .filter(|item| {
                word_prefix.is_empty()
                    || item
                        .label
                        .to_lowercase()
                        .starts_with(&word_prefix.to_lowercase())
            })
            .collect();
        if !filtered.is_empty() {
            let sel = app.completion_selected.min(filtered.len() - 1);
            let item = &filtered[sel];
            let insert = item.insert_text.as_ref().unwrap_or(&item.label).clone();
            let char_idx = app.cursor_char_idx;
            let chars: Vec<char> = app.editor_text.chars().collect();
            let end = char_idx.min(chars.len());
            let word_start = chars[..end]
                .iter()
                .rposition(|c| !c.is_alphanumeric() && *c != '_')
                .map(|i| i + 1)
                .unwrap_or(0);
            let before: String = chars[..word_start].iter().collect();
            let after: String = chars[end..].iter().collect();
            let new_cursor_idx = word_start + insert.chars().count();
            app.editor_text = format!("{}{}{}", before, insert, after);
            app.cursor_char_idx = new_cursor_idx;
            app.show_completion = false;
            if let Some(tab) = app.open_tabs.get_mut(app.active_tab) {
                tab.content = app.editor_text.clone();
                tab.is_dirty = true;
            }
            app.is_dirty = true;
            // カーソル位置をTextEditに反映
            let mut state = egui::TextEdit::load_state(ui.ctx(), te_id).unwrap_or_default();
            let cursor = egui::text::CCursor::new(new_cursor_idx);
            state
                .cursor
                .set_char_range(Some(egui::text::CCursorRange::one(cursor)));
            egui::TextEdit::store_state(ui.ctx(), te_id, state);
        }
    }

    // ─ スニペットフィルタ入力 ─
    ui.horizontal(|ui| {
        ui.label("✂ Snippets:");
        ui.text_edit_singleline(&mut app.snippet_query);
    });

    let board = &crate::core::board::BOARD_PRESETS
        .get(app.selected_board)
        .map(|p| p.kind.clone())
        .unwrap_or(crate::core::board::BoardKind::Stm32F4);

    let matched = crate::core::snippets::filter_snippets(board, &app.snippet_query);
    if !matched.is_empty() {
        egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                for snippet in matched.iter().take(10) {
                    ui.horizontal(|ui| {
                        let cat_color = match snippet.category {
                            crate::core::snippets::SnippetCategory::Gpio => egui::Color32::GREEN,
                            crate::core::snippets::SnippetCategory::Uart => {
                                ui.visuals().warn_fg_color
                            }
                            crate::core::snippets::SnippetCategory::Spi
                            | crate::core::snippets::SnippetCategory::I2c => {
                                egui::Color32::LIGHT_BLUE
                            }
                            crate::core::snippets::SnippetCategory::Timer => egui::Color32::GOLD,
                            _ => egui::Color32::GRAY,
                        };
                        ui.colored_label(cat_color, format!("[{}]", snippet.trigger));
                        if ui
                            .button(snippet.label)
                            .on_hover_text(snippet.description)
                            .clicked()
                        {
                            app.editor_text.push_str("\n\n");
                            app.editor_text.push_str(snippet.code);
                            app.is_dirty = true;
                        }
                    });
                }
            });
    }

    // ─ Diagnostics ─
    if !app.lsp_diagnostics.is_empty() {
        ui.separator();
        ui.label(egui::RichText::new("⚠ Diagnostics").small());
        for diag in app.lsp_diagnostics.iter().take(5) {
            let color = match diag.severity {
                crate::core::lsp::DiagSeverity::Error => egui::Color32::RED,
                crate::core::lsp::DiagSeverity::Warning => ui.visuals().warn_fg_color,
                _ => egui::Color32::GRAY,
            };
            ui.label(
                egui::RichText::new(format!(
                    "  L{}:{} {}",
                    diag.line + 1,
                    diag.col + 1,
                    diag.message
                ))
                .small()
                .color(color),
            );
        }
    }
}

fn word_before_cursor(text: &str, char_idx: usize) -> &str {
    let chars: Vec<char> = text.chars().collect();
    let end = char_idx.min(chars.len());
    let start = chars[..end]
        .iter()
        .rposition(|c| !c.is_alphanumeric() && *c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);
    // return as str slice
    let byte_start: usize = text.chars().take(start).map(|c| c.len_utf8()).sum();
    let byte_end: usize = text.chars().take(end).map(|c| c.len_utf8()).sum();
    &text[byte_start..byte_end]
}

// ── Bracket matching helpers for bracket pair highlighting ──
fn find_matching_bracket(
    text: &[char],
    pos: usize,
    open: char,
    close: char,
    forward: bool,
) -> Option<usize> {
    let mut depth = 1i32;
    if forward {
        for (i, ch) in text.iter().enumerate().skip(pos + 1) {
            if *ch == open {
                depth += 1;
            }
            if *ch == close {
                depth -= 1;
            }
            if depth == 0 {
                return Some(i);
            }
        }
    } else {
        for (i, ch) in text[..pos].iter().enumerate().rev() {
            if *ch == close {
                depth += 1;
            }
            if *ch == open {
                depth -= 1;
            }
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

fn bracket_pair_at_cursor(text: &str, cursor_idx: usize) -> Option<(usize, usize)> {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    // Check cursor pos and one before
    for &pos in &[cursor_idx, cursor_idx.saturating_sub(1)] {
        if pos >= len {
            continue;
        }
        match chars[pos] {
            '{' => {
                if let Some(m) = find_matching_bracket(&chars, pos, '{', '}', true) {
                    return Some((pos, m));
                }
            }
            '(' => {
                if let Some(m) = find_matching_bracket(&chars, pos, '(', ')', true) {
                    return Some((pos, m));
                }
            }
            '[' => {
                if let Some(m) = find_matching_bracket(&chars, pos, '[', ']', true) {
                    return Some((pos, m));
                }
            }
            '}' => {
                if let Some(m) = find_matching_bracket(&chars, pos, '{', '}', false) {
                    return Some((m, pos));
                }
            }
            ')' => {
                if let Some(m) = find_matching_bracket(&chars, pos, '(', ')', false) {
                    return Some((m, pos));
                }
            }
            ']' => {
                if let Some(m) = find_matching_bracket(&chars, pos, '[', ']', false) {
                    return Some((m, pos));
                }
            }
            _ => {}
        }
    }
    None
}
