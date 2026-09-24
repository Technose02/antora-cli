use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct LogDestination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    All,
    Debug,
    Error,
    Fatal,
    Info,
    Silent,
    Warn,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Pretty,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LogLevelFormat {
    Label,
    Number,
}

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct Log {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_level: Option<LogLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<LogFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<LogLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_format: Option<LogLevelFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<LogDestination>,
}

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct RuntimeSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Log>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
}

// region:      --- Builder(s)

#[derive(Default)]
pub struct LogDestinationBuilder {
    value: LogDestination,
}

#[expect(unused)]
impl LogDestinationBuilder {
    pub fn new() -> Self {
        LogDestinationBuilder {
            value: LogDestination::default(),
        }
    }

    pub fn init(log_destination: LogDestination) -> Self {
        LogDestinationBuilder {
            value: log_destination,
        }
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.value.file = Some(file.into());
        self
    }

    pub fn append(mut self, append: bool) -> Self {
        self.value.append = Some(append);
        self
    }

    pub fn buffer_size(mut self, buffer_size: u32) -> Self {
        self.value.buffer_size = Some(buffer_size);
        self
    }

    pub fn sync(mut self, sync: bool) -> Self {
        self.value.sync = Some(sync);
        self
    }

    pub fn build(self) -> LogDestination {
        self.value
    }
}

#[derive(Default)]
pub struct LogBuilder {
    value: Log,
}

impl LogBuilder {
    pub fn new() -> Self {
        LogBuilder {
            value: Log::default(),
        }
    }

    pub fn init(log: Log) -> Self {
        LogBuilder { value: log }
    }

    pub fn failure_level(mut self, failure_level: LogLevel) -> Self {
        self.value.failure_level = Some(failure_level);
        self
    }

    pub fn format(mut self, format: LogFormat) -> Self {
        self.value.format = Some(format);
        self
    }

    pub fn level(mut self, level: LogLevel) -> Self {
        self.value.level = Some(level);
        self
    }

    pub fn level_format(mut self, level_format: LogLevelFormat) -> Self {
        self.value.level_format = Some(level_format);
        self
    }

    pub fn destination(mut self, destination: impl Into<LogDestination>) -> Self {
        self.value.destination = Some(destination.into());
        self
    }

    pub fn build(self) -> Log {
        self.value
    }
}

#[derive(Default)]
pub struct RuntimeSectionBuilder {
    value: RuntimeSection,
}

impl RuntimeSectionBuilder {
    pub fn new() -> Self {
        RuntimeSectionBuilder {
            value: RuntimeSection::default(),
        }
    }

    pub fn init(runtime_section: RuntimeSection) -> Self {
        RuntimeSectionBuilder {
            value: runtime_section,
        }
    }

    pub fn cache_dir(mut self, cache_dir: impl Into<String>) -> Self {
        self.value.cache_dir = Some(cache_dir.into());
        self
    }

    pub fn fetch(mut self, fetch: bool) -> Self {
        self.value.fetch = Some(fetch);
        self
    }

    pub fn log(mut self, log: impl Into<Log>) -> Self {
        self.value.log = Some(log.into());
        self
    }

    pub fn quiet(mut self, quiet: bool) -> Self {
        self.value.quiet = Some(quiet);
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.value.silent = Some(silent);
        self
    }

    pub fn build(self) -> RuntimeSection {
        self.value
    }
}

// endregion:   --- Builder(s)
