// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use anyhow::{Result, Context};
use crossbeam_channel::{bounded, Sender};
use std::io::{BufRead, BufReader, Write};
use std::thread;

pub const VIRTUAL_PORT_NAME: &str = "OxIDE Virtual Board";

pub enum SerialEvent {
    Opened,
    Closed,
    Data(String),
    #[allow(dead_code)]
    Error(String),
}

pub fn list_ports() -> Result<Vec<String>> {
    let mut ports: Vec<String> = serialport::available_ports()
        .unwrap_or_default()
        .into_iter()
        .map(|p| p.port_name)
        .collect();

    // Windows: DAPLink/SDカード書き込み用にドライブレターも追加
    #[cfg(windows)]
    for letter in b'A'..=b'Z' {
        let drive = format!("{}:\\", letter as char);
        if std::path::Path::new(&drive).exists() {
            ports.push(drive);
        }
    }

    if !ports.iter().any(|port| port == VIRTUAL_PORT_NAME) {
        ports.push(VIRTUAL_PORT_NAME.to_string());
    }

    Ok(ports)
}

pub struct SerialHandle {
    pub write_tx: Sender<String>,
    pub stop_tx: Sender<()>,
}

pub fn connect(port_name: &str, baud: u32, tx: Sender<SerialEvent>) -> Result<SerialHandle> {
    let port = serialport::new(port_name, baud).timeout(std::time::Duration::from_millis(100)).open().with_context(|| format!("failed to open serial port '{}'", port_name))?;
    tx.send(SerialEvent::Opened).ok();
    let (write_tx, write_rx) = bounded::<String>(10);
    let (stop_tx, stop_rx) = bounded::<()>(1);
    let stop_rx_reader = stop_rx.clone();
    let stop_rx_writer = stop_rx.clone();
    let mut reader = BufReader::new(port.try_clone()?);
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => { // no data
                    if stop_rx_reader.try_recv().is_ok() { break; }
                    thread::sleep(std::time::Duration::from_millis(10));
                    continue;
                }
                Ok(_) => {
                    let _ = tx_clone.send(SerialEvent::Data(line.clone()));
                }
                Err(e) => {
                    let _ = tx_clone.send(SerialEvent::Error(format!("read error: {}", e)));
                    break;
                }
            }
        }
        let _ = tx_clone.send(SerialEvent::Closed);
    });
    // writer thread
    let mut port_writer = port;
    thread::spawn(move || {
        loop {
            crossbeam_channel::select! {
                recv(write_rx) -> msg => match msg {
                    Ok(s) => { let _ = port_writer.write_all(s.as_bytes()); }
                    Err(_) => break,
                },
                recv(stop_rx_writer) -> _ => break,
            }
        }
    });
    Ok(SerialHandle { write_tx, stop_tx })
}

// UIが期待する型を追加
pub struct SerialSettings {
    pub port_name: String,
    pub baud_rate: u32,
}

#[allow(dead_code)]
pub struct SerialLine {
    pub text: String,
}

pub enum SerialCommand {
    Send(String),
    Disconnect,
}

pub fn connect_async(
    settings: SerialSettings,
    app_tx: crossbeam_channel::Sender<crate::app::AppMessage>,
    cmd_rx: crossbeam_channel::Receiver<SerialCommand>,
) {
    std::thread::spawn(move || {
        if settings.port_name == VIRTUAL_PORT_NAME {
            run_virtual_serial(app_tx, cmd_rx);
            return;
        }
        let (event_tx, event_rx) = bounded(32);
        match connect(&settings.port_name, settings.baud_rate, event_tx) {
            Ok(handle) => {
                app_tx.send(crate::app::AppMessage::Serial(crate::app::SerialMsg::Connected)).ok();
                // forward events
                std::thread::spawn(move || {
                    loop {
                        // forward serial commands
                        while let Ok(cmd) = cmd_rx.try_recv() {
                            match cmd {
                                SerialCommand::Send(s) => { let _ = handle.write_tx.send(s); }
                                SerialCommand::Disconnect => { let _ = handle.stop_tx.send(()); return; }
                            }
                        }
                        // receive events
                        match event_rx.try_recv() {
                            Ok(SerialEvent::Data(s)) => { app_tx.send(crate::app::AppMessage::Serial(crate::app::SerialMsg::Line(s))).ok(); }
                            Ok(SerialEvent::Closed) | Ok(SerialEvent::Error(_)) => {
                                app_tx.send(crate::app::AppMessage::Serial(crate::app::SerialMsg::Disconnected)).ok();
                                return;
                            }
                            _ => {}
                        }
                        std::thread::sleep(std::time::Duration::from_millis(5));
                    }
                });
            }
            Err(e) => {
                app_tx.send(crate::app::AppMessage::Error(format!("Serial error: {}", e))).ok();
            }
        }
    });
}

fn run_virtual_serial(
    app_tx: crossbeam_channel::Sender<crate::app::AppMessage>,
    cmd_rx: crossbeam_channel::Receiver<SerialCommand>,
) {
    use crate::app::{AppMessage, SerialMsg};
    use std::time::Duration;

    app_tx.send(AppMessage::Serial(SerialMsg::Connected)).ok();
    let ticker = crossbeam_channel::tick(Duration::from_millis(250));
    let mut sample = 0;
    loop {
        crossbeam_channel::select! {
            recv(cmd_rx) -> command => match command {
                Ok(SerialCommand::Send(text)) => {
                    app_tx.send(AppMessage::Serial(SerialMsg::Line(format!("echo:{}", text)))).ok();
                }
                Ok(SerialCommand::Disconnect) | Err(_) => break,
            },
            recv(ticker) -> _ => {
                app_tx.send(AppMessage::Serial(SerialMsg::Line(format!("sensor:{}", sample)))).ok();
                sample = (sample + 1) % 100;
            }
        }
    }
    app_tx.send(AppMessage::Serial(SerialMsg::Disconnected)).ok();
}
