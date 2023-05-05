#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use minifier::js::minify as js_minify;
use minifier::css::minify as css_minify;


#[php_class]
#[derive(Default)]
struct Minifier;

#[php_impl]
impl Minifier {
    #[php_method]
    pub fn __construct() -> Minifier {
        Minifier {}
    }

    #[php_method]
    pub fn js_minify(&self, code: String) -> String {
        let js_minified = js_minify(&code[..]);
        js_minified.to_string()
    }

    #[php_method]
    pub fn css_minify(&self, code: String) -> String {
        let css_minified = css_minify(&code[..]).expect("minification failed");
        css_minified.to_string()
    }
}

// Required to register the extension with PHP.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}