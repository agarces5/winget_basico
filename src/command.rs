use std::process::Command;

use crate::command_builder::WingetCommandBuilder;

pub struct WingetCommand {
    id: String,
    version: Option<String>,
}

impl WingetCommand {
    pub fn new(id: String, version: Option<String>) -> Self {
        Self { id, version }
    }

    pub fn builder() -> WingetCommandBuilder {
        WingetCommandBuilder::default()
    }

    pub fn execute(&self) {
        let id = format!("--id={}", &self.id);
        let mut args = vec![
            "install".to_string(),
            "-e".to_string(),
            "-h".to_string(),
            "--accept-package-agreements".to_string(),
            "--accept-source-agreements".to_string(),
            id.clone(),
        ];
        if let Some(ver) = self.version.clone() {
            args.push("-v".to_string());
            args.push(ver);
        }
        println!("\nIniciando la instalacion de {}\n", &id);
        Command::new("winget")
            .args(args)
            .status()
            .unwrap_or_else(|_| panic!("Failed to install {id}"));
    }
}
