#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use minifier::js::minify as js_minify;
use minifier::css::minify as css_minify;


/// The Minifier class.
/// This class provides methods to minify JavaScript and CSS code.
/// @package Minifier
/// @version 1
/// @license MIT
///
/// @example
///  $minifier = new Minifier();
///  $minified_js = $minifier->jsMinify($js_code);
#[php_class]
#[derive(Default)]
struct Minifier;

#[php_impl]
impl Minifier {
    // Constructor
    #[php_method]
    pub fn __construct() -> Minifier {
        Minifier {}
    }

    /// Minifies the given JavaScript code.
    ///
    /// @param string $code
    ///   The JavaScript code to minify.
    ///
    /// @return string
    ///    The minified JavaScript code.
    #[php_method]
    pub fn js_minify(&self, code: String) -> String {
        let js_minified = js_minify(&code[..]);
        js_minified.to_string()
    }

    /// Minifies the given CSS code.
    ///
    /// @param string $code
    ///   The CSS code to minify.
    ///
    /// @return string
    ///   The minified CSS code.
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