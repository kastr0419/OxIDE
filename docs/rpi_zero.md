# Raspberry Pi Zero ベアメタル開発ガイド (Rust)

## 概要
- SoC: BCM2835
- CPU: ARM1176JZF-S (ARMv6, 1GHz)
- RAM: 256MB / 512MB (Pi Zero W)
- ボードLED: ACT LED = GPIO 47

## 前提ツール
- arm-none-eabi-gcc
- arm-none-eabi-objcopy  
- Rust stable ツールチェーン

### Windows インストール
ARM GNU Toolchain を含むパッケージを入手して PATH に登録してください。

### Linux インストール
```bash
sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi
```

## ビルド手順
1. このIDEでRPi Zeroボードを選択してテンプレートを作成
2. `cargo build --release --target ./armv6-rpi-zero.json`
3. ELF → kernel.img 変換:
```bash
arm-none-eabi-objcopy -O binary target/armv6-rpi-zero/release/blink kernel.img
```

## SD カードへの書き込み
SD カードのBOOTパーティション（FAT）に必要なファイル:
- `bootcode.bin` — GPU ブートローダ
- `start.elf` — GPU ファームウェア
- `kernel.img` — あなたのプログラム（上書き）
- `config.txt` — ブート設定

### config.txt 最小例
```
kernel=kernel.img
enable_uart=1
gpu_mem=16
```

### Windows
1. SD カードを挿入（ドライブレターが割り当てられる）
2. BOOTパーティション（FAT）に kernel.img をコピー

### Linux
```bash
sudo mount /dev/mmcblk0p1 /mnt
sudo cp kernel.img /mnt/kernel.img
sudo umount /mnt
```

## GPIOピン配置
BCM2835 GPIO レジスタベースアドレス: `0x2020_0000`

| 用途 | GPIO番号 | 備考 |
|------|---------|------|
| ACT LED | GPIO47 | アクティブHigh |
| UART TX | GPIO14 | ALT0 |
| UART RX | GPIO15 | ALT0 |

## カスタムターゲット JSON
このプロジェクトには `armv6-rpi-zero.json` が含まれています。
Rust の標準ターゲットには ARM1176JZF-S ベアメタルが存在しないため、カスタムターゲットが必要です。

## トラブルシューティング
- `arm-none-eabi-gcc not found` → ARM GNU Toolchain をインストール
- ボードが起動しない → kernel.img が正しくコピーされているか確認
- ACT LED が点灯しない → config.txt で `dtparam=act_led_trigger=none` を設定
