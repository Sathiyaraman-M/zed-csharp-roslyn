use std::env;

use zed_extension_api::{self as zed};
use zed_extension_api::{Command, LanguageServerId, Result, Worktree};

const CACHE_BINARY_DLL: &str = "/Users/sathyar/Developer/Configuration/Roslyn/content/LanguageServer/neutral/Microsoft.CodeAnalysis.LanguageServer.dll";

struct CsharpRoslynExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for CsharpRoslynExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: Some(CACHE_BINARY_DLL.to_string()),
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let cached_binary_path = self
            .cached_binary_path
            .as_ref()
            .ok_or("Cached binary path not found")?;
        let dotnet_command =
            env::var("DOTNET_ROOT").unwrap_or("/usr/local/share/dotnet".to_string());
        let extension_log_directory = format!("{}{}", worktree.root_path(), "/.zed/csharp-roslyn");
        let command = Command {
            command: dotnet_command,
            args: vec![
                cached_binary_path.to_string(),
                "--stdio".to_string(),
                "--logLevel".to_string(),
                "Information".to_string(),
                "--extensionLogDirectory".to_string(),
                extension_log_directory,
            ],
            env: Default::default(),
        };
        Ok(command)
    }
}

zed::register_extension!(CsharpRoslynExtension);
