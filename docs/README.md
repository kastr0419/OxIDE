# Rust 組み込みクレート使用ガイド

このドキュメントは各マイコンファミリー向けのクレート使用ガイドへの入口です。各ファミリーごとに個別 Markdown ファイル（例: docs/avr.md）を作成し、以下のテンプレートに従って内容を記述してください。

## 対応ファミリー一覧
- [AVR](./avr.md)
- [RP2040](./rp2040.md)
- [STM32](./stm32.md)
- [nRF / micro:bit](./nrf_microbit.md)
- [ESP32](./esp32.md)
- [SAMD](./samd.md)
- [Teensy](./teensy.md)
- [RISC-V](./riscv.md)
- [🔧 書き込みトラブルシューティング](./troubleshooting.md)

## 参照テンプレート
既存のサンプルは src\templates\blink\ に AVR/RP/STM32/nRF/ESP/SAMD/Teensy/RISC-V のファイルがあります。実装例として参照してください。

---

各ファミリーの Markdown ファイルに必須のセクション（プログラマへの指示）:

```markdown
# {ファミリー名} クレート使用ガイド

## 対応ボード一覧
| ボード名 | チップ | クレート |
|---------|--------|---------|
| ...     | ...    | ...     |

## Cargo.toml 設定
```toml
[dependencies]
...
```

## GPIO ピン初期化
（GPIOピンを出力/入力として設定する方法）
```rust
...
```

## LED 制御
（LEDのON/OFFの方法、ボード固有のLEDピン情報）
```rust
...
```

## PWM 設定
（PWMチャンネルの設定、デューティ比の変更）
```rust
...
```

## UART / シリアル通信
```rust
...
```

## タイマー / 遅延
```rust
...
```

## 注意事項・Tips
```

---

作成後は docs/ 以下にファイルを追加し、本 README のリンクを更新してください。
