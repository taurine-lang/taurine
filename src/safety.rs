//! Security Module for Taurine
//!
//! This module provides security features for safe script execution:
//! - Sandboxing capabilities
//! - Input validation
//! - Resource limits
//! - Dangerous operation prevention
//!
//! # Safety Limits
//!
//! The original SafetyLimits struct is preserved for backward compatibility:
//! - Recursion depth limits
//! - Memory limits for tables/arrays
//! - Execution timeout
//! - Panic handling for FFI

use std::collections::HashSet;
use std::time::Duration;

// ============================================================================
// Legacy Safety Limits (for backward compatibility)
// ============================================================================

pub const DEFAULT_MAX_RECURSION_DEPTH: usize = 1000;
pub const DEFAULT_MAX_MEMORY_BYTES: usize = 256 * 1024 * 1024;
pub const DEFAULT_MAX_COLLECTION_SIZE: usize = 10_000_000;
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

#[derive(Clone, Debug)]
pub struct SafetyLimits {
    pub max_recursion_depth: usize,
    pub max_memory_bytes: usize,
    pub max_collection_size: usize,
    pub timeout: Option<Duration>,
}

impl Default for SafetyLimits {
    fn default() -> Self {
        Self {
            max_recursion_depth: DEFAULT_MAX_RECURSION_DEPTH,
            max_memory_bytes: DEFAULT_MAX_MEMORY_BYTES,
            max_collection_size: DEFAULT_MAX_COLLECTION_SIZE,
            timeout: Some(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS)),
        }
    }
}

impl SafetyLimits {
    pub fn builder() -> SafetyLimitsBuilder {
        SafetyLimitsBuilder::new()
    }
}

pub struct SafetyLimitsBuilder {
    limits: SafetyLimits,
}

impl SafetyLimitsBuilder {
    pub fn new() -> Self {
        Self {
            limits: SafetyLimits::default(),
        }
    }

    pub fn max_recursion_depth(mut self, depth: usize) -> Self {
        self.limits.max_recursion_depth = depth;
        self
    }

    pub fn max_memory_bytes(mut self, bytes: usize) -> Self {
        self.limits.max_memory_bytes = bytes;
        self
    }

