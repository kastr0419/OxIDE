# RISC-V マイコン クレート使用ガイド

## 対応ボード一覧
| ボード名 | チップ | クレート | ターゲット |
|---|---|---|---|
| Sipeed Longan Nano | GD32VF103 | gd32vf103xx-hal = "0.4" | riscv32imac-unknown-none-elf |
| GD32VF103 ボード (一般) | GD32VF103 | gd32vf103xx-hal = "0.4" | riscv32imac-unknown-none-elf |
| CH32V003 ボード | CH32V003 | ch32v-hal = "0.1" | riscv32ec-unknown-none-elf (unstable) |

## Cargo.toml 設定
例: GD32VF103
```toml
[dependencies]
gd32vf103xx-hal = "0.4"
riscv = "0.10"
riscv-rt = "0.12"
panic-halt = "0.2"
```

例: CH32V003
```toml
[dependencies]
ch32v-hal = "0.1"
riscv = "0.10"
riscv-rt = "0.12"
panic-halt = "0.2"
```
必要に応じて Cargo.toml とは別に `.cargo/config.toml` で build.target を指定してください。

## ツールチェーン設定
ターゲットトリプル:
- GD32VF103: riscv32imac-unknown-none-elf
- CH32V003: riscv32ec-unknown-none-elf (unstable / implementor-specific)

rustup でターゲットを追加する例:
```powershell
rustup target add riscv32imac-unknown-none-elf
rustup target add riscv32ec-unknown-none-elf
```
CH32V003 の riscv32ec は非標準でツールチェーン側で対応状況が変わるため注意してください。

## GPIO ピン初期化
RCU (クロック) の設定が必要な HAL と、単純に split() するだけの HAL が存在します。

GD32VF103 (rcu が必要な例):
```rust
let dp = pac::Peripherals::take().unwrap();
let mut rcu = dp.RCU.configure().freeze();
let mut gpiob = dp.GPIOB.split(&mut rcu);
let mut led: PB0<Output<PushPull>> = gpiob.pb0.into_push_pull_output(&mut gpiob.config);
```

CH32V003 (簡易な split の例):
```rust
let dp = pac::Peripherals::take().unwrap();
let rcc = dp.RCC.constrain();
let _ = rcc.cfgr.freeze();
let gpiod = dp.GPIOD.split();
let mut led = gpiod.pd0.into_push_pull_output();
```

RCU の有無は HAL に依存します。GD32 系はクロック制御を explicit に行う設計が多いです。

## LED 制御
基本は GPIO の set_low / set_high を使います。

GD32VF103:
```rust
led.set_low().unwrap(); // turn on (board dependent)
riscv::asm::delay(8_000_000);
led.set_high().unwrap();
```

CH32V003:
```rust
led.set_low().unwrap();
unsafe { riscv::asm::delay(480_000) };
led.set_high().unwrap();
```
LED の論理（LOW が点灯か HIGH が点灯か）はボードに依存します。

## PWM 設定
タイマベースの PWM は各 HAL の timer モジュールを使います。概略:
```rust
// pseudo example (check your HAL API)
let mut timer = Timer::new(dp.TIMER0, /*freq*/ 1.khz(), &mut rcu);
let mut pwm = timer.pwm(channel);
pwm.enable();
pwm.set_duty(max_duty / 2);
```
具体的な型・関数名は gd32vf103xx-hal / ch32v-hal のドキュメントを参照してください。

## UART / シリアル通信
各 HAL の serial モジュールを用います。一般的な流れ:
```rust
let serial = Serial::new(dp.USART0, pins, config, &mut rcu);
let (mut tx, mut rx) = serial.split();
nb::block!(tx.bwrite_all(b"hello\n"));
```
CH32V003 の場合も同様に RCC を初期化してからシリアルを構成してください。

## タイマー / 遅延
riscv::asm::delay は CPU サイクル数ベースの遅延です。値はクロック周波数に依存します。
- サイクル数を大きく取りすぎると長時間ブロッキングします。
- CH32V003 のサンプルでは unsafe ブロックで呼び出しています（`unsafe { riscv::asm::delay(...) }`）。

注意: 高精度な遅延や低消費電力待機はハードウェアタイマや WFI 等を使って設計してください。

## 割り込み（基本）
riscv-rt の #[interrupt] 属性を使って割り込みハンドラを定義します。例:
```rust
#[interrupt]
fn TIM0() {
    // interrupt handler
}
```
割り込み名とシンボルは PAC によって定義されます。必ず PAC のドキュメントで確認してください。

## 注意事項・Tips
- CH32V003 は riscv32ec という RV32EC 拡張（実装固有）ターゲットを必要とする場合があり、Rust の標準ターゲットに存在しないことがあります。
- build-std = ["core"] を `.cargo/config.toml` の [unstable] セクションで指定する必要がある場合があります（例: CH32V003）。
- GD32VF103 は Sipeed Longan Nano で動作確認されている例が多く、gd32vf103xx-hal のサンプルが参考になります。
- riscv::asm::delay は単純なサイクルループです。クロックに依存するため正確な時間を期待する場合はタイマーを使ってください。
- panic のハンドラは組み込み向けに panic-halt / panic-abort 等を選んでください。

## よくあるエラーと対処法
- 非標準ターゲット (riscv32ec など) の扱い: rustup/rust-toolchain によるターゲット追加が必要だが、特定の実装では対応ツールチェーンが限定されるため注意してください。
- riscv::asm::delay の値が合わない: サイクル数に依存するため、クロック周波数に合わせて調整してください。
- リンカ/メモリ関連のエラー: memory.x やリンカスクリプトがボードに一致しているか確認してください。

---

参考: template の blink 実装（`src/templates/blink/riscv.rs`）をベースに、GD32VF103 と CH32V003 の動作例を示しました。実際のボード向けにはメモリマップやフラッシュサイズ等を PAC / データシートで必ず確認してください。
