use std::cmp::Ordering;
use std::io::Write;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::Context;
use tracing::info;

use crate::shortcut_parser::DesktopFile;
use crate::utils::find_binary;

#[derive(Debug)]
pub enum EventType {
    MenuUp,
    MenuDown,
    Scroll(i32),
}

pub struct GuiState {
    gui_process: Option<Child>,
    gui_stdin: Option<ChildStdin>,
    highlight_idx: Option<usize>,
    idle_duration: Option<Instant>,
}

impl GuiState {
    pub fn new() -> Self {
        GuiState {
            gui_process: None,
            gui_stdin: None,
            highlight_idx: None,
            idle_duration: None,
        }
    }

    pub fn tick(
        &mut self,
        event: Option<EventType>,
        segments: usize,
        shortcut_files: &[DesktopFile],
    ) -> anyhow::Result<()> {
        match event {
            Some(event) => {
                self.handle_event(event, segments, shortcut_files)?;

                if let (Some(stdin), Some(idx)) = (self.gui_stdin.as_mut(), self.highlight_idx) {
                    writeln!(stdin, "HIGHLIGHT {}", idx).context("Failed to write GUI stdin")?;
                    stdin.flush().context("Failed to flush stdin")?;
                }
                Ok(())
            }

            None => {
                if let (Some(start), Some(stdin), Some(idx)) = (
                    self.idle_duration,
                    self.gui_stdin.as_mut(),
                    self.highlight_idx,
                ) {
                    if start.elapsed() > Duration::from_secs(1) {
                        writeln!(stdin, "QUIT").context("Failed to write GUI stdin")?;
                        self.idle_duration = None;
                        shortcut_files[idx].spawn_process()?;

                        if let Some(mut child) = self.gui_process.take() {
                            let status = child.wait().context("Failed to wait for GUI process")?;

                            match status.code() {
                                Some(code) => info!("GUI process exited with status code: {code}"),
                                None => info!("GUI process terminated by signal"),
                            }
                        }
                        self.gui_stdin = None;
                    }
                }
                Ok(())
            }
        }
    }

    pub fn handle_event(
        &mut self,
        event: EventType,
        segments: usize,
        shortcut_files: &[DesktopFile],
    ) -> anyhow::Result<()> {
        match event {
            EventType::MenuUp | EventType::MenuDown | EventType::Scroll(_)
                if self.gui_process.is_none() =>
            {
                let gui_exe_path = find_binary("gui");

                let mut cmd = Command::new(gui_exe_path);
                cmd.arg(segments.to_string());

                for desktop_file in shortcut_files {
                    if let Some(icon_path) = &desktop_file.icon {
                        cmd.arg(icon_path);
                    } else {
                        cmd.arg("default");
                    }
                }

                let mut child = cmd
                    .stdin(Stdio::piped())
                    .spawn()
                    .context("Failed to run GUI")?;

                self.idle_duration = Some(Instant::now());
                self.gui_stdin = Some(child.stdin.take().unwrap());
                self.gui_process = Some(child);

                self.highlight_idx = Some(match event {
                    EventType::MenuUp => segments - 1,
                    EventType::MenuDown => 0,
                    EventType::Scroll(d) if d < 0 => segments - 1,
                    EventType::Scroll(d) if d > 0 => 0,
                    _ => unreachable!(),
                })
            }

            EventType::MenuUp => {
                self.idle_duration = Some(Instant::now());
                self.highlight_idx = Some(match self.highlight_idx {
                    Some(val) => (val + 1) % segments,
                    None => 0,
                });
            }

            EventType::MenuDown => {
                self.idle_duration = Some(Instant::now());
                self.highlight_idx = Some(match self.highlight_idx {
                    Some(val) => (val + segments - 1) % segments,
                    None => segments - 1,
                });
            }

            EventType::Scroll(scroll_delta) => {
                self.idle_duration = Some(Instant::now());
                match scroll_delta.cmp(&0) {
                    Ordering::Greater => {
                        self.highlight_idx = Some(match self.highlight_idx {
                            Some(val) => (val + 1) % segments,
                            None => 0,
                        })
                    }
                    Ordering::Less => {
                        self.highlight_idx = Some(match self.highlight_idx {
                            Some(val) => (val + segments - 1) % segments,
                            None => segments - 1,
                        });
                    }
                    Ordering::Equal => {}
                }
            }
        }
        Ok(())
    }
}
