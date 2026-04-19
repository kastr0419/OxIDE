# ビルドと書き込みの使い方

## ボード選択
- 左パネルの Board Picker で対象のボードを選択してください。

## Build ボタン
- 「Build」ボタンを押すと `cargo build` が実行され、出力は Build ログに表示されます。
- ビルドに失敗した場合はログを確認して修正してください。

## Flash ボタン
- 「Flash」ボタンを押すと、選択されたボードに応じたフラッシュツール（avrdude / esptool.py / probe-rs 等）で書き込みが実行されます。

## Build & Flash
- 「Build & Flash」はビルドが成功した後、自動でフラッシュを実行します。

## Flash / RAM 使用量メーター
- ビルド成功後、Flash と RAM の使用量が表示されます（例: Flash 12.3 KB / 32 KB = 38%）。

## ポート選択
- 書き込みに使用するポートは Board Picker の COM ポートドロップダウンから選択します。
