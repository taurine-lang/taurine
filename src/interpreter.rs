use crate::ast::{Expr, Stmt, Program};
use crate::value::Value;
use crate::environment::Environment;
use crate::lexer;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum ControlFlow {
    None,
    Break,
    Continue,
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    pub function: String,
    pub line: usize,
}

pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    return_value: Option<Value>,
    base_path: PathBuf,
    call_stack: Vec<StackFrame>,
    current_line: usize,
    control_flow: ControlFlow,
}

impl Interpreter {
    pub fn new(base_path: PathBuf) -> Self {
        let global = Rc::new(RefCell::new(Environment::new()));
        let mut interp = Interpreter {
            global,
            return_value: None,
            base_path,
            call_stack: Vec::new(),
            current_line: 1,
            control_flow: ControlFlow::None,
        };
        interp.register_builtins();
        interp
    }

    pub fn optimize(&mut self) {}

    fn register_builtins(&mut self) {
        // print
        let print_fn = |args: &[Value]| -> Result<Value, String> {
            let output = args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
            println!("{output}");
            Ok(Value::Nil)
        };
        self.global.borrow_mut().define("print".to_string(), Value::NativeFunction(print_fn));

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
        self.global.borrow_mut().define("assert".to_string(), Value::NativeFunction(assert_fn));

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
        self.global.borrow_mut().define("assert_eq".to_string(), Value::NativeFunction(assert_eq_fn));

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
                Value::NativeFunction(_) => "function",
                Value::Error(_) => "error",
            };
            Ok(Value::String(type_name.to_string()))
        };
        self.global.borrow_mut().define("type".to_string(), Value::NativeFunction(type_fn));

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
        self.global.borrow_mut().define("tonumber".to_string(), Value::NativeFunction(tonumber_fn));

        // tostring
        let tostring_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() {
                return Err("tostring() requires 1 argument".to_string());
            }
            Ok(Value::String(args[0].to_string()))
        };
        self.global.borrow_mut().define("tostring".to_string(), Value::NativeFunction(tostring_fn));

        // io_read
        let io_read_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_read() requires 1 argument".to_string()); }
            if let Value::String(path) = &args[0] {
                std::fs::read_to_string(path)
                    .map(Value::String)
                    .map_err(|e| format!("Cannot read file: {e}"))
            } else { Err("io_read() requires string path".to_string()) }
        };
        self.global.borrow_mut().define("io_read".to_string(), Value::NativeFunction(io_read_fn));

        // io_write
        let io_write_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("io_write() requires 2 arguments".to_string()); }
            if let (Value::String(path), Value::String(content)) = (&args[0], &args[1]) {
                std::fs::write(path, content)
                    .map(|_| Value::Bool(true))
                    .map_err(|e| format!("Cannot write file: {e}"))
            } else { Err("io_write() requires string path and content".to_string()) }
        };
        self.global.borrow_mut().define("io_write".to_string(), Value::NativeFunction(io_write_fn));

        // io_append
        let io_append_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("io_append() requires 2 arguments".to_string()); }
            if let (Value::String(path), Value::String(content)) = (&args[0], &args[1]) {
                use std::io::Write;
                let mut file = std::fs::OpenOptions::new()
                    .append(true).create(true).open(path)
                    .map_err(|e| format!("Cannot open file: {e}"))?;
                file.write_all(content.as_bytes())
                    .map(|_| Value::Bool(true))
                    .map_err(|e| format!("Cannot append: {e}"))
            } else { Err("io_append() requires string path and content".to_string()) }
        };
        self.global.borrow_mut().define("io_append".to_string(), Value::NativeFunction(io_append_fn));

        // io_exists
        let io_exists_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_exists() requires 1 argument".to_string()); }
            if let Value::String(path) = &args[0] {
                Ok(Value::Bool(std::path::Path::new(path).exists()))
            } else { Err("io_exists() requires string path".to_string()) }
        };
        self.global.borrow_mut().define("io_exists".to_string(), Value::NativeFunction(io_exists_fn));

        // io_remove
        let io_remove_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_remove() requires 1 argument".to_string()); }
            if let Value::String(path) = &args[0] {
                std::fs::remove_file(path)
                    .or_else(|_| std::fs::remove_dir_all(path))
                    .map(|_| Value::Bool(true))
                    .map_err(|e| format!("Cannot remove: {e}"))
            } else { Err("io_remove() requires string path".to_string()) }
        };
        self.global.borrow_mut().define("io_remove".to_string(), Value::NativeFunction(io_remove_fn));

        // io_mkdir
        let io_mkdir_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_mkdir() requires 1 argument".to_string()); }
            if let Value::String(path) = &args[0] {
                std::fs::create_dir_all(path)
                    .map(|_| Value::Bool(true))
                    .map_err(|e| format!("Cannot create directory: {e}"))
            } else { Err("io_mkdir() requires string path".to_string()) }
        };
        self.global.borrow_mut().define("io_mkdir".to_string(), Value::NativeFunction(io_mkdir_fn));

        // io_platform
        let io_platform_fn = |_args: &[Value]| -> Result<Value, String> {
            Ok(Value::String(std::env::consts::OS.to_string()))
        };
        self.global.borrow_mut().define("io_platform".to_string(), Value::NativeFunction(io_platform_fn));

        // io_arch
        let io_arch_fn = |_args: &[Value]| -> Result<Value, String> {
            Ok(Value::String(std::env::consts::ARCH.to_string()))
        };
        self.global.borrow_mut().define("io_arch".to_string(), Value::NativeFunction(io_arch_fn));

        // io_cwd
        let io_cwd_fn = |_args: &[Value]| -> Result<Value, String> {
            std::env::current_dir()
                .map(|p| Value::String(p.to_string_lossy().to_string()))
                .map_err(|e| format!("Cannot get cwd: {e}"))
        };
        self.global.borrow_mut().define("io_cwd".to_string(), Value::NativeFunction(io_cwd_fn));

        // io_exit
        let io_exit_fn = |args: &[Value]| -> Result<Value, String> {
            let code = if args.is_empty() { 0 } else {
                match &args[0] {
                    Value::Number(n) => *n as i32,
                    _ => 0,
                }
            };
            std::process::exit(code);
        };
        self.global.borrow_mut().define("io_exit".to_string(), Value::NativeFunction(io_exit_fn));

        // io_sleep
        let io_sleep_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_sleep() requires 1 argument".to_string()); }
            if let Value::Number(ms) = &args[0] {
                std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
                Ok(Value::Nil)
            } else { Err("io_sleep() requires number".to_string()) }
        };
        self.global.borrow_mut().define("io_sleep".to_string(), Value::NativeFunction(io_sleep_fn));

        // io_time
        let io_time_fn = |_args: &[Value]| -> Result<Value, String> {
            use std::time::{SystemTime, UNIX_EPOCH};
            let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            Ok(Value::Number(duration.as_secs_f64()))
        };
        self.global.borrow_mut().define("io_time".to_string(), Value::NativeFunction(io_time_fn));

        // io_random
        let io_random_fn = |_args: &[Value]| -> Result<Value, String> {
            use std::time::{SystemTime, UNIX_EPOCH};
            let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as f64;
            Ok(Value::Number((seed.sin() + 1.0) / 2.0))
        };
        self.global.borrow_mut().define("io_random".to_string(), Value::NativeFunction(io_random_fn));

        // io_strupper
        let io_strupper_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_strupper() requires 1 argument".to_string()); }
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.to_uppercase()))
            } else { Err("io_strupper() requires string".to_string()) }
        };
        self.global.borrow_mut().define("io_strupper".to_string(), Value::NativeFunction(io_strupper_fn));

        // io_strlower
        let io_strlower_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_strlower() requires 1 argument".to_string()); }
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.to_lowercase()))
            } else { Err("io_strlower() requires string".to_string()) }
        };
        self.global.borrow_mut().define("io_strlower".to_string(), Value::NativeFunction(io_strlower_fn));

        // io_strtrim
        let io_strtrim_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_strtrim() requires 1 argument".to_string()); }
            if let Value::String(s) = &args[0] {
                Ok(Value::String(s.trim().to_string()))
            } else { Err("io_strtrim() requires string".to_string()) }
        };
        self.global.borrow_mut().define("io_strtrim".to_string(), Value::NativeFunction(io_strtrim_fn));

        // io_strsubstr
        let io_strsubstr_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 3 { return Err("io_strsubstr() requires 3 arguments".to_string()); }
            if let (Value::String(s), Value::Number(start), Value::Number(length)) = (&args[0], &args[1], &args[2]) {
                let start = *start as usize;
                let length = *length as usize;
                let chars: Vec<char> = s.chars().collect();
                if start >= chars.len() {
                    return Ok(Value::String("".to_string()));
                }
                let end = (start + length).min(chars.len());
                Ok(Value::String(chars[start..end].iter().collect()))
            } else { Err("io_strsubstr() requires string, number, number".to_string()) }
        };
        self.global.borrow_mut().define("io_strsubstr".to_string(), Value::NativeFunction(io_strsubstr_fn));

        // io_strfind
        let io_strfind_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("io_strfind() requires 2 arguments".to_string()); }
            if let (Value::String(s), Value::String(pattern)) = (&args[0], &args[1]) {
                match s.find(pattern.as_str()) {
                    Some(pos) => Ok(Value::Number(pos as f64)),
                    None => Ok(Value::Number(-1.0)),
                }
            } else { Err("io_strfind() requires string, string".to_string()) }
        };
        self.global.borrow_mut().define("io_strfind".to_string(), Value::NativeFunction(io_strfind_fn));

        // io_strreplace
        let io_strreplace_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 3 { return Err("io_strreplace() requires 3 arguments".to_string()); }
            if let (Value::String(s), Value::String(pattern), Value::String(replacement)) = (&args[0], &args[1], &args[2]) {
                Ok(Value::String(s.replacen(pattern.as_str(), replacement.as_str(), 1)))
            } else { Err("io_strreplace() requires string, string, string".to_string()) }
        };
        self.global.borrow_mut().define("io_strreplace".to_string(), Value::NativeFunction(io_strreplace_fn));

        // io_strreplaceall
        let io_strreplaceall_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 3 { return Err("io_strreplaceall() requires 3 arguments".to_string()); }
            if let (Value::String(s), Value::String(pattern), Value::String(replacement)) = (&args[0], &args[1], &args[2]) {
                Ok(Value::String(s.replace(pattern.as_str(), replacement.as_str())))
            } else { Err("io_strreplaceall() requires string, string, string".to_string()) }
        };
        self.global.borrow_mut().define("io_strreplaceall".to_string(), Value::NativeFunction(io_strreplaceall_fn));

        // io_strsplit
        let io_strsplit_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("io_strsplit() requires 2 arguments".to_string()); }
            if let (Value::String(s), Value::String(delimiter)) = (&args[0], &args[1]) {
                let arr: Vec<Value> = s.split(delimiter.as_str())
                    .map(|x| Value::String(x.to_string()))
                    .collect();
                Ok(Value::Array(Rc::new(RefCell::new(arr))))
            } else { Err("io_strsplit() requires string, string".to_string()) }
        };
        self.global.borrow_mut().define("io_strsplit".to_string(), Value::NativeFunction(io_strsplit_fn));

        // io_arraypush
        let io_arraypush_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("io_arraypush() requires 2 arguments".to_string()); }
            if let Value::Array(arr) = &args[0] {
                arr.borrow_mut().push(args[1].clone());
                Ok(Value::Nil)
            } else { Err("io_arraypush() requires array".to_string()) }
        };
        self.global.borrow_mut().define("io_arraypush".to_string(), Value::NativeFunction(io_arraypush_fn));

        // io_arraypop
        let io_arraypop_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_arraypop() requires 1 argument".to_string()); }
            if let Value::Array(arr) = &args[0] {
                let mut array = arr.borrow_mut();
                if let Some(val) = array.pop() {
                    Ok(val)
                } else {
                    Ok(Value::Nil)
                }
            } else { Err("io_arraypop() requires array".to_string()) }
        };
        self.global.borrow_mut().define("io_arraypop".to_string(), Value::NativeFunction(io_arraypop_fn));

        // io_arraylen
        let io_arraylen_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_arraylen() requires 1 argument".to_string()); }
            if let Value::Array(arr) = &args[0] {
                Ok(Value::Number(arr.borrow().len() as f64))
            } else { Err("io_arraylen() requires array".to_string()) }
        };
        self.global.borrow_mut().define("io_arraylen".to_string(), Value::NativeFunction(io_arraylen_fn));

        // io_arrayget
        let io_arrayget_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("io_arrayget() requires 2 arguments".to_string()); }
            if let (Value::Array(arr), Value::Number(idx)) = (&args[0], &args[1]) {
                let idx = *idx as usize;
                let array = arr.borrow();
                if idx < array.len() {
                    Ok(array[idx].clone())
                } else {
                    Ok(Value::Nil)
                }
            } else { Err("io_arrayget() requires array, number".to_string()) }
        };
        self.global.borrow_mut().define("io_arrayget".to_string(), Value::NativeFunction(io_arrayget_fn));

        // io_arrayset
        let io_arrayset_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 3 { return Err("io_arrayset() requires 3 arguments".to_string()); }
            if let (Value::Array(arr), Value::Number(idx), value) = (&args[0], &args[1], &args[2]) {
                let idx = *idx as usize;
                let mut array = arr.borrow_mut();
                if idx < array.len() {
                    array[idx] = value.clone();
                    Ok(Value::Nil)
                } else {
                    Err(format!("Index {idx} out of bounds"))
                }
            } else { Err("io_arrayset() requires array, number, value".to_string()) }
        };
        self.global.borrow_mut().define("io_arrayset".to_string(), Value::NativeFunction(io_arrayset_fn));

        // io_arrayconcat
        let io_arrayconcat_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("io_arrayconcat() requires 2 arguments".to_string()); }
            if let (Value::Array(arr1), Value::Array(arr2)) = (&args[0], &args[1]) {
                let array1 = arr1.borrow();
                let array2 = arr2.borrow();
                let mut result = Vec::new();
                for item in array1.iter() {
                    result.push(item.clone());
                }
                for item in array2.iter() {
                    result.push(item.clone());
                }
                Ok(Value::Array(Rc::new(RefCell::new(result))))
            } else { Err("io_arrayconcat() requires array, array".to_string()) }
        };
        self.global.borrow_mut().define("io_arrayconcat".to_string(), Value::NativeFunction(io_arrayconcat_fn));

        // io_arrayreverse
        let io_arrayreverse_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_arrayreverse() requires 1 argument".to_string()); }
            if let Value::Array(arr) = &args[0] {
                arr.borrow_mut().reverse();
                Ok(Value::Nil)
            } else { Err("io_arrayreverse() requires array".to_string()) }
        };
        self.global.borrow_mut().define("io_arrayreverse".to_string(), Value::NativeFunction(io_arrayreverse_fn));

        // io_arrayclear
        let io_arrayclear_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_arrayclear() requires 1 argument".to_string()); }
            if let Value::Array(arr) = &args[0] {
                arr.borrow_mut().clear();
                Ok(Value::Nil)
            } else { Err("io_arrayclear() requires array".to_string()) }
        };
        self.global.borrow_mut().define("io_arrayclear".to_string(), Value::NativeFunction(io_arrayclear_fn));

        // io_char
        let io_char_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("io_char() requires 1 argument".to_string()); }
            if let Value::Number(code) = &args[0] {
                if let Some(c) = char::from_u32(*code as u32) {
                    Ok(Value::String(c.to_string()))
                } else {
                    Err(format!("Invalid character code: {code}"))
                }
            } else { Err("io_char() requires number".to_string()) }
        };
        self.global.borrow_mut().define("io_char".to_string(), Value::NativeFunction(io_char_fn));

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
        self.global.borrow_mut().define("io_byte".to_string(), Value::NativeFunction(io_byte_fn));

        // json_parse
        let json_parse_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("json_parse() requires 1 argument".to_string()); }
            if let Value::String(json_str) = &args[0] {
                let json_value: serde_json::Value = serde_json::from_str(json_str)
                    .map_err(|e| format!("Invalid JSON: {e}"))?;
                
                fn json_to_value(v: &serde_json::Value) -> Value {
                    match v {
                        serde_json::Value::Null => Value::Nil,
                        serde_json::Value::Bool(b) => Value::Bool(*b),
                        serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
                        serde_json::Value::String(s) => Value::String(s.clone()),
                        serde_json::Value::Array(arr) => {
                            let items: Vec<Value> = arr.iter().map(json_to_value).collect();
                            Value::Array(Rc::new(RefCell::new(items)))
                        }
                        serde_json::Value::Object(obj) => {
                            let mut map = HashMap::new();
                            for (k, v) in obj {
                                map.insert(k.clone(), json_to_value(v));
                            }
                            Value::Table(Rc::new(RefCell::new(map)))
                        }
                    }
                }
                Ok(json_to_value(&json_value))
            } else { Err("json_parse() requires string".to_string()) }
        };
        self.global.borrow_mut().define("json_parse".to_string(), Value::NativeFunction(json_parse_fn));

        // json_stringify
        let json_stringify_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("json_stringify() requires 1 argument".to_string()); }
            
            fn value_to_json(v: &Value) -> serde_json::Value {
                match v {
                    Value::Nil => serde_json::Value::Null,
                    Value::Bool(b) => serde_json::Value::Bool(*b),
                    Value::Number(n) => serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0))),
                    Value::String(s) => serde_json::Value::String(s.clone()),
                    Value::Array(arr) => {
                        let items: Vec<serde_json::Value> = arr.borrow().iter().map(value_to_json).collect();
                        serde_json::Value::Array(items)
                    }
                    Value::Table(t) => {
                        let mut map = serde_json::Map::new();
                        for (k, v) in t.borrow().iter() {
                            map.insert(k.clone(), value_to_json(v));
                        }
                        serde_json::Value::Object(map)
                    }
                    _ => serde_json::Value::Null,
                }
            }
            
            let json_value = value_to_json(&args[0]);
            let json_str = serde_json::to_string(&json_value)
                .map_err(|e| format!("Cannot stringify: {e}"))?;
            Ok(Value::String(json_str))
        };
        self.global.borrow_mut().define("json_stringify".to_string(), Value::NativeFunction(json_stringify_fn));

        // http_get
        let http_get_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("http_get() requires 1 argument (URL)".to_string()); }
            if let Value::String(url) = &args[0] {
                let client = reqwest::blocking::Client::new();
                let response = client.get(url.as_str())
                    .send()
                    .map_err(|e| format!("HTTP GET failed: {e}"))?;
                
                let status = response.status().as_u16();
                let body = response.text()
                    .map_err(|e| format!("Cannot read response: {e}"))?;
                
                let result = Value::new_table();
                if let Value::Table(t) = &result {
                    let mut table = t.borrow_mut();
                    table.insert("status".to_string(), Value::Number(status as f64));
                    table.insert("body".to_string(), Value::String(body));
                }
                Ok(result)
            } else { Err("http_get() requires string URL".to_string()) }
        };
        self.global.borrow_mut().define("http_get".to_string(), Value::NativeFunction(http_get_fn));

        // http_post
        let http_post_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("http_post() requires 2 arguments (URL, body)".to_string()); }
            if let (Value::String(url), Value::String(body)) = (&args[0], &args[1]) {
                let client = reqwest::blocking::Client::new();
                let response = client.post(url.as_str())
                    .body(body.clone())
                    .header("Content-Type", "application/json")
                    .send()
                    .map_err(|e| format!("HTTP POST failed: {e}"))?;
                
                let status = response.status().as_u16();
                let resp_body = response.text()
                    .map_err(|e| format!("Cannot read response: {e}"))?;
                
                let result = Value::new_table();
                if let Value::Table(t) = &result {
                    let mut table = t.borrow_mut();
                    table.insert("status".to_string(), Value::Number(status as f64));
                    table.insert("body".to_string(), Value::String(resp_body));
                }
                Ok(result)
            } else { Err("http_post() requires string URL and string body".to_string()) }
        };
        self.global.borrow_mut().define("http_post".to_string(), Value::NativeFunction(http_post_fn));

        // http_put
        let http_put_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("http_put() requires 2 arguments (URL, body)".to_string()); }
            if let (Value::String(url), Value::String(body)) = (&args[0], &args[1]) {
                let client = reqwest::blocking::Client::new();
                let response = client.put(url.as_str())
                    .body(body.clone())
                    .header("Content-Type", "application/json")
                    .send()
                    .map_err(|e| format!("HTTP PUT failed: {e}"))?;
                
                let status = response.status().as_u16();
                let resp_body = response.text()
                    .map_err(|e| format!("Cannot read response: {e}"))?;
                
                let result = Value::new_table();
                if let Value::Table(t) = &result {
                    let mut table = t.borrow_mut();
                    table.insert("status".to_string(), Value::Number(status as f64));
                    table.insert("body".to_string(), Value::String(resp_body));
                }
                Ok(result)
            } else { Err("http_put() requires string URL and string body".to_string()) }
        };
        self.global.borrow_mut().define("http_put".to_string(), Value::NativeFunction(http_put_fn));

        // http_delete
        let http_delete_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("http_delete() requires 1 argument (URL)".to_string()); }
            if let Value::String(url) = &args[0] {
                let client = reqwest::blocking::Client::new();
                let response = client.delete(url.as_str())
                    .send()
                    .map_err(|e| format!("HTTP DELETE failed: {e}"))?;
                
                let status = response.status().as_u16();
                let body = response.text()
                    .map_err(|e| format!("Cannot read response: {e}"))?;
                
                let result = Value::new_table();
                if let Value::Table(t) = &result {
                    let mut table = t.borrow_mut();
                    table.insert("status".to_string(), Value::Number(status as f64));
                    table.insert("body".to_string(), Value::String(body));
                }
                Ok(result)
            } else { Err("http_delete() requires string URL".to_string()) }
        };
        self.global.borrow_mut().define("http_delete".to_string(), Value::NativeFunction(http_delete_fn));

        // crypto_md5
        let crypto_md5_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("crypto_md5() requires 1 argument".to_string()); }
            let data = match &args[0] {
                Value::String(s) => s.as_bytes(),
                _ => return Err("crypto_md5() requires string".to_string()),
            };
            let hash = md5::compute(data);
            Ok(Value::String(format!("{hash:x}")))
        };
        self.global.borrow_mut().define("crypto_md5".to_string(), Value::NativeFunction(crypto_md5_fn));

        // crypto_sha256
        let crypto_sha256_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("crypto_sha256() requires 1 argument".to_string()); }
            use sha2::{Sha256, Digest};
            let data = match &args[0] {
                Value::String(s) => s.as_bytes(),
                _ => return Err("crypto_sha256() requires string".to_string()),
            };
            let mut hasher = Sha256::new();
            hasher.update(data);
            let hash = hasher.finalize();
            Ok(Value::String(format!("{hash:x}")))
        };
        self.global.borrow_mut().define("crypto_sha256".to_string(), Value::NativeFunction(crypto_sha256_fn));

        // crypto_base64_encode
        let crypto_base64_encode_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("crypto_base64_encode() requires 1 argument".to_string()); }
            let data = match &args[0] {
                Value::String(s) => s.as_bytes(),
                _ => return Err("crypto_base64_encode() requires string".to_string()),
            };
            use base64::{Engine as _, engine::general_purpose};
            Ok(Value::String(general_purpose::STANDARD.encode(data)))
        };
        self.global.borrow_mut().define("crypto_base64_encode".to_string(), Value::NativeFunction(crypto_base64_encode_fn));

        // crypto_base64_decode
        let crypto_base64_decode_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("crypto_base64_decode() requires 1 argument".to_string()); }
            let data = match &args[0] {
                Value::String(s) => s.as_bytes(),
                _ => return Err("crypto_base64_decode() requires string".to_string()),
            };
            use base64::{Engine as _, engine::general_purpose};
            let decoded = general_purpose::STANDARD.decode(data)
                .map_err(|e| format!("Invalid base64: {e}"))?;
            Ok(Value::String(String::from_utf8_lossy(&decoded).to_string()))
        };
        self.global.borrow_mut().define("crypto_base64_decode".to_string(), Value::NativeFunction(crypto_base64_decode_fn));

        // crypto_uuid
        let crypto_uuid_fn = |_args: &[Value]| -> Result<Value, String> {
            Ok(Value::String(uuid::Uuid::new_v4().to_string()))
        };
        self.global.borrow_mut().define("crypto_uuid".to_string(), Value::NativeFunction(crypto_uuid_fn));

        // crypto_random_bytes
        let crypto_random_bytes_fn = |args: &[Value]| -> Result<Value, String> {
            if args.is_empty() { return Err("crypto_random_bytes() requires 1 argument".to_string()); }
            let len = match &args[0] {
                Value::Number(n) => *n as usize,
                _ => return Err("crypto_random_bytes() requires number".to_string()),
            };
            use std::time::{SystemTime, UNIX_EPOCH};
            let mut bytes = Vec::new();
            for i in 0..len {
                let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64;
                bytes.push((seed + i as u64) as u8);
            }
            Ok(Value::Array(Rc::new(RefCell::new(
                bytes.into_iter().map(|b| Value::Number(b as f64)).collect()
            ))))
        };
        self.global.borrow_mut().define("crypto_random_bytes".to_string(), Value::NativeFunction(crypto_random_bytes_fn));

        // date_now
        let date_now_fn = |_args: &[Value]| -> Result<Value, String> {
            use chrono::Utc;
            Ok(Value::Number(Utc::now().timestamp() as f64))
        };
        self.global.borrow_mut().define("date_now".to_string(), Value::NativeFunction(date_now_fn));

        // date_format
        let date_format_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("date_format() requires 2 arguments".to_string()); }
            let timestamp = match &args[0] {
                Value::Number(n) => *n as i64,
                _ => return Err("date_format() requires number timestamp".to_string()),
            };
            let format_str = match &args[1] {
                Value::String(s) => s,
                _ => return Err("date_format() requires string format".to_string()),
            };
            use chrono::{DateTime, Utc};
            let dt: DateTime<Utc> = DateTime::from_timestamp(timestamp, 0)
                .ok_or("Invalid timestamp")?;
            let formatted = dt.format(format_str).to_string();
            Ok(Value::String(formatted))
        };
        self.global.borrow_mut().define("date_format".to_string(), Value::NativeFunction(date_format_fn));

        // regex_match
        let regex_match_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("regex_match() requires 2 arguments".to_string()); }
            let pattern = match &args[0] {
                Value::String(s) => s,
                _ => return Err("regex_match() requires string pattern".to_string()),
            };
            let text = match &args[1] {
                Value::String(s) => s,
                _ => return Err("regex_match() requires string text".to_string()),
            };
            let re = regex::Regex::new(pattern)
                .map_err(|e| format!("Invalid regex: {e}"))?;
            Ok(Value::Bool(re.is_match(text)))
        };
        self.global.borrow_mut().define("regex_match".to_string(), Value::NativeFunction(regex_match_fn));

        // regex_find
        let regex_find_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("regex_find() requires 2 arguments".to_string()); }
            let pattern = match &args[0] {
                Value::String(s) => s,
                _ => return Err("regex_find() requires string pattern".to_string()),
            };
            let text = match &args[1] {
                Value::String(s) => s,
                _ => return Err("regex_find() requires string text".to_string()),
            };
            let re = regex::Regex::new(pattern)
                .map_err(|e| format!("Invalid regex: {e}"))?;
            if let Some(m) = re.find(text) {
                let result = Value::new_table();
                if let Value::Table(t) = &result {
                    let mut table = t.borrow_mut();
                    table.insert("start".to_string(), Value::Number(m.start() as f64));
                    table.insert("end".to_string(), Value::Number(m.end() as f64));
                    table.insert("text".to_string(), Value::String(m.as_str().to_string()));
                }
                Ok(result)
            } else {
                Ok(Value::Nil)
            }
        };
        self.global.borrow_mut().define("regex_find".to_string(), Value::NativeFunction(regex_find_fn));

        // regex_replace
        let regex_replace_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 3 { return Err("regex_replace() requires 3 arguments".to_string()); }
            let pattern = match &args[0] {
                Value::String(s) => s,
                _ => return Err("regex_replace() requires string pattern".to_string()),
            };
            let text = match &args[1] {
                Value::String(s) => s,
                _ => return Err("regex_replace() requires string text".to_string()),
            };
            let replacement = match &args[2] {
                Value::String(s) => s,
                _ => return Err("regex_replace() requires string replacement".to_string()),
            };
            let re = regex::Regex::new(pattern)
                .map_err(|e| format!("Invalid regex: {e}"))?;
            Ok(Value::String(re.replace(text, replacement).to_string()))
        };
        self.global.borrow_mut().define("regex_replace".to_string(), Value::NativeFunction(regex_replace_fn));

        // regex_find_all
        let regex_find_all_fn = |args: &[Value]| -> Result<Value, String> {
            if args.len() < 2 { return Err("regex_find_all() requires 2 arguments".to_string()); }
            let pattern = match &args[0] {
                Value::String(s) => s,
                _ => return Err("regex_find_all() requires string pattern".to_string()),
            };
            let text = match &args[1] {
                Value::String(s) => s,
                _ => return Err("regex_find_all() requires string text".to_string()),
            };
            let re = regex::Regex::new(pattern)
                .map_err(|e| format!("Invalid regex: {e}"))?;
            let matches: Vec<Value> = re.find_iter(text)
                .map(|m| {
                    let result = Value::new_table();
                    if let Value::Table(t) = &result {
                        let mut table = t.borrow_mut();
                        table.insert("start".to_string(), Value::Number(m.start() as f64));
                        table.insert("end".to_string(), Value::Number(m.end() as f64));
                        table.insert("text".to_string(), Value::String(m.as_str().to_string()));
                    }
                    result
                })
                .collect();
            Ok(Value::Array(Rc::new(RefCell::new(matches))))
        };
        self.global.borrow_mut().define("regex_find_all".to_string(), Value::NativeFunction(regex_find_all_fn));
    }

    pub fn interpret(&mut self, program: Program) -> Result<(), String> {
        for stmt in program.statements {
            if let Err(e) = self.execute_stmt(stmt) {
                return Err(self.format_error_with_traceback(&e));
            }
        }
        Ok(())
    }

    fn format_error_with_traceback(&self, error: &str) -> String {
        let mut msg = format!("❌ Error: {error}\n");
        msg.push_str("\n📍 Stack traceback:\n");
        
        if self.call_stack.is_empty() {
            msg.push_str("  [main chunk]\n");
        } else {
            for (i, frame) in self.call_stack.iter().enumerate() {
                msg.push_str(&format!("  [{}]: {} at line {}\n", i, frame.function, frame.line));
            }
            msg.push_str("  [main chunk]\n");
        }
        
        msg
    }

    fn execute_stmt(&mut self, stmt: Stmt) -> Result<(), String> {
        let line = self.get_stmt_line(&stmt);
        self.current_line = line;
        
        match stmt {
            Stmt::Declaration { name, initializer, line: _, is_const } => {
                let value = if let Some(expr) = initializer {
                    self.evaluate_expr(expr)?
                } else {
                    Value::Nil
                };
                if is_const {
                    // Помечаем как константу (добавляем префикс __const__)
                    self.global.borrow_mut().define(format!("__const__{name}"), value);
                } else {
                    self.global.borrow_mut().define(name, value);
                }
                Ok(())
            }
            Stmt::Destructure { names, initializer, line: _ } => {
                let value = self.evaluate_expr(initializer)?;
                match value {
                    Value::Array(arr) => {
                        let array = arr.borrow();
                        for (i, name) in names.iter().enumerate() {
                            if i < array.len() {
                                self.global.borrow_mut().define(name.clone(), array[i].clone());
                            } else {
                                self.global.borrow_mut().define(name.clone(), Value::Nil);
                            }
                        }
                    }
                    _ => {
                        // Single value - assign to first variable, rest are nil
                        self.global.borrow_mut().define(names[0].clone(), value);
                        for name in names.iter().skip(1) {
                            self.global.borrow_mut().define(name.clone(), Value::Nil);
                        }
                    }
                }
                Ok(())
            }
            Stmt::Assignment { name, value, line, .. } => {
                let val = self.evaluate_expr(value)?;
                // Проверяем, не является ли переменная константой
                if self.global.borrow().get(&format!("__const__{name}")).is_ok() {
                    return Err(format!("Cannot assign to const '{name}' at line {line}"));
                }
                self.global.borrow().assign(&name, val.clone())
                    .map_err(|e| format!("{e} at line {line}"))?;
                Ok(())
            }
            Stmt::Expression(expr) => {
                self.evaluate_expr(expr)?;
                Ok(())
            }
            Stmt::Return(expr) => {
                let value = if let Some(e) = expr {
                    self.evaluate_expr(e)?
                } else {
                    Value::Nil
                };
                self.return_value = Some(value.clone());
                Ok(())
            }
            Stmt::ReturnMulti(values) => {
                let arr = Rc::new(RefCell::new(Vec::new()));
                for v in values {
                    let val = self.evaluate_expr(v)?;
                    arr.borrow_mut().push(val);
                }
                self.return_value = Some(Value::Array(arr));
                Ok(())
            }
            Stmt::If { condition, then_branch, else_branch, line: _ } => {
                let cond_val = self.evaluate_expr(condition)?;
                if cond_val.is_truthy() {
                    for stmt in then_branch {
                        self.execute_stmt(stmt)?;
                        if self.return_value.is_some() { return Ok(()); }
                    }
                } else if let Some(else_b) = else_branch {
                    for stmt in else_b {
                        self.execute_stmt(stmt)?;
                        if self.return_value.is_some() { return Ok(()); }
                    }
                }
                Ok(())
            }
            Stmt::While { condition, body, line: _ } => {
                while self.evaluate_expr(condition.clone())?.is_truthy() {
                    for stmt in &body {
                        self.execute_stmt(stmt.clone())?;
                        if self.return_value.is_some() { return Ok(()); }
                        if self.control_flow == ControlFlow::Break {
                            self.control_flow = ControlFlow::None;
                            return Ok(());
                        }
                        if self.control_flow == ControlFlow::Continue {
                            self.control_flow = ControlFlow::None;
                            break;
                        }
                    }
                }
                Ok(())
            }
            Stmt::For { initializer, condition, increment, body, line: _ } => {
                if let Some(init) = initializer {
                    self.evaluate_expr(*init)?;
                }
                while self.evaluate_expr((*condition).clone())?.is_truthy() {
                    for stmt in &body {
                        self.execute_stmt(stmt.clone())?;
                        if self.return_value.is_some() { return Ok(()); }
                        if self.control_flow == ControlFlow::Break {
                            self.control_flow = ControlFlow::None;
                            return Ok(());
                        }
                        if self.control_flow == ControlFlow::Continue {
                            self.control_flow = ControlFlow::None;
                            break;
                        }
                    }
                    if let Some(ref inc) = increment {
                        self.evaluate_expr((**inc).clone())?;
                    }
                }
                Ok(())
            }
            Stmt::ForIn { variable, iterable, body, line: _ } => {
                let iter_val = self.evaluate_expr(iterable)?;
                match iter_val {
                    Value::Array(arr) => {
                        for item in arr.borrow().iter() {
                            let parent = self.global.clone();
                            let env = Environment::with_parent(parent);
                            let old_global = self.global.clone();
                            self.global = Rc::new(RefCell::new(env));
                            self.global.borrow_mut().define(variable.clone(), item.clone());
                            for stmt in &body {
                                self.execute_stmt(stmt.clone())?;
                                if self.control_flow == ControlFlow::Break {
                                    self.control_flow = ControlFlow::None;
                                    self.global = old_global;
                                    return Ok(());
                                }
                                if self.control_flow == ControlFlow::Continue {
                                    self.control_flow = ControlFlow::None;
                                    break;
                                }
                            }
                            self.global = old_global;
                        }
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            let parent = self.global.clone();
                            let env = Environment::with_parent(parent);
                            let old_global = self.global.clone();
                            self.global = Rc::new(RefCell::new(env));
                            self.global.borrow_mut().define(variable.clone(), Value::String(ch.to_string()));
                            for stmt in &body {
                                self.execute_stmt(stmt.clone())?;
                                if self.control_flow == ControlFlow::Break {
                                    self.control_flow = ControlFlow::None;
                                    self.global = old_global;
                                    return Ok(());
                                }
                                if self.control_flow == ControlFlow::Continue {
                                    self.control_flow = ControlFlow::None;
                                    break;
                                }
                            }
                            self.global = old_global;
                        }
                    }
                    Value::Range { start, end } => {
                        let start_i = start as i64;
                        let end_i = end as i64;
                        for i in start_i..end_i {
                            let parent = self.global.clone();
                            let env = Environment::with_parent(parent);
                            let old_global = self.global.clone();
                            self.global = Rc::new(RefCell::new(env));
                            self.global.borrow_mut().define(variable.clone(), Value::Number(i as f64));
                            for stmt in &body {
                                self.execute_stmt(stmt.clone())?;
                                if self.control_flow == ControlFlow::Break {
                                    self.control_flow = ControlFlow::None;
                                    self.global = old_global;
                                    return Ok(());
                                }
                                if self.control_flow == ControlFlow::Continue {
                                    self.control_flow = ControlFlow::None;
                                    break;
                                }
                            }
                            self.global = old_global;
                        }
                    }
                    _ => return Err(format!("Cannot iterate over {iter_val}")),
                }
                Ok(())
            }
            Stmt::Function { name, params, body, line: _ } => {
                let param_names: Vec<String> = params.iter().map(|(p, _)| p.clone()).collect();
                let defaults: Vec<crate::ast::Expr> = params.iter()
                    .filter_map(|(_, d)| d.clone())
                    .collect();
                let func = Value::Function {
                    name: name.clone(),
                    params: param_names,
                    default_params: defaults,
                    body,
                    closure: self.global.clone(),
                };
                self.global.borrow_mut().define(name, func);
                Ok(())
            }
            Stmt::Block(stmts) => {
                let parent = self.global.clone();
                let env = Environment::with_parent(parent);
                let old_global = self.global.clone();
                self.global = Rc::new(RefCell::new(env));
                for stmt in stmts {
                    self.execute_stmt(stmt)?;
                    if self.return_value.is_some() {
                        self.global = old_global;
                        return Ok(());
                    }
                }
                self.global = old_global;
                Ok(())
            }
            Stmt::Import { path, alias, line } => {
                let full_path = self.base_path.join(&path);
                let source = fs::read_to_string(&full_path)
                    .map_err(|e| format!("Cannot read module '{path}': {e} at line {line}"))?;

                let tokens = lexer::tokenize(&source);
                let mut parser = crate::parser::Parser::new(tokens);
                let program = parser.parse()
                    .map_err(|e| format!("Parse error in '{path}': {e} at line {line}"))?;

                // Модуль использует ту же глобальную среду для доступа к встроенным функциям
                let module_env = Rc::new(RefCell::new(Environment::with_parent(self.global.clone())));
                let mut module_interp = Interpreter::new(self.base_path.clone());
                module_interp.global = module_env.clone();
                module_interp.interpret(program)?;

                // Экспортируем только функции, определённые в модуле
                let exports = Value::new_table();
                if let Value::Table(ref t) = exports {
                    let env_lock = module_env.borrow();
                    for (k, v) in env_lock.values.borrow().iter() {
                        t.borrow_mut().insert(k.clone(), v.clone());
                    }
                }

                let name = alias.unwrap_or_else(|| {
                    PathBuf::from(&path).file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("module")
                        .to_string()
                });
                self.global.borrow_mut().define(name, exports);
                Ok(())
            }
            Stmt::Break => {
                self.control_flow = ControlFlow::Break;
                Ok(())
            }
            Stmt::Continue => {
                self.control_flow = ControlFlow::Continue;
                Ok(())
            }
            Stmt::Try { body, catch_var, catch_body, line: _ } => {
                let mut caught_error = None;
                
                for stmt in body {
                    match self.execute_stmt(stmt) {
                        Ok(_) => {}
                        Err(e) => {
                            caught_error = Some(Value::Error(e));
                            break;
                        }
                    }
                }
                
                if let Some(err) = caught_error {
                    if !catch_body.is_empty() {
                        if let Some(var_name) = catch_var {
                            let parent = self.global.clone();
                            let env = Environment::with_parent(parent);
                            let old_global = self.global.clone();
                            self.global = Rc::new(RefCell::new(env));
                            self.global.borrow_mut().define(var_name, err);
                            for stmt in catch_body {
                                self.execute_stmt(stmt)?;
                            }
                            self.global = old_global;
                        } else {
                            for stmt in catch_body {
                                self.execute_stmt(stmt)?;
                            }
                        }
                    }
                }
                
                Ok(())
            }
        }
    }

    fn get_stmt_line(&self, stmt: &Stmt) -> usize {
        match stmt {
            Stmt::Declaration { line, .. } => *line,
            Stmt::Destructure { line, .. } => *line,
            Stmt::Assignment { line, .. } => *line,
            Stmt::If { line, .. } => *line,
            Stmt::While { line, .. } => *line,
            Stmt::For { line, .. } => *line,
            Stmt::ForIn { line, .. } => *line,
            Stmt::Function { line, .. } => *line,
            Stmt::Import { line, .. } => *line,
            Stmt::Try { line, .. } => *line,
            _ => self.current_line,
        }
    }

    fn evaluate_expr(&mut self, expr: Expr) -> Result<Value, String> {
        let line = self.get_line(&expr);
        self.current_line = line;
        
        match expr {
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::String(s) => Ok(Value::String(s)),
            Expr::FString { parts, line: _ } => {
                let mut result = String::new();
                for (s, expr_opt) in parts {
                    result.push_str(&s);
                    if let Some(expr) = expr_opt {
                        let val = self.evaluate_expr(*expr)?;
                        result.push_str(&val.to_string());
                    }
                }
                Ok(Value::String(result))
            }
            Expr::LiteralTrue => Ok(Value::Bool(true)),
            Expr::LiteralFalse => Ok(Value::Bool(false)),
            Expr::LiteralNil => Ok(Value::Nil),
            Expr::Identifier(name) => {
                // Сначала пробуем получить как обычную переменную
                match self.global.borrow().get(&name) {
                    Ok(val) => Ok(val),
                    Err(_) => {
                        // Если не найдено, пробуем как const (с префиксом)
                        self.global.borrow().get(&format!("__const__{name}"))
                            .map_err(|e| format!("{e} at line {line}"))
                    }
                }
            }
            Expr::Unary { op, expr, line } => {
                let val = self.evaluate_expr(*expr)?;
                match op {
                    lexer::TokenKind::Minus => {
                        if let Value::Number(n) = val { Ok(Value::Number(-n)) }
                        else { Err(format!("Unary '-' requires a number at line {line}")) }
                    }
                    lexer::TokenKind::Not => Ok(Value::Bool(!val.is_truthy())),
                    _ => Err(format!("Unknown unary operator at line {line}")),
                }
            }
            Expr::Binary { left, op, right, line } => {
                let l = self.evaluate_expr(*left)?;
                let r = self.evaluate_expr(*right)?;
                self.evaluate_binary(l, op, r, line)
            }
            Expr::Call { callee, arguments, line } => {
                let func = self.evaluate_expr(*callee)?;
                let args = arguments.into_iter().map(|a| self.evaluate_expr(a)).collect::<Result<Vec<_>, _>>()?;
                match func {
                    Value::NativeFunction(f) => {
                        self.call_stack.push(StackFrame { function: "<native>".to_string(), line });
                        let result = f(&args);
                        self.call_stack.pop();
                        result
                    }
                    Value::Function { name, body, closure, params, default_params } => {
                        self.call_stack.push(StackFrame { function: name.clone(), line });

                        let func_env = Environment::with_parent(closure.clone());
                        let func_env = Rc::new(RefCell::new(func_env));
                        let old_global = self.global.clone();
                        let old_return = self.return_value.take();
                        self.global = func_env.clone();
                        self.return_value = None;

                        for (i, param) in params.iter().enumerate() {
                            let value = if i < args.len() {
                                args[i].clone()
                            } else if i < default_params.len() {
                                let mut default_interp = Interpreter::new(self.base_path.clone());
                                default_interp.global = func_env.clone();
                                default_interp.evaluate_expr(default_params[i].clone()).unwrap_or(Value::Nil)
                            } else {
                                Value::Nil
                            };
                            func_env.borrow_mut().define(param.clone(), value);
                        }

                        let mut result = Ok(Value::Nil);
                        for stmt in body {
                            if let Err(e) = self.execute_stmt(stmt) {
                                result = Err(e);
                                break;
                            }
                            if let Some(ret) = self.return_value.take() {
                                self.global = old_global;
                                self.return_value = old_return;
                                self.call_stack.pop();
                                return Ok(ret);
                            }
                        }

                        self.global = old_global;
                        self.return_value = old_return;
                        self.call_stack.pop();

                        result?;
                        Ok(Value::Nil)
                    }
                    _ => Err(format!("Can only call functions at line {line}")),
                }
            }
            Expr::Table { entries, line: _ } => {
                let table = Value::new_table();
                if let Value::Table(ref t) = table {
                    for (key, expr) in entries {
                        let value = self.evaluate_expr(expr)?;
                        t.borrow_mut().insert(key, value);
                    }
                }
                Ok(table)
            }
            Expr::Array { items, line: _ } => {
                let arr = Rc::new(RefCell::new(Vec::new()));
                for item in items {
                    let val = self.evaluate_expr(item)?;
                    arr.borrow_mut().push(val);
                }
                Ok(Value::Array(arr))
            }
            Expr::Range { start, end, line: _ } => {
                let start_val = self.evaluate_expr(*start)?;
                let end_val = self.evaluate_expr(*end)?;
                match (start_val, end_val) {
                    (Value::Number(s), Value::Number(e)) => {
                        Ok(Value::Range { start: s, end: e })
                    }
                    _ => Err("Range requires numbers".to_string()),
                }
            }
            Expr::Index { object, index, line } => {
                let obj = self.evaluate_expr(*object)?;
                let idx = self.evaluate_expr(*index)?;
                match (obj, idx) {
                    (Value::Array(arr), Value::Number(i)) => {
                        let idx = i as usize;
                        let array = arr.borrow();
                        if idx < array.len() {
                            Ok(array[idx].clone())
                        } else {
                            Err(format!("Index {idx} out of bounds at line {line}"))
                        }
                    }
                    (Value::Table(t), Value::String(key)) => {
                        t.borrow().get(&key).cloned()
                            .ok_or_else(|| format!("Field '{key}' not found at line {line}"))
                    }
                    _ => Err(format!("Invalid index operation at line {line}")),
                }
            }
            Expr::SetIndex { object, index, value, line } => {
                let obj = self.evaluate_expr(*object)?;
                let idx = self.evaluate_expr(*index)?;
                let val = self.evaluate_expr(*value)?;
                match (obj, idx) {
                    (Value::Array(arr), Value::Number(i)) => {
                        let idx = i as usize;
                        let mut array = arr.borrow_mut();
                        if idx < array.len() {
                            array[idx] = val;
                            Ok(Value::Nil)
                        } else {
                            Err(format!("Index {idx} out of bounds at line {line}"))
                        }
                    }
                    (Value::Table(t), Value::String(key)) => {
                        t.borrow_mut().insert(key, val);
                        Ok(Value::Nil)
                    }
                    _ => Err(format!("Invalid index assignment at line {line}")),
                }
            }
            Expr::Length { expr, line } => {
                let val = self.evaluate_expr(*expr)?;
                match val {
                    Value::Array(arr) => Ok(Value::Number(arr.borrow().len() as f64)),
                    Value::String(s) => Ok(Value::Number(s.len() as f64)),
                    Value::Table(t) => Ok(Value::Number(t.borrow().len() as f64)),
                    _ => Err(format!("Length operator requires array, string or table at line {line}")),
                }
            }
            Expr::Get { object, name, line } => {
                let obj = self.evaluate_expr(*object)?;
                match obj {
                    Value::Table(t) => t.borrow().get(&name).cloned()
                        .ok_or_else(|| format!("Field '{name}' not found at line {line}")),
                    _ => Err(format!("Can only get fields from tables at line {line}")),
                }
            }
            Expr::SafeGet { object, name, line: _ } => {
                let obj = self.evaluate_expr(*object)?;
                match obj {
                    Value::Nil => Ok(Value::Nil),
                    Value::Table(t) => Ok(t.borrow().get(&name).cloned().unwrap_or(Value::Nil)),
                    _ => Ok(Value::Nil),
                }
            }
            Expr::Set { object, name, value, line } => {
                let obj = self.evaluate_expr(*object)?;
                let val = self.evaluate_expr(*value)?;
                match obj {
                    Value::Table(t) => { t.borrow_mut().insert(name, val); Ok(Value::Nil) }
                    _ => Err(format!("Can only set fields on tables at line {line}")),
                }
            }
            Expr::Throw { expr, line } => {
                let val = self.evaluate_expr(*expr)?;
                let msg = match val {
                    Value::String(s) => s,
                    _ => val.to_string(),
                };
                Err(format!("{msg} at line {line}"))
            }
            Expr::FunctionLiteral { params, body, line: _ } => {
                let param_names: Vec<String> = params.iter().map(|(p, _)| p.clone()).collect();
                let defaults: Vec<crate::ast::Expr> = params.iter()
                    .filter_map(|(_, d)| d.clone())
                    .collect();
                Ok(Value::Function {
                    name: "".to_string(),
                    params: param_names,
                    default_params: defaults,
                    body,
                    closure: self.global.clone(),
                })
            }
        }
    }

    fn get_line(&self, expr: &Expr) -> usize {
        match expr {
            Expr::Binary { line, .. } => *line,
            Expr::Unary { line, .. } => *line,
            Expr::Call { line, .. } => *line,
            Expr::Table { line, .. } => *line,
            Expr::Array { line, .. } => *line,
            Expr::Index { line, .. } => *line,
            Expr::SetIndex { line, .. } => *line,
            Expr::Range { line, .. } => *line,
            Expr::Length { line, .. } => *line,
            Expr::Get { line, .. } => *line,
            Expr::SafeGet { line, .. } => *line,
            Expr::Set { line, .. } => *line,
            Expr::Throw { line, .. } => *line,
            Expr::FunctionLiteral { line, .. } => *line,
            Expr::FString { line, .. } => *line,
            _ => self.current_line,
        }
    }

    fn evaluate_binary(&self, left: Value, op: lexer::TokenKind, right: Value, line: usize) -> Result<Value, String> {
        use lexer::TokenKind::*;
        match op {
            Plus => {
                match (&left, &right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{l}{r}"))),
                    (Value::String(l), Value::Number(r)) => Ok(Value::String(format!("{l}{r}"))),
                    (Value::Number(l), Value::String(r)) => Ok(Value::String(format!("{l}{r}"))),
                    (Value::String(l), Value::Bool(r)) => Ok(Value::String(format!("{l}{r}"))),
                    (Value::Bool(l), Value::String(r)) => Ok(Value::String(format!("{l}{r}"))),
                    (Value::String(l), Value::Nil) => Ok(Value::String(format!("{l}nil"))),
                    (Value::Nil, Value::String(r)) => Ok(Value::String(format!("nil{r}"))),
                    _ => Err(format!("Type mismatch at line {line}")),
                }
            }
            Minus => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Number(l - r))
                } else {
                    Err(format!("Operator '-' requires numbers at line {line}"))
                }
            }
            Star => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Number(l * r))
                } else {
                    Err(format!("Operator '*' requires numbers at line {line}"))
                }
            }
            Slash => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Number(l / r))
                } else {
                    Err(format!("Operator '/' requires numbers at line {line}"))
                }
            }
            Less => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Bool(l < r))
                } else {
                    Err(format!("Operator '<' requires numbers at line {line}"))
                }
            }
            Greater => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Bool(l > r))
                } else {
                    Err(format!("Operator '>' requires numbers at line {line}"))
                }
            }
            LessEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Bool(l <= r))
                } else {
                    Err(format!("Operator '<=' requires numbers at line {line}"))
                }
            }
            GreaterEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Bool(l >= r))
                } else {
                    Err(format!("Operator '>=' requires numbers at line {line}"))
                }
            }
            EqualEqual => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool((l - r).abs() < f64::EPSILON)),
                    (Value::String(l), Value::String(r)) => Ok(Value::Bool(l == r)),
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l == r)),
                    (Value::Nil, Value::Nil) => Ok(Value::Bool(true)),
                    _ => Ok(Value::Bool(false)),
                }
            }
            NotEqual => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool((l - r).abs() >= f64::EPSILON)),
                    (Value::String(l), Value::String(r)) => Ok(Value::Bool(l != r)),
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l != r)),
                    (Value::Nil, Value::Nil) => Ok(Value::Bool(false)),
                    _ => Ok(Value::Bool(true)),
                }
            }
            And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
            _ => Err(format!("Unknown operator at line {line}")),
        }
    }
}