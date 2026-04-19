// SPDX-License-Identifier: MIT OR Apache-2.0
// Pinout definitions and helpers

use crate::core::board::BoardKind;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinFunction {
    Gpio,
    Uart,
    Spi,
    I2C,
    Pwm,
    Adc,
    Power,
    Gnd,
    Nc,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PinInfo {
    pub number: u8,
    pub name: &'static str,
    pub functions: &'static [PinFunction],
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BoardPinout {
    pub board: BoardKind,
    pub pins: &'static [PinInfo],
}

// ─── Arduino Uno (ATmega328P) ────────────────────────────────────────────────
// Real layout: digital header along TOP edge (D0-D13 left→right),
//              analog header along BOTTOM-RIGHT (A0-A5),
//              power header along BOTTOM-LEFT (IOREF→VIN)
static ARDUINO_UNO_PINS: &[PinInfo] = &[
    // ── Digital header — top edge (x=0.07 start, step≈0.065) ─────────────
    PinInfo { number: 0,  name: "D0/RX",     functions: &[PinFunction::Uart],                   x: 0.07, y: 0.04 },
    PinInfo { number: 1,  name: "D1/TX",     functions: &[PinFunction::Uart],                   x: 0.14, y: 0.04 },
    PinInfo { number: 2,  name: "D2",        functions: &[PinFunction::Gpio],                   x: 0.20, y: 0.04 },
    PinInfo { number: 3,  name: "D3~",       functions: &[PinFunction::Gpio, PinFunction::Pwm], x: 0.27, y: 0.04 },
    PinInfo { number: 4,  name: "D4",        functions: &[PinFunction::Gpio],                   x: 0.33, y: 0.04 },
    PinInfo { number: 5,  name: "D5~",       functions: &[PinFunction::Gpio, PinFunction::Pwm], x: 0.40, y: 0.04 },
    PinInfo { number: 6,  name: "D6~",       functions: &[PinFunction::Gpio, PinFunction::Pwm], x: 0.46, y: 0.04 },
    PinInfo { number: 7,  name: "D7",        functions: &[PinFunction::Gpio],                   x: 0.53, y: 0.04 },
    PinInfo { number: 8,  name: "D8",        functions: &[PinFunction::Gpio],                   x: 0.59, y: 0.04 },
    PinInfo { number: 9,  name: "D9~",       functions: &[PinFunction::Gpio, PinFunction::Pwm], x: 0.66, y: 0.04 },
    PinInfo { number: 10, name: "D10~/SS",   functions: &[PinFunction::Pwm,  PinFunction::Spi], x: 0.72, y: 0.04 },
    PinInfo { number: 11, name: "D11~/MOSI", functions: &[PinFunction::Pwm,  PinFunction::Spi], x: 0.79, y: 0.04 },
    PinInfo { number: 12, name: "D12/MISO",  functions: &[PinFunction::Gpio, PinFunction::Spi], x: 0.85, y: 0.04 },
    PinInfo { number: 13, name: "D13/SCK",   functions: &[PinFunction::Gpio, PinFunction::Spi], x: 0.92, y: 0.04 },
    // ── Power header — bottom-left (IOREF→VIN, step=0.06) ────────────────
    PinInfo { number: 24, name: "IOREF", functions: &[PinFunction::Power], x: 0.04, y: 0.96 },
    PinInfo { number: 25, name: "RESET", functions: &[PinFunction::Gpio],  x: 0.10, y: 0.96 },
    PinInfo { number: 23, name: "3V3",   functions: &[PinFunction::Power], x: 0.16, y: 0.96 },
    PinInfo { number: 22, name: "5V",    functions: &[PinFunction::Power], x: 0.22, y: 0.96 },
    PinInfo { number: 21, name: "GND",   functions: &[PinFunction::Gnd],   x: 0.28, y: 0.96 },
    PinInfo { number: 26, name: "GND",   functions: &[PinFunction::Gnd],   x: 0.34, y: 0.96 },
    PinInfo { number: 20, name: "VIN",   functions: &[PinFunction::Power], x: 0.40, y: 0.96 },
    // ── Analog header — bottom-right ─────────────────────────────────────
    PinInfo { number: 14, name: "A0",     functions: &[PinFunction::Adc, PinFunction::Gpio], x: 0.58, y: 0.96 },
    PinInfo { number: 15, name: "A1",     functions: &[PinFunction::Adc, PinFunction::Gpio], x: 0.65, y: 0.96 },
    PinInfo { number: 16, name: "A2",     functions: &[PinFunction::Adc, PinFunction::Gpio], x: 0.71, y: 0.96 },
    PinInfo { number: 17, name: "A3",     functions: &[PinFunction::Adc, PinFunction::Gpio], x: 0.77, y: 0.96 },
    PinInfo { number: 18, name: "A4/SDA", functions: &[PinFunction::Adc, PinFunction::I2C],  x: 0.83, y: 0.96 },
    PinInfo { number: 19, name: "A5/SCL", functions: &[PinFunction::Adc, PinFunction::I2C],  x: 0.89, y: 0.96 },
];

// ─── micro:bit V2 (nRF52833) ─────────────────────────────────────────────────
// Edge connector: 5 large pads (row y=0.92) + 16 small pads (row y=0.78)
// Matches real board layout: large pads P0/P1/P2/3V/GND evenly spaced at bottom,
// small pads P3-P20 spread across just above them.
static MICROBIT_PINS: &[PinInfo] = &[
    // ── Large banana-clip pads (bottom row, evenly spaced) ───────────────
    PinInfo { number: 0,  name: "P0",    functions: &[PinFunction::Gpio, PinFunction::Adc],  x: 0.10, y: 0.92 },
    PinInfo { number: 1,  name: "P1",    functions: &[PinFunction::Gpio, PinFunction::Adc],  x: 0.30, y: 0.92 },
    PinInfo { number: 2,  name: "P2",    functions: &[PinFunction::Gpio, PinFunction::Adc],  x: 0.50, y: 0.92 },
    PinInfo { number: 30, name: "3V",    functions: &[PinFunction::Power],                   x: 0.70, y: 0.92 },
    PinInfo { number: 31, name: "GND",   functions: &[PinFunction::Gnd],                     x: 0.90, y: 0.92 },
    // ── Small edge pads (row just above large pads) ───────────────────────
    PinInfo { number: 3,  name: "P3",         functions: &[PinFunction::Gpio, PinFunction::Adc],  x: 0.03, y: 0.78 },
    PinInfo { number: 4,  name: "P4",         functions: &[PinFunction::Gpio, PinFunction::Adc],  x: 0.10, y: 0.78 },
    PinInfo { number: 5,  name: "P5/BTN_A",   functions: &[PinFunction::Gpio],                    x: 0.16, y: 0.78 },
    PinInfo { number: 6,  name: "P6",         functions: &[PinFunction::Gpio],                    x: 0.22, y: 0.78 },
    PinInfo { number: 7,  name: "P7",         functions: &[PinFunction::Gpio],                    x: 0.28, y: 0.78 },
    PinInfo { number: 8,  name: "P8",         functions: &[PinFunction::Gpio],                    x: 0.34, y: 0.78 },
    PinInfo { number: 9,  name: "P9",         functions: &[PinFunction::Gpio],                    x: 0.40, y: 0.78 },
    PinInfo { number: 10, name: "P10",        functions: &[PinFunction::Gpio, PinFunction::Adc],  x: 0.46, y: 0.78 },
    PinInfo { number: 11, name: "P11/BTN_B",  functions: &[PinFunction::Gpio],                    x: 0.52, y: 0.78 },
    PinInfo { number: 12, name: "P12",        functions: &[PinFunction::Gpio],                    x: 0.58, y: 0.78 },
    PinInfo { number: 13, name: "P13/SCK",    functions: &[PinFunction::Gpio, PinFunction::Spi],  x: 0.64, y: 0.78 },
    PinInfo { number: 14, name: "P14/MISO",   functions: &[PinFunction::Gpio, PinFunction::Spi],  x: 0.70, y: 0.78 },
    PinInfo { number: 15, name: "P15/MOSI",   functions: &[PinFunction::Gpio, PinFunction::Spi],  x: 0.76, y: 0.78 },
    PinInfo { number: 16, name: "P16/CS",     functions: &[PinFunction::Gpio, PinFunction::Spi],  x: 0.82, y: 0.78 },
    PinInfo { number: 19, name: "P19/SCL",    functions: &[PinFunction::Gpio, PinFunction::I2C],  x: 0.88, y: 0.78 },
    PinInfo { number: 20, name: "P20/SDA",    functions: &[PinFunction::Gpio, PinFunction::I2C],  x: 0.94, y: 0.78 },
    // ── On-board components ───────────────────────────────────────────────
    PinInfo { number: 40, name: "LED matrix", functions: &[PinFunction::Gpio], x: 0.50, y: 0.35 },
    PinInfo { number: 41, name: "Button A",   functions: &[PinFunction::Gpio], x: 0.12, y: 0.55 },
    PinInfo { number: 42, name: "Button B",   functions: &[PinFunction::Gpio], x: 0.88, y: 0.55 },
];

// ─── ESP32 (Xtensa LX6, DevKitC 38-pin) ──────────────────────────────────────
// Real layout: two vertical columns (flash-only SD2/SD3/CMD pins omitted).
// Left col  (15 pins): 3V3, EN, VP/IO36, VN/IO39, IO34..IO12, GND, IO13
// Right col (16 pins): GND, 5V, IO23..IO21, GND, IO19..IO5, IO17..IO4, IO0, IO2
static ESP32_PINS: &[PinInfo] = &[
    // ── Left column (top → bottom, 15 pins, step≈0.066) ──────────────────
    PinInfo { number: 50, name: "3V3",        functions: &[PinFunction::Power],                                    x: 0.06, y: 0.04 },
    PinInfo { number: 53, name: "EN",         functions: &[PinFunction::Gpio],                                     x: 0.06, y: 0.11 },
    PinInfo { number: 36, name: "GPIO36/VP",  functions: &[PinFunction::Adc],                                      x: 0.06, y: 0.17 },
    PinInfo { number: 39, name: "GPIO39/VN",  functions: &[PinFunction::Adc],                                      x: 0.06, y: 0.24 },
    PinInfo { number: 34, name: "GPIO34",     functions: &[PinFunction::Adc],                                      x: 0.06, y: 0.30 },
    PinInfo { number: 35, name: "GPIO35",     functions: &[PinFunction::Adc],                                      x: 0.06, y: 0.37 },
    PinInfo { number: 32, name: "GPIO32",     functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Pwm], x: 0.06, y: 0.43 },
    PinInfo { number: 33, name: "GPIO33",     functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Pwm], x: 0.06, y: 0.50 },
    PinInfo { number: 25, name: "GPIO25/DAC1",functions: &[PinFunction::Gpio, PinFunction::Adc],                   x: 0.06, y: 0.57 },
    PinInfo { number: 26, name: "GPIO26/DAC2",functions: &[PinFunction::Gpio, PinFunction::Adc],                   x: 0.06, y: 0.63 },
    PinInfo { number: 27, name: "GPIO27",     functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Pwm], x: 0.06, y: 0.70 },
    PinInfo { number: 14, name: "GPIO14",     functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Spi], x: 0.06, y: 0.76 },
    PinInfo { number: 12, name: "GPIO12",     functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Spi], x: 0.06, y: 0.83 },
    PinInfo { number: 54, name: "GND",        functions: &[PinFunction::Gnd],                                      x: 0.06, y: 0.89 },
    PinInfo { number: 13, name: "GPIO13",     functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Spi], x: 0.06, y: 0.96 },
    // ── Right column (top → bottom, 16 pins, step≈0.061) ─────────────────
    PinInfo { number: 51, name: "GND",          functions: &[PinFunction::Gnd],                                        x: 0.94, y: 0.04 },
    PinInfo { number: 52, name: "5V",           functions: &[PinFunction::Power],                                      x: 0.94, y: 0.10 },
    PinInfo { number: 23, name: "GPIO23/MOSI",  functions: &[PinFunction::Gpio, PinFunction::Spi],                     x: 0.94, y: 0.16 },
    PinInfo { number: 22, name: "GPIO22/SCL",   functions: &[PinFunction::Gpio, PinFunction::I2C],                     x: 0.94, y: 0.22 },
    PinInfo { number: 1,  name: "TXD0",         functions: &[PinFunction::Uart],                                       x: 0.94, y: 0.29 },
    PinInfo { number: 3,  name: "RXD0",         functions: &[PinFunction::Uart],                                       x: 0.94, y: 0.35 },
    PinInfo { number: 21, name: "GPIO21/SDA",   functions: &[PinFunction::Gpio, PinFunction::I2C],                     x: 0.94, y: 0.41 },
    PinInfo { number: 55, name: "GND",          functions: &[PinFunction::Gnd],                                        x: 0.94, y: 0.47 },
    PinInfo { number: 19, name: "GPIO19/MISO",  functions: &[PinFunction::Gpio, PinFunction::Spi],                     x: 0.94, y: 0.53 },
    PinInfo { number: 18, name: "GPIO18/SCK",   functions: &[PinFunction::Gpio, PinFunction::Spi],                     x: 0.94, y: 0.59 },
    PinInfo { number: 5,  name: "GPIO5/SS",     functions: &[PinFunction::Gpio, PinFunction::Spi, PinFunction::Pwm],   x: 0.94, y: 0.65 },
    PinInfo { number: 17, name: "GPIO17",       functions: &[PinFunction::Gpio, PinFunction::Uart],                    x: 0.94, y: 0.71 },
    PinInfo { number: 16, name: "GPIO16",       functions: &[PinFunction::Gpio, PinFunction::Uart],                    x: 0.94, y: 0.78 },
    PinInfo { number: 4,  name: "GPIO4",        functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Pwm],   x: 0.94, y: 0.84 },
    PinInfo { number: 0,  name: "GPIO0",        functions: &[PinFunction::Gpio, PinFunction::Pwm],                     x: 0.94, y: 0.90 },
    PinInfo { number: 2,  name: "GPIO2",        functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Pwm],   x: 0.94, y: 0.96 },
];

