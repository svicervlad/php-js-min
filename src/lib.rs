#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use minifier::js::minify;

/// Gives you a nice greeting!
/// 
/// @param string $name Your name.
/// 
/// @return string Nice greeting!
#[php_function]
pub fn js_minify(code: String) -> String {
    let js_minified = minify(&code[..]);
    js_minified.to_string()
}

// Required to register the extension with PHP.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}