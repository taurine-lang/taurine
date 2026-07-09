//! Native Functions Module
//! This module contains all built-in native functions for the Taurine interpreter.
//! These functions provide access to system resources and common operations.

use crate::value::Value;
use crate::string_intern::InternedString;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::io::Read;
use smallvec::SmallVec;
use std::sync::OnceLock;

/// Sanitize user-provided path to prevent path traversal attacks
fn sanitize_path(user_path: &str) -> Result<PathBuf, String> {
    let full_path = Path::new(user_path);

    // Always canonicalize to resolve the actual path
    let canonical = full_path.canonicalize()
        .map_err(|e| format!("Invalid path: {e}"))?;

    // Get current directory
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Cannot get current directory: {e}"))?;

    // Verify the resolved path is within or accessible from current directory
    // Allow absolute paths but check for obvious traversal patterns
    if user_path.starts_with('/') || user_path.chars().nth(1) == Some(':') {
        // Absolute path - just return canonicalized
        return Ok(canonical);
    }

    // For relative paths, ensure they resolve within or below current directory
    // This prevents ".." traversal to parent directories
    if !canonical.starts_with(&current_dir) {
        return Err("Path traversal detected: access outside current directory is not allowed".to_string());
    }

    Ok(canonical)
}

/// Helper to create an interned string for built-in function names
const fn intern_builtin(id: u32) -> InternedString {
    InternedString::new(id as usize)
}

/// Register all built-in functions in the given environment
pub fn register_builtins(global: &Rc<RefCell<crate::environment::Environment>>) {
    register_core_functions(global);
    register_io_functions(global);
    register_string_functions(global);
    register_array_functions(global);
    register_json_functions(global);
    register_http_functions(global);
    register_crypto_functions(global);
    register_date_functions(global);
    register_regex_functions(global);
    // Async functions available through async_rt module directly
}

// ============================================================================
// Core Functions
// ============================================================================

fn register_core_functions(global: &Rc<RefCell<crate::environment::Environment>>) {
    // print
    let print_fn = |args: &[Value]| -> Result<Value, String> {
        let output = args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
        println!("{output}");
        Ok(Value::Nil)
    };
    global.borrow_mut().define(intern_builtin(1), Value::NativeFunction(print_fn));

    // assert
    let assert_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() {
            return Err("assert() requires at least 1 argument".to_string());
        }
        if !args[0].is_truthy() {
            let msg = if args.len() > 1 {
                args[1].to_string()
            } else {
                "Assertion failed".to_string()
            };
            return Err(format!("ASSERTION FAILED: {msg}"));
        }
        Ok(Value::Nil)
    };
    global.borrow_mut().define(intern_builtin(2), Value::NativeFunction(assert_fn));

    // assert_eq
    let assert_eq_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 {
            return Err("assert_eq() requires 2 arguments".to_string());
        }
        if args[0] != args[1] {
            let msg = if args.len() > 2 {
                args[2].to_string()
            } else {
                format!("Expected {:?} == {:?}", args[0], args[1])
            };
            return Err(format!("ASSERTION FAILED: {msg}"));
        }
        Ok(Value::Nil)
    };
    global.borrow_mut().define(intern_builtin(3), Value::NativeFunction(assert_eq_fn));

    // type
    let type_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() {
            return Err("type() requires 1 argument".to_string());
        }
        let type_name = match &args[0] {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Bool(_) => "boolean",
            Value::Nil => "nil",
            Value::Table(_) => "table",
            Value::Array(_) => "array",
            Value::Range { .. } => "range",
            Value::Function { .. } => "function",
            Value::AsyncFunction { .. } => "async_function",
            Value::Generator { .. } => "generator",
            Value::Future(_) => "future",
            Value::NativeFunction(_) => "function",
            Value::Error(_) => "error",
        };
        Ok(Value::String(type_name.to_string()))
    };
    global.borrow_mut().define(intern_builtin(4), Value::NativeFunction(type_fn));

    // tonumber
    let tonumber_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() {
            return Err("tonumber() requires 1 argument".to_string());
        }
        match &args[0] {
            Value::Number(n) => Ok(Value::Number(*n)),
            Value::String(s) => s.parse::<f64>()
                .map(Value::Number)
                .map_err(|_| format!("Cannot convert '{s}' to number")),
            _ => Err("tonumber() requires number or string".to_string()),
        }
    };
    global.borrow_mut().define(intern_builtin(5), Value::NativeFunction(tonumber_fn));

    // tostring
    let tostring_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() {
            return Err("tostring() requires 1 argument".to_string());
        }
        Ok(Value::String(args[0].to_string()))
    };
    global.borrow_mut().define(intern_builtin(6), Value::NativeFunction(tostring_fn));
}