// ─── STM32F4 Discovery (STM32F407VG) ────────────────────────────────────────
// Left col = PA0-PA10 (user button + UART/SPI/GPIO)
// Right col = PB6-PB11 (I2C) + PB0/1/3/4 (GPIO) + SWD debug
static STM32F4_PINS: &[PinInfo] = &[
    // ── Left column: PORTA ───────────────────────────────────────────────
    PinInfo { number: 0,  name: "PA0/BTN",   functions: &[PinFunction::Gpio, PinFunction::Adc],           x: 0.06, y: 0.10 },
    PinInfo { number: 1,  name: "PA1",       functions: &[PinFunction::Gpio, PinFunction::Adc],           x: 0.06, y: 0.18 },
    PinInfo { number: 2,  name: "PA2/TX2",   functions: &[PinFunction::Gpio, PinFunction::Uart],          x: 0.06, y: 0.26 },
    PinInfo { number: 3,  name: "PA3/RX2",   functions: &[PinFunction::Gpio, PinFunction::Uart],          x: 0.06, y: 0.34 },
    PinInfo { number: 4,  name: "PA4/DAC1",  functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Spi], x: 0.06, y: 0.42 },
    PinInfo { number: 5,  name: "PA5/SCK1",  functions: &[PinFunction::Gpio, PinFunction::Spi],           x: 0.06, y: 0.50 },
    PinInfo { number: 6,  name: "PA6/MISO1", functions: &[PinFunction::Gpio, PinFunction::Spi],           x: 0.06, y: 0.58 },
    PinInfo { number: 7,  name: "PA7/MOSI1", functions: &[PinFunction::Gpio, PinFunction::Spi],           x: 0.06, y: 0.66 },
    PinInfo { number: 8,  name: "PA8",       functions: &[PinFunction::Gpio, PinFunction::Pwm],           x: 0.06, y: 0.70 },
    PinInfo { number: 9,  name: "PA9/TX1",   functions: &[PinFunction::Gpio, PinFunction::Uart],          x: 0.06, y: 0.78 },
    PinInfo { number: 10, name: "PA10/RX1",  functions: &[PinFunction::Gpio, PinFunction::Uart],          x: 0.06, y: 0.86 },
    // ── Right column: PORTB + SWD ─────────────────────────────────────────
    PinInfo { number: 20, name: "PB6/SCL1",  functions: &[PinFunction::Gpio, PinFunction::I2C],           x: 0.94, y: 0.10 },
    PinInfo { number: 21, name: "PB7/SDA1",  functions: &[PinFunction::Gpio, PinFunction::I2C],           x: 0.94, y: 0.18 },
    PinInfo { number: 22, name: "PB10/SCL2", functions: &[PinFunction::Gpio, PinFunction::I2C],           x: 0.94, y: 0.26 },
    PinInfo { number: 23, name: "PB11/SDA2", functions: &[PinFunction::Gpio, PinFunction::I2C],           x: 0.94, y: 0.34 },
    PinInfo { number: 24, name: "PB0",       functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Pwm], x: 0.94, y: 0.42 },
    PinInfo { number: 25, name: "PB1",       functions: &[PinFunction::Gpio, PinFunction::Adc, PinFunction::Pwm], x: 0.94, y: 0.50 },
    PinInfo { number: 26, name: "PB3",       functions: &[PinFunction::Gpio, PinFunction::Spi],           x: 0.94, y: 0.58 },
    PinInfo { number: 27, name: "PB4",       functions: &[PinFunction::Gpio, PinFunction::Spi],           x: 0.94, y: 0.66 },
    PinInfo { number: 60, name: "PA13/SWDIO",functions: &[PinFunction::Gpio],                             x: 0.94, y: 0.74 },
    PinInfo { number: 61, name: "PA14/SWCLK",functions: &[PinFunction::Gpio],                             x: 0.94, y: 0.82 },
    // ── On-board LEDs (center of board) ──────────────────────────────────
    PinInfo { number: 44, name: "PD12/LED緑", functions: &[PinFunction::Gpio, PinFunction::Pwm], x: 0.38, y: 0.28 },
    PinInfo { number: 45, name: "PD13/LED橙", functions: &[PinFunction::Gpio, PinFunction::Pwm], x: 0.52, y: 0.28 },
    PinInfo { number: 46, name: "PD14/LED赤", functions: &[PinFunction::Gpio, PinFunction::Pwm], x: 0.38, y: 0.38 },
    PinInfo { number: 47, name: "PD15/LED青", functions: &[PinFunction::Gpio, PinFunction::Pwm], x: 0.52, y: 0.38 },
    // ── Power ─────────────────────────────────────────────────────────────
    PinInfo { number: 70, name: "3V3", functions: &[PinFunction::Power], x: 0.25, y: 0.94 },
    PinInfo { number: 71, name: "GND", functions: &[PinFunction::Gnd],   x: 0.45, y: 0.94 },
    PinInfo { number: 72, name: "5V",  functions: &[PinFunction::Power], x: 0.65, y: 0.94 },
];

pub static ARDUINO_UNO_PINOUT: BoardPinout = BoardPinout {
    board: BoardKind::ArduinoUno,
    pins: ARDUINO_UNO_PINS,
};

pub static MICROBIT_PINOUT: BoardPinout = BoardPinout {
    board: BoardKind::MicroBitV2,
    pins: MICROBIT_PINS,
};

pub static ESP32_PINOUT: BoardPinout = BoardPinout {
    board: BoardKind::Esp32,
    pins: ESP32_PINS,
};

pub static STM32F4_PINOUT: BoardPinout = BoardPinout {
    board: BoardKind::Stm32F4,
    pins: STM32F4_PINS,
};

pub fn get_pinout(board: BoardKind) -> Option<&'static BoardPinout> {
    match board {
        BoardKind::ArduinoUno  => Some(&ARDUINO_UNO_PINOUT),
        BoardKind::ArduinoNano => Some(&ARDUINO_UNO_PINOUT), // same MCU
        BoardKind::MicroBitV2  => Some(&MICROBIT_PINOUT),
        BoardKind::Esp32       => Some(&ESP32_PINOUT),
        BoardKind::Stm32F4     => Some(&STM32F4_PINOUT),
        _ => None,
    }
}

