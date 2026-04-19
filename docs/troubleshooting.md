# マイコン書き込みトラブルシューティングガイド

## 📋 最初に確認するチェックリスト
- 電源: ボードに正しく電源供給されているか（外部電源が必要な場合は接続確認）
- ケーブル: データ対応USBケーブルを使用しているか（充電専用ケーブルでないこと）
- ポート: 接続したCOM/TTYポートがOS上で認識されているか（Windows: COM#, Linux: /dev/ttyUSB0, /dev/ttyACM0）
- ツール: avrdude / esptool.py / probe-rs 等、使用するフラッシャーのドライバやバージョンが適切か
- 権限: Windows のドライバ権限、Linux のグループ/udev 権限があるか
- .cargo/config.toml: runner と target の設定が正しいか（付録参照）

## 🪟 Windows 固有の準備
- ドライバ: CP210x/CH340/ST-Link/J-Link 等の公式ドライバをインストール
- Zadig: CMSIS-DAP 等でドライバ置換が必要な場合は Zadig を使用。注意: ST-Link は専用ドライバのままにすることが多い（無闇に置換しない）
  - 推奨選択肢: WinUSB または libusbK（デバイスと用途を確認して選択）
- Device Manager でポートを確認、ドライバ更新やCOM番号固定のトラブルシュートを行う

## 🐧 Linux 固有の準備
- udev: probe-rs・シリアルデバイス等で利用するデバイス用に udev ルールを作成してパーミッションを付与（サンプルは付録参照）
- dialout グループ: シリアルポート利用ユーザーは dialout（または plugdev）グループに追加
- udev ルール適用後:
```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
``` 

## 🔧 共通エラーと事前確認
- ターゲットトリプル/runner設定の誤り: `.cargo/config.toml` の runner・target を確認（付録参照）
- 充電専用ケーブル: データ線が接続されていないケーブルを使用していないか
- 書き込みモード: ボード固有のブートモード（GPIO0, BOOTSEL など）を確認
- ポート名: Windows は COM#, Linux は /dev/ttyUSB* または /dev/ttyACM* を確認

## 🤖 avrdude（AVR: Arduino Uno/Nano）

### 🔌 ser_open(): can't open device
**対象**: AVR
**症状**:
```bash
ser_open(): can't open device "COM3": The system cannot find the file specified.
```
**原因**:
- ポート名が間違っている
- 権限やドライバが不足している
- 充電専用ケーブルを使用している
**解決策**:
1. Device Manager で正しい COM ポートを確認する
2. 別のUSBケーブル（データ対応）に交換する
3. ドライバ（CH340/CP210x）を再インストールする

**コマンド例**:
```bash
avrdude -v -p m328p -c arduino -P COM3 -b 115200 -U flash:w:firmware.hex
```

### ⚠️ stk500_recv(): programmer is not responding
**対象**: AVR
**症状**:
```bash
stk500_recv(): programmer is not responding
```
**原因**:
- ボード設定（board/processor/bootloader）が間違っている
- ブートローダーが壊れている
- 旧ブートローダーを使っている（ボーレート/プロトコル不一致）
**解決策**:
1. ボードとシリアル設定を確認する（IDE/Cargo runner）
2. 別のボード設定や低いボーレートを試す（例: 57600）
3. ISP プログラマでブートローダーを再書き込みする（USBasp 等）

**コマンド例**:
```bash
# ブートローダー再書込（USBasp を使用）
avrdude -c usbasp -p m328p -U flash:w:bootloader.hex
```

### 🔁 ブートローダー破損リカバリ
**対象**: AVR
**症状**: 書き込み中に失敗して起動しない
**原因**: ブートローダー領域が破損している
**解決策**:
1. ISP プログラマ（USBasp など）を接続する
2. avrdude でブートローダーを書き込む

**コマンド例**:
```bash
avrdude -c usbasp -p m328p -U flash:w:bootloader.hex
```

## 🌐 esptool.py（ESP32 / ESP8266）

### 🔌 Failed to connect to ESP32
**対象**: ESP32
**症状**:
```bash
A fatal error occurred: Failed to connect to ESP32: Timed out waiting for packet header
```
**原因**:
- BOOT/RESET の操作が不十分（自動リセット回路が機能していない）
- ドライバ未導入（CP210x / CH340）
- ボーレートが高すぎる
**解決策**:
1. 手動で BOOT（GPIO0）を押しながら RESET を押す、または BOOT を押してから書込みを開始
2. ドライバをインストールする
3. ボーレートを下げて再試行（例: 115200, 57600）

