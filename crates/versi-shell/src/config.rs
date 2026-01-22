use crate::detect::{FnmShellOptions, ShellType};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    FileNotFound(PathBuf),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Shell type does not support config files")]
    UnsupportedShell,
}

pub struct ShellConfig {
    pub shell_type: ShellType,
    pub config_path: PathBuf,
    pub content: String,
}

impl ShellConfig {
    pub fn load(shell_type: ShellType, config_path: PathBuf) -> Result<Self, ConfigError> {
        let content = if config_path.exists() {
            fs::read_to_string(&config_path)?
        } else {
            String::new()
        };

        Ok(Self {
            shell_type,
            config_path,
            content,
        })
    }

    pub fn has_fnm_init(&self) -> bool {
        self.content.contains("fnm env")
    }

    pub fn detect_fnm_options(&self) -> Option<FnmShellOptions> {
        if !self.has_fnm_init() {
            return None;
        }

        Some(FnmShellOptions {
            use_on_cd: self.content.contains("--use-on-cd"),
            resolve_engines: self.content.contains("--resolve-engines"),
            corepack_enabled: self.content.contains("--corepack-enabled"),
        })
    }

    pub fn add_fnm_init(&mut self, options: &FnmShellOptions) -> ShellConfigEdit {
        let init_command = self.shell_type.fnm_init_command(options);

        if self.has_fnm_init() {
            return self.update_fnm_flags(options);
        }

        let addition = format!("\n# fnm (Fast Node Manager)\n{}\n", init_command);
        let modified = format!("{}{}", self.content, addition);

        ShellConfigEdit {
            original: self.content.clone(),
            modified,
            changes: vec![format!("Add fnm initialization: {}", init_command)],
        }
    }

    pub fn apply_edit(&mut self, edit: &ShellConfigEdit) -> Result<(), ConfigError> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.config_path, &edit.modified)?;
        self.content = edit.modified.clone();

        Ok(())
    }

    pub fn update_fnm_flags(&mut self, options: &FnmShellOptions) -> ShellConfigEdit {
        if !self.has_fnm_init() {
            return self.add_fnm_init(options);
        }

        let mut modified = self.content.clone();
        let mut changes = Vec::new();

        let flags = [
            ("--use-on-cd", options.use_on_cd),
            ("--resolve-engines", options.resolve_engines),
            ("--corepack-enabled", options.corepack_enabled),
        ];

        for (flag, enabled) in flags {
            let has_flag = modified.contains(flag);

            if enabled && !has_flag {
                modified = Self::add_flag_to_fnm_env(&modified, flag);
                changes.push(format!("Added {}", flag));
            } else if !enabled && has_flag {
                modified = Self::remove_flag_from_fnm_env(&modified, flag);
                changes.push(format!("Removed {}", flag));
            }
        }

        ShellConfigEdit {
            original: self.content.clone(),
            modified,
            changes,
        }
    }

    fn add_flag_to_fnm_env(content: &str, flag: &str) -> String {
        let mut result = String::new();
        for line in content.lines() {
            if line.contains("fnm env") && !line.contains(flag) {
                let modified_line = line.replacen("fnm env", &format!("fnm env {}", flag), 1);
                result.push_str(&modified_line);
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }
        if !content.ends_with('\n') && result.ends_with('\n') {
            result.pop();
        }
        result
    }

    fn remove_flag_from_fnm_env(content: &str, flag: &str) -> String {
        let mut result = String::new();
        for line in content.lines() {
            if line.contains("fnm env") && line.contains(flag) {
                let modified_line = line
                    .replace(&format!("{} ", flag), "")
                    .replace(&format!(" {}", flag), "")
                    .replace(flag, "");
                result.push_str(&modified_line);
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }
        if !content.ends_with('\n') && result.ends_with('\n') {
            result.pop();
        }
        result
    }
}

pub struct ShellConfigEdit {
    pub original: String,
    pub modified: String,
    pub changes: Vec<String>,
}

impl ShellConfigEdit {
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    pub fn diff_preview(&self) -> String {
        if !self.has_changes() {
            return "No changes needed.".to_string();
        }

        let mut preview = String::new();

        for change in &self.changes {
            preview.push_str(&format!("+ {}\n", change));
        }

        preview
    }
}
