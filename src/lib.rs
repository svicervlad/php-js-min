#![cfg_attr(windows, feature(abi_vectorcall))]

use std::collections::HashMap;

use ext_php_rs::prelude::*;
use minifier::js::minify as js_minify;
use minifier::css::minify as css_minify;
use tokio::runtime::Runtime;


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

    /// Bulk async version of js_minify
    ///
    /// @param array $data
    ///   The JavaScript code to minify.
    ///   The array key is the name of the file.
    ///   The array value is the JavaScript code.
    ///
    /// @return array
    ///   The minified JavaScript code.
    #[php_method]
    fn js_minify_async(&self, data: HashMap<String, String>) -> HashMap<String, String> {
        let mut result = HashMap::new();
        // Use tokio to run the tasks concurrently
        let mut tasks = Vec::new();
        let rt = Runtime::new().unwrap();
        for (key, value) in data {
            let task = rt.spawn(async move {
                let js_minified = js_minify(&value[..]);
                (key, js_minified.to_string())
            });
            tasks.push(task);
        }
        for task in tasks {
            futures::executor::block_on(async{
                let (key, value) = task.await.unwrap();
                result.insert(key, value);
            })
        }
        result
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

    /// Bulk async version of css_minify
    ///
    /// @param array $data
    ///   The CSS code to minify.
    ///   The array key is the name of the file.
    ///   The array value is the CSS code.
    ///
    /// @return array
    ///   The minified CSS code.
    #[php_method]
    fn css_minify_async(&self, data: HashMap<String, String>) -> HashMap<String, String> {
        let mut result = HashMap::new();
        // Use tokio to run the tasks concurrently
        let mut tasks = Vec::new();
        let rt = Runtime::new().unwrap();
        for (key, value) in data {
            let task = rt.spawn(async move {
                let css_minified = css_minify(&value[..]).expect("minification failed");
                (key, css_minified.to_string())
            });
            tasks.push(task);
        }
        for task in tasks {
            futures::executor::block_on(async{
                let (key, value) = task.await.unwrap();
                result.insert(key, value);
            })
        }
        result
    }
}

// Required to register the extension with PHP.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_minify() {
        let minifier = Minifier::default();
        let js_code = "function test() { return 1 + 1; }".to_string();
        let js_minified = minifier.js_minify(js_code);
        assert_eq!(js_minified, "function test(){return 1+1}");
    }

    #[test]
    fn test_css_minify() {
        let minifier = Minifier::default();
        let css_code = "body { color: red; }".to_string();
        let css_minified = minifier.css_minify(css_code);
        assert_eq!(css_minified, "body{color:red;}");
    }

    #[test]
    fn test_js_minify_async() {
        let minifier = Minifier::default();
        let mut data = HashMap::new();
        data.insert("test.js".to_string(), "function test() { return 1 + 1; }".to_string());
        let js_minified = minifier.js_minify_async(data);
        assert_eq!(js_minified.get("test.js").unwrap(), "function test(){return 1+1}");
    }

    #[test]
    fn test_css_minify_async() {
        let minifier = Minifier::default();
        let mut data = HashMap::new();
        data.insert("test.css".to_string(), "body { color: red; }".to_string());
        let css_minified = minifier.css_minify_async(data);
        assert_eq!(css_minified.get("test.css").unwrap(), "body{color:red;}");
    }
}