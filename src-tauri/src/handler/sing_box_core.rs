use arc_swap::ArcSwap;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{broadcast, mpsc};

// 进程状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    NotRunning,
    Starting,
    Running,
    StartFailed(Error),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NoPermission(String),
    Unknown(String),
}

// // 日志消息结构
// #[derive(Debug, Clone)]
// pub struct LogMessage {
//     pub timestamp: chrono::DateTime<chrono::Utc>,
//     pub content: String,
// }

type LogMessage = String;

// 进程管理器结构
pub struct ProcessManager {
    process: Arc<Mutex<Option<Child>>>,
    state: ArcSwap<ProcessState>,
    log_sender: broadcast::Sender<LogMessage>,
    program_path: String,
    program_args: Vec<String>,
}

impl ProcessManager {
    pub fn new(program_path: String, program_args: Vec<String>) -> Self {
        let (log_sender, _) = broadcast::channel(100);

        ProcessManager {
            process: Arc::new(Mutex::new(None)),
            state: ArcSwap::from_pointee(ProcessState::NotRunning),
            log_sender,
            program_path,
            program_args,
        }
    }

    // 启动外部程序
    pub async fn start(&self) -> Result<(), String> {
        match **self.state.load() {
            ProcessState::Starting | ProcessState::Running => {
                return Err("Process is already running".to_string());
            }
            _ => {}
        }

        let process_result = Command::new(&self.program_path)
            .args(&self.program_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match process_result {
            Ok(mut child) => {
                let stdout = child.stdout.take().unwrap();
                let stderr = child.stderr.take().unwrap();
                let stdout_sender = self.log_sender.clone();
                let stderr_sender = self.log_sender.clone();

                let mut rx = self.log_sender.subscribe();

                tokio::spawn(async move {
                    let mut reader = BufReader::new(stdout);
                    let mut line = String::new();
                    while let Ok(n) = reader.read_line(&mut line).await {
                        if n == 0 {
                            break;
                        }
                        stdout_sender.send(line.clone()).unwrap();
                        line.clear();
                    }
                });

                // 处理标准错误
                tokio::spawn(async move {
                    let mut reader = BufReader::new(stderr);
                    let mut line = String::new();
                    while let Ok(n) = reader.read_line(&mut line).await {
                        if n == 0 {
                            break;
                        }
                        stderr_sender.send(line.clone()).unwrap();
                        line.clear();
                    }
                });

                while let Ok(msg) = rx.recv().await {
                    if msg.contains("Access is denied") {
                        self.state
                            .store(Arc::new(ProcessState::StartFailed(Error::NoPermission(
                                msg,
                            ))));
                        break;
                    }

                    if msg.contains("sing-box started") {
                        self.state.store(Arc::new(ProcessState::Running));
                        break;
                    }

                    if msg.contains("FATAL") {
                        self.state
                            .store(Arc::new(ProcessState::StartFailed(Error::Unknown(msg))));
                        break;
                    }
                }

                Ok(())
            }
            Err(e) => {
                // self.state.store(Arc::new(ProcessState::StartFailed));
                Err(format!("Failed to start process: {}", e))
            }
        }
    }

    // 停止外部程序
    pub async fn stop(&self) -> Result<(), String> {
        let process_guard = self.process.lock().unwrap().take();
        if let Some(mut process_guard) = process_guard {
            match process_guard.kill().await {
                Ok(_) => {
                    self.state.store(Arc::new(ProcessState::NotRunning));
                    Ok(())
                }
                Err(e) => Err(format!("Failed to stop process: {}", e)),
            }
        } else {
            Ok(())
        }
    }

    pub fn subscrible_log(&self) -> broadcast::Receiver<LogMessage> {
        self.log_sender.subscribe()
    }

    // 获取当前状态
    pub fn get_state(&self) -> ProcessState {
        self.state.load_full().as_ref().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::mpsc::channel, time};

    #[tokio::test]
    async fn test_process_manager() {
        let manager = ProcessManager::new(
            r"C:\Users\chika\Documents\Soft\SingBox\sing-box.exe".to_string(),
            vec![
                "run".to_string(),
                "-c".to_string(),
                r"C:\Users\chika\Documents\Soft\SingBox\config.json".to_string(),
            ],
        );
        assert_eq!(manager.get_state(), ProcessState::NotRunning);

        let mut log = manager.subscrible_log();
        manager.start().await.unwrap();

        let timeout = time::Duration::from_secs(5);
        let now = time::Instant::now();
        loop {
            if let Ok(msg) = log.try_recv() {
                println!("{}", msg);
            }
            if now.elapsed() > timeout {
                break;
            }

            tokio::time::sleep(time::Duration::from_millis(100));
        }

        println!("{:?}", manager.get_state());
    }
}
