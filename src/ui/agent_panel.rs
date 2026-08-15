// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use std::fmt::Write;

const MAX_EDITOR_CHARS: usize = 12_000;
const MAX_BUILD_LOG_CHARS: usize = 4_000;
const MAX_DIAGNOSTICS: usize = 20;
const MAX_DIAGNOSTIC_CHARS: usize = 500;

fn head_chars(text: &str, max: usize) -> String {
    let mut chars = text.chars();
    let mut result: String = chars.by_ref().take(max).collect();
    if chars.next().is_some() {
        result.push_str("\n… (truncated)");
    }
    result
}

fn tail_chars(text: &str, max: usize) -> String {
    let count = text.chars().count();
    let result: String = text.chars().skip(count.saturating_sub(max)).collect();
    if count > max {
        format!("… (truncated)\n{result}")
    } else {
        result
    }
}

fn agent_prompt_editor(prompt: &mut String) -> egui::TextEdit<'_> {
    egui::TextEdit::multiline(prompt)
        .desired_rows(5)
        .hint_text("例: このプロジェクトを確認して改善案を教えて")
        .lock_focus(true)
}

fn show_agent_prompt(ui: &mut egui::Ui, prompt: &mut String) -> egui::Response {
    let prompt_id = ui.make_persistent_id("agent_prompt");
    let ime_id = prompt_id.with("ime_active");
    let mut ime_active = ui
        .ctx()
        .data_mut(|data| data.get_temp(ime_id).unwrap_or(false));

    if ui.memory(|memory| memory.had_focus_last_frame(prompt_id)) {
        ui.input_mut(|input| {
            let suppress_tab = ime_active
                || input.events.iter().any(|event| {
                    matches!(
                        event,
                        egui::Event::Ime(egui::ImeEvent::Enabled | egui::ImeEvent::Preedit(_))
                    )
                });
            if suppress_tab {
                input.events.retain(|event| {
                    !matches!(
                        event,
                        egui::Event::Key {
                            key: egui::Key::Tab,
                            pressed: true,
                            ..
                        }
                    )
                });
            }
            for event in &input.events {
                match event {
                    egui::Event::Ime(egui::ImeEvent::Enabled | egui::ImeEvent::Preedit(_)) => {
                        ime_active = true;
                    }
                    egui::Event::Ime(egui::ImeEvent::Commit(_) | egui::ImeEvent::Disabled) => {
                        ime_active = false;
                    }
                    _ => {}
                }
            }
        });
    }
    ui.ctx()
        .data_mut(|data| data.insert_temp(ime_id, ime_active));

    ui.add(agent_prompt_editor(prompt).id(prompt_id))
}

fn open_agent_settings_folder() -> anyhow::Result<()> {
    let path = crate::core::agent::ensure_agent_settings()?;
    let folder = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("agent settings path has no parent"))?;

    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        let opener = if cfg!(target_os = "windows") {
            "explorer"
        } else if cfg!(target_os = "linux") {
            "xdg-open"
        } else {
            "open"
        };
        std::process::Command::new(opener).arg(folder).spawn()?;
        Ok(())
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    anyhow::bail!("opening the agent settings folder is unsupported on this OS")
}

fn oxide_context(app: &crate::app::IdeApp) -> String {
    let board = app.selected_board_preset();
    let port = app
        .available_ports
        .get(app.selected_port)
        .map(String::as_str)
        .unwrap_or("<none>");
    let active_file = app
        .file_path
        .as_deref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "<none>".to_owned());
    let mut context = format!(
        "- OxIDE version: {}\n\
         - Workspace: `{}`\n\
         - Board: {} (`{}`)\n\
         - Port: `{port}`\n\
         - Active file: `{active_file}`\n\
         - Cursor: line {}, column {}\n\
         - Active editor dirty: {}\n\
         - Debug: connected={}, halted={}, chip=`{}`\n\
         - Operations: building={}, flashing={}, serial_connected={}\n",
        env!("CARGO_PKG_VERSION"),
        app.config.workspace_dir.display(),
        board.display_name,
        board.target_triple,
        app.cursor_line,
        app.cursor_col,
        app.is_dirty,
        app.debug_connected,
        app.debug_halted,
        app.debug_chip_name,
        app.is_building,
        app.is_flashing,
        app.is_serial_connected,
    );

    if !app.lsp_diagnostics.is_empty() {
        context.push_str("\n## LSP diagnostics\n");
        for diagnostic in app.lsp_diagnostics.iter().take(MAX_DIAGNOSTICS) {
            let message = head_chars(
                &diagnostic.message.replace(['\r', '\n'], " "),
                MAX_DIAGNOSTIC_CHARS,
            );
            let _ = writeln!(
                context,
                "- {:?} at {}:{}: {}",
                diagnostic.severity,
                diagnostic.line + 1,
                diagnostic.col + 1,
                message
            );
        }
    }

    if app.is_dirty {
        context.push_str("\n## Unsaved active editor text\n```rust\n");
        context.push_str(&head_chars(&app.editor_text, MAX_EDITOR_CHARS));
        context.push_str("\n```\n");
    }

    if !app.build_log.is_empty() {
        context.push_str("\n## Build log (tail)\n```text\n");
        context.push_str(&tail_chars(&app.build_log, MAX_BUILD_LOG_CHARS));
        context.push_str("\n```\n");
    }

    context
}

