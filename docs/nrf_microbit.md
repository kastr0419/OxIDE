# nRF / micro:bit クレート使用ガイド

## 対応ボード一覧
| ボード名 | チップ | クレート |
|---|---:|---|
| nRF52840 DK | nRF52840 | nrf52840-hal = "0.16" |
| nRF51822 (旧世代) | nRF51-series | nrf51-hal = "0.14" |
| micro:bit v2 | nRF52833 (子基板) | microbit-v2 = "0.15" + embedded-hal = "1" |

## Cargo.toml 設定
- nRF52840 用の最小例 (blink 用):

```toml
[package]
name = "blink"
version = "0.1.0"
edition = "2021"

[dependencies]
nrf52840-hal = "0.16"
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"
```

- micro:bit v2 用の最小例:

```toml
[package]
name = "blink"
version = "0.1.0"
edition = "2021"

[dependencies]
microbit-v2 = "0.15"
embedded-hal = "1"
cortex-m-rt = "0.7"
panic-halt = "0.2"

[profile.release]
lto = true
opt-level = "s"
```

## ツールチェーン設定
- nRF52840 (Cortex-M4F, thumbv7em-none-eabihf)

rust-toolchain.toml:

```toml
[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
```

.cargo/config または .cargo/config.toml に runner / target を指定してください。
micro:bit v2 は probe-rs を使った runner 指定例をテンプレートに含めています。

## GPIO ピン初期化
nrf-hal と microbit-v2 の初期化は次のように異なります。

- nRF HAL (例: nrf52840-hal)

```rust
use nrf52840_hal::{pac, gpio::{p0, Level, Output, PushPull}, prelude::*};

let p = pac::Peripherals::take().unwrap();
let port0 = p0::Parts::new(p.P0);
let mut led: Output<PushPull> = port0.p0_13.into_push_pull_output(Level::Low).degrade();
```

- micro:bit v2 (microbit-v2 crate)

```rust
use microbit::board::Board;
use microbit::hal::gpio::Level;

let board = Board::take().unwrap();
let mut row1 = board.display_pins.row1.into_push_pull_output(Level::Low);
```

主な違い:
- nrf-hal は PAC (pac::Peripherals) から直接 Peripherals を取り、ポート分割（p0::Parts）経由でピンを扱います。
- microbit-v2 は Board 構造体がまとめて提供され、display_pins や TIMER0 などボード固有の周辺機器が整理されています。

## LED 制御
- nRF の多くのボードでは LED がアクティブ LOW（GPIO を Low にすると LED が点灯）です。
  - 例: led.set_low().unwrap(); // 点灯
  - 消灯: led.set_high().unwrap();

- micro:bit の LED マトリクスは行(row)・列(col) を直接制御します。一般的な動作:
  - 行を HIGH にし、列を LOW にすることで該当セルが点灯する（ROW=HIGH, COL=LOW）。
  - 行/列は逆論理になっていることがあるためデータシート／crate ドキュメントを確認してください。

micro:bit マトリクスの例:

```rust
use embedded_hal::digital::OutputPin;

// row1 を HIGH にするとその行の LED が点灯（列が LOW のとき）
row1.set_high().unwrap();
// 消灯
row1.set_low().unwrap();
```

## 5×5 LED マトリクス（micro:bit v2）

micro:bit v2 の LED は 5 行 × 5 列のマトリクス配置で、ROW=HIGH / COL=LOW のセルが点灯します。

### マトリクス構造
```
     col1 col2 col3 col4 col5
row1 [ 1,  1,  1,  1,  1 ]   ← row1 HIGH + 全 col LOW → 1行目全灯
row2 [ 0,  0,  0,  0,  0 ]
row3 [ 0,  0,  1,  0,  0 ]   ← 中央だけ点灯
row4 [ 0,  0,  0,  0,  0 ]
row5 [ 1,  1,  1,  1,  1 ]
```
画像配列の要素は `1` = 点灯, `0` = 消灯。

---

