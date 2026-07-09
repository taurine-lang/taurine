//! FFI (Foreign Function Interface) for Taurine
//! 
//! This module provides a C API for embedding Taurine in other languages.
//! 
//! # Example (C)
//! 
//! ```c
//! #include <stdio.h>
//! #include "taurine.h"
//! 
//! int main() {
//!     TaurineVM* vm = taurine_new();
//!     taurine_run(vm, "print(\"Hello from C!\")");
//!     taurine_free(vm);
//!     return 0;
//! }
//! ```

use crate::interpreter::Interpreter;
use crate::value::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::path::PathBuf;

/// Opaque pointer to Taurine VM
pub struct TaurineVM {
    interpreter: Interpreter,
    last_error: Option<String>,
}

/// Opaque pointer to Taurine Value
pub struct TaurineValue {
    value: Value,
}

// VM Management

/// Create a new Taurine VM
#[no_mangle]
pub extern "C" fn taurine_new() -> *mut TaurineVM {
    let vm = TaurineVM {
        interpreter: Interpreter::new(PathBuf::from(".")),
        last_error: None,
    };
    Box::into_raw(Box::new(vm))
}

/// Create a new Taurine VM with custom base path
#[no_mangle]
pub extern "C" fn taurine_new_with_path(path: *const c_char) -> *mut TaurineVM {
    if path.is_null() {
        return taurine_new();
    }
    
    let path_str = unsafe {
        match CStr::from_ptr(path).to_str() {
            Ok(s) => PathBuf::from(s),
            Err(_) => return std::ptr::null_mut(),
        }
    };
    
    let vm = TaurineVM {
        interpreter: Interpreter::new(path_str),
        last_error: None,
    };
    Box::into_raw(Box::new(vm))
}

/// Free a Taurine VM
#[no_mangle]
pub extern "C" fn taurine_free(vm: *mut TaurineVM) {
    if !vm.is_null() {
        unsafe {
            let _ = Box::from_raw(vm);
        }
    }
}

// Execution

/// Run Taurine code
#[no_mangle]
pub extern "C" fn taurine_run(vm: *mut TaurineVM, code: *const c_char) -> c_int {
    if vm.is_null() || code.is_null() {
        return -1;
    }
    
    let vm = unsafe { &mut *vm };
    let code_str = unsafe {
        match CStr::from_ptr(code).to_str() {
            Ok(s) => s,
            Err(_) => {
                vm.last_error = Some("Invalid UTF-8 in code".to_string());
                return -1;
            }
        }
    };
    
    match vm.interpreter.run(code_str) {
        Ok(_) => 0,
        Err(e) => {
            vm.last_error = Some(e);
            -1
        }
    }
}

/// Run a Taurine file
#[no_mangle]
pub extern "C" fn taurine_run_file(vm: *mut TaurineVM, filename: *const c_char) -> c_int {
    if vm.is_null() || filename.is_null() {
        return -1;
    }
    
    let vm = unsafe { &mut *vm };
    let filename_str = unsafe {
        match CStr::from_ptr(filename).to_str() {
            Ok(s) => s,
            Err(_) => {
                vm.last_error = Some("Invalid UTF-8 in filename".to_string());
                return -1;
            }
        }
    };
    
    match std::fs::read_to_string(filename_str) {
        Ok(code) => match vm.interpreter.run(&code) {
            Ok(_) => 0,
            Err(e) => {
                vm.last_error = Some(e);
                -1
            }
        },
        Err(e) => {
            vm.last_error = Some(format!("Failed to read file: {}", e));
            -1
        }
    }
}

/// Get last error message
#[no_mangle]
pub extern "C" fn taurine_get_error(vm: *mut TaurineVM) -> *const c_char {
    if vm.is_null() {
        return std::ptr::null();
    }
    
    let vm = unsafe { &mut *vm };
    match &vm.last_error {
        Some(e) => match CString::new(e.as_str()) {
            Ok(cstr) => cstr.into_raw() as *const c_char,
            Err(_) => std::ptr::null(),
        },
        None => std::ptr::null(),
    }
}

// Value Creation

/// Create a new number value
#[no_mangle]
pub extern "C" fn taurine_new_number(n: f64) -> *mut TaurineValue {
    Box::into_raw(Box::new(TaurineValue {
        value: Value::Number(n),
    }))
}

/// Create a new string value
#[no_mangle]
pub extern "C" fn taurine_new_string(s: *const c_char) -> *mut TaurineValue {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    
    let str_val = unsafe {
        match CStr::from_ptr(s).to_str() {
            Ok(str) => str.to_string(),
            Err(_) => return std::ptr::null_mut(),
        }
    };
    
    Box::into_raw(Box::new(TaurineValue {
        value: Value::String(str_val),
    }))
}

/// Create a new boolean value
#[no_mangle]
pub extern "C" fn taurine_new_bool(b: c_int) -> *mut TaurineValue {
    Box::into_raw(Box::new(TaurineValue {
        value: Value::Bool(b != 0),
    }))
}

/// Create a new nil value
#[no_mangle]
pub extern "C" fn taurine_new_nil() -> *mut TaurineValue {
    Box::into_raw(Box::new(TaurineValue {
        value: Value::Nil,
    }))
}

/// Free a TaurineValue
#[no_mangle]
pub extern "C" fn taurine_value_free(val: *mut TaurineValue) {
    if !val.is_null() {
        unsafe {
            let _ = Box::from_raw(val);
        }
    }
}

// Value Type Checking

/// Check if value is a number
#[no_mangle]
pub extern "C" fn taurine_is_number(val: *const TaurineValue) -> c_int {
    if val.is_null() {
        return 0;
    }
    let val = unsafe { &*val };
    matches!(val.value, Value::Number(_)) as c_int
}

/// Check if value is a string
#[no_mangle]
pub extern "C" fn taurine_is_string(val: *const TaurineValue) -> c_int {
    if val.is_null() {
        return 0;
    }
    let val = unsafe { &*val };
    matches!(val.value, Value::String(_)) as c_int
}

/// Check if value is a boolean
#[no_mangle]
pub extern "C" fn taurine_is_bool(val: *const TaurineValue) -> c_int {
    if val.is_null() {
        return 0;
    }
    let val = unsafe { &*val };
    matches!(val.value, Value::Bool(_)) as c_int
}

/// Check if value is nil
#[no_mangle]
pub extern "C" fn taurine_is_nil(val: *const TaurineValue) -> c_int {
    if val.is_null() {
        return 0;
    }
    let val = unsafe { &*val };
    matches!(val.value, Value::Nil) as c_int
}

/// Check if value is an array
#[no_mangle]
pub extern "C" fn taurine_is_array(val: *const TaurineValue) -> c_int {
    if val.is_null() {
        return 0;
    }
    let val = unsafe { &*val };
    matches!(val.value, Value::Array(_)) as c_int
}

/// Check if value is a table
#[no_mangle]
pub extern "C" fn taurine_is_table(val: *const TaurineValue) -> c_int {
    if val.is_null() {
        return 0;
    }
    let val = unsafe { &*val };
    matches!(val.value, Value::Table(_)) as c_int
}

// Value Conversion

/// Convert value to number
#[no_mangle]
pub extern "C" fn taurine_as_number(val: *const TaurineValue) -> f64 {
    if val.is_null() {
        return 0.0;
    }
    let val = unsafe { &*val };
    match val.value {
        Value::Number(n) => n,
        _ => 0.0,
    }
}