**コマンド例（Windows）**:
```bash
esptool.py --chip esp32 --port COM4 --baud 115200 write_flash -z 0x1000 firmware.bin
```
**コマンド例（Linux）**:
```bash
esptool.py --chip esp32 --port /dev/ttyUSB0 --baud 115200 write_flash --flash_size detect 0x1000 firmware.bin
```

### ⚙️ Wrong boot mode detected
**対象**: ESP32
**症状**:
```bash
Failed to connect to ESP32: Failed to detect target chip
Wrong boot mode detected
```
**原因**:
- GPIO0 / EN の配線やボタン回路が正しくない
- DTR/RTS による自動ブート回路が壊れている
**解決策**:
1. ハードウェア配線（GPIO0, EN）を確認する
2. 手動で BOOT/RESET 操作を行い書き込みを試す

### 📡 シリアルポートが見えない（ESP系）
**原因**: USBシリアルチップのドライバ未導入や権限不足
**解決策**:
1. CP210x/CH340 ドライバをインストール
2. Linux: dmesg | tail でデバイス認識を確認

### 🔧 フラッシュサイズ不一致
**対象**: ESP32
**症状**: フラッシュサイズエラーや書き込み後の異常挙動
**解決策**:
- esptool のオプション `--flash_size detect` を利用して自動検出する

## 🔍 probe-rs（STM32 / nRF / RP2040 等）

### ❗ No connected probes were found
**対象**: probe-rs（全般）
**症状**:
```bash
No connected probes were found
```
**原因**:
- probe 用ドライバがない／udev ルールがない
- ケーブルの不良
- Windows のドライバ問題（Zadig 未設定など）
**解決策**:
1. `probe-rs-cli ls` で接続状況を確認
2. Linux: udev ルールを追加（付録参照）
3. Windows: Zadig でドライバを確認（CMSIS-DAP など）
4. ケーブルを交換する

**コマンド例**:
```bash
probe-rs-cli ls
```

### 🔎 Target not found
**対象**: probe-rs（全般）
**症状**:
```bash
Target not found
```
**原因**:
- SWD 配線（SWCLK/SWDIO/GND/VCC）が間違っている
- ターゲットに電源が供給されていない
**解決策**:
1. SWD 配線を確認する（接続ピンの再確認）
2. ターゲットの電源をオンにする

**コマンド例**:
```bash
probe-rs-cli download --chip STM32F407VG firmware.bin
```

### 💾 Failed to write to flash / 書込保護
**対象**: probe-rs（STM32 等）
**症状**:
```bash
Failed to write to flash
```
**原因**:
- 書込保護やセクタロック
**解決策**:
1. mass_erase を実行して保護を解除
2. probe-rs の強制消去オプションを利用してから再試行

**コマンド例**:
```bash
probe-rs-cli erase --chip STM32F407VG --all
probe-rs-cli download --chip STM32F407VG firmware.bin
```

### 🔧 ST-Link/J-Link 認識問題
**対象**: probe-rs（全般）
**症状**: デバイスが認識されない
**原因**:
- ドライバが古い／ファームウェアが古い
**解決策**:
1. 公式サイトからドライバ/ファームウェアを更新する
2. Windows の場合は Device Manager で VID/PID を確認

## 🍓 RP2040（Raspberry Pi Pico）
- BOOTSEL モード: BOOTSEL ボタンを押しながら USB 接続するとマスストレージ（RPI-RP2）が現れる
- UF2 書込み: RPI-RP2 ドライブに .uf2 をドラッグ＆ドロップ
- picotool（SWD 操作が必要な場合）:
```bash
# 接続済み probe を使ってフラッシュする例（probe-rs か picotool を使用）
picotool info
# または probe-rs を利用して書き込む
probe-rs-cli download --chip rp2040 firmware.bin
```

## 🔁 再書き込み（Flash済みデバイスへの上書き）

既にプログラムが書き込まれているマイコンに再度書き込もうとすると、
動作中のアプリがシリアルポートやフラッシュを占有してうまく書き込めないことがあります。

---

### AVR（Arduino Uno / Nano / Mega）

**問題**: プログラム実行中でブートローダーに入れない  
**解決策**: ボードを**ブートローダーモード**に入れてから書き込む

1. **リセットボタン**を押して離す直前 or 直後に書き込みコマンドを実行する  
   （avrdude が `Connecting...` を表示したタイミングでリセット）
2. それでも失敗する場合は Arduino IDE の書き込みと同じタイミングで試す
3. Nano の場合は Old Bootloader を選択（`-b 57600`）

