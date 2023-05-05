#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use minify_js::{Session, TopLevelMode, minify};

/// Gives you a nice greeting!
/// 
/// @param string $name Your name.
/// 
/// @return string Nice greeting!
#[php_function]
pub fn js_minify(code: String) -> String {
    // Convert string to u8
    let code: &[u8] = code.as_bytes();
    let session = Session::new();
    let mut out = Vec::new();
    minify(&session, TopLevelMode::Global, code, &mut out).unwrap();
    let out = String::from_utf8(out).unwrap();
    out
}

// Required to register the extension with PHP.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}