// ============================================================================
// I/O Functions
// ============================================================================

fn register_io_functions(global: &Rc<RefCell<crate::environment::Environment>>) {
    // io_read
    let io_read_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_read() requires 1 argument".to_string()); }
        if let Value::String(path) = &args[0] {
            let sanitized = sanitize_path(path)?;
            std::fs::read_to_string(&sanitized)
                .map(Value::String)
                .map_err(|e| format!("Cannot read file: {e}"))
        } else { Err("io_read() requires string path".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(10), Value::NativeFunction(io_read_fn));

    // io_write
    let io_write_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("io_write() requires 2 arguments".to_string()); }
        if let (Value::String(path), Value::String(content)) = (&args[0], &args[1]) {
            let sanitized = sanitize_path(path)?;
            std::fs::write(&sanitized, content)
                .map(|_| Value::Bool(true))
                .map_err(|e| format!("Cannot write file: {e}"))
        } else { Err("io_write() requires string path and content".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(11), Value::NativeFunction(io_write_fn));

    // io_append
    let io_append_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("io_append() requires 2 arguments".to_string()); }
        if let (Value::String(path), Value::String(content)) = (&args[0], &args[1]) {
            use std::io::Write;
            let sanitized = sanitize_path(path)?;
            let mut file = std::fs::OpenOptions::new()
                .append(true).create(true).open(&sanitized)
                .map_err(|e| format!("Cannot open file: {e}"))?;
            file.write_all(content.as_bytes())
                .map(|_| Value::Bool(true))
                .map_err(|e| format!("Cannot append: {e}"))
        } else { Err("io_append() requires string path and content".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(12), Value::NativeFunction(io_append_fn));

    // io_exists
    let io_exists_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_exists() requires 1 argument".to_string()); }
        if let Value::String(path) = &args[0] {
            // Sanitize path for consistency, but allow checking existence
            match sanitize_path(path) {
                Ok(sanitized) => Ok(Value::Bool(std::path::Path::new(&sanitized).exists())),
                Err(_) => Ok(Value::Bool(false)),  // If path is invalid, it doesn't exist
            }
        } else { Err("io_exists() requires string path".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(13), Value::NativeFunction(io_exists_fn));

    // io_remove
    let io_remove_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_remove() requires 1 argument".to_string()); }
        if let Value::String(path) = &args[0] {
            let sanitized = sanitize_path(path)?;
            std::fs::remove_file(&sanitized)
                .or_else(|_| std::fs::remove_dir_all(&sanitized))
                .map(|_| Value::Bool(true))
                .map_err(|e| format!("Cannot remove: {e}"))
        } else { Err("io_remove() requires string path".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(14), Value::NativeFunction(io_remove_fn));

    // io_mkdir
    let io_mkdir_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_mkdir() requires 1 argument".to_string()); }
        if let Value::String(path) = &args[0] {
            let sanitized = sanitize_path(path)?;
            std::fs::create_dir_all(&sanitized)
                .map(|_| Value::Bool(true))
                .map_err(|e| format!("Cannot create directory: {e}"))
        } else { Err("io_mkdir() requires string path".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(15), Value::NativeFunction(io_mkdir_fn));

    // io_platform
    let io_platform_fn = |_args: &[Value]| -> Result<Value, String> {
        Ok(Value::String(std::env::consts::OS.to_string()))
    };
    global.borrow_mut().define(intern_builtin(16), Value::NativeFunction(io_platform_fn));

    // io_arch
    let io_arch_fn = |_args: &[Value]| -> Result<Value, String> {
        Ok(Value::String(std::env::consts::ARCH.to_string()))
    };
    global.borrow_mut().define(intern_builtin(17), Value::NativeFunction(io_arch_fn));

    // io_cwd
    let io_cwd_fn = |_args: &[Value]| -> Result<Value, String> {
        std::env::current_dir()
            .map(|p| Value::String(p.to_string_lossy().to_string()))
            .map_err(|e| format!("Cannot get current directory: {e}"))
    };
    global.borrow_mut().define(intern_builtin(18), Value::NativeFunction(io_cwd_fn));

    // io_exit
    let io_exit_fn = |args: &[Value]| -> Result<Value, String> {
        let code = if args.is_empty() { 0 } else { 
            match &args[0] {
                Value::Number(n) => *n as i32,
                Value::Bool(b) => if *b { 0 } else { 1 },
                _ => 0
            }
        };
        std::process::exit(code);
    };
    global.borrow_mut().define(intern_builtin(19), Value::NativeFunction(io_exit_fn));

    // io_sleep
    let io_sleep_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_sleep() requires 1 argument".to_string()); }
        match &args[0] {
            Value::Number(n) => {
                std::thread::sleep(std::time::Duration::from_secs_f64(*n));
                Ok(Value::Nil)
            }
            _ => Err("io_sleep() requires number of seconds".to_string())
        }
    };
    global.borrow_mut().define(intern_builtin(20), Value::NativeFunction(io_sleep_fn));

    // io_time
    let io_time_fn = |_args: &[Value]| -> Result<Value, String> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {e}"))?;
        Ok(Value::Number(duration.as_secs_f64()))
    };
    global.borrow_mut().define(intern_builtin(21), Value::NativeFunction(io_time_fn));
}

// ============================================================================
// String Functions
// ============================================================================

fn register_string_functions(global: &Rc<RefCell<crate::environment::Environment>>) {
    // io_strupper
    let io_strupper_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_strupper() requires 1 argument".to_string()); }
        if let Value::String(s) = &args[0] {
            Ok(Value::String(s.to_uppercase()))
        } else { Err("io_strupper() requires string".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(30), Value::NativeFunction(io_strupper_fn));

    // io_strlower
    let io_strlower_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_strlower() requires 1 argument".to_string()); }
        if let Value::String(s) = &args[0] {
            Ok(Value::String(s.to_lowercase()))
        } else { Err("io_strlower() requires string".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(31), Value::NativeFunction(io_strlower_fn));

    // io_strtrim
    let io_strtrim_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_strtrim() requires 1 argument".to_string()); }
        if let Value::String(s) = &args[0] {
            Ok(Value::String(s.trim().to_string()))
        } else { Err("io_strtrim() requires string".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(32), Value::NativeFunction(io_strtrim_fn));

    // io_strsubstr
    let io_strsubstr_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("io_strsubstr() requires 2 arguments".to_string()); }
        if let (Value::String(s), Value::Number(start)) = (&args[0], &args[1]) {
            let len = if args.len() > 2 {
                match &args[2] { Value::Number(l) => *l as usize, _ => s.len() }
            } else { s.len() };
            let start = *start as usize;
            let end = (start + len).min(s.len());
            if start >= s.len() {
                Ok(Value::String("".to_string()))
            } else {
                Ok(Value::String(s[start..end].to_string()))
            }
        } else { Err("io_strsubstr() requires string and number".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(33), Value::NativeFunction(io_strsubstr_fn));

    // io_strfind
    let io_strfind_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("io_strfind() requires 2 arguments".to_string()); }
        if let (Value::String(s), Value::String(pattern)) = (&args[0], &args[1]) {
            s.find(pattern.as_str())
                .map(|i| Value::Number(i as f64 + 1.0))
                .ok_or_else(|| "Pattern not found".to_string())
        } else { Err("io_strfind() requires two strings".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(34), Value::NativeFunction(io_strfind_fn));

    // io_strreplace
    let io_strreplace_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 3 { return Err("io_strreplace() requires 3 arguments".to_string()); }
        if let (Value::String(s), Value::String(from), Value::String(to)) = (&args[0], &args[1], &args[2]) {
            Ok(Value::String(s.replacen(from.as_str(), to.as_str(), 1)))
        } else { Err("io_strreplace() requires three strings".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(35), Value::NativeFunction(io_strreplace_fn));

    // io_strreplaceall
    let io_strreplaceall_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 3 { return Err("io_strreplaceall() requires 3 arguments".to_string()); }
        if let (Value::String(s), Value::String(from), Value::String(to)) = (&args[0], &args[1], &args[2]) {
            Ok(Value::String(s.replace(from.as_str(), to.as_str())))
        } else { Err("io_strreplaceall() requires three strings".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(36), Value::NativeFunction(io_strreplaceall_fn));

    // io_strsplit
    let io_strsplit_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("io_strsplit() requires 2 arguments".to_string()); }
        if let (Value::String(s), Value::String(delimiter)) = (&args[0], &args[1]) {
            let arr = Rc::new(RefCell::new(
                s.split(delimiter.as_str()).map(|x| Value::String(x.to_string())).collect()
            ));
            Ok(Value::Array(arr))
        } else { Err("io_strsplit() requires string and delimiter".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(37), Value::NativeFunction(io_strsplit_fn));

    // io_char
    let io_char_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_char() requires 1 argument".to_string()); }
        match &args[0] {
            Value::Number(n) => {
                let c = *n as u8 as char;
                Ok(Value::String(c.to_string()))
            }
            _ => Err("io_char() requires number".to_string())
        }
    };
    global.borrow_mut().define(intern_builtin(38), Value::NativeFunction(io_char_fn));

    // io_byte
    let io_byte_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_byte() requires 1 argument".to_string()); }
        if let Value::String(s) = &args[0] {
            if let Some(c) = s.chars().next() {
                Ok(Value::Number(c as u32 as f64))
            } else {
                Ok(Value::Number(0.0))
            }
        } else { Err("io_byte() requires string".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(39), Value::NativeFunction(io_byte_fn));
}

// ============================================================================
// Array Functions
// ============================================================================

fn register_array_functions(global: &Rc<RefCell<crate::environment::Environment>>) {
    // io_arraypush
    let io_arraypush_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("io_arraypush() requires 2 arguments".to_string()); }
        if let Value::Array(arr) = &args[0] {
            arr.borrow_mut().push(args[1].clone());
            Ok(Value::Nil)
        } else { Err("io_arraypush() requires array".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(40), Value::NativeFunction(io_arraypush_fn));

    // io_arraypop
    let io_arraypop_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_arraypop() requires 1 argument".to_string()); }
        if let Value::Array(arr) = &args[0] {
            let mut borrowed = arr.borrow_mut();
            Ok(borrowed.pop().unwrap_or(Value::Nil))
        } else { Err("io_arraypop() requires array".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(41), Value::NativeFunction(io_arraypop_fn));

    // io_arraylen
    let io_arraylen_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_arraylen() requires 1 argument".to_string()); }
        if let Value::Array(arr) = &args[0] {
            Ok(Value::Number(arr.borrow().len() as f64))
        } else { Err("io_arraylen() requires array".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(42), Value::NativeFunction(io_arraylen_fn));

    // io_arrayget
    let io_arrayget_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("io_arrayget() requires 2 arguments".to_string()); }
        if let Value::Array(arr) = &args[0] {
            let idx = match &args[1] {
                Value::Number(n) => *n as usize,
                _ => return Err("io_arrayget() requires number index".to_string())
            };
            if idx < arr.borrow().len() {
                Ok(arr.borrow()[idx].clone())
            } else {
                Ok(Value::Nil)
            }
        } else { Err("io_arrayget() requires array".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(43), Value::NativeFunction(io_arrayget_fn));

    // io_arrayset
    let io_arrayset_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 3 { return Err("io_arrayset() requires 3 arguments".to_string()); }
        if let Value::Array(arr) = &args[0] {
            let idx = match &args[1] {
                Value::Number(n) => *n as usize,
                _ => return Err("io_arrayset() requires number index".to_string())
            };
            if idx < arr.borrow().len() {
                arr.borrow_mut()[idx] = args[2].clone();
                Ok(Value::Nil)
            } else {
                Err(format!("Index {} out of bounds", idx))
            }
        } else { Err("io_arrayset() requires array".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(44), Value::NativeFunction(io_arrayset_fn));

    // io_arrayconcat
    let io_arrayconcat_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("io_arrayconcat() requires 2 arguments".to_string()); }
        if let (Value::Array(arr1), Value::Array(arr2)) = (&args[0], &args[1]) {
            let mut result = arr1.borrow().clone();
            result.extend(arr2.borrow().iter().cloned());
            Ok(Value::Array(Rc::new(RefCell::new(result))))
        } else { Err("io_arrayconcat() requires two arrays".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(45), Value::NativeFunction(io_arrayconcat_fn));

    // io_arrayreverse
    let io_arrayreverse_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_arrayreverse() requires 1 argument".to_string()); }
        if let Value::Array(arr) = &args[0] {
            let mut vec = arr.borrow().clone();
            vec.reverse();
            *arr.borrow_mut() = vec;
            Ok(Value::Nil)
        } else { Err("io_arrayreverse() requires array".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(46), Value::NativeFunction(io_arrayreverse_fn));

    // io_arrayclear
    let io_arrayclear_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("io_arrayclear() requires 1 argument".to_string()); }
        if let Value::Array(arr) = &args[0] {
            arr.borrow_mut().clear();
            Ok(Value::Nil)
        } else { Err("io_arrayclear() requires array".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(47), Value::NativeFunction(io_arrayclear_fn));
}

// ============================================================================
// JSON Functions
// ============================================================================

fn register_json_functions(global: &Rc<RefCell<crate::environment::Environment>>) {
    // json_parse
    let json_parse_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("json_parse() requires 1 argument".to_string()); }
        if let Value::String(s) = &args[0] {
            json_str_to_value(s)
        } else { Err("json_parse() requires string".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(50), Value::NativeFunction(json_parse_fn));

    // json_stringify
    let json_stringify_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("json_stringify() requires 1 argument".to_string()); }
        let json = value_to_json_str(&args[0])?;
        Ok(Value::String(json))
    };
    global.borrow_mut().define(intern_builtin(51), Value::NativeFunction(json_stringify_fn));
}

// Helper: JSON string to Value
// Maximum JSON nesting depth to prevent stack overflow
const MAX_JSON_DEPTH: usize = 100;

fn json_str_to_value(s: &str) -> Result<Value, String> {
    let parsed = serde_json::from_str::<serde_json::Value>(s)
        .map_err(|e| format!("JSON parse error: {e}"))?;

    fn convert_json(v: &serde_json::Value, depth: usize) -> Result<Value, String> {
        if depth > MAX_JSON_DEPTH {
            return Err(format!("JSON depth limit exceeded (max: {})", MAX_JSON_DEPTH));
        }
        
        match v {
            serde_json::Value::Null => Ok(Value::Nil),
            serde_json::Value::Bool(b) => Ok(Value::Bool(*b)),
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Ok(Value::Number(f))
                } else {
                    Err(format!("JSON Error: Number '{}' is out of f64 bounds", n))
                }
            }
            serde_json::Value::String(s) => Ok(Value::String(s.clone())),
            serde_json::Value::Array(arr) => {
                let vec: Result<Vec<Value>, String> = arr.iter()
                    .map(|item| convert_json(item, depth + 1))
                    .collect();
                let mut smallvec = SmallVec::new();
                for v in vec? {
                    smallvec.push(v);
                }
                Ok(Value::Array(Rc::new(RefCell::new(smallvec))))
            }
            serde_json::Value::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    // Use hash of key as the ID (not perfect but works for basic JSON)
                    let key_id = k.len() + k.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32).wrapping_mul(31)) as usize;
                    map.insert(key_id, convert_json(v, depth + 1)?);
                }
                Ok(Value::Table(Rc::new(RefCell::new(map))))
            }
        }
    }

    convert_json(&parsed, 0)
}

// Helper: Value to JSON string
fn value_to_json_str(v: &Value) -> Result<String, String> {
    fn convert_value(v: &Value) -> Result<serde_json::Value, String> {
        Ok(match v {
            Value::Number(n) => serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0))),
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::Bool(b) => serde_json::Value::Bool(*b),
            Value::Nil => serde_json::Value::Null,
            Value::Array(arr) => {
                let vec: Result<Vec<_>, _> = arr.borrow().iter().map(convert_value).collect();
                serde_json::Value::Array(vec?)
            }
            Value::Table(t) => {
                let mut map = serde_json::Map::new();
                for (k, v) in t.borrow().iter() {
                    // Convert usize key back to string for JSON
                    map.insert(k.to_string(), convert_value(v)?);
                }
                serde_json::Value::Object(map)
            }
            _ => return Err(format!("Cannot convert {:?} to JSON", v))
        })
    }
    
    let json = convert_value(v)?;
    Ok(serde_json::to_string(&json).map_err(|e| format!("JSON stringify error: {e}"))?)
}

