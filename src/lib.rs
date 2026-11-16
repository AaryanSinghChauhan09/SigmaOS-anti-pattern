//! SigmaOS-anti-pattern: Anti-pattern detection model.
//!
//! Real implementation: patterns, detections, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detection {
    pub id: String,
    pub pattern_id: String,
    pub location: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AntiPatternError {
    EmptyPatternName,
    DuplicatePattern(String),
}

impl fmt::Display for AntiPatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPatternName => write!(f, "pattern name must not be empty"),
            Self::DuplicatePattern(id) => write!(f, "duplicate pattern {:?}", id),
        }
    }
}

impl std::error::Error for AntiPatternError {}

pub struct AntiPattern {
    pub name: String,
    patterns: BTreeMap<String, Pattern>,
    detections: BTreeMap<String, Detection>,
}

impl AntiPattern {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), patterns: BTreeMap::new(), detections: BTreeMap::new() }
    }

    pub fn add_pattern(&mut self, p: Pattern) -> Result<(), AntiPatternError> {
        if p.name.trim().is_empty() { return Err(AntiPatternError::EmptyPatternName); }
        let id = p.id.clone();
        if self.patterns.contains_key(&id) {
            return Err(AntiPatternError::DuplicatePattern(id.clone()));
        }
        self.patterns.insert(id, p);
        Ok(())
    }

    pub fn pattern_count(&self) -> usize { self.patterns.len() }

    pub fn get_pattern(&self, id: &str) -> Option<&Pattern> { self.patterns.get(id) }

    pub fn add_detection(&mut self, d: Detection) {
        self.detections.insert(d.id.clone(), d);
    }

    pub fn get_detection(&self, id: &str) -> Option<&Detection> { self.detections.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = AntiPattern::new("AntiPattern");
        assert_eq!(a.name, "AntiPattern");
        assert_eq!(a.pattern_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = AntiPattern::new("AntiPattern");
        let pattern = Pattern { id: "p1".into(), name: "GodObject".into(), description: "Single class doing everything".into() };
        a.add_pattern(pattern).unwrap();
        assert_eq!(a.pattern_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = AntiPattern::new("AntiPattern");
        let pattern = Pattern { id: "x".into(), name: "".into(), description: "x".into() };
        assert!(a.add_pattern(pattern).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AntiPattern::new("AntiPattern");
        let pattern = Pattern { id: "x".into(), name: "X".into(), description: "x".into() };
        a.add_pattern(pattern).unwrap();
        let pattern2 = Pattern { id: "x".into(), name: "X".into(), description: "x".into() };
        assert!(a.add_pattern(pattern2).is_err());
    }
}
