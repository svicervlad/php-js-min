<?php

// Stubs for rust-php-js-min

namespace {
    /**
     * The Minifier class.
     * This class provides methods to minify JavaScript and CSS code.
     * @package Minifier
     * @version 1
     * @license MIT
     *
     * @example
     *  $minifier = new Minifier();
     *  $minified_js = $minifier->jsMinify($js_code);
     */
    class Minifier {
        public function __construct() {}

        /**
         * Minifies the given JavaScript code.
         *
         * @param string $code
         *   The JavaScript code to minify.
         *
         * @return string
         *    The minified JavaScript code.
         */
        public function jsMinify(string $code): string {}

        /**
         * Bulk async version of js_minify
         *
         * @param array $data
         *   The JavaScript code to minify.
         *   The array key is the name of the file.
         *   The array value is the JavaScript code.
         *
         * @return array
         *   The minified JavaScript code.
         */
        public function jsMinifyAsync(array $data): array {}

        /**
         * Minifies the given CSS code.
         *
         * @param string $code
         *   The CSS code to minify.
         *
         * @return string
         *   The minified CSS code.
         */
        public function cssMinify(string $code): string {}

        /**
         * Bulk async version of css_minify
         *
         * @param array $data
         *   The CSS code to minify.
         *   The array key is the name of the file.
         *   The array value is the CSS code.
         *
         * @return array
         *   The minified CSS code.
         */
        public function cssMinifyAsync(array $data): array {}
    }
}