// ============================================================================
// HTTP Functions
// ============================================================================

/// Maximum HTTP response size (10 MB)
const MAX_HTTP_RESPONSE_SIZE: usize = 10 * 1024 * 1024;

static HTTP_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

fn get_http_client() -> &'static reqwest::blocking::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| reqwest::blocking::Client::new())
    })
}

fn register_http_functions(global: &Rc<RefCell<crate::environment::Environment>>) {
    // http_get
    let http_get_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("http_get() requires 1 argument".to_string()); }
        if let Value::String(url) = &args[0] {
            let client = get_http_client();
            let response = client.get(url.as_str())
                .send()
                .map_err(|e| format!("HTTP GET error: {e}"))?;
            
            if let Some(content_length) = response.content_length() {
                if content_length > MAX_HTTP_RESPONSE_SIZE as u64 {
                    return Err(format!("Response too large: {} bytes", content_length));
                }
            }
            
            let mut reader = response.take((MAX_HTTP_RESPONSE_SIZE + 1) as u64);
            let mut text = String::new();
            reader.read_to_string(&mut text).map_err(|e| format!("Read response error: {e}"))?;
            
            if text.len() > MAX_HTTP_RESPONSE_SIZE {
                return Err(format!("HTTP Error: Response truncated. Exceeded limit of {} bytes", MAX_HTTP_RESPONSE_SIZE));
            }
            Ok(Value::String(text))
        } else { Err("http_get() requires string URL".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(60), Value::NativeFunction(http_get_fn));

    // http_post
    let http_post_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("http_post() requires 2 arguments".to_string()); }
        if let (Value::String(url), Value::String(data)) = (&args[0], &args[1]) {
            let client = get_http_client();
            let response = client.post(url.as_str())
                .body(data.clone())
                .send()
                .map_err(|e| format!("HTTP POST error: {e}"))?;
            
            let mut reader = response.take((MAX_HTTP_RESPONSE_SIZE + 1) as u64);
            let mut text = String::new();
            reader.read_to_string(&mut text).map_err(|e| format!("Read response error: {e}"))?;
            
            if text.len() > MAX_HTTP_RESPONSE_SIZE {
                return Err(format!("HTTP Error: Response truncated. Exceeded limit of {} bytes", MAX_HTTP_RESPONSE_SIZE));
            }
            Ok(Value::String(text))
        } else { Err("http_post() requires string URL and data".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(61), Value::NativeFunction(http_post_fn));

    // http_put
    let http_put_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("http_put() requires 2 arguments".to_string()); }
        if let (Value::String(url), Value::String(data)) = (&args[0], &args[1]) {
            let client = get_http_client();
            let response = client.put(url.as_str())
                .body(data.clone())
                .send()
                .map_err(|e| format!("HTTP PUT error: {e}"))?;
            
            let mut reader = response.take((MAX_HTTP_RESPONSE_SIZE + 1) as u64);
            let mut text = String::new();
            reader.read_to_string(&mut text).map_err(|e| format!("Read response error: {e}"))?;
            
            if text.len() > MAX_HTTP_RESPONSE_SIZE {
                return Err(format!("HTTP Error: Response truncated. Exceeded limit of {} bytes", MAX_HTTP_RESPONSE_SIZE));
            }
            Ok(Value::String(text))
        } else { Err("http_put() requires string URL and data".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(62), Value::NativeFunction(http_put_fn));

    // http_delete
    let http_delete_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("http_delete() requires 1 argument".to_string()); }
        if let Value::String(url) = &args[0] {
            let client = get_http_client();
            let response = client.delete(url.as_str())
                .send()
                .map_err(|e| format!("HTTP DELETE error: {e}"))?;
            
            let mut reader = response.take((MAX_HTTP_RESPONSE_SIZE + 1) as u64);
            let mut text = String::new();
            reader.read_to_string(&mut text).map_err(|e| format!("Read response error: {e}"))?;
            
            if text.len() > MAX_HTTP_RESPONSE_SIZE {
                return Err(format!("HTTP Error: Response truncated. Exceeded limit of {} bytes", MAX_HTTP_RESPONSE_SIZE));
            }
            Ok(Value::String(text))
        } else { Err("http_delete() requires string URL".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(63), Value::NativeFunction(http_delete_fn));
}

