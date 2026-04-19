# Teensy クレート使用ガイド

## 対応ボード一覧
| ボード名 | チップ | クレート |
|---|---|---|
| Teensy 4.0 | IMXRT1062 | teensy4-bsp = "0.5" |
| Teensy 4.1 | IMXRT1062 | teensy4-bsp = "0.5" |

## Cargo.toml 設定
teensy4-bsp を dependencies に追加します。ボード固有の feature を持つ場合は features で有効化してください（多くの場合、実行時に board::t40 / board::t41 で切替えます）。

```toml
[dependencies]
teensy4-bsp = "0.5"
cortex-m = "0.7"
cortex-m-rt = "0.7"
embedded-hal = "1.0"
panic-halt = "0.2"
```

## ツールチェーン設定
ターゲットは thumbv7em-none-eabihf を指定します（.cargo/config または .cargo/config.toml）。

```toml
[build]
target = "thumbv7em-none-eabihf"
```

## GPIO ピン初期化
board::t40 / board::t41 を使ってボードリソースを取得します。以下はテンプレートの一例です。

```rust
use teensy4_bsp as bsp;
use bsp::board;

let instances = board::instances();
let board::Resources { mut gpio2, pins, .. } = board::t40(instances);
let mut led = bsp::board::led(&mut gpio2, pins.p13);
```

GPIO は内部で gpio1〜gpio9 のようなポート単位で管理されます。出力ピンは embedded-hal の OutputPin トレイトで扱えます。

## LED 制御
オンボード LED は通常 P13（pin 13）です。テンプレートでの操作は次の通りです。

```rust
led.set_high().unwrap();
// delay
led.set_low().unwrap();
```

テンプレート（src/templates/blink/teensy.rs）では cortex_m::asm::delay と GPT の有効化を組み合わせて遅延しています：

```rust
bsp::ral::modify_reg!(bsp::ral::gpt, instances.GPT1, CR, EN: 1);
cortex_m::asm::delay(600_000_000 / 2);
```

## PWM 設定
Teensy 4.x では FlexPWM（および一部の用途で GPT）を利用します。teensy4-bsp は低レベルのレジスタアクセス（RAL）を提供するため、FlexPWM を直接設定して PWM 波形を生成できます。概略例：

```rust
// Pseudocode outline
// 1. enable clock for FlexPWM
// 2. configure PWM module and channel
// 3. set duty cycle and enable output
```

実装は FlexPWM のレジスタ説明に従って行ってください。場合によっては hal ライブラリや既存の PWM ラッパーが存在するか確認してください。

## UART / シリアル通信
teensy4-bsp はボードリソースとしてシリアル用のピンやコントローラ（LPUART など）を提供します。一般的な流れ：

```rust
// acquire instances and pins
let instances = board::instances();
let board::Resources { mut gpio1, pins, .. } = board::t40(instances);
// configure uart with selected pins and baudrate using hal or raw registers
```

embedded-hal の serial traits を使うか、teensy4-bsp の提供する初期化関数を利用してください。

## タイマー / 遅延
短い遅延には cortex_m::asm::delay(count) を使えます。より正確な ms 単位の遅延やタイマー割り込みには GPT タイマーを使用します。テンプレート例では GPT を有効化してから delay を呼んでいます。

```rust
// enable GPT1
bsp::ral::modify_reg!(bsp::ral::gpt, instances.GPT1, CR, EN: 1);
// busy-wait delay
cortex_m::asm::delay(loops);
```

GPT を使って ms タイマーを構成し、count をベースに待機する方法が推奨されます。

## USB シリアル（概要）
teensy4-usb-serial などのクレートを使うと USB CDC（Serial over USB）を利用できます。用途に応じて teense4 の USB コントローラを初期化し、CDC を有効にしてください。

## 注意事項・Tips
- teensy4-bsp はボード選択を board::t40 / board::t41 で切り替えます（コード内で選択）。
- GPIO はポート番号（gpio1〜gpio9）で管理されます。どのピンがどのポートかはボードピンマッピングを確認してください。
- link.x（リンクスクリプト）は多くの場合 crate が自動で提供します。自前で用意する必要は通常ありません。
- テンプレートのように低レベルレジスタ（RAL）を直接操作する場合はデータシートを参照してレジスタ設定を行ってください。

## 割り込み
短時間の割り込み処理を行うには GPT やピン割り込みを用います。割り込みハンドラは短くし、重い処理はメインループに委譲してください。

## よくあるエラーと対処法
- USB シリアルが認識されない: USB 初期化やクロック設定、USB PHY の設定を確認してください。
- GPT が動作しない: GPT のクロック有効化や CCR/PRSC の設定が正しいか確認してください。

---

参考: src/templates/blink/teensy.rs のサンプルコードをベースにしています。
