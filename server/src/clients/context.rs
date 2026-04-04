use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ContextTeam {
    pub team: String,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ContextChannel {
    pub team: String,
    pub channel: String,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ContextThread {
    pub team: String,
    pub channel: String,
    pub thread: String,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Context {
    None,
    Team(ContextTeam),
    Channel(ContextChannel),
    Thread(ContextThread),
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Context::None => write!(f, "NONE"),
            Context::Team(_) => write!(f, "TEAM"),
            Context::Channel(_) => write!(f, "CHANNEL"),
            Context::Thread(_) => write!(f, "THREAD"),
        }
    }
}
