// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use crate::core::event::CoreEvent;
use anyhow::{anyhow, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

const DEFAULT_AGENT_SETTINGS: &str = include_str!("../../agent_setting/agent.md");

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentModel {
    #[default]
    #[serde(rename = "codex-default")]
    CodexDefault,
    #[serde(rename = "gpt-5.6-sol")]
    Gpt56Sol,
    #[serde(rename = "gpt-5.6-terra")]
    Gpt56Terra,
    #[serde(rename = "gpt-5.6-luna")]
    Gpt56Luna,
}

impl AgentModel {
    pub const ALL: [Self; 4] = [
        Self::CodexDefault,
        Self::Gpt56Sol,
        Self::Gpt56Terra,
        Self::Gpt56Luna,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::CodexDefault => "Codex（既定）",
            Self::Gpt56Sol => "GPT-5.6 Sol",
            Self::Gpt56Terra => "GPT-5.6 Terra",
            Self::Gpt56Luna => "GPT-5.6 Luna",
        }
    }

    pub const fn model_id(self) -> Option<&'static str> {
        match self {
            Self::CodexDefault => None,
            Self::Gpt56Sol => Some("gpt-5.6-sol"),
            Self::Gpt56Terra => Some("gpt-5.6-terra"),
            Self::Gpt56Luna => Some("gpt-5.6-luna"),
        }
    }
}

pub struct AgentRequest {
    pub workspace: PathBuf,
    pub context: String,
    pub prompt: String,
    pub allow_edits: bool,
    pub model: AgentModel,
}

pub enum AgentEvent {
    Started,
    Output(String),
    Finished(Result<()>),
}

pub fn agent_settings_path() -> Result<PathBuf> {
    let executable = std::env::current_exe().context("failed to locate the ALLoIDE executable")?;
    Ok(executable
        .parent()
        .context("the ALLoIDE executable has no parent directory")?
        .join("agent_setting")
        .join("agent.md"))
}

pub fn ensure_agent_settings() -> Result<PathBuf> {
    ensure_agent_settings_at(&agent_settings_path()?)
}

pub fn run_async(req: AgentRequest, tx: crossbeam_channel::Sender<CoreEvent>) {
    std::thread::spawn(move || {
        tx.send(CoreEvent::Agent(AgentEvent::Started)).ok();
        let result = run(req, &tx);
        tx.send(CoreEvent::Agent(AgentEvent::Finished(result))).ok();
    });
}

pub fn login_async(tx: crossbeam_channel::Sender<CoreEvent>) {
    std::thread::spawn(move || {
        tx.send(CoreEvent::Agent(AgentEvent::Started)).ok();
        let result = login(&tx);
        tx.send(CoreEvent::Agent(AgentEvent::Finished(result))).ok();
    });
}

fn run(req: AgentRequest, tx: &crossbeam_channel::Sender<CoreEvent>) -> Result<()> {
    let user_prompt = req.prompt.trim();
    ensure!(!user_prompt.is_empty(), "agent prompt is empty");
    ensure!(
        req.workspace.is_dir(),
        "agent workspace is not a directory: {}",
        req.workspace.display()
    );
    let settings_path = ensure_agent_settings()?;
    let instructions = read_agent_settings(&settings_path)?;
    let prompt = compose_prompt(&instructions, &req.context, user_prompt);

    let mut command = agent_command(&req)?;
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().context("failed to start `codex exec`")?;

    let (stdout, stderr) = capture_output(&mut child, tx)?;

    let stdin_result = (|| -> Result<()> {
        let mut stdin = child.stdin.take().context("failed to open codex stdin")?;
        stdin
            .write_all(prompt.as_bytes())
            .context("failed to send prompt to codex")
    })();
    if stdin_result.is_err() {
        let _ = child.kill();
    }

    let status = child.wait().context("failed to wait for codex");
    let stdout_result = join_reader(stdout, "stdout");
    let stderr_result = join_reader(stderr, "stderr");

    stdin_result?;
    let status = status?;
    stdout_result?;
    stderr_result?;
    ensure!(status.success(), "`codex exec` exited with {status}");
    Ok(())
}

fn compose_prompt(instructions: &str, context: &str, user_prompt: &str) -> String {
    format!(
        "{}\n\n## ALLoIDE runtime context\n\n{}\n\n## User request\n\n{}",
        instructions.trim(),
        context.trim(),
        user_prompt
    )
}

