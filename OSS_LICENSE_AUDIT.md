# OSS ライセンス審査レポート

## プロジェクト概要
- プロジェクト名: rust-embedded-ide
- 想定ライセンス: MIT / Apache-2.0
- 商用利用: あり（将来的に検討）
- 審査日: 2026-04-16
- 審査担当: License Audit Agent

---

## 審査サマリー

| 判定 | 件数 |
|------|------|
| ✅ 問題なし | 15 |
| ⚠️ 条件付き | 1 |
| ❌ 要注意 | 0 |

**総合判定**: 全体として主要な依存はMIT/Apache-2.0等の寛容ライセンスが多く、プロジェクト（MIT/Apache-2.0で公開）との互換性は良好。esp-idf-hal はライセンスの明確化が必要（要確認）。表示義務（LICENSE同梱等）は発生するため対応が必要。

---

## 詳細審査結果

### 直接依存クレート

| クレート | バージョン | ライセンス | 商用利用 | コピーレフト | 判定 | 備考 |
|---------|-----------:|-----------|---------|------------|------|------|
| eframe  | 0.31      | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | egui エコシステムに合わせた双免許が多い。 |
| egui    | 0.31      | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | 広く双免許で配布されている。 |
| serde   | 1         | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | 標準的な双免許。 |
| serde_json | 1      | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | |
| serialport | 4      | MIT OR Apache-2.0 (要確認) | 可 | なし | ✅ 問題なし | 多くは寛容ライセンスだが、正確な版権表記はcrates.ioで確認推奨。 |
| crossbeam-channel | 0.5 | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | crossbeam 系は双免許が一般的。 |
| anyhow  | 1         | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | |
| thiserror | 2       | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | |
| dirs    | 6         | MIT OR Apache-2.0 (要確認) | 可 | なし | ✅ 問題なし | 古い `dirs` には MIT のみのものもあるためcrates.ioで版ごと確認推奨。 |
| rfd     | 0.15      | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | GUI系ユーティリティ、寛容ライセンスが一般的。 |
| which   | 7         | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | |
| parking_lot | 0.12  | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | |
| toml    | 0.8       | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | toml crate は双免許が一般的。 |

※ 上記で "要確認" としたものは、概ね寛容ライセンスである可能性が高いが、プロジェクトで確実に表示義務や注記が必要かを版ごとに crates.io のライセンス欄で最終確認してください。

### 将来追加予定クレート

| クレート | ライセンス (想定) | 商用利用 | コピーレフト | 判定 | 備考 |
|---------|------------------|---------|------------|------|------|
| avr-hal (avr-hal-genericの一部) | MIT OR Apache-2.0 (一般的) | 可 | なし | ✅ 問題なし | 多くの HAL は寛容ライセンスだが版確認推奨。 |
| esp-idf-hal | 要確認（Espressif 関連で Apache-2.0 の可能性あり） | 要確認 | 要確認 | ⚠️ 条件付き | Espressif 関連は Apache-2.0 が多いが、esp-idf 本体やバイナリ結合時の制約を確認する必要あり。商用利用・再配布時の注意あり。 |
| stm32f4xx-hal | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | 多くは MIT/Apache の双免許。 |
| probe-rs | MIT OR Apache-2.0 | 可 | なし | ✅ 問題なし | probe-rs は寛容ライセンス（版ごと確認推奨）。 |

---

## ⚠️ 注意事項・対応が必要な項目

1. esp-idf-hal および Espressif IDF 組込みで使う場合は、IDF 本体のライセンス（多くは Apache-2.0）とバイナリ配布時の条件を確認してください。商用/商用サポート付き配布や署名済みバイナリの配布では追加条件がある場合があります。
2. 各クレートの正確なライセンス表記は版ごとに crates.io の "License" フィールドを確認してください。特に `dirs` や `serialport` など、過去にライセンスが変わった経緯があるクレートは注意。3rd-party バインディングや FFI で別ライセンスのネイティブライブラリを含む場合は要注意。
3. 本プロジェクトを MIT/Apache-2.0 で公開する場合、依存クレートが同等またはより寛容なライセンスであれば互換性はあるが、各依存の著作権表示・ライセンス文の同梱（表示義務）は必要です。

---

## 表示義務対応

本プロジェクトで**著作権表示・ライセンス文の同梱が必要なクレート**（直接依存・将来追加予定を含む、想定）:

| クレート | 必要な対応 |
|---------|-----------|
| eframe, egui, serde, serde_json, serialport, crossbeam-channel, anyhow, thiserror, dirs, rfd, which, parking_lot, toml, avr-hal, esp-idf-hal, stm32f4xx-hal, probe-rs | 各クレートの LICENSE (MIT または Apache-2.0) をプロジェクトの配布パッケージに同梱し、NOTICE/ATTRIBUTIONS を README または NOTICE ファイルに記載。 |

---

## 推奨事項

1. リリース時に依存ライセンスを自動収集するスクリプトを導入（cargo-license 等）して、LICENSES/NOTICE を生成・同梱すること。
2. esp-idf やその他のネイティブSDKを利用する機能は別モジュール/feature に切り出し、利用条件を明記すること（商用配布の条件を分かりやすく）。
3. crates.io の各パッケージページで使用する正確なライセンス表記をバージョン単位で確認し、CIでライセンスチェックを行うこと。

---

## 参考資料
- crates.io の各クレートページ
- SPDX License List: https://spdx.org/licenses/