// ============================================================================
// Crypto Functions
// ============================================================================

fn register_crypto_functions(global: &Rc<RefCell<crate::environment::Environment>>) {
    // crypto_md5
    let crypto_md5_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("crypto_md5() requires 1 argument".to_string()); }
        let data = match &args[0] {
            Value::String(s) => s.as_bytes(),
            Value::Number(n) => &n.to_le_bytes(),
            _ => return Err("crypto_md5() requires string or number".to_string())
        };
        let hash = md5::compute(data);
        Ok(Value::String(format!("{hash:x}")))
    };
    global.borrow_mut().define(intern_builtin(70), Value::NativeFunction(crypto_md5_fn));

    // crypto_sha256
    let crypto_sha256_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("crypto_sha256() requires 1 argument".to_string()); }
        use sha2::{Sha256, Digest};
        let data = match &args[0] {
            Value::String(s) => s.as_bytes(),
            Value::Number(n) => &n.to_le_bytes(),
            _ => return Err("crypto_sha256() requires string or number".to_string())
        };
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        Ok(Value::String(format!("{hash:x}")))
    };
    global.borrow_mut().define(intern_builtin(71), Value::NativeFunction(crypto_sha256_fn));

    // crypto_base64_encode
    let crypto_base64_encode_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("crypto_base64_encode() requires 1 argument".to_string()); }
        if let Value::String(s) = &args[0] {
            use base64::{Engine, engine::general_purpose};
            let encoded = general_purpose::STANDARD.encode(s.as_bytes());
            Ok(Value::String(encoded))
        } else { Err("crypto_base64_encode() requires string".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(72), Value::NativeFunction(crypto_base64_encode_fn));

    // crypto_base64_decode
    let crypto_base64_decode_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("crypto_base64_decode() requires 1 argument".to_string()); }
        if let Value::String(s) = &args[0] {
            use base64::{Engine, engine::general_purpose};
            let decoded = general_purpose::STANDARD.decode(s.as_str())
                .map_err(|e| format!("Base64 decode error: {e}"))?;
            String::from_utf8(decoded)
                .map(Value::String)
                .map_err(|e| format!("UTF-8 decode error: {e}"))
        } else { Err("crypto_base64_decode() requires string".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(73), Value::NativeFunction(crypto_base64_decode_fn));

    // crypto_uuid
    let crypto_uuid_fn = |_args: &[Value]| -> Result<Value, String> {
        use uuid::Uuid;
        Ok(Value::String(Uuid::new_v4().to_string()))
    };
    global.borrow_mut().define(intern_builtin(74), Value::NativeFunction(crypto_uuid_fn));

    // crypto_random_bytes
    let crypto_random_bytes_fn = |args: &[Value]| -> Result<Value, String> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let count = if args.is_empty() { 16 } else {
            match &args[0] {
                Value::Number(n) => *n as usize,
                _ => return Err("crypto_random_bytes() requires number".to_string())
            }
        };
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64;
        let mut bytes = Vec::with_capacity(count);
        let mut s = seed;
        for _ in 0..count {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
            bytes.push((s >> 32) as u8);
        }
        Ok(Value::Array(Rc::new(RefCell::new(
            bytes.into_iter().map(|b| Value::Number(b as f64)).collect()
        ))))
    };
    global.borrow_mut().define(intern_builtin(75), Value::NativeFunction(crypto_random_bytes_fn));
}