fn ensure_agent_settings_at(path: &Path) -> Result<PathBuf> {
    fs::create_dir_all(
        path.parent()
            .context("the agent settings path has no parent directory")?,
    )
    .with_context(|| {
        format!(
            "failed to create agent settings directory for {}",
            path.display()
        )
    })?;

    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => file
            .write_all(DEFAULT_AGENT_SETTINGS.as_bytes())
            .with_context(|| {
                format!(
                    "failed to write default agent settings to {}",
                    path.display()
                )
            })?,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => {
            return Err(error)
                .with_context(|| format!("failed to create agent settings at {}", path.display()))
        }
    }

    read_agent_settings(path)?;
    Ok(path.to_path_buf())
}

fn read_agent_settings(path: &Path) -> Result<String> {
    let settings = fs::read_to_string(path)
        .with_context(|| format!("failed to read agent settings from {}", path.display()))?;
    ensure!(
        !settings.trim().is_empty(),
        "agent settings are empty: {}",
        path.display()
    );
    Ok(settings)
}

fn login(tx: &crossbeam_channel::Sender<CoreEvent>) -> Result<()> {
    let mut command = login_command()?;
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().context("failed to start `codex login`")?;
    let (stdout, stderr) = capture_output(&mut child, tx)?;

    let status = child.wait().context("failed to wait for codex login")?;
    join_reader(stdout, "stdout")?;
    join_reader(stderr, "stderr")?;
    ensure!(status.success(), "`codex login` exited with {status}");
    Ok(())
}

fn agent_command(req: &AgentRequest) -> Result<Command> {
    Ok(agent_command_with(&codex_executable()?, req))
}

fn agent_command_with(executable: &Path, req: &AgentRequest) -> Command {
    let mut command = Command::new(executable);
    crate::core::no_window(&mut command);
    command.arg("exec");
    if let Some(model) = req.model.model_id() {
        command.args(["--model", model]);
    }
    command
        .arg("--ephemeral")
        .arg("--color")
        .arg("never")
        .arg("--sandbox")
        .arg(if req.allow_edits {
            "workspace-write"
        } else {
            "read-only"
        })
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(&req.workspace)
        .arg("-");
    command
}

fn login_command() -> Result<Command> {
    Ok(login_command_with(&codex_executable()?))
}

fn login_command_with(executable: &Path) -> Command {
    let mut command = Command::new(executable);
    crate::core::no_window(&mut command);
    command.arg("login");
    command
}

fn codex_executable() -> Result<PathBuf> {
    #[cfg(windows)]
    {
        if let Ok(path) = which::which("codex.exe") {
            return Ok(path);
        }

        let path = npm_codex_executable(
            &dirs::data_dir().context("failed to locate the user data directory")?,
        )?;
        ensure!(
            path.is_file(),
            "Codex CLI executable was not found; install Codex and restart ALLoIDE"
        );
        Ok(path)
    }

    #[cfg(not(windows))]
    {
        which::which("codex").context("Codex CLI executable was not found in PATH")
    }
}

#[cfg(windows)]
fn npm_codex_executable(data_dir: &Path) -> Result<PathBuf> {
    let (package_arch, target_arch) = match std::env::consts::ARCH {
        "x86_64" => ("x64", "x86_64"),
        "aarch64" => ("arm64", "aarch64"),
        arch => return Err(anyhow!("unsupported Windows architecture: {arch}")),
    };
    Ok(data_dir
        .join("npm")
        .join("node_modules")
        .join("@openai")
        .join("codex")
        .join("node_modules")
        .join("@openai")
        .join(format!("codex-win32-{package_arch}"))
        .join("vendor")
        .join(format!("{target_arch}-pc-windows-msvc"))
        .join("bin")
        .join("codex.exe"))
}

type Reader = std::thread::JoinHandle<Result<()>>;

fn capture_output(
    child: &mut Child,
    tx: &crossbeam_channel::Sender<CoreEvent>,
) -> Result<(Reader, Reader)> {
    Ok((
        stream_output(
            child
                .stdout
                .take()
                .context("failed to capture codex stdout")?,
            tx.clone(),
            "stdout",
        ),
        stream_output(
            child
                .stderr
                .take()
                .context("failed to capture codex stderr")?,
            tx.clone(),
            "stderr",
        ),
    ))
}

