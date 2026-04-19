# RustでGUI IDEを作った技術解説 — OxIDEの実装パターン

この記事は中〜上級者向けに、OxIDEで採用した主要な実装パターンと注意点を技術的にまとめます。実装は実プロジェクトの典型的なパターンに基づいています。

## 1. egui / eframeによるUI設計
OxIDEはeframe（egui）を使った即時モードUIです。基本的にアプリはeframe::Appを実装し、update()で毎フレームUIを描画します。

```rust
impl eframe::App for IdeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            // メニューバー
        });
        egui::SidePanel::left("board_picker").show(ctx, |ui| {
            // ボード選択
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // エディタ領域
        });
    }
}
```

レイアウトはSidePanel/TopBottomPanel/CentralPanelを組み合わせるのが分かりやすく保守性も高いです。

## 2. バックグラウンド処理とcrossbeam-channel
長時間の処理（cargo build, flash, serial read）は別スレッドで実行し、結果をUIへ送ります。典型的なパターンはSender/Receiverを使うものです。

```rust
enum AppMessage { BuildOutput(String), BuildDone(bool), SerialLine(String) }

let (tx, rx) = crossbeam_channel::unbounded::<AppMessage>();
// ビルド実行スレッド
let tx2 = tx.clone();
std::thread::spawn(move || {
    // cargo build 実行 -> 出力をtx2.send(AppMessage::BuildOutput(...))
});

// UIスレッドのupdate()内で受信
while let Ok(msg) = rx.try_recv() {
    match msg { ... }
}
```

この設計によりUIはブロックせずにログやステータスを即座に反映できます。

## 3. Windowsの子プロセスとCREATE_NO_WINDOW問題
WindowsではGUIアプリが子プロセスをspawnすると新しいコンソールウィンドウが開くことがあります。これを防ぐため、CommandExt::creation_flagsでCREATE_NO_WINDOW(0x0800_0000)を設定します。

```rust
pub fn no_window(cmd: &mut std::process::Command) -> &mut std::process::Command {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    cmd
}
```

これにより外部ツール（cargo, avrdude, esptool.py等）が別ウィンドウを開かず、ログは標準出力で受け取れます。

## 4. probe-rsによるフラッシュ処理（STM32 / nRF）
probe-rsはデバイス検出と書き込みを行う便利なクレートですが、APIはバージョンで変わるため抽象化レイヤーでラップしておくと安定します。エラーはanyhow::Resultで統一して上下に伝搬させます。

## 5. シリアル通信
serialportクレートでポートを開き、受信を専用スレッドでループしてUIへ送信します。受信はバイナリ→UTF-8変換で落ちることがあるためエラーハンドリングを慎重に行います。

## 6. 実装で苦労した点
- AVRターゲットはRustのnightlyが必要で環境依存が強い
- probe-rsのAPI変化への対応
- eguiの即時モード特有のレイアウト設計（状態の保持方法）

## まとめ
OxIDEはGUIで組み込みワークフローをまとめることを目標に設計されています。eframe/eguiによる高速なUI開発、crossbeam-channelによるスレッド間通信、Windows固有の子プロセス問題への対策など、現場で役立つ実装パターンを多く含みます。

## 参考・リポジトリ
https://github.com/kastr0419/OxIDE

#タグ
`Rust` `egui` `eframe` `組み込み` `Windows`
