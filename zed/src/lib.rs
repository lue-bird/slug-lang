struct SlugZedExtension();

impl zed_extension_api::Extension for SlugZedExtension {
    fn new() -> Self {
        SlugZedExtension()
    }
    fn language_server_command(
        &mut self,
        _: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        if let Some(path) = worktree.which("slug") {
            Ok(zed_extension_api::Command::new(path).arg("lsp"))
        } else {
            Err("executable slug not found in the PATH environment".into())
        }
    }
}

zed_extension_api::register_extension!(SlugZedExtension);
