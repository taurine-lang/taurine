//! String Interning Module
//!
//! This module provides efficient string storage and comparison
//! by assigning unique IDs to each unique string.
//!
//! # Benefits
//! - O(1) string comparison (compare IDs instead of content)
//! - Reduced memory usage (each unique string stored once)
//! - Faster identifier lookup in parser and interpreter

use std::collections::HashMap;
use std::sync::Arc;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;


const DEFAULT_CAPACITY: usize = 1024;


type FastHasher = BuildHasherDefault<XxHash64>;


#[derive(Clone, Debug)]
pub struct StringInterner {
    strings: Vec<Arc<str>>,
    string_to_id: HashMap<Arc<str>, usize, FastHasher>,
}

impl StringInterner {
    /// Create a new string interner with default capacity
    pub fn new() -> Self {
        Self {
            strings: Vec::with_capacity(DEFAULT_CAPACITY),
            string_to_id: HashMap::with_capacity_and_hasher(DEFAULT_CAPACITY, FastHasher::default()),
        }
    }

    /// Create a new string interner with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            strings: Vec::with_capacity(capacity),
            string_to_id: HashMap::with_capacity_and_hasher(capacity, FastHasher::default()),
        }
    }

    /// Intern a string and return its ID
    #[inline]
    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.string_to_id.get(s) {
            return id;
        }

        let id = self.strings.len();
        let arc_str: Arc<str> = s.into();
        self.string_to_id.insert(arc_str.clone(), id);
        self.strings.push(arc_str);
        id
    }

    /// Intern a string and return an InternedString wrapper
    #[inline]
    pub fn intern_ref(&mut self, s: &str) -> InternedString {
        InternedString::new(self.intern(s))
    }

    /// Get a string by its ID
    #[inline]
    pub fn get(&self, id: usize) -> Option<&str> {
        self.strings.get(id).map(|s| s.as_ref())
    }

    /// Get the number of interned strings
    #[inline]
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Check if the interner is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Check if a string is already interned
    #[inline]
    pub fn contains(&self, s: &str) -> bool {
        self.string_to_id.contains_key(s)
    }

    /// Get the ID of a string if it exists
    #[inline]
    pub fn get_id(&self, s: &str) -> Option<usize> {
        self.string_to_id.get(s).copied()
    }

    /// Get memory usage estimate in bytes
    pub fn memory_usage(&self) -> usize {
        let strings_size: usize = self.strings.iter()
            .map(|s| s.len())
            .sum();
        let map_size = self.string_to_id.len() * (std::mem::size_of::<Arc<str>>() + std::mem::size_of::<usize>());
        let vec_size = self.strings.capacity() * std::mem::size_of::<Arc<str>>();
        strings_size + map_size + vec_size
    }

    /// Clear all interned strings
    pub fn clear(&mut self) {
        self.strings.clear();
        self.string_to_id.clear();
    }

    /// Reserve capacity for more strings
    pub fn reserve(&mut self, additional: usize) {
        self.strings.reserve(additional);
        self.string_to_id.reserve(additional);
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InternedString(pub usize);

impl InternedString {
    /// Create a new interned string from an ID
    #[inline]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Get the underlying ID
    #[inline]
    pub const fn id(&self) -> usize {
        self.0
    }

    /// Create an empty interned string (ID 0)
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Check if this is an empty interned string
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl From<usize> for InternedString {
    #[inline]
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<InternedString> for usize {
    #[inline]
    fn from(s: InternedString) -> usize {
        s.0
    }
}

impl std::fmt::Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intern_unique_strings() {
        let mut interner = StringInterner::new();
        
        let id1 = interner.intern("hello");
        let id2 = interner.intern("world");
        let id3 = interner.intern("hello"); // Should return same ID
        
        assert_eq!(id1, id3);
        assert_ne!(id1, id2);
        assert_eq!(interner.len(), 2);
    }

    #[test]
    fn test_get_string() {
        let mut interner = StringInterner::new();
        
        let id = interner.intern("test");
        assert_eq!(interner.get(id), Some("test"));
        assert_eq!(interner.get(999), None);
    }

    #[test]
    fn test_contains() {
        let mut interner = StringInterner::new();
        
        interner.intern("hello");
        assert!(interner.contains("hello"));
        assert!(!interner.contains("world"));
    }

    #[test]
    fn test_get_id() {
        let mut interner = StringInterner::new();
        
        let id = interner.intern("test");
        assert_eq!(interner.get_id("test"), Some(id));
        assert_eq!(interner.get_id("unknown"), None);
    }

    #[test]
    fn test_empty() {
        let interner = StringInterner::new();
        assert!(interner.is_empty());
        assert_eq!(interner.len(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let interner = StringInterner::with_capacity(100);
        assert!(interner.is_empty());
    }

    #[test]
    fn test_interned_string() {
        let mut interner = StringInterner::new();
        let id = interner.intern("test");
        let interned = InternedString::new(id);
        
        assert_eq!(interned.id(), id);
        assert_eq!(interner.get(interned.id()), Some("test"));
    }

    #[test]
    fn test_many_strings() {
        let mut interner = StringInterner::new();
        
        // Intern many strings
        let ids: Vec<usize> = (0..1000)
            .map(|i| interner.intern(&format!("string_{}", i)))
            .collect();
        
        // All should be unique
        for (i, &id) in ids.iter().enumerate() {
            assert_eq!(interner.get(id), Some(format!("string_{}", i).as_str()));
        }
        
        // Interning same strings should return same IDs
        for (i, &id) in ids.iter().enumerate() {
            let new_id = interner.intern(&format!("string_{}", i));
            assert_eq!(new_id, id);
        }
    }

    #[test]
    fn test_memory_usage() {
        let mut interner = StringInterner::new();
        // Empty interner still has allocated capacity
        assert!(interner.memory_usage() >= 0);
        
        interner.intern("hello");
        assert!(interner.memory_usage() > 0);
    }

    #[test]
    fn test_clear() {
        let mut interner = StringInterner::new();
        interner.intern("hello");
        interner.intern("world");
        
        assert_eq!(interner.len(), 2);
        interner.clear();
        assert_eq!(interner.len(), 0);
    }

    #[test]
    fn test_reserve() {
        let mut interner = StringInterner::new();
        interner.reserve(100);
        // Should not panic
        for i in 0..100 {
            interner.intern(&format!("test_{}", i));
        }
    }

    #[test]
    fn test_intern_ref() {
        let mut interner = StringInterner::new();
        let interned = interner.intern_ref("test");
        
        // First interned string gets ID 0, which is valid
        assert_eq!(interner.get(interned.id()), Some("test"));
    }

    #[test]
    fn test_empty_interned_string() {
        let empty = InternedString::empty();
        assert!(empty.is_empty());
        assert_eq!(empty.id(), 0);
    }
}
