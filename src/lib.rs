use std::path::Path;
use zed_extension_api::{self as zed, Result};

const LANGUAGE_SERVER_ID: &str = "mojo-lsp-server";
const SERVER_NAME: &str = "mojo-lsp-server";
const DEFAULT_ARGS: &[&str] = &[];

struct MojoExtension;

impl MojoExtension {
    fn resolve_binary_path(
        binary_setting: Option<&str>,
        worktree: &zed::Worktree,
        env: &[(String, String)],
    ) -> Result<String> {
        let home = env
            .iter()
            .find(|(k, _)| k == "HOME")
            .map(|(_, v)| v.as_str());

        // 1. Explicit path specified by user in Zed settings
        if let Some(user_path) = binary_setting {
            let expanded_path = if let Some(stripped) = user_path.strip_prefix("~/") {
                if let Some(home_dir) = home {
                    format!("{home_dir}/{stripped}")
                } else {
                    user_path.to_string()
                }
            } else {
                user_path.to_string()
            };

            if expanded_path.contains('/') {
                return Ok(expanded_path);
            }

            return worktree.which(&expanded_path).ok_or_else(|| {
                format!("Configured Mojo LSP binary '{expanded_path}' not found in PATH")
            });
        }

        // 2. Look for binary in current PATH
        if let Some(path) = worktree.which(SERVER_NAME) {
            return Ok(path);
        }

        // 3. Check workspace root relative paths (Magic / Pixi / venv)
        let root = worktree.root_path();
        let workspace_candidates = [
            format!("{root}/.magic/envs/default/bin/{SERVER_NAME}"),
            format!("{root}/.pixi/envs/default/bin/{SERVER_NAME}"),
            format!("{root}/.venv/bin/{SERVER_NAME}"),
        ];
        for candidate in &workspace_candidates {
            if Path::new(candidate).is_file() {
                return Ok(candidate.clone());
            }
        }

        // 4. Check user home & system standard locations
        if let Some(home_dir) = home {
            let user_candidates = [
                format!("{home_dir}/.mojo/bin/{SERVER_NAME}"),
                format!("{home_dir}/.modular/pkg/packages.modular.com_mojo/bin/{SERVER_NAME}"),
                format!("{home_dir}/.magic/envs/default/bin/{SERVER_NAME}"),
                format!("{home_dir}/.local/bin/{SERVER_NAME}"),
            ];
            for candidate in &user_candidates {
                if Path::new(candidate).is_file() {
                    return Ok(candidate.clone());
                }
            }
        }

        // 5. Check common macOS / Linux system paths
        let system_candidates = [
            format!("/opt/homebrew/bin/{SERVER_NAME}"),
            format!("/usr/local/bin/{SERVER_NAME}"),
            format!("/usr/bin/{SERVER_NAME}"),
        ];
        for candidate in &system_candidates {
            if Path::new(candidate).is_file() {
                return Ok(candidate.clone());
            }
        }

        // 6. If not found, return an actionable error with troubleshooting guide
        Err(format!(
            "Could not locate '{SERVER_NAME}' in PATH or standard installation locations.\n\
             Searched:\n\
             - Worktree: .magic/envs/default/bin, .pixi/envs/default/bin, .venv/bin\n\
             - User home: ~/.mojo/bin, ~/.modular, ~/.magic, ~/.local/bin\n\
             - System: /opt/homebrew/bin, /usr/local/bin\n\n\
             Please install Mojo (via `curl -s https://get.modular.com | sh`) or specify the binary in Zed settings:\n\
             {{\n  \"lsp\": {{\n    \"mojo-lsp-server\": {{\n      \"binary\": {{ \"path\": \"/path/to/mojo-lsp-server\" }}\n    }}\n  }}\n}}"
        ))
    }
}

impl zed::Extension for MojoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let lsp_settings =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let binary = lsp_settings.binary;
        let mut args = DEFAULT_ARGS
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<_>>();
        let mut env = worktree.shell_env();

        if let Some(binary) = &binary {
            if let Some(arguments) = &binary.arguments {
                args.extend(arguments.clone());
            }

            if let Some(binary_env) = &binary.env {
                env.extend(binary_env.clone());
            }
        }

        let command = Self::resolve_binary_path(
            binary.as_ref().and_then(|b| b.path.as_deref()),
            worktree,
            &env,
        )?;

        // Prepend binary parent directory to PATH so child processes (like lldb) can be resolved
        if let Some(parent_dir) = Path::new(&command).parent().and_then(|p| p.to_str()) {
            if let Some((_, path_val)) = env.iter_mut().find(|(k, _)| k == "PATH") {
                if !path_val.split(':').any(|part| part == parent_dir) {
                    *path_val = format!("{parent_dir}:{path_val}");
                }
            } else {
                env.push(("PATH".to_string(), parent_dir.to_string()));
            }
        }

        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        if language_server_id.as_ref() != LANGUAGE_SERVER_ID {
            return Ok(None);
        }

        zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|settings| settings.initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        if language_server_id.as_ref() != LANGUAGE_SERVER_ID {
            return Ok(None);
        }

        zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|settings| settings.settings)
    }
}

zed::register_extension!(MojoExtension);
