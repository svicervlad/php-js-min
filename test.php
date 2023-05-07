<?php

// Get sample js
$url = 'https://raw.githubusercontent.com/drupal/core/9.4.x/themes/olivero/js/navigation.js';
$non_minified_js = file_get_contents($url);
$url = 'https://raw.githubusercontent.com/drupal/core/9.4.x/themes/olivero/css/base/base.css';
$non_minified_css = file_get_contents($url);

// Create minifier class
$current_microtime = microtime(true);
$minifier= new Minifier();
$microtime_minifier_class_creation = microtime(true) - $current_microtime;

// Test Minify JS
$current_microtime = microtime(true);
$minified_js = $minifier->jsMinify($non_minified_js);
$microtime_minify_js = microtime(true) - $current_microtime;

// Test Minify CSS
$current_microtime = microtime(true);
$minified_css = $minifier->cssMinify($non_minified_css);
$microtime_minify_css = microtime(true) - $current_microtime;

// Test jsMinifyAsync
$files_urls =[
    'navigation.js' => 'https://raw.githubusercontent.com/drupal/core/10.1.x/themes/olivero/js/navigation.js',
    'message.theme.js' => 'https://raw.githubusercontent.com/drupal/core/10.1.x/themes/olivero/js/message.theme.js',
    'messages.js' => 'https://raw.githubusercontent.com/drupal/core/10.1.x/themes/olivero/js/messages.js',
    'search.js' => 'https://raw.githubusercontent.com/drupal/core/10.1.x/themes/olivero/js/search.js',
    'second-level-navigation.js' => 'https://raw.githubusercontent.com/drupal/core/10.1.x/themes/olivero/js/second-level-navigation.js',
];

$non_minified_js_async = [];
foreach ($files_urls as $file_name => $url) {
    $non_minified_js_async[$file_name] = file_get_contents($url);
}

// Get size of non minified js
$non_minified_js_async_size = 0;
foreach ($non_minified_js_async as $file_name => $js) {
    $non_minified_js_async_size += mb_strlen($js, '8bit');
}

$current_microtime = microtime(true);
$minified_js_async = $minifier->jsMinifyAsync($non_minified_js_async);
$microtime_minify_js_async = microtime(true) - $current_microtime;

// Get size of minified js
$minified_js_async_size = 0;
foreach ($minified_js_async as $file_name => $js) {
    $minified_js_async_size += mb_strlen($js, '8bit');
}

// Print results
print "Minifier class creation: " . $microtime_minifier_class_creation . " seconds" . PHP_EOL;
print PHP_EOL;
print "Non minified JS: " . mb_strlen($non_minified_js, '8bit') . " bytes" . PHP_EOL;
print "Minified JS: " . mb_strlen($minified_js, '8bit') . " bytes" . PHP_EOL;
print "Minified JS size ratio: " . round(mb_strlen($minified_js, '8bit') / mb_strlen($non_minified_js, '8bit') * 100, 2) . "%" . PHP_EOL;
print PHP_EOL;
print "Minify JS: " . $microtime_minify_js . " seconds" . PHP_EOL;
print PHP_EOL;
var_dump($minified_js);
print PHP_EOL;
print "Minify CSS: " . $microtime_minify_css . " seconds" . PHP_EOL;
print PHP_EOL;
print "Non minified CSS: " . mb_strlen($non_minified_css, '8bit') . " bytes" . PHP_EOL;
print "Minified CSS: " . mb_strlen($minified_css, '8bit') . " bytes" . PHP_EOL;
print "Minified CSS size ratio: " . round(mb_strlen($minified_css, '8bit') / mb_strlen($non_minified_css, '8bit') * 100, 2) . "%" . PHP_EOL;
print PHP_EOL;
var_dump($minified_css);
print PHP_EOL;
print "Minify JS Async: " . $microtime_minify_js_async . " seconds" . PHP_EOL;
print PHP_EOL;
print "Non minified JS size: " . $non_minified_js_async_size . " bytes" . PHP_EOL;
print "Minified JS size: " . $minified_js_async_size . " bytes" . PHP_EOL;
print "Minified JS size ratio: " . round($minified_js_async_size / $non_minified_js_async_size * 100, 2) . "%" . PHP_EOL;
print PHP_EOL;
var_dump($minified_js_async);