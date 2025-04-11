use std::fs;

use zed::LanguageServerInstallationStatus;
use zed_extension_api::{
    self as zed, settings::LspSettings, Architecture, Command, DownloadedFileType,
    GithubReleaseOptions, LanguageServerId, Os, Result, Worktree,
};

struct RoslynLspBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct CsharpRoslynExtension {
    cached_binary_path: Option<String>,
}

impl CsharpRoslynExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<RoslynLspBinary> {
        let binary_settings = LspSettings::for_worktree("roslyn", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(RoslynLspBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("roslyn") {
            return Ok(RoslynLspBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(RoslynLspBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "SofusA/csharp-language-server",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let release_name = self.binary_release_name();
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == release_name)
            .ok_or_else(|| format!("Not found: {}", release_name))?;

        let version_dir = format!("csharp-lsp-{}", release.version);
        let binary_path = format!("{version_dir}/csharp-language-server");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match zed::current_platform().0 {
                    Os::Windows => DownloadedFileType::Zip,
                    _ => DownloadedFileType::GzipTar,
                },
            )
            .map_err(|err| format!("Failed to download file: {}", err))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());

        Ok(RoslynLspBinary {
            path: binary_path,
            args: binary_args,
        })
    }

    fn binary_release_name(&self) -> String {
        let (platform, architecture) = zed::current_platform();
        let suffix = match platform {
            Os::Mac => match architecture {
                Architecture::Aarch64 => "aarch64-apple-darwin".to_owned(),
                _ => "x86_64-apple-darwin".to_owned(),
            },
            Os::Linux => "x86_64-unknown-linux-gnu".to_owned(),
            Os::Windows => "x86_64-pc-windows-msvc".to_owned(),
        };
        let extension = match platform {
            Os::Windows => "zip",
            _ => "tar.gz",
        };
        format!("csharp-language-server-{suffix}.{extension}")
    }
}

impl zed::Extension for CsharpRoslynExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(Command {
            command: binary.path,
            args: binary.args.unwrap_or(vec![]),
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(CsharpRoslynExtension);
