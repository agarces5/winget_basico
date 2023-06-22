use crate::command::WingetCommand;

pub trait Builder {
    type OutputType;
    fn with_id(self, id: &str) -> Self;
    fn with_version(self, version: &str) -> Self;
    fn build(self) -> Self::OutputType;
}

#[derive(Default, Clone)]
pub struct WingetCommandBuilder {
    id: String,
    version: Option<String>,
}

impl Builder for WingetCommandBuilder {
    type OutputType = WingetCommand;

    fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    fn with_version(mut self, version: &str) -> Self {
        self.version = Some(version.to_owned());
        self
    }
    fn build(self) -> WingetCommand {
        WingetCommand::new(self.id, self.version)
    }
}