    pub fn max_collection_size(mut self, size: usize) -> Self {
        self.limits.max_collection_size = size;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.limits.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> SafetyLimits {
        self.limits
    }
}

impl Default for SafetyLimitsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Safety Context (legacy)
// ============================================================================

pub struct SafetyContext {
    limits: SafetyLimits,
    interrupted: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl SafetyContext {
    pub fn new(limits: SafetyLimits) -> Self {
        Self {
            limits,
            interrupted: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub fn interrupt(&self) {
        self.interrupted.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn reset(&self) {
        self.interrupted.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_interrupted(&self) -> bool {
        self.interrupted.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn safety_check(&self) -> Result<(), String> {
        if self.is_interrupted() {
            return Err("Execution interrupted".to_string());
        }
        Ok(())
    }

    pub fn limits(&self) -> &SafetyLimits {
        &self.limits
    }
}

// ============================================================================
// Security Levels
// ============================================================================

/// Security level for script execution
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Full access - no restrictions
    Full,
    /// Standard - basic restrictions
    Standard,
    /// Restricted - limited file/network access
    Restricted,
    /// Sandbox - isolated execution
    Sandbox,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        SecurityLevel::Standard
    }
}

// ============================================================================
// Permissions
// ============================================================================

/// Granular permissions for script execution
#[derive(Clone, Debug, Default)]
pub struct Permissions {
    /// Allow file system access
    pub allow_fs: bool,
    /// Allow network access
    pub allow_network: bool,
    /// Allow environment variable access
    pub allow_env: bool,
    /// Allow process execution
    pub allow_process: bool,
    /// Allow native function calls
    pub allow_native: bool,
    /// Allowed file paths (empty = all)
    pub allowed_paths: HashSet<String>,
    /// Allowed network hosts (empty = all)
    pub allowed_hosts: HashSet<String>,
    /// Maximum file size for I/O
    pub max_file_size: usize,
    /// Maximum network payload size
    pub max_network_size: usize,
}

impl Permissions {
    /// Create permissive permissions (full access)
    pub fn full() -> Self {
        Self {
            allow_fs: true,
            allow_network: true,
            allow_env: true,
            allow_process: true,
            allow_native: true,
            allowed_paths: HashSet::new(),
            allowed_hosts: HashSet::new(),
            max_file_size: usize::MAX,
            max_network_size: usize::MAX,
        }
    }

    /// Create restricted permissions
    pub fn restricted() -> Self {
        Self {
            allow_fs: true,
            allow_network: false,
            allow_env: false,
            allow_process: false,
            allow_native: true,
            allowed_paths: HashSet::new(),
            allowed_hosts: HashSet::new(),
            max_file_size: 10 * 1024 * 1024, // 10 MB
            max_network_size: 1 * 1024 * 1024, // 1 MB
        }
    }

    /// Create sandbox permissions (isolated)
    pub fn sandbox() -> Self {
        Self {
            allow_fs: false,
            allow_network: false,
            allow_env: false,
            allow_process: false,
            allow_native: false,
            allowed_paths: HashSet::new(),
            allowed_hosts: HashSet::new(),
            max_file_size: 0,
            max_network_size: 0,
        }
    }

    /// Check if path is allowed
    pub fn is_path_allowed(&self, path: &str) -> bool {
        if !self.allow_fs {
            return false;
        }
        if self.allowed_paths.is_empty() {
            return true;
        }
        self.allowed_paths.iter().any(|allowed| path.starts_with(allowed))
    }

    /// Check if host is allowed
    pub fn is_host_allowed(&self, host: &str) -> bool {
        if !self.allow_network {
            return false;
        }
        if self.allowed_hosts.is_empty() {
            return true;
        }
        self.allowed_hosts.iter().any(|allowed| host.contains(allowed))
    }
}

// ============================================================================
// Security Context
// ============================================================================

/// Security context for script execution
pub struct SecurityContext {
    /// Security level
    level: SecurityLevel,
    /// Permissions
    permissions: Permissions,
    /// Dangerous functions that are blocked
    blocked_functions: HashSet<String>,
    /// Execution timeout
    timeout: Option<Duration>,
    /// Maximum memory usage
    max_memory: usize,
    /// Maximum recursion depth
    max_recursion: usize,
}

impl SecurityContext {
    /// Create new security context with default settings
    pub fn new() -> Self {
        Self {
            level: SecurityLevel::default(),
            permissions: Permissions::default(),
            blocked_functions: Self::default_blocked_functions(),
            timeout: Some(Duration::from_secs(30)),
            max_memory: 256 * 1024 * 1024,
            max_recursion: 1000,
        }
    }

    /// Create security context with specified level
    pub fn with_level(level: SecurityLevel) -> Self {
        let mut ctx = Self::new();
        ctx.level = level.clone();
        ctx.apply_level(&level);
        ctx
    }

    /// Create security context with custom permissions
    pub fn with_permissions(permissions: Permissions) -> Self {
        Self {
            permissions,
            ..Self::new()
        }
    }

    /// Get default blocked functions
    fn default_blocked_functions() -> HashSet<String> {
        let mut blocked = HashSet::new();
        // Dangerous I/O functions
        blocked.insert("io_execute".to_string());
        blocked.insert("io_system".to_string());
        blocked.insert("os_execute".to_string());
        // Dangerous environment functions
        blocked.insert("env_set".to_string());
        blocked.insert("env_remove".to_string());
        blocked
    }

    /// Apply security level settings
    pub fn apply_level(&mut self, level: &SecurityLevel) {
        self.level = level.clone();
        match level {
            SecurityLevel::Full => {
                self.permissions = Permissions::full();
                self.blocked_functions.clear();
            }
            SecurityLevel::Standard => {
                self.permissions = Permissions::default();
                self.blocked_functions = Self::default_blocked_functions();
            }
            SecurityLevel::Restricted => {
                self.permissions = Permissions::restricted();
                self.blocked_functions = Self::default_blocked_functions();
            }
            SecurityLevel::Sandbox => {
                self.permissions = Permissions::sandbox();
                self.blocked_functions.extend([
                    "io_read".to_string(),
                    "io_write".to_string(),
                    "io_append".to_string(),
                    "io_remove".to_string(),
                    "http_get".to_string(),
                    "http_post".to_string(),
                ]);
            }
        }
    }

    /// Check if function is allowed
    pub fn is_function_allowed(&self, name: &str) -> bool {
        !self.blocked_functions.contains(name)
    }

    /// Block a function
    pub fn block_function(&mut self, name: &str) {
        self.blocked_functions.insert(name.to_string());
    }

    /// Unblock a function
    pub fn unblock_function(&mut self, name: &str) {
        self.blocked_functions.remove(name);
    }

    /// Validate path access
    pub fn validate_path(&self, path: &str) -> Result<(), SecurityError> {
        if !self.permissions.is_path_allowed(path) {
            return Err(SecurityError::PathNotAllowed(path.to_string()));
        }
        Ok(())
    }

    /// Validate network access
    pub fn validate_host(&self, host: &str) -> Result<(), SecurityError> {
        if !self.permissions.is_host_allowed(host) {
            return Err(SecurityError::HostNotAllowed(host.to_string()));
        }
        Ok(())
    }

    /// Validate file size
    pub fn validate_file_size(&self, size: usize) -> Result<(), SecurityError> {
        if size > self.permissions.max_file_size {
            return Err(SecurityError::FileSizeExceeded {
                size,
                max: self.permissions.max_file_size,
            });
        }
        Ok(())
    }

    /// Validate network payload size
    pub fn validate_network_size(&self, size: usize) -> Result<(), SecurityError> {
        if size > self.permissions.max_network_size {
            return Err(SecurityError::NetworkSizeExceeded {
                size,
                max: self.permissions.max_network_size,
            });
        }
        Ok(())
    }

    /// Get timeout
    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Set timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = Some(timeout);
    }

    /// Get max memory
    pub fn max_memory(&self) -> usize {
        self.max_memory
    }

    /// Set max memory
    pub fn set_max_memory(&mut self, memory: usize) {
        self.max_memory = memory;
    }

    /// Get max recursion depth
    pub fn max_recursion(&self) -> usize {
        self.max_recursion
    }

    /// Set max recursion depth
    pub fn set_max_recursion(&mut self, depth: usize) {
        self.max_recursion = depth;
    }

    /// Get security level
    pub fn level(&self) -> &SecurityLevel {
        &self.level
    }

    /// Get permissions
    pub fn permissions(&self) -> &Permissions {
        &self.permissions
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Security Errors
// ============================================================================

/// Security-related errors
#[derive(Clone, Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Path not allowed: {0}")]
    PathNotAllowed(String),

    #[error("Host not allowed: {0}")]
    HostNotAllowed(String),

    #[error("File size exceeded: {size} > {max}")]
    FileSizeExceeded { size: usize, max: usize },

    #[error("Network payload size exceeded: {size} > {max}")]
    NetworkSizeExceeded { size: usize, max: usize },

    #[error("Function not allowed: {0}")]
    FunctionNotAllowed(String),

    #[error("Memory limit exceeded: {used} > {max}")]
    MemoryLimitExceeded { used: usize, max: usize },

    #[error("Recursion depth exceeded: {depth} > {max}")]
    RecursionDepthExceeded { depth: usize, max: usize },

    #[error("Execution timeout exceeded")]
    TimeoutExceeded,

    #[error("Sandbox violation: {0}")]
    SandboxViolation(String),
}

// ============================================================================
// Input Validator
// ============================================================================

/// Input validation utilities
pub struct InputValidator;

impl InputValidator {
    /// Validate string input for injection attacks
    pub fn validate_string(input: &str, max_length: usize) -> Result<(), SecurityError> {
        if input.len() > max_length {
            return Err(SecurityError::SandboxViolation(format!(
                "Input too long: {} > {}",
                input.len(),
                max_length
            )));
        }

        // Check for common injection patterns
        let dangerous_patterns = [
            "../",
            "..\\",
            "${",
            "#{",
            "<script",
            "javascript:",
            "data:",
            "file://",
        ];

        for pattern in dangerous_patterns {
            if input.to_lowercase().contains(pattern) {
                return Err(SecurityError::SandboxViolation(format!(
                    "Dangerous pattern detected: {}",
                    pattern
                )));
            }
        }

        Ok(())
    }

    /// Validate identifier name
    pub fn validate_identifier(name: &str) -> Result<(), SecurityError> {
        if name.is_empty() {
            return Err(SecurityError::SandboxViolation(
                "Empty identifier".to_string(),
            ));
        }

        let first = name.chars().next().unwrap();
        if !first.is_alphabetic() && first != '_' {
            return Err(SecurityError::SandboxViolation(format!(
                "Invalid identifier start: {}",
                first
            )));
        }

        Ok(())
    }

    /// Validate number
    pub fn validate_number(num: f64) -> Result<(), SecurityError> {
        if num.is_nan() || num.is_infinite() {
            return Err(SecurityError::SandboxViolation(
                "Invalid number: NaN or Infinity".to_string(),
            ));
        }
        Ok(())
    }
}

// ============================================================================
// Resource Tracker
// ============================================================================

/// Track resource usage during execution
pub struct ResourceTracker {
    /// Memory used
    memory_used: usize,
    /// Operations count
    operations: usize,
    /// Start time
    start_time: std::time::Instant,
    /// Maximum operations
    max_operations: Option<usize>,
}

impl ResourceTracker {
    pub fn new() -> Self {
        Self {
            memory_used: 0,
            operations: 0,
            start_time: std::time::Instant::now(),
            max_operations: None,
        }
    }

    pub fn with_max_operations(max: usize) -> Self {
        Self {
            max_operations: Some(max),
            ..Self::new()
        }
    }

    /// Record memory allocation
    pub fn record_memory(&mut self, bytes: usize) {
        self.memory_used += bytes;
    }

    /// Record operation
    pub fn record_operation(&mut self) -> Result<(), SecurityError> {
        self.operations += 1;
        if let Some(max) = self.max_operations {
            if self.operations > max {
                return Err(SecurityError::SandboxViolation(format!(
                    "Operation limit exceeded: {} > {}",
                    self.operations, max
                )));
            }
        }
        Ok(())
    }

    /// Check timeout
    pub fn check_timeout(&self, timeout: Duration) -> Result<(), SecurityError> {
        if self.start_time.elapsed() > timeout {
            return Err(SecurityError::TimeoutExceeded);
        }
        Ok(())
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get memory used
    pub fn memory_used(&self) -> usize {
        self.memory_used
    }

    /// Get operations count
    pub fn operations(&self) -> usize {
        self.operations
    }

    /// Reset tracker
    pub fn reset(&mut self) {
        self.memory_used = 0;
        self.operations = 0;
        self.start_time = std::time::Instant::now();
    }
}

impl Default for ResourceTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_levels() {
        let full = SecurityLevel::Full;
        let sandbox = SecurityLevel::Sandbox;
        assert_ne!(full, sandbox);
    }

    #[test]
    fn test_permissions_full() {
        let perms = Permissions::full();
        assert!(perms.allow_fs);
        assert!(perms.allow_network);
        assert!(perms.is_path_allowed("/any/path"));
        assert!(perms.is_host_allowed("any.host.com"));
    }

    #[test]
    fn test_permissions_sandbox() {
        let perms = Permissions::sandbox();
        assert!(!perms.allow_fs);
        assert!(!perms.allow_network);
        assert!(!perms.is_path_allowed("/any/path"));
    }

    #[test]
    fn test_security_context_default() {
        let ctx = SecurityContext::new();
        assert_eq!(*ctx.level(), SecurityLevel::Standard);
        assert!(ctx.timeout().is_some());
    }

    #[test]
    fn test_security_context_levels() {
        let mut ctx = SecurityContext::with_level(SecurityLevel::Sandbox);
        assert_eq!(*ctx.level(), SecurityLevel::Sandbox);
        assert!(!ctx.is_function_allowed("io_read"));
        
        ctx.apply_level(&SecurityLevel::Full);
        assert_eq!(*ctx.level(), SecurityLevel::Full);
        assert!(ctx.is_function_allowed("io_read"));
    }

    #[test]
    fn test_block_function() {
        let mut ctx = SecurityContext::new();
        ctx.block_function("custom_function");
        assert!(!ctx.is_function_allowed("custom_function"));
        
        ctx.unblock_function("custom_function");
        assert!(ctx.is_function_allowed("custom_function"));
    }

    #[test]
    fn test_path_validation() {
        let mut ctx = SecurityContext::new();
        ctx.permissions.allow_fs = true;
        assert!(ctx.validate_path("/safe/path").is_ok());
        
        ctx.permissions.allow_fs = false;
        assert!(ctx.validate_path("/any/path").is_err());
    }

    #[test]
    fn test_host_validation() {
        let mut ctx = SecurityContext::new();
        ctx.permissions.allow_network = true;
        assert!(ctx.validate_host("example.com").is_ok());
        
        ctx.permissions.allow_network = false;
        assert!(ctx.validate_host("example.com").is_err());
    }

    #[test]
    fn test_input_validator_string() {
        assert!(InputValidator::validate_string("safe input", 100).is_ok());
        assert!(InputValidator::validate_string(&"x".repeat(101), 100).is_err());
        assert!(InputValidator::validate_string("../etc/passwd", 100).is_err());
        assert!(InputValidator::validate_string("<script>alert(1)</script>", 100).is_err());
    }

    #[test]
    fn test_input_validator_identifier() {
        assert!(InputValidator::validate_identifier("valid_name").is_ok());
        assert!(InputValidator::validate_identifier("_private").is_ok());
        assert!(InputValidator::validate_identifier("").is_err());
        assert!(InputValidator::validate_identifier("123invalid").is_err());
    }

    #[test]
    fn test_input_validator_number() {
        assert!(InputValidator::validate_number(42.0).is_ok());
        assert!(InputValidator::validate_number(f64::NAN).is_err());
        assert!(InputValidator::validate_number(f64::INFINITY).is_err());
    }

    #[test]
    fn test_resource_tracker() {
        let mut tracker = ResourceTracker::with_max_operations(100);
        
        for _ in 0..50 {
            assert!(tracker.record_operation().is_ok());
        }
        
        assert_eq!(tracker.operations(), 50);
        
        for _ in 0..50 {
            assert!(tracker.record_operation().is_ok());
        }
        
        // Next should fail
        assert!(tracker.record_operation().is_err());
    }

    #[test]
    fn test_resource_tracker_timeout() {
        let tracker = ResourceTracker::new();
        assert!(tracker.check_timeout(Duration::from_secs(1)).is_ok());
        
        std::thread::sleep(Duration::from_millis(100));
        assert!(tracker.check_timeout(Duration::from_millis(50)).is_err());
    }

    #[test]
    fn test_resource_tracker_memory() {
        let mut tracker = ResourceTracker::new();
        tracker.record_memory(1024);
        tracker.record_memory(2048);
        assert_eq!(tracker.memory_used(), 3072);
    }

    #[test]
    fn test_resource_tracker_reset() {
        let mut tracker = ResourceTracker::new();
        tracker.record_memory(1024);
        tracker.record_operation().unwrap();
        
        tracker.reset();
        
        assert_eq!(tracker.memory_used(), 0);
        assert_eq!(tracker.operations(), 0);
    }

    #[test]
    fn test_security_error_display() {
        let err = SecurityError::PathNotAllowed("/forbidden".to_string());
        assert!(err.to_string().contains("/forbidden"));
        
        let err = SecurityError::FileSizeExceeded { size: 1000, max: 500 };
        assert!(err.to_string().contains("1000"));
        assert!(err.to_string().contains("500"));
    }

    #[test]
    fn test_permissions_allowed_paths() {
        let mut perms = Permissions::default();
        perms.allow_fs = true;
        perms.allowed_paths.insert("/safe".to_string());
        perms.allowed_paths.insert("/tmp".to_string());
        
        assert!(perms.is_path_allowed("/safe/file.txt"));
        assert!(perms.is_path_allowed("/tmp/file.txt"));
        assert!(!perms.is_path_allowed("/etc/passwd"));
    }

    #[test]
    fn test_permissions_allowed_hosts() {
        let mut perms = Permissions::default();
        perms.allow_network = true;
        perms.allowed_hosts.insert("api.".to_string());
        perms.allowed_hosts.insert("trusted.com".to_string());
        
        assert!(perms.is_host_allowed("api.example.com"));
        assert!(perms.is_host_allowed("trusted.com"));
        assert!(!perms.is_host_allowed("malicious.com"));
    }

    #[test]
    fn test_security_context_builder_pattern() {
        let mut ctx = SecurityContext::new();
        ctx.set_timeout(Duration::from_secs(60));
        ctx.set_max_memory(512 * 1024 * 1024);
        ctx.set_max_recursion(2000);
        
        assert_eq!(ctx.timeout(), Some(Duration::from_secs(60)));
        assert_eq!(ctx.max_memory(), 512 * 1024 * 1024);
        assert_eq!(ctx.max_recursion(), 2000);
    }
}
