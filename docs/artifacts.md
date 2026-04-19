# 📦 コンパイル成果物ガイド

ビルド成功後、成果物は `<プロジェクト>/dist/` フォルダにコピーされます。
各マイコンが必要とするファイル形式と書き込み方法をまとめます。

---

## 成果物マップ

| ボード / ファミリ | 必要ファイル | 拡張子 | 書き込みツール |
|-----------------|------------|--------|--------------|
| Arduino Uno / Nano / Mega | Intel HEX | `.hex` | avrdude |
| Arduino Leonardo | Intel HEX | `.hex` | avrdude |
| ESP32 / S2 / S3 | バイナリ | `.bin` | esptool.py |
| ESP32-C3 / C6 / H2 (RISC-V) | バイナリ | `.bin` | esptool.py |
| STM32F1 / F4 / L4 / F7 / H7 / G0 | ELF | `.elf` | probe-rs |
| nRF52840 | ELF | `.elf` | probe-rs |
| **BBC micro:bit v2** | **Intel HEX** | **`.hex`** | **DAPLink (USB ドライブ)** |
| RP2040 (Raspberry Pi Pico) | UF2 | `.uf2` | USB ドラッグ&ドロップ |
| RP2350 (Pico 2) | UF2 | `.uf2` | USB ドラッグ&ドロップ |
| SAMD21 / SAMD51 | バイナリ | `.bin` | bossac |
| Arduino Due | バイナリ | `.bin` | bossac |
| Teensy 4.x | Intel HEX | `.hex` | teensy_loader_cli |
| Raspberry Pi Zero | 生バイナリ | `.img` | SD カードコピー (`kernel.img`) |
| GD32VF103 / CH32V003 | ELF | `.elf` | OpenOCD |

---

## ファイル形式の違い

### ELF (Executable and Linkable Format)
- **特徴**: シンボル情報・デバッグ情報を含む完全な実行可能ファイル
- **用途**: probe-rs、OpenOCD など高機能デバッガが直接読み込める
- **生成**: `cargo build` で自動生成（拡張子なし or `.elf`）

### Intel HEX (`.hex`)
- **特徴**: バイナリをASCIIテキストで表現。アドレス情報付き
- **用途**: avrdude (AVR)、DAPLink (micro:bit)、TeensyLoader
- **生成**: `avr-objcopy -O ihex <elf> <hex>` (AVR) / `arm-none-eabi-objcopy -O ihex <elf> <hex>` (ARM)

### バイナリ (`.bin`)
- **特徴**: 純粋なバイト列。フラッシュアドレスの指定が別途必要
- **用途**: esptool.py (ESP32)、bossac (SAMD)、st-flash
- **生成**: `arm-none-eabi-objcopy -O binary <elf> <bin>` / `riscv32-unknown-elf-objcopy -O binary <elf> <bin>`

### UF2 (USB Flashing Format) (`.uf2`)
- **特徴**: RP2040/RP2350 専用。USB マスストレージとして認識されたデバイスにドラッグ&ドロップするだけで書き込み完了
- **用途**: Raspberry Pi Pico / Pico 2
- **生成**: `elf2uf2-rs <elf> <uf2>`

### 生バイナリ `.img`
- **特徴**: アドレス 0x8000 から始まる生バイナリ
- **用途**: Raspberry Pi Zero SD カードブート (`kernel.img`)
- **生成**: `arm-none-eabi-objcopy -O binary <elf> kernel.img`

---

## ボード別 書き込み手順

### Arduino Uno / Nano (avrdude)
```bash
# 自動: IDEのFlashボタン
# 手動:
avrdude -p m328p -c arduino -P COM3 -b 115200 -U flash:w:blink.hex:i
```
`dist/blink.hex` を使用

---

### ESP32 (esptool.py)
```bash
# 自動: IDEのFlashボタン
# 手動:
esptool.py --chip esp32 --port COM3 write_flash 0x10000 blink.bin
```
`dist/blink.bin` を使用  
> ⚠️ フラッシュアドレスはボードによって異なる場合があります (`0x0` / `0x10000`)

---

### BBC micro:bit v2 (DAPLink)
```
1. micro:bit を USB 接続 → "MICROBIT" ドライブが出現
2. dist/blink.hex を MICROBIT ドライブにコピー（ドラッグ&ドロップ）
3. 自動で再起動・書き込み完了
```
> IDEのFlash機能では `port` にMICROBITドライブのパス（例: `E:\`）を指定

---

### Raspberry Pi Pico (RP2040)
```
1. BOOTSELボタンを押しながらUSB接続 → "RPI-RP2" ドライブが出現
2. dist/blink.uf2 を RPI-RP2 ドライブにコピー
3. 自動で再起動・書き込み完了
```
> uf2 が生成されない場合: `cargo install elf2uf2-rs` でツールをインストール

---

### STM32 / nRF52840 (probe-rs)
```bash
# 自動: IDEのFlashボタン
# 手動:
probe-rs download --chip STM32F411RETx blink     # STM32F4の例
probe-rs download --chip nRF52840_xxAA blink      # nRF52840の例
```
`dist/blink.elf` を使用（または ELF ファイルを直接指定）

---

### SAMD21 / SAMD51 (bossac)
```bash
# 手動:
bossac -p COM3 -e -w -v -R blink.bin
```
`dist/blink.bin` を使用  
> リセットボタンをダブルクリックしてブートローダーモードにしてから実行

---

### Teensy (teensy_loader_cli)
```bash
# 手動:
teensy_loader_cli --mcu=TEENSY41 -w -v blink.hex
```
`dist/blink.hex` を使用

---

### Raspberry Pi Zero (SD カード)
```
1. dist/blink.elf を arm-none-eabi-objcopy で kernel.img に変換
   arm-none-eabi-objcopy -O binary blink.elf kernel.img
2. SDカードの /boot/ に kernel.img をコピー（既存のものを上書き）
3. SDカードを取り出してRPi Zeroに挿入・起動
```
> IDEのFlash機能では `port` にSDカードのドライブパス（例: `E:\`）を指定

---

## objcopy ツールのインストール

| ツールチェーン | パッケージ名 |
|-------------|-----------|
| ARM (Cortex-M / RPi Zero) | `arm-none-eabi-binutils` (Windows: GNU Arm Embedded Toolchain) |
| AVR (Arduino) | `avr-gcc` + `binutils-avr` |
| RISC-V (ESP32-C3等) | `riscv32-unknown-elf-binutils` |

### Windows (winget)
```powershell
winget install ArmKeilMDK.ArmGNUToolchain   # arm-none-eabi-objcopy
```

### Linux
```bash
sudo apt install gcc-arm-none-eabi binutils-avr binutils-riscv64-linux-gnu
```

---

## dist フォルダの場所

```
<ワークスペース>/
├── src/
│   └── main.rs
├── Cargo.toml
├── dist/               ← ここにコピーされます
│   ├── blink.elf
│   ├── blink.hex       (HEX 生成可能な場合)
│   ├── blink.bin       (BIN 生成可能な場合)
│   └── blink.uf2       (UF2 生成可能な場合)
└── target/
    └── ...             (cargo の出力先)
```

Build パネルの `📁 開く` ボタンで dist フォルダをエクスプローラーで開けます。
