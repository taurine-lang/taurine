use std::path::PathBuf;
use taurine::safety::SecurityLevel;
use taurine::Interpreter;

#[test]
fn test_sandbox_blocks_absolute_paths() {
    let mut interp = Interpreter::new(PathBuf::from("."));
    interp.set_security_level(SecurityLevel::Sandbox);

    let result = interp.run(r#"io_read("/etc/passwd")"#);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().message();
    assert!(err_msg.contains("sandbox") || err_msg.contains("blocked"));
}

#[test]
fn test_sandbox_blocks_path_traversal() {
    let mut interp = Interpreter::new(PathBuf::from("."));
    interp.set_security_level(SecurityLevel::Sandbox);

    let result = interp.run(r#"io_read("../../etc/passwd")"#);
    assert!(result.is_err());
}

#[test]
fn test_full_level_allows_absolute_paths() {
    let mut interp = Interpreter::new(PathBuf::from("."));
    interp.set_security_level(SecurityLevel::Full);

    // Should not error on security, only on file not found
    let result = interp.run(r#"io_exists("/nonexistent")"#);
    // This might error because file doesn't exist, but not because of security
    if let Err(e) = result {
        assert!(!e.message().contains("blocked"));
        assert!(!e.message().contains("sandbox"));
    }
}

#[test]
fn test_http_validation_blocks_localhost() {
    let mut interp = Interpreter::new(PathBuf::from("."));

    let result = interp.run(r#"http_get("http://localhost:8080")"#);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().message();
    assert!(err_msg.contains("localhost") || err_msg.contains("blocked"));
}

#[test]
fn test_http_validation_blocks_private_networks() {
    let mut interp = Interpreter::new(PathBuf::from("."));

    let result = interp.run(r#"http_get("http://192.168.1.1")"#);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().message();
    assert!(err_msg.contains("private") || err_msg.contains("blocked"));
}

#[test]
fn test_http_validation_blocks_metadata_endpoint() {
    let mut interp = Interpreter::new(PathBuf::from("."));

    // AWS metadata endpoint
    let result = interp.run(r#"http_get("http://169.254.169.254/latest/meta-data/")"#);
    assert!(result.is_err());
}

#[test]
fn test_http_validation_allows_public_urls() {
    let mut interp = Interpreter::new(PathBuf::from("."));

    // Should not error on security validation (may error on network)
    let result = interp.run(r#"http_get("https://example.com")"#);
    if let Err(e) = result {
        let msg = e.message();
        // Should be network error, not security error
        assert!(!msg.contains("blocked"));
        assert!(!msg.contains("localhost"));
        assert!(!msg.contains("private"));
    }
}

#[test]
fn test_io_exit_returns_error_not_exits() {
    let mut interp = Interpreter::new(PathBuf::from("."));

    // Should return error, not actually exit process
    let result = interp.run(r#"io_exit(0)"#);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().message();
    assert!(err_msg.contains("__EXIT__"));
}

#[test]
fn test_timeout_enforcement() {
    let limits = taurine::SafetyLimits::builder()
        .timeout(std::time::Duration::from_millis(100))
        .build();

    let mut interp = Interpreter::with_limits(PathBuf::from("."), limits);

    let start = std::time::Instant::now();
    let result = interp.run(
        r#"
let x = 0
while true {
    x = x + 1
}
"#,
    );
    let elapsed = start.elapsed();

    assert!(result.is_err());
    let err_msg = result.unwrap_err().message();
    assert!(err_msg.contains("timeout"));
    assert!(
        elapsed.as_millis() < 200,
        "Timeout not enforced quickly enough"
    );
}