### `microbit::display::blocking::Display` を使う方法（推奨）

`microbit-v2` クレートには行スキャンを自動処理する `Display` 型が含まれています。

```toml
# Cargo.toml
[dependencies]
microbit-v2 = "0.15"
embedded-hal = "1"
cortex-m-rt = "0.7"
panic-halt = "0.2"
```

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};
use panic_halt as _;

// 表示する画像パターン（5×5、各要素 0〜9 の輝度）
const HEART: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
];

const SMILE: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    loop {
        // ハートを 1000ms 表示
        display.show(&mut timer, HEART, 1000);
        // スマイルを 1000ms 表示
        display.show(&mut timer, SMILE, 1000);
    }
}
```

**`Display::show` のシグネチャ:**
```rust
pub fn show<T: DelayNs>(
    &mut self,
    timer: &mut T,
    image: [[u8; 5]; 5],
    duration_ms: u32,
)
```
- `image`: 5×5 配列。各値は `0`（消灯）〜 `9`（最大輝度）
- `duration_ms`: 表示時間（ミリ秒）

---

### 手動行スキャンで個別 LED を制御する方法

`Display` 型を使わずに ROW/COL を直接操作する例です。  
行スキャンは **1 行ずつ高速に切り替える** ことで全体が点灯して見えます（ダイナミック点灯）。

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::{board::Board, hal::gpio::Level};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let p = board.display_pins;

    // ROW ピンを配列にまとめる
    let mut rows = [
        p.row1.into_push_pull_output(Level::Low),
        p.row2.into_push_pull_output(Level::Low),
        p.row3.into_push_pull_output(Level::Low),
        p.row4.into_push_pull_output(Level::Low),
        p.row5.into_push_pull_output(Level::Low),
    ];
    // COL ピンを LOW（有効化）に固定して全列点灯
    let mut _col1 = p.col1.into_push_pull_output(Level::Low);
    let mut _col2 = p.col2.into_push_pull_output(Level::Low);
    let mut _col3 = p.col3.into_push_pull_output(Level::Low);
    let mut _col4 = p.col4.into_push_pull_output(Level::Low);
    let mut _col5 = p.col5.into_push_pull_output(Level::Low);

    loop {
        // 1 行ずつ HIGH → 短時間 → LOW を繰り返す（簡易スキャン）
        for row in rows.iter_mut() {
            row.set_high().unwrap();
            // 約 2ms 待つ（cortex_m::asm::delay はクロック依存）
            cortex_m::asm::delay(128_000);
            row.set_low().unwrap();
        }
    }
}
```

> **注意:** 手動スキャンは タイマー割り込みと組み合わせると安定します。  
> 簡単な用途には `Display::show` の使用を推奨します。

---

### よく使うパターン定数の例

