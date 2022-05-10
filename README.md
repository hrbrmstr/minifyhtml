
# minifyhtml

Minify HTML Documents

## Description

Minifies HTML (no CSS or JavaScript yet) via the Rust minify-html crate.

## NOTE

Minifying CSS and JavaScript requires another Rust module with some
features enabled in `minify-html` that doesn’t seem to work on arm64
Macs (yet). Ima figure it out and get that working at some point.

## What’s Inside The Tin

The following functions are implemented:

-   `minify`: Minify UTF-8 HTML code

## Installation

``` r
remotes::install_github("hrbrmstr/minifyhtml")
```

NOTE: To use the ‘remotes’ install options you will need to have the
[{remotes} package](https://github.com/r-lib/remotes) installed.

## Usage

``` r
library(imagecompare)

# current version
packageVersion("minifyhtml")
## [1] '0.1.0'
```

``` r
library(minifyhtml)

'
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <meta charset="UTF-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <!-- COMMENT -->
  <style>
    * { color: black; }
  </style>
  <title>TITTLE</title>
  </head>
  <body>
    <p>
       Some tex
    </p>
    <script>
      console.log("This is a console log message.");
    </script>
  </body>
</html>
' -> src

cat(minify(src, minify_css = TRUE, minify_js = TRUE))
## <html xmlns=http://www.w3.org/1999/xhtml><meta charset=UTF-8><meta content=width=device-width,initial-scale=1 name=viewport><style>* { color: black; }</style><title>TITTLE</title><body><p>Some tex</p><script>console.log("This is a console log message.");</script>
```

``` r
src <- httr::content(httr::GET("https://rud.is"), as = "text")

cat(substr(src, 1, 300))
## <html xmlns="http://www.w3.org/1999/xhtml">
## <head>
## <meta charset="UTF-8"/>
## <meta name="viewport" content="width=device-width, initial-scale=1"/>
## <meta name="theme-color" content="#4575b4"/>
## <!-- meta name="viewport" content="user-scalable=false" / -->
## <meta name="wot-verification" content="682597b61

cat(substr(minify(src), 1, 300))
## <html xmlns=http://www.w3.org/1999/xhtml><meta charset=UTF-8><meta content=width=device-width,initial-scale=1 name=viewport><meta content=#4575b4 name=theme-color><meta content=682597b61c21873545ca name=wot-verification><meta content=ltsLJXWktlxKHpTw8xKoHsF2MnAjV9o7O0FmtGk1c_Y name=google-site-verif
```

## minifyhtml Metrics

| Lang | \# Files |  (%) |  LoC |  (%) | Blank lines |  (%) | \# Lines |  (%) |
|:-----|---------:|-----:|-----:|-----:|------------:|-----:|---------:|-----:|
| Rust |        2 | 0.02 | 7895 | 0.48 |          10 | 0.05 |        4 | 0.03 |
| D    |       20 | 0.19 |  221 | 0.01 |          52 | 0.28 |        0 | 0.00 |
| Rmd  |        1 | 0.01 |   33 | 0.00 |          22 | 0.12 |       32 | 0.23 |
| R    |        3 | 0.03 |   30 | 0.00 |           5 | 0.03 |       31 | 0.22 |
| JSON |       25 | 0.24 |   25 | 0.00 |           0 | 0.00 |        0 | 0.00 |
| TOML |        1 | 0.01 |   13 | 0.00 |           2 | 0.01 |        0 | 0.00 |
| C    |        1 | 0.01 |    4 | 0.00 |           2 | 0.01 |        2 | 0.01 |
| SUM  |       53 | 0.50 | 8221 | 0.50 |          93 | 0.50 |       69 | 0.50 |

clock Package Metrics for minifyhtml

## Code of Conduct

Please note that this project is released with a Contributor Code of
Conduct. By participating in this project you agree to abide by its
terms.
