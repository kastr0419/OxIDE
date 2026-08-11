// SPDX-License-Identifier: MIT OR Apache-2.0

use oxide::app::{AppMessage, FlashMsg, SerialMsg};
use oxide::core::board::BoardKind;
use oxide::core::flasher::{flash_async, FlashRequest, FlashResult};
use oxide::core::serial::VIRTUAL_PORT_NAME;
use oxide::core::serial::{connect_async, list_ports, SerialCommand, SerialSettings};
use std::time::Duration;

#[test]
fn virtual_port_connects_echoes_and_disconnects() {
    assert_eq!(
        list_ports()
            .unwrap()
            .iter()
            .filter(|port| port.as_str() == VIRTUAL_PORT_NAME)
            .count(),
        1
    );
    let (app_tx, app_rx) = crossbeam_channel::bounded(8);
    let (cmd_tx, cmd_rx) = crossbeam_channel::bounded(2);
    connect_async(
        SerialSettings {
            port_name: VIRTUAL_PORT_NAME.into(),
            baud_rate: 115_200,
        },
        app_tx,
        cmd_rx,
    );
    assert!(matches!(
        app_rx.recv_timeout(Duration::from_secs(1)),
        Ok(AppMessage::Serial(SerialMsg::Connected))
    ));
    assert!(matches!(
        app_rx.recv_timeout(Duration::from_secs(1)),
        Ok(AppMessage::Serial(SerialMsg::Line(line))) if line == "sensor:0"
    ));
    cmd_tx.send(SerialCommand::Send("ping".into())).unwrap();
    assert!(
        matches!(app_rx.recv_timeout(Duration::from_secs(1)), Ok(AppMessage::Serial(SerialMsg::Line(line))) if line == "echo:ping")
    );
    cmd_tx.send(SerialCommand::Disconnect).unwrap();
    assert!(matches!(
        app_rx.recv_timeout(Duration::from_secs(1)),
        Ok(AppMessage::Serial(SerialMsg::Disconnected))
    ));
}

#[test]
fn flashes_existing_artifact_to_virtual_board() {
    let artifact = std::env::temp_dir().join(format!("oxide-virtual-{}.elf", std::process::id()));
    std::fs::write(&artifact, b"test").unwrap();
    let (tx, rx) = crossbeam_channel::bounded(3);

    flash_async(
        FlashRequest {
            board: BoardKind::ArduinoUno,
            artifact: artifact.clone(),
            port: VIRTUAL_PORT_NAME.into(),
        },
        tx,
    );

    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(AppMessage::Flash(FlashMsg::Started))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(AppMessage::Flash(FlashMsg::Progress(_)))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(AppMessage::Flash(FlashMsg::Finished(FlashResult {
            success: true,
            ..
        })))
    ));
    std::fs::remove_file(artifact).unwrap();
}

#[test]
fn virtual_flash_rejects_missing_artifact() {
    let (tx, rx) = crossbeam_channel::bounded(2);
    flash_async(
        FlashRequest {
            board: BoardKind::ArduinoUno,
            artifact: std::env::temp_dir().join(format!(
                "oxide-missing-virtual-artifact-{}.elf",
                std::process::id()
            )),
            port: VIRTUAL_PORT_NAME.into(),
        },
        tx,
    );

    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(AppMessage::Flash(FlashMsg::Started))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(AppMessage::Flash(FlashMsg::Finished(FlashResult {
            success: false,
            ..
        })))
    ));
}
