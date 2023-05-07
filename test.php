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

// Minify JS
$current_microtime = microtime(true);
$minified_js = $minifier->jsMinify($non_minified_js);
$microtime_minify_js = microtime(true) - $current_microtime;

// Minify CSS
$current_microtime = microtime(true);
$minified_css = $minifier->cssMinify($non_minified_css);
$microtime_minify_css = microtime(true) - $current_microtime;

// Print results
print "Minifier class creation: " . $microtime_minifier_class_creation . " seconds" . PHP_EOL;
print PHP_EOL;
print "Minify JS: " . $microtime_minify_js . " seconds" . PHP_EOL;
print PHP_EOL;
var_dump($minified_js);
print PHP_EOL;
print "Minify CSS: " . $microtime_minify_css . " seconds" . PHP_EOL;
print PHP_EOL;
var_dump($minified_css);