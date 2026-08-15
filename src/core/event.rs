// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

#[allow(dead_code)]
pub enum BuildMsg {
    Started,
    Progress(String),
    Finished(super::build::compiler::BuildResult),
}

#[allow(dead_code)]
pub enum FlashMsg {
    Started,
    Progress(String),
    Finished(super::build::flasher::FlashResult),
}

#[allow(dead_code)]
pub enum SerialMsg {
    Line(String),
    Error(String),
    Connected,
    Disconnected,
}

#[allow(dead_code)]
pub enum ToolchainMsg {
    InstallStarted,
    InstallFinished(Result<super::build::toolchain::RustAnalyzerStatus, String>),
}

#[allow(dead_code)]
pub enum CoreEvent {
    Agent(super::agent::AgentEvent),
    Build(BuildMsg),
    Flash(FlashMsg),
    Serial(SerialMsg),
    VirtualBoard(super::simulator::VirtualBoardEvent),
    Toolchain(ToolchainMsg),
    BoardDetected(Option<super::board::detector::DetectedBoard>),
    LspCompletion(Vec<super::editor::lsp::CompletionItem>),
    LspDiagnostic(Vec<super::editor::lsp::Diagnostic>),
    LspInitialized,
    BuildAnalysis(super::build::analyzer::BuildStats),
    RttData { channel: u32, data: String },
    ElfAnalysis(super::inspect::elf::ElfInfo),
    StackAnalysis(super::inspect::stack::StackReport),
    Error(String),
}
