// SPDX-License-Identifier: MIT OR Apache-2.0

use alloide::core::board::BoardKind;
use alloide::core::event::{CoreEvent, FlashMsg, SerialMsg};
use alloide::core::flasher::{flash_async, FlashRequest, FlashResult};
use alloide::core::serial::VIRTUAL_PORT_NAME;
use alloide::core::serial::{connect_async, list_ports, SerialCommand, SerialSettings};
use alloide::core::simulator::VirtualBoardEvent;
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
        Ok(CoreEvent::Serial(SerialMsg::Connected))
    ));
    assert!(matches!(
        app_rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::VirtualBoard(VirtualBoardEvent::SerialConnected))
    ));
    assert!(matches!(
        app_rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::Serial(SerialMsg::Line(line))) if line == "sensor:0"
    ));
    assert!(matches!(
        app_rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::VirtualBoard(VirtualBoardEvent::SerialLine(line))) if line == "sensor:0"
    ));
    cmd_tx.send(SerialCommand::Send("ping".into())).unwrap();
    assert!(
        matches!(app_rx.recv_timeout(Duration::from_secs(1)), Ok(CoreEvent::Serial(SerialMsg::Line(line))) if line == "echo:ping")
    );
    assert!(
        matches!(app_rx.recv_timeout(Duration::from_secs(1)), Ok(CoreEvent::VirtualBoard(VirtualBoardEvent::SerialLine(line))) if line == "echo:ping")
    );
    cmd_tx.send(SerialCommand::Disconnect).unwrap();
    assert!(matches!(
        app_rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::Serial(SerialMsg::Disconnected))
    ));
    assert!(matches!(
        app_rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::VirtualBoard(
            VirtualBoardEvent::SerialDisconnected
        ))
    ));
}

#[test]
fn flashes_existing_artifact_to_virtual_board() {
    let artifact = std::env::temp_dir().join(format!("alloide-virtual-{}.elf", std::process::id()));
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
        Ok(CoreEvent::Flash(FlashMsg::Started))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::VirtualBoard(VirtualBoardEvent::FlashStarted))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::Flash(FlashMsg::Progress(_)))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::Flash(FlashMsg::Finished(FlashResult {
            success: true,
            ..
        })))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::VirtualBoard(VirtualBoardEvent::FlashFinished(
            true
        )))
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
                "alloide-missing-virtual-artifact-{}.elf",
                std::process::id()
            )),
            port: VIRTUAL_PORT_NAME.into(),
        },
        tx,
    );

    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::Flash(FlashMsg::Started))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::VirtualBoard(VirtualBoardEvent::FlashStarted))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::Flash(FlashMsg::Finished(FlashResult {
            success: false,
            ..
        })))
    ));
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(1)),
        Ok(CoreEvent::VirtualBoard(VirtualBoardEvent::FlashFinished(
            false
        )))
    ));
}
