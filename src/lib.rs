// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-etc-group: etc group component model.
//!
//! Real implementation: groups, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
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
pub enum EtcGroupError {
    EmptyGroupName,
    DuplicateGroup(String),
    EmptyEventSubject,
}

impl fmt::Display for EtcGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGroupName => write!(f, "group name must not be empty"),
            Self::DuplicateGroup(id) => write!(f, "duplicate group {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for EtcGroupError {}

pub struct EtcGroup {
    pub name: String,
    groups: BTreeMap<String, Group>,
    events: BTreeMap<String, Event>,
}

impl EtcGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            groups: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_group(&mut self, item: Group) -> Result<(), EtcGroupError> {
        if item.name.trim().is_empty() {
            return Err(EtcGroupError::EmptyGroupName);
        }
        let id = item.id.clone();
        if self.groups.contains_key(&id) {
            return Err(EtcGroupError::DuplicateGroup(id.clone()));
        }
        self.groups.insert(id, item);
        Ok(())
    }

    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    pub fn get_group(&self, id: &str) -> Option<&Group> {
        self.groups.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), EtcGroupError> {
        if event.subject.trim().is_empty() {
            return Err(EtcGroupError::EmptyEventSubject);
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

    fn sample_group(id: &str, name: &str) -> Group {
        Group {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = EtcGroup::new("EtcGroup");
        assert_eq!(m.name, "EtcGroup");
        assert_eq!(m.group_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_group() {
        let mut m = EtcGroup::new("EtcGroup");
        m.add_group(sample_group("a1", "Alpha")).unwrap();
        assert_eq!(m.group_count(), 1);
        assert!(m.get_group("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = EtcGroup::new("EtcGroup");
        assert!(m.add_group(sample_group("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = EtcGroup::new("EtcGroup");
        m.add_group(sample_group("x", "One")).unwrap();
        assert!(m.add_group(sample_group("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = EtcGroup::new("EtcGroup");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
