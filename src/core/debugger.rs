// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};

#[derive(Debug, Clone)]
pub struct RegisterValue {
    pub name: String,
    pub raw: u64,
}

impl RegisterValue {
    pub fn hex(&self) -> String { format!("0x{:08X}", self.raw) }
    pub fn dec(&self) -> String { format!("{}", self.raw) }
}

#[derive(Debug)]
pub enum DebugCommand {
    Connect { chip: String },
    Disconnect,
    Halt,
    Continue,
    Step,
    ReadRegisters,
    ReadMemory { addr: u64, len: usize },
    StartRtt { channel: u32 },
    StopRtt,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum DebugEvent {
    Connected { probes: Vec<String> },
    Disconnected,
    Halted,
    Continued,
    Registers(Vec<RegisterValue>),
    MemoryRead { addr: u64, data: Vec<u8> },
    RttData { channel: u32, data: String },
    ProbeList(Vec<String>),
    Error(String),
}

/// Spawn the debugger thread. Returns (cmd_tx, event_rx).
pub fn spawn_debugger() -> (Sender<DebugCommand>, Receiver<DebugEvent>) {
    let (cmd_tx, cmd_rx) = crossbeam_channel::bounded::<DebugCommand>(32);
    let (evt_tx, evt_rx) = crossbeam_channel::bounded::<DebugEvent>(64);

    std::thread::spawn(move || {
        debugger_loop(cmd_rx, evt_tx);
    });

    (cmd_tx, evt_rx)
}

fn debugger_loop(cmd_rx: Receiver<DebugCommand>, evt_tx: Sender<DebugEvent>) {
    use probe_rs::{Permissions, probe::list::Lister};
    use probe_rs::MemoryInterface;

    let mut session: Option<probe_rs::Session> = None;

    // RTT worker management (mock/stub implementation)
    let mut rtt_worker: Option<std::thread::JoinHandle<()>> = None;
    let mut rtt_stop_tx: Option<crossbeam_channel::Sender<()>> = None;

    // List probes immediately on start
    let lister = Lister::new();
    let probes: Vec<String> = lister.list_all()
        .iter()
        .map(|p| format!("{:?}", p))
        .collect();
    let _ = evt_tx.send(DebugEvent::ProbeList(probes));

    loop {
        // Use timeout so thread can remain responsive to potential future needs
        match cmd_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(cmd) => {
                match cmd {
                    DebugCommand::Connect { chip } => {
                        let lister = Lister::new();
                        let probes_list = lister.list_all();
                        if probes_list.is_empty() {
                            let _ = evt_tx.send(DebugEvent::Error("No debug probe found. Connect ST-Link/J-Link.".to_string()));
                            continue;
                        }
                        match probes_list[0].open() {
                            Ok(probe) => {
                                match probe.attach(&chip, Permissions::default()) {
                                    Ok(s) => {
                                        let names: Vec<String> = probes_list.iter().map(|p| format!("{:?}", p)).collect();
                                        session = Some(s);
                                        let _ = evt_tx.send(DebugEvent::Connected { probes: names });
                                    }
                                    Err(e) => {
                                        let _ = evt_tx.send(DebugEvent::Error(format!("Attach failed: {}", e)));
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = evt_tx.send(DebugEvent::Error(format!("Open probe failed: {}", e)));
                            }
                        }
                    }

                    DebugCommand::Disconnect => {
                        // Stop RTT worker if running
                        if let Some(tx) = rtt_stop_tx.take() {
                            let _ = tx.send(());
                        }
                        if let Some(handle) = rtt_worker.take() {
                            let _ = handle.join();
                        }

                        session = None;
                        let _ = evt_tx.send(DebugEvent::Disconnected);
                    }

                    DebugCommand::Halt => {
                        if let Some(ref mut s) = session {
                            match s.core(0) {
                                Ok(mut core) => {
                                    match core.halt(Duration::from_millis(1000)) {
                                        Ok(_) => {
                                            let _ = evt_tx.send(DebugEvent::Halted);
                                            // Read registers after halt
                                            send_registers(&mut core, &evt_tx);
                                        }
                                        Err(e) => { let _ = evt_tx.send(DebugEvent::Error(format!("Halt failed: {}", e))); }
                                    }
                                }
                                Err(e) => { let _ = evt_tx.send(DebugEvent::Error(format!("Core access failed: {}", e))); }
                            }
                        } else {
                            let _ = evt_tx.send(DebugEvent::Error("Not connected".to_string()));
                        }
                    }

                    DebugCommand::Continue => {
                        if let Some(ref mut s) = session {
                            match s.core(0) {
                                Ok(mut core) => {
                                    match core.run() {
                                        Ok(_) => { let _ = evt_tx.send(DebugEvent::Continued); }
                                        Err(e) => { let _ = evt_tx.send(DebugEvent::Error(format!("Continue failed: {}", e))); }
                                    }
                                }
                                Err(e) => { let _ = evt_tx.send(DebugEvent::Error(format!("Core access failed: {}", e))); }
                            }
                        }
                    }

                    DebugCommand::Step => {
                        if let Some(ref mut s) = session {
                            match s.core(0) {
                                Ok(mut core) => {
                                    match core.step() {
                                        Ok(_) => {
                                            send_registers(&mut core, &evt_tx);
                                        }
                                        Err(e) => { let _ = evt_tx.send(DebugEvent::Error(format!("Step failed: {}", e))); }
                                    }
                                }
                                Err(e) => { let _ = evt_tx.send(DebugEvent::Error(format!("Core access failed: {}", e))); }
                            }
                        }
                    }

                    DebugCommand::ReadRegisters => {
                        if let Some(ref mut s) = session {
                            match s.core(0) {
                                Ok(mut core) => { send_registers(&mut core, &evt_tx); }
                                Err(e) => { let _ = evt_tx.send(DebugEvent::Error(format!("Core access: {}", e))); }
                            }
                        }
                    }

                    DebugCommand::ReadMemory { addr, len } => {
                        if let Some(ref mut s) = session {
                            match s.core(0) {
                                Ok(mut core) => {
                                    let mut buf = vec![0u8; len];
                                    match core.read(addr, &mut buf) {
                                        Ok(_) => { let _ = evt_tx.send(DebugEvent::MemoryRead { addr, data: buf }); }
                                        Err(e) => { let _ = evt_tx.send(DebugEvent::Error(format!("Memory read failed: {}", e))); }
                                    }
                                }
                                Err(e) => { let _ = evt_tx.send(DebugEvent::Error(format!("Core access: {}", e))); }
                            }
                        }
                    }

                    DebugCommand::StartRtt { channel } => {
                        // Spawn a mock RTT reader thread that sends periodic messages
                        if rtt_worker.is_some() {
                            // already running
                            continue;
                        }
                        let (stop_tx, stop_rx) = crossbeam_channel::bounded::<()>(1);
                        rtt_stop_tx = Some(stop_tx.clone());
                        let evt_tx_clone = evt_tx.clone();
                        let handle = std::thread::spawn(move || {
                            let mut counter: u64 = 0;
                            while stop_rx.try_recv().is_err() {
                                std::thread::sleep(Duration::from_millis(300));
                                counter += 1;
                                let _ = evt_tx_clone.send(DebugEvent::RttData { channel, data: format!("Mock RTT line {}", counter) });
                            }
                        });
                        rtt_worker = Some(handle);
                    }

                    DebugCommand::StopRtt => {
                        if let Some(tx) = rtt_stop_tx.take() {
                            let _ = tx.send(());
                        }
                        if let Some(handle) = rtt_worker.take() {
                            let _ = handle.join();
                        }
                    }
                }
            }
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => {
                // no command; continue loop
                continue;
            }
            Err(_) => break,
        }
    }
}

fn send_registers(core: &mut probe_rs::Core, evt_tx: &Sender<DebugEvent>) {
    let mut regs = Vec::new();
    let reg_file = core.registers();

    // ARM register names
    let arm_reg_names = [
        ("R0", 0u16), ("R1", 1), ("R2", 2), ("R3", 3),
        ("R4", 4), ("R5", 5), ("R6", 6), ("R7", 7),
        ("R8", 8), ("R9", 9), ("R10", 10), ("R11", 11),
        ("R12", 12), ("SP", 13), ("LR", 14), ("PC", 15),
    ];

    for (name, _id) in &arm_reg_names {
        // Try to get register by name from the register file
        if let Some(reg) = reg_file.all_registers().find(|r| r.name() == *name) {
            match core.read_core_reg(reg) {
                Ok(val) => {
                    let raw: u64 = match val {
                        probe_rs::RegisterValue::U32(v) => v as u64,
                        probe_rs::RegisterValue::U64(v) => v,
                        probe_rs::RegisterValue::U128(v) => v as u64,
                    };
                    regs.push(RegisterValue { name: name.to_string(), raw });
                }
                Err(_) => {
                    regs.push(RegisterValue { name: name.to_string(), raw: 0 });
                }
            }
        }
    }

    // Also try xPSR
    if let Some(reg) = reg_file.all_registers().find(|r| r.name() == "XPSR" || r.name() == "xPSR") {
        if let Ok(val) = core.read_core_reg(reg) {
            let raw: u64 = match val {
                probe_rs::RegisterValue::U32(v) => v as u64,
                probe_rs::RegisterValue::U64(v) => v,
                probe_rs::RegisterValue::U128(v) => v as u64,
            };
            regs.push(RegisterValue { name: "xPSR".to_string(), raw });
        }
    }

    let _ = evt_tx.send(DebugEvent::Registers(regs));
}