// ============================================================================
// Date Functions
// ============================================================================

fn register_date_functions(global: &Rc<RefCell<crate::environment::Environment>>) {
    // date_now
    let date_now_fn = |_args: &[Value]| -> Result<Value, String> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {e}"))?;
        Ok(Value::Number(duration.as_secs_f64()))
    };
    global.borrow_mut().define(intern_builtin(80), Value::NativeFunction(date_now_fn));

    // date_format
    let date_format_fn = |args: &[Value]| -> Result<Value, String> {
        if args.is_empty() { return Err("date_format() requires 1 argument".to_string()); }
        let timestamp = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err("date_format() requires number timestamp".to_string())
        };
        let format = if args.len() > 1 {
            match &args[1] {
                Value::String(s) => s.clone(),
                _ => "%Y-%m-%d %H:%M:%S".to_string()
            }
        } else {
            "%Y-%m-%d %H:%M:%S".to_string()
        };
        
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
            .ok_or("Invalid timestamp")?;
        let formatted = datetime.format(&format).to_string();
        Ok(Value::String(formatted))
    };
    global.borrow_mut().define(intern_builtin(81), Value::NativeFunction(date_format_fn));
}

// ============================================================================
// Regex Functions
// ============================================================================

fn register_regex_functions(global: &Rc<RefCell<crate::environment::Environment>>) {
    // regex_match
    let regex_match_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("regex_match() requires 2 arguments".to_string()); }
        if let (Value::String(pattern), Value::String(text)) = (&args[0], &args[1]) {
            let re = regex::Regex::new(pattern.as_str())
                .map_err(|e| format!("Invalid regex: {e}"))?;
            Ok(Value::Bool(re.is_match(text.as_str())))
        } else { Err("regex_match() requires two strings".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(90), Value::NativeFunction(regex_match_fn));

    // regex_find
    let regex_find_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("regex_find() requires 2 arguments".to_string()); }
        if let (Value::String(pattern), Value::String(text)) = (&args[0], &args[1]) {
            let re = regex::Regex::new(pattern.as_str())
                .map_err(|e| format!("Invalid regex: {e}"))?;
            if let Some(m) = re.find(text.as_str()) {
                let result = Value::new_table();
                if let Value::Table(t) = &result {
                    let mut map = t.borrow_mut();
                    map.insert(1, Value::String(m.as_str().to_string()));
                    map.insert(2, Value::Number(m.start() as f64 + 1.0));
                    map.insert(3, Value::Number(m.end() as f64));
                }
                Ok(result)
            } else {
                Ok(Value::Nil)
            }
        } else { Err("regex_find() requires two strings".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(91), Value::NativeFunction(regex_find_fn));

    // regex_replace
    let regex_replace_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 3 { return Err("regex_replace() requires 3 arguments".to_string()); }
        if let (Value::String(pattern), Value::String(text), Value::String(replacement)) = (&args[0], &args[1], &args[2]) {
            let re = regex::Regex::new(pattern.as_str())
                .map_err(|e| format!("Invalid regex: {e}"))?;
            let result = re.replacen(text.as_str(), 1, replacement.as_str());
            Ok(Value::String(result.to_string()))
        } else { Err("regex_replace() requires pattern, text, and replacement".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(92), Value::NativeFunction(regex_replace_fn));

    // regex_find_all
    let regex_find_all_fn = |args: &[Value]| -> Result<Value, String> {
        if args.len() < 2 { return Err("regex_find_all() requires 2 arguments".to_string()); }
        if let (Value::String(pattern), Value::String(text)) = (&args[0], &args[1]) {
            let re = regex::Regex::new(pattern.as_str())
                .map_err(|e| format!("Invalid regex: {e}"))?;
            let matches: Vec<Value> = re.find_iter(text.as_str())
                .map(|m| {
                    let result = Value::new_table();
                    if let Value::Table(t) = &result {
                        let mut map = t.borrow_mut();
                        map.insert(1, Value::String(m.as_str().to_string()));
                        map.insert(2, Value::Number(m.start() as f64 + 1.0));
                        map.insert(3, Value::Number(m.end() as f64));
                    }
                    result
                })
                .collect();
            let mut smallvec = SmallVec::new();
            for m in matches {
                smallvec.push(m);
            }
            Ok(Value::Array(Rc::new(RefCell::new(smallvec))))
        } else { Err("regex_find_all() requires two strings".to_string()) }
    };
    global.borrow_mut().define(intern_builtin(93), Value::NativeFunction(regex_find_all_fn));
}

// ============================================================================
// Async Functions (requires async feature)
// ============================================================================

// Async functions will be added here when async feature is fully implemented
// For now, async support is available through the async_rt module directly

// Re-export register_builtins to include async functions
#[cfg(feature = "async")]
pub fn register_builtins_with_async(global: &Rc<RefCell<crate::environment::Environment>>) {
    register_builtins(global);
    // Async functions available through async_rt module directly
}