```rust
const ALL_ON: [[u8; 5]; 5] = [[1; 5]; 5];
const ALL_OFF: [[u8; 5]; 5] = [[0; 5]; 5];

// 「X」字
const CROSS: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
];

// 「↑」矢印
const ARROW_UP: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
    [1, 0, 1, 0, 1],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

## A/B ボタンの使い方（micro:bit v2）

概要

micro:bit v2 のユーザー・ボタンは基板上の物理ピンに接続されており、左ボタン（A）は P0.14、右ボタン（B）は P0.23 に割り当てられます。両ボタンは "アクティブ LOW"（押すと LOW）なので、押下は is_low() で検出します。

Cargo.toml（最小依存）

```toml
[dependencies]
microbit-v2 = "0.15"
embedded-hal = "1"
cortex-m-rt = "0.7"
cortex-m = "0.7"
panic-halt = "0.2"
```

InputPin トレイトの明示インポート（embedded-hal 1.0 注意）

embedded-hal 1.0 系では prelude が廃止されているため、InputPin 等のデジタルトレイトは明示的に import してください。

```rust
use microbit::board::Board;
use embedded_hal::digital::InputPin; // <- 明示的に import
```

Polling パターン（例）

下はポーリングでボタンを監視する完全な例です。A が押されている間は中央の点を表示し、A+B 同時押しでは全点 ON を表示します。簡易デバウンスとして cortex_m::asm::delay を用いています。

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use embedded_hal::digital::InputPin;
use panic_halt as _;

const DOT: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

const ALL_ON: [[u8; 5]; 5] = [[9; 5]; 5];
const ALL_OFF: [[u8; 5]; 5] = [[0; 5]; 5];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    // TIMER0 は display.show に使う
    let mut timer = Timer::new(board.TIMER0);
    // display_pins を Display に渡す（所有権を移す）
    let mut display = Display::new(board.display_pins);
    // ボタンは board.buttons.button_a / button_b
    let mut buttons = board.buttons;

    loop {
        // A+B 同時押しの検出（両方とも LOW なら押下）
        if buttons.button_a.is_low().unwrap() && buttons.button_b.is_low().unwrap() {
            // 同時押し
            display.show(&mut timer, ALL_ON, 200);
            // 簡易デバウンス
            cortex_m::asm::delay(128_000);
            continue;
        }

        // A ボタン押下で点灯、離すと消灯
        if buttons.button_a.is_low().unwrap() {
            display.show(&mut timer, DOT, 50);
            cortex_m::asm::delay(64_000);
        } else {
            // 非押下時は消灯
            display.show(&mut timer, ALL_OFF, 50);
        }
    }
}
```

ポイント:
- button_a / button_b の is_low() が true を返すときが "押下中" です。
- unwrap() は簡易例のため使用。実際は適切なエラーハンドリングを追加してください。

Interrupt パターン（GPIOTE）（概念）

GPIOTE を使うとボタンの立ち下がりエッジで割り込みを受け取り、低消費電力で待機できます。実装の流れは概略で次の通りです:

- nrf52833-hal の GPIOTE 機能を有効化する
- 対応ピン（P0.14 / P0.23）を入力に設定し、立ち下がり（Falling）トリガを設定する
- 割り込みハンドラを定義し、フラグをセットしてメインループで処理する（ハンドラ内は短く保つ）
- 必要に応じてソフトウェア側でデバウンス（タイマー or タスク）を行う

詳細実装は HAL のバージョンに依存します。参考: https://docs.rs/nrf52833-hal/latest/

デバウンスのベストプラクティス

- 短時間の delay による簡易デバウンスは初歩的・一時的には有効ですが、CPU をブロックします。
- より良い方法はタイマを使った非ブロッキングデバウンス、または割り込みハンドラ内で最小限の処理を行いメインループで確定処理をする設計です。
- ハードウェア側（RC フィルタやシュミットトリガ）でノイズを低減できるならそちらを優先。

よくあるミス

- ボタンが "押されているのに" 反応しない: ボードのボタンがアクティブ LOW であることを見落としている場合があります。HIGH/LOW を逆に扱わないよう確認してください。
- Display/ピンの所有権を移動した後に同じ board のフィールドを使おうとしてコンパイルエラーになる（所有権のムーブ）: display_pins を移したら board の他のフィールドは別変数に移して使ってください。
- embedded-hal 1.0 の prelude を使おうとしてトレイト未解決エラーになる: 必ず use embedded_hal::digital::InputPin を明示的に import してください。

---

```

## PWM 設定
nrf52840-hal の PWM モジュールを使った基本的な例（概念例）。crate のバージョンで API が変わるため Cargo.toml のバージョンにあわせて調整してください。

```rust
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use nrf52840_hal::{pac, gpio::{p0, Level, Output, PushPull}, prelude::*, pwm::{Pwm, Prescaler}};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = p0::Parts::new(p.P0);
    // PWM 出力を割り当てたいピンに変換
    let pwm_pin = port0.p0_13.into_push_pull_output(Level::Low).degrade();

    let mut pwm = Pwm::new(p.PWM0);
    // prescaler / period の設定は実際の API に合わせてください
    pwm.set_prescaler(Prescaler::Div128);
    pwm.set_max_duty(0xFFFF);
    pwm.set_duty(0, 0x7FFF); // 50% duty on channel 0
    pwm.enable();

    loop {}
}
```

注: 上記は代表的な初期化手順を示しています。`nrf52840-hal` のバージョン間で API 名が変わることがあるため、`docs.rs` またはクレートの README を参照してください。

## UART / シリアル通信
nRF 系では UARTE 周辺機器を使うことが多いです。以下は UARTE の基本的な初期化例（nrf52840-hal）:

```rust
use nrf52840_hal::{pac, gpio::p0, uarte::{self, Uarte, Parity}, prelude::*};

