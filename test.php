<?php

// Get sample js
$url = 'https://raw.githubusercontent.com/drupal/core/9.4.x/themes/olivero/js/navigation.js';
$non_minified_js = file_get_contents($url);

// Minify js
$minified_js = js_minify($non_minified_js);

var_dump($minified_js);