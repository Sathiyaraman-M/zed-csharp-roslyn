use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

const CACHE_BINARY: &str = "/Users/sathyar/Developer/Configuration/roslyn-language-server";

struct CsharpRoslynExtension;

impl zed::Extension for CsharpRoslynExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        Ok(Command {
            command: CACHE_BINARY.to_string(),
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(CsharpRoslynExtension);
