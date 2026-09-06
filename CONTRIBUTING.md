# Contributing to ALLoIDE

ありがとうございます！ALLoIDE への貢献は大歓迎です。以下に貢献ガイドラインを示します。

## ライセンス
本プロジェクトに対するすべてのコントリビューターは、MIT License と Apache-2.0 のデュアルライセンス（MIT + Apache-2.0）に同意したものとみなされます。PR を送ることで同意したことになります。

## ブランチ名規則
新しいブランチは以下のプリフィックスを使用してください：
- feat/ (新機能)
- fix/ (バグ修正)
- docs/ (ドキュメント)
- refactor/ (リファクタリング)

例: `feat/add-esp32-support`

## Pull Request の流れ
1. Issue を作成（必要に応じて）
2. ブランチを作成（上記の規則に従う）
3. コードを実装、テストを追加
4. PR を作成。PR の説明に変更点と関連 Issue を記載

## コミットメッセージ規則
Conventional Commits 形式を採用します。例:
- feat(core): add new board support
- fix(serial): handle disconnect gracefully

## コードスタイル
- フォーマット: `cargo fmt`
- 静的解析: `cargo clippy` (可能な限り `-- -D warnings` を避けてください)

PR を提出する前に `cargo fmt` と `cargo clippy` を実行し、テストがある場合は通してください。

## Issue / PR テンプレート
Issue や PR のテンプレートを導入しています。テンプレートに沿って情報を記載してください。