let p = pac::Peripherals::take().unwrap();
let ports = p0::Parts::new(p.P0);
let tx = ports.p0_06.into_push_pull_output(Level::High).degrade();
let rx = ports.p0_08.into_floating_input().degrade();

let config = uarte::Config::default().baudrate(uarte::Baudrate::BAUD115200);
let mut uarte = Uarte::new(p.UARTE0, uarte::Pins { tx: tx.into(), rx: rx.into(), cts: None, rts: None }, config);

// 送信例（blocking）
let _ = uarte.write(b"Hello\r\n");
```

API の詳細はクレートのバージョンに依存します。non-blocking や DMA 相当の使い方はドキュメント参照。

## タイマー / 遅延
- cortex_m::asm::delay
  - コアサイクル数で待つ単純な遅延。短時間・正確性より簡便さを重視する場面で使用。
  - 例: cortex_m::asm::delay(64_000_000);
  - 注意: CPU をブロッキングするため長時間の待ちには向かない。

- micro:bit (microbit-v2) の Timer + Delay
  - board の TIMER0 を用いると embedded-hal の DelayMs/DelayNs を通じて待機可能。

micro:bit の例:

```rust
use microbit::board::Board;
use microbit::hal::Timer;

let board = Board::take().unwrap();
let mut timer = Timer::new(board.TIMER0);
// 500ms 待つ
timer.delay_ms(500u32);
```

使い分け:
- 正確さや低消費電力で複数タスクを扱う場合は周辺タイマ／割り込みを使う。
- 簡単な Lチカなど短いブロックでは cortex_m::asm::delay が手早い。

## embedded-hal 1.0 対応
embedded-hal 1.0 系では prelude が廃止されつつあるため、トレイトは明示的に import してください。

```rust
use embedded_hal::digital::OutputPin; // OutputPin を明示的に import
use embedded_hal::delay::DelayMs;     // DelayMs / DelayNs も同様
```

例 (micro:bit):

```rust
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayMs;

row1.set_high().unwrap();
// timer.delay_ms(200u32);
```

## 注意事項・Tips
- micro:bit LEDマトリクスは ROW=HIGH, COL=LOW で LED が点灯する（通常のボードと論理が逆になる箇所があるため注意）。
- embedded-hal 1.0 の prelude が廃止されたため明示的に use する必要があります（OutputPin / DelayNs など）。例:

```rust
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayMs;
```
- nRF ボードはボードによって LED の論理（アクティブLOW/ACTIVE HIGH）が違うことがあります。まず回路図やボード定義を確認してください。
- 長時間の待ちには割り込みベースや低消費電力モードの利用を検討すること。

## 割り込み
nRF 系では GPIOTE や TIMER 割り込みを用いることが一般的です。割り込みハンドラの定義や優先度設定は PAC/HAL の API に従ってください。割り込み内での長い処理は避け、フラグでメインループへ通知する設計を推奨します。

## よくあるエラーと対処法
- LED が点灯しない: ボードの回路で LED がアクティブ LOW の場合、High/Low の動作が逆になります。データシートを確認してください。
- `Peripherals::take()` が None: 既に取得されている可能性があります。テスト環境や unit-test での実行に注意してください。

---

参考テンプレート:
- D:\rust_embedded\src\templates\blink\nrf.rs
- D:\rust_embedded\src\templates\blink\microbit.rs



