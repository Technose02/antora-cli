use std::{fmt::Debug, sync::Arc};

#[derive(Clone)]
pub struct LineProcessor(Arc<dyn Fn(String) -> String + Send + Sync>);
impl Debug for LineProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LineProcessor(Arc<dyn Fn(String) -> String + Send + Sync>)"
        )
    }
}
impl LineProcessor {
    pub fn new(op: impl Fn(String) -> String + Send + Sync + 'static) -> Self {
        Self(Arc::new(op))
    }
    pub fn run(&self, line: String) -> String {
        self.0(line)
    }
}

pub fn no_op_lineprocessor() -> LineProcessor {
    LineProcessor::new(move |line| line)
}
