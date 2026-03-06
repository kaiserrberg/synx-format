//! # SYNX Core — The Active Data Format
//!
//! High-performance SYNX parser with full `!active` engine support.
//! Single Rust crate powering all language bindings.
//!
//! ```rust
//! use synx_core::{Synx, Value};
//!
//! let data = Synx::parse("name Wario\nage 30\nactive true");
//! assert_eq!(data["name"], Value::String("Wario".into()));
//! assert_eq!(data["age"], Value::Int(30));
//! ```

mod value;
mod parser;
mod engine;
mod calc;

pub use value::{Value, Mode, ParseResult, Meta, MetaMap, Options};
pub use parser::parse;
pub use engine::resolve;
pub use calc::safe_calc;

/// Main entry point for the SYNX parser.
pub struct Synx;

impl Synx {
    /// Parse a SYNX string into a key-value map (static mode only).
    pub fn parse(text: &str) -> std::collections::HashMap<String, Value> {
        let result = parse(text);
        match result.root {
            Value::Object(map) => map,
            _ => std::collections::HashMap::new(),
        }
    }

    /// Parse with full engine resolution (!active mode).
    pub fn parse_active(text: &str, opts: &Options) -> std::collections::HashMap<String, Value> {
        let mut result = parse(text);
        if result.mode == Mode::Active {
            resolve(&mut result, opts);
        }
        match result.root {
            Value::Object(map) => map,
            _ => std::collections::HashMap::new(),
        }
    }

    /// Parse and return full result including mode and metadata.
    pub fn parse_full(text: &str) -> ParseResult {
        parse(text)
    }

    /// Stringify a Value back to SYNX format.
    pub fn stringify(value: &Value) -> String {
        serialize(value, 0)
    }
}

fn serialize(value: &Value, indent: usize) -> String {
    match value {
        Value::Object(map) => {
            let mut out = String::new();
            let spaces = " ".repeat(indent);
            for (key, val) in map {
                match val {
                    Value::Array(arr) => {
                        out.push_str(&spaces);
                        out.push_str(key);
                        out.push('\n');
                        for item in arr {
                            match item {
                                Value::Object(inner) => {
                                    let entries: Vec<_> = inner.iter().collect();
                                    if let Some((k, v)) = entries.first() {
                                        out.push_str(&spaces);
                                        out.push_str("  - ");
                                        out.push_str(k);
                                        out.push(' ');
                                        out.push_str(&format_primitive(v));
                                        out.push('\n');
                                        for (k, v) in entries.iter().skip(1) {
                                            out.push_str(&spaces);
                                            out.push_str("    ");
                                            out.push_str(k);
                                            out.push(' ');
                                            out.push_str(&format_primitive(v));
                                            out.push('\n');
                                        }
                                    }
                                }
                                _ => {
                                    out.push_str(&spaces);
                                    out.push_str("  - ");
                                    out.push_str(&format_primitive(item));
                                    out.push('\n');
                                }
                            }
                        }
                    }
                    Value::Object(_) => {
                        out.push_str(&spaces);
                        out.push_str(key);
                        out.push('\n');
                        out.push_str(&serialize(val, indent + 2));
                    }
                    Value::String(s) if s.contains('\n') => {
                        out.push_str(&spaces);
                        out.push_str(key);
                        out.push_str(" |\n");
                        for line in s.lines() {
                            out.push_str(&spaces);
                            out.push_str("  ");
                            out.push_str(line);
                            out.push('\n');
                        }
                    }
                    _ => {
                        out.push_str(&spaces);
                        out.push_str(key);
                        out.push(' ');
                        out.push_str(&format_primitive(val));
                        out.push('\n');
                    }
                }
            }
            out
        }
        _ => format_primitive(value),
    }
}

fn format_primitive(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Int(n) => n.to_string(),
        Value::Float(f) => {
            let s = f.to_string();
            if s.contains('.') { s } else { format!("{}.0", s) }
        }
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_primitive).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(_) => "[Object]".to_string(),
        Value::Secret(_) => "[SECRET]".to_string(),
    }
}

/// Write a Value as JSON string (for FFI output).
pub fn write_json(out: &mut String, val: &Value) {
    match val {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Int(n) => {
            let mut buf = itoa::Buffer::new();
            out.push_str(buf.format(*n));
        }
        Value::Float(f) => {
            let mut buf = ryu::Buffer::new();
            out.push_str(buf.format(*f));
        }
        Value::String(s) | Value::Secret(s) => {
            out.push('"');
            for ch in s.chars() {
                match ch {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c if (c as u32) < 0x20 => {
                        out.push_str(&format!("\\u{:04x}", c as u32));
                    }
                    c => out.push(c),
                }
            }
            out.push('"');
        }
        Value::Array(arr) => {
            out.push('[');
            for (i, item) in arr.iter().enumerate() {
                if i > 0 { out.push(','); }
                write_json(out, item);
            }
            out.push(']');
        }
        Value::Object(map) => {
            out.push('{');
            let mut first = true;
            for (key, val) in map {
                if !first { out.push(','); }
                first = false;
                out.push('"');
                out.push_str(key);
                out.push_str("\":");
                write_json(out, val);
            }
            out.push('}');
        }
    }
}

/// Convert a Value to a JSON string.
pub fn to_json(val: &Value) -> String {
    let mut out = String::with_capacity(2048);
    write_json(&mut out, val);
    out
}
