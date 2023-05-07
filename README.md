# CSS and JS minimization extension for PHP

Extention to provide minification of CSS and JS code for PHP.

## Installation

```bash
cargo install cargo-php
cargo php install --release
```

## Rebulding stub for PHP intelephense

```bash
cargo php stubs
```

## Usage

```php
<?php

$minifier = new Minifier();
$minified_js = $minifier->jsMinify($js_code);
```

# Testing

```bash
php -f test.php
```