fn stream_output(
    stream: impl Read + Send + 'static,
    tx: crossbeam_channel::Sender<CoreEvent>,
    name: &'static str,
) -> std::thread::JoinHandle<Result<()>> {
    std::thread::spawn(move || {
        for line in BufReader::new(stream).lines() {
            tx.send(CoreEvent::Agent(AgentEvent::Output(
                line.with_context(|| format!("failed to read codex {name}"))?,
            )))
            .ok();
        }
        Ok(())
    })
}

fn join_reader(handle: std::thread::JoinHandle<Result<()>>, name: &str) -> Result<()> {
    handle
        .join()
        .map_err(|_| anyhow!("codex {name} reader panicked"))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn command_uses_requested_sandbox() {
        let args = |allow_edits| {
            let req = AgentRequest {
                workspace: PathBuf::from("workspace"),
                context: String::new(),
                prompt: "test".into(),
                allow_edits,
                model: AgentModel::default(),
            };
            agent_command_with(Path::new("codex.exe"), &req)
                .get_args()
                .map(OsString::from)
                .collect::<Vec<_>>()
        };

        let expected = |sandbox| {
            [
                "exec",
                "--ephemeral",
                "--color",
                "never",
                "--sandbox",
                sandbox,
                "--skip-git-repo-check",
                "-C",
                "workspace",
                "-",
            ]
            .map(OsString::from)
            .to_vec()
        };

        assert_eq!(args(false), expected("read-only"));
        assert_eq!(args(true), expected("workspace-write"));
    }

    #[test]
    fn command_uses_requested_model_and_omits_default() {
        let args = |model| {
            let req = AgentRequest {
                workspace: PathBuf::from("workspace"),
                context: String::new(),
                prompt: "test".into(),
                allow_edits: false,
                model,
            };
            agent_command_with(Path::new("codex.exe"), &req)
                .get_args()
                .map(OsString::from)
                .collect::<Vec<_>>()
        };

        assert!(!args(AgentModel::CodexDefault).contains(&OsString::from("--model")));
        assert_eq!(
            &args(AgentModel::Gpt56Sol)[..3],
            ["exec", "--model", "gpt-5.6-sol"].map(OsString::from)
        );
    }

    #[test]
    fn prompt_contains_instructions_context_and_request() {
        assert_eq!(
            compose_prompt(
                DEFAULT_AGENT_SETTINGS,
                "board: micro:bit v2",
                "inspect the firmware"
            ),
            format!(
                "{}\n\n## ALLoIDE runtime context\n\nboard: micro:bit v2\n\n## User request\n\ninspect the firmware",
                DEFAULT_AGENT_SETTINGS.trim()
            )
        );
    }

    #[test]
    fn settings_are_created_once_and_external_edits_are_loaded() {
        let directory = std::env::temp_dir().join(format!(
            "alloide-agent-settings-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock is after Unix epoch")
                .as_nanos()
        ));
        let path = directory.join("agent_setting").join("agent.md");

        assert_eq!(
            read_agent_settings(&ensure_agent_settings_at(&path).expect("create settings"))
                .expect("read settings"),
            DEFAULT_AGENT_SETTINGS
        );
        fs::write(&path, "custom instructions").expect("edit settings");
        assert_eq!(
            read_agent_settings(&ensure_agent_settings_at(&path).expect("keep settings"))
                .expect("read edited settings"),
            "custom instructions"
        );

        fs::remove_dir_all(directory).expect("remove test settings");
    }

    #[test]
    fn login_command_uses_browser_login() {
        assert_eq!(
            login_command_with(Path::new("codex.exe"))
                .get_args()
                .map(OsString::from)
                .collect::<Vec<_>>(),
            ["login"].map(OsString::from).to_vec()
        );
    }

    #[cfg(windows)]
    #[test]
    fn npm_codex_path_uses_native_binary() {
        let path = npm_codex_executable(Path::new("data")).expect("supported architecture");
        assert!(path.ends_with(Path::new("bin").join("codex.exe")));
        assert!(path.to_string_lossy().contains("codex-win32-"));
    }
}