pub fn ui_agent_panel(
    app: &mut crate::app::IdeApp,
    ui: &mut egui::Ui,
    tx: &crossbeam_channel::Sender<crate::core::event::CoreEvent>,
) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Codex のログインはブラウザで完了します。")
                .small()
                .weak(),
        );
        if ui
            .add_enabled(!app.agent_running, egui::Button::new("🔐 ログイン"))
            .clicked()
        {
            app.agent_running = true;
            app.agent_log = "ブラウザでログインを完了してください。\n".to_owned();
            crate::core::agent::login_async(tx.clone());
        }
    });

    ui.horizontal_wrapped(|ui| {
        let path = crate::core::agent::agent_settings_path()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| "<取得できません>".to_owned());
        if ui.button("📂 Agent設定").clicked() {
            if let Err(error) = open_agent_settings_folder() {
                if !app.agent_log.is_empty() && !app.agent_log.ends_with('\n') {
                    app.agent_log.push('\n');
                }
                let _ = writeln!(app.agent_log, "[ERROR] Agent設定: {error}");
            }
        }
        ui.label(
            egui::RichText::new(format!("設定: {path}"))
                .small()
                .monospace()
                .weak(),
        );
    });

    let previous_model = app.config.agent_model;
    ui.add_enabled_ui(!app.agent_running, |ui| {
        ui.horizontal(|ui| {
            ui.label("モデル:");
            egui::ComboBox::from_id_salt("agent_model")
                .selected_text(app.config.agent_model.label())
                .show_ui(ui, |ui| {
                    for model in crate::core::agent::AgentModel::ALL.iter().copied() {
                        ui.selectable_value(&mut app.config.agent_model, model, model.label());
                    }
                });
        });
    });
    if app.config.agent_model != previous_model {
        let _ = app.config.save();
    }

    ui.checkbox(
        &mut app.agent_allow_edits,
        "AIによるワークスペースの編集を許可",
    );
    ui.label(
        egui::RichText::new("OxIDEの状態（ボード、診断、ログなど）を依頼に共有します。")
            .small()
            .weak(),
    );

    let has_unsaved = app.is_dirty || app.open_tabs.iter().any(|tab| tab.is_dirty);
    if app.agent_allow_edits && has_unsaved {
        ui.colored_label(
            ui.visuals().warn_fg_color,
            "編集を許可する前に、未保存のファイルを保存してください。",
        );
    }

    ui.label("依頼:");
    show_agent_prompt(ui, &mut app.agent_prompt);

    ui.horizontal(|ui| {
        let can_run = !app.agent_running
            && !app.agent_prompt.trim().is_empty()
            && (!app.agent_allow_edits || !has_unsaved);
        if ui
            .add_enabled(can_run, egui::Button::new("▶ 実行"))
            .clicked()
        {
            app.agent_running = true;
            app.agent_log.clear();
            crate::core::agent::run_async(
                crate::core::agent::AgentRequest {
                    workspace: app.config.workspace_dir.clone(),
                    prompt: app.agent_prompt.trim().to_owned(),
                    allow_edits: app.agent_allow_edits,
                    context: oxide_context(app),
                    model: app.config.agent_model,
                },
                tx.clone(),
            );
        }
        if ui.button("クリア").clicked() {
            app.agent_log.clear();
        }
        if app.agent_running {
            ui.spinner();
            ui.label("実行中…");
        }
    });

    ui.separator();
    ui.label("ログ:");
    egui::ScrollArea::vertical()
        .id_salt("agent_log_scroll")
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut app.agent_log)
                    .desired_rows(12)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::{head_chars, show_agent_prompt, tail_chars};

    fn show_prompt(ctx: &egui::Context, prompt: &mut String, request_focus: bool) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = show_agent_prompt(ui, prompt);
            if request_focus {
                response.request_focus();
            }
        });
    }

    #[test]
    fn tab_is_inserted_into_agent_prompt() {
        let ctx = egui::Context::default();
        let mut prompt = String::new();

        let _ = ctx.run(Default::default(), |ctx| {
            show_prompt(ctx, &mut prompt, true);
        });
        let _ = ctx.run(Default::default(), |ctx| {
            show_prompt(ctx, &mut prompt, false);
        });
        let input = egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Tab,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            show_prompt(ctx, &mut prompt, false);
        });

        assert_eq!(prompt, "\t");
    }

    #[test]
    fn ime_prediction_survives_tab_selection() {
        let ctx = egui::Context::default();
        let mut prompt = String::new();

        let _ = ctx.run(Default::default(), |ctx| {
            show_prompt(ctx, &mut prompt, true);
        });
        let _ = ctx.run(Default::default(), |ctx| {
            show_prompt(ctx, &mut prompt, false);
        });
        let input = egui::RawInput {
            events: vec![
                egui::Event::Ime(egui::ImeEvent::Enabled),
                egui::Event::Ime(egui::ImeEvent::Preedit("よそく".into())),
            ],
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            show_prompt(ctx, &mut prompt, false);
        });
        let input = egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Tab,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            show_prompt(ctx, &mut prompt, false);
        });
        let input = egui::RawInput {
            events: vec![egui::Event::Ime(egui::ImeEvent::Commit("予測".into()))],
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            show_prompt(ctx, &mut prompt, false);
        });

        assert_eq!(prompt, "予測");
    }

    #[test]
    fn context_limits_are_utf8_safe() {
        assert_eq!(head_chars("a界b", 2), "a界\n… (truncated)");
        assert_eq!(tail_chars("a界b", 2), "… (truncated)\n界b");
    }
}