/// Convert value to string
#[no_mangle]
pub extern "C" fn taurine_as_string(val: *const TaurineValue) -> *const c_char {
    if val.is_null() {
        return std::ptr::null();
    }
    
    let val = unsafe { &*val };
    match &val.value {
        Value::String(s) => match CString::new(s.as_str()) {
            Ok(cstr) => cstr.into_raw() as *const c_char,
            Err(_) => std::ptr::null(),
        },
        _ => std::ptr::null(),
    }
}

/// Convert value to boolean
#[no_mangle]
pub extern "C" fn taurine_as_bool(val: *const TaurineValue) -> c_int {
    if val.is_null() {
        return 0;
    }
    let val = unsafe { &*val };
    match val.value {
        Value::Bool(b) => b as c_int,
        _ => 0,
    }
}

// Variable Access

/// Get a variable value from VM
#[no_mangle]
pub extern "C" fn taurine_get(vm: *mut TaurineVM, name: *const c_char) -> *mut TaurineValue {
    if vm.is_null() || name.is_null() {
        return std::ptr::null_mut();
    }

    let vm = unsafe { &mut *vm };
    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        }
    };

    match vm.interpreter.get(name_str) {
        Ok(value) => Box::into_raw(Box::new(TaurineValue { value })),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Set a variable value in VM
#[no_mangle]
pub extern "C" fn taurine_set(
    vm: *mut TaurineVM,
    name: *const c_char,
    value: *const TaurineValue,
) -> c_int {
    if vm.is_null() || name.is_null() || value.is_null() {
        return -1;
    }

    let vm = unsafe { &mut *vm };
    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => {
                vm.last_error = Some("Invalid UTF-8 in name".to_string());
                return -1;
            }
        }
    };

    let taurine_val = unsafe { &*value };
    
    // Use the public set method for FFI string-based access
    vm.interpreter.set(name_str, taurine_val.value.clone()).map(|_| 0).unwrap_or(-1)
}

// Function Calls

/// Call a Taurine function
#[no_mangle]
pub extern "C" fn taurine_call(
    vm: *mut TaurineVM,
    name: *const c_char,
    args: *const *const TaurineValue,
    arg_count: c_int,
) -> *mut TaurineValue {
    if vm.is_null() || name.is_null() {
        return std::ptr::null_mut();
    }

    let vm = unsafe { &mut *vm };
    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => {
                vm.last_error = Some("Invalid UTF-8 in name".to_string());
                return std::ptr::null_mut();
            }
        }
    };

    // Get the function from the VM using the public API
    let func_result = vm.interpreter.get(name_str);
    
    match func_result {
        Ok(func_value) => {
            // Convert args from C array to Vec<Value>
            let mut arg_vec = Vec::new();
            if !args.is_null() && arg_count > 0 {
                for i in 0..arg_count {
                    let arg_ptr = unsafe { *args.offset(i as isize) };
                    if !arg_ptr.is_null() {
                        let arg_val = unsafe { &*arg_ptr };
                        arg_vec.push(arg_val.value.clone());
                    } else {
                        arg_vec.push(Value::Nil);
                    }
                }
            }

            // Call the function
            match &func_value {
                Value::NativeFunction(native_fn) => {
                    match native_fn(&arg_vec) {
                        Ok(result) => Box::into_raw(Box::new(TaurineValue { value: result })),
                        Err(e) => {
                            vm.last_error = Some(e);
                            std::ptr::null_mut()
                        }
                    }
                }
                Value::Function { .. } => {
                    // For Taurine functions, we need to execute them
                    // This is a simplified implementation
                    vm.last_error = Some("Calling Taurine functions from FFI requires bytecode VM".to_string());
                    std::ptr::null_mut()
                }
                _ => {
                    vm.last_error = Some(format!("'{name_str}' is not a function"));
                    std::ptr::null_mut()
                }
            }
        }
        Err(e) => {
            vm.last_error = Some(e);
            std::ptr::null_mut()
        }
    }
}

// String Freeing

/// Free a string returned by Taurine API
#[no_mangle]
pub extern "C" fn taurine_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
