// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ssh-server2: ssh server2 component model.
//!
//! Real implementation: servers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

/// SSH server: connections, authentication, session lifecycle.
pub mod sigmassh;

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SshServer2Error {
    EmptyServerName,
    DuplicateServer(String),
    EmptyEventSubject,
}

impl fmt::Display for SshServer2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyServerName => write!(f, "server name must not be empty"),
            Self::DuplicateServer(id) => write!(f, "duplicate server {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SshServer2Error {}

pub struct SshServer2 {
    pub name: String,
    servers: BTreeMap<String, Server>,
    events: BTreeMap<String, Event>,
}

impl SshServer2 {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            servers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_server(&mut self, item: Server) -> Result<(), SshServer2Error> {
        if item.name.trim().is_empty() {
            return Err(SshServer2Error::EmptyServerName);
        }
        let id = item.id.clone();
        if self.servers.contains_key(&id) {
            return Err(SshServer2Error::DuplicateServer(id.clone()));
        }
        self.servers.insert(id, item);
        Ok(())
    }

    pub fn server_count(&self) -> usize {
        self.servers.len()
    }

    pub fn get_server(&self, id: &str) -> Option<&Server> {
        self.servers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SshServer2Error> {
        if event.subject.trim().is_empty() {
            return Err(SshServer2Error::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_server(id: &str, name: &str) -> Server {
        Server {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SshServer2::new("SshServer2");
        assert_eq!(m.name, "SshServer2");
        assert_eq!(m.server_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_server() {
        let mut m = SshServer2::new("SshServer2");
        m.add_server(sample_server("a1", "Alpha")).unwrap();
        assert_eq!(m.server_count(), 1);
        assert!(m.get_server("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SshServer2::new("SshServer2");
        assert!(m.add_server(sample_server("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SshServer2::new("SshServer2");
        m.add_server(sample_server("x", "One")).unwrap();
        assert!(m.add_server(sample_server("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SshServer2::new("SshServer2");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