```bash
# リセット直後に実行
avrdude -v -p m328p -c arduino -P COM3 -b 115200 -U flash:w:firmware.hex
```

---

### ESP32 / ESP8266

**問題**: `Failed to connect to ESP32: Wrong boot mode detected` や書き込み失敗  

**解決策 1 — BOOT ボタン操作**:
1. `BOOT`（または `IO0`）ボタンを**押したまま**
2. `RESET` または `EN` ボタンを一瞬押して離す
3. `BOOT` ボタンを離す → ダウンロードモードに入る
4. 書き込みコマンドを実行する

**解決策 2 — フラッシュを全消去してから書き込む**:
```bash
# フラッシュ全消去
esptool.py --chip esp32 --port COM4 erase_flash

# 消去後に書き込み
esptool.py --chip esp32 --port COM4 --baud 115200 write_flash -z 0x1000 firmware.bin
```

---

### STM32（probe-rs / ST-Link）

**問題**: フラッシュ書き込み保護（RDP）が有効になっている  
**症状**:
```
Error: Failed to write to flash
Error: Flash protection active
```

**解決策 — 書き込み保護を解除してから書き込む**:
```bash
# フラッシュ全消去（保護解除を含む）
probe-rs erase --chip STM32F407VG --allow-erase-all

# 再書き込み
probe-rs download --chip STM32F407VG firmware.elf
```

**解決策 2 — STM32CubeProgrammer で解除**:
1. STM32CubeProgrammer を起動
2. ST-Link で接続
3. `OB`（Option Bytes）→ `Read Protection` を `AA`（Level 0）に設定
4. Apply → 自動リセット後に通常書き込みが可能

---

### nRF52 / micro:bit（probe-rs）

**問題**: `--allow-erase-all` なしだとアクセス拒否  
**症状**:
```
Error: An error with the usage of the nRF52 UICR occurred
```

**解決策**:
```bash
# erase-all フラグ付きで書き込む
probe-rs download --chip nRF52833_xxAA --allow-erase-all firmware.elf
```

---

### RP2040（Raspberry Pi Pico）

**問題**: 通常接続では書き込めない（アプリ実行中）  
**解決策 — BOOTSEL モードに入れる**:
1. **BOOTSEL ボタンを押したまま** USB を接続
2. `RPI-RP2` ドライブが現れる
3. `.uf2` ファイルをドロップ or probe-rs で書き込む

```bash
# probe-rs 経由（SWD プローブが必要）
probe-rs erase --chip RP2040
probe-rs download --chip RP2040 firmware.elf
```

---

### micro:bit v2（DAPLink）

**問題**: アプリが動いていて書き込めない  
**解決策**:
1. リセットボタンを長押しして `MICROBIT` ドライブを表示
2. `.hex` をドライブにドロップ
3. probe-rs 使用時は `--allow-erase-all` を付ける

---

## 📎 付録

### udev ルールサンプル
```bash
# /etc/udev/rules.d/99-embedded.rules
# ST-Link v2
SUBSYSTEM=="usb", ATTR{idVendor}=="0483", ATTR{idProduct}=="3748", MODE="0666", GROUP="plugdev"
# CMSIS-DAP (e.g. DAPLink)
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE="0666", GROUP="plugdev"
# SEGGER J-Link
SUBSYSTEM=="usb", ATTR{idVendor}=="1366", MODE="0666", GROUP="plugdev"
# Raspberry Pi Pico (BOOTSEL / UF2)
SUBSYSTEM=="usb", ATTR{idVendor}=="2e8a", ATTR{idProduct}=="0003", MODE="0666", GROUP="plugdev"
```

### .cargo/config.toml の例（runner / target）
```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32F407VG"
```

### よく使うコマンド集
```bash
# AVR 書込
avrdude -v -p m328p -c arduino -P COM3 -b 115200 -U flash:w:firmware.hex

# ESP32 全消去
esptool.py --chip esp32 --port COM4 erase_flash

# ESP32 書込
esptool.py --chip esp32 --port COM4 --baud 115200 write_flash --flash_size detect 0x1000 firmware.bin

# STM32 全消去 + 書込
probe-rs erase --chip STM32F407VG --allow-erase-all
probe-rs download --chip STM32F407VG firmware.elf

# nRF 書込（erase-all）
probe-rs download --chip nRF52833_xxAA --allow-erase-all firmware.elf

# udev ルール再読み込み
sudo udevadm control --reload-rules && sudo udevadm trigger
```

---

*このドキュメントは Rust 組み込み初心者〜中級者向けの書き込みトラブルシューティング集です。*
