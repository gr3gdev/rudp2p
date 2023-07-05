use std::fmt::Display;

use crate::TypeEvent;

impl Display for TypeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeEvent::CONNECTED => write!(f, "{}", "CONNECTED"),
            TypeEvent::DISCONNECTED => write!(f, "{}", "DISCONNECTED"),
            TypeEvent::MESSAGE => write!(f, "{}", "MESSAGE"),
        }
    }
}
