<?php

// Get sample js
$url = 'https://raw.githubusercontent.com/drupal/core/9.4.x/themes/olivero/js/navigation.js';
$non_minified_js = file_get_contents($url);
$url = 'https://raw.githubusercontent.com/drupal/core/9.4.x/themes/olivero/css/base/base.css';
$non_minified_css = file_get_contents($url);

// Minify js
$minifier= new Minifier();
$minified_js = $minifier->jsMinify($non_minified_js);
$minified_css = $minifier->cssMinify($non_minified_css);

var_dump($minified_js);
var_dump($minified_css);