
<!-- README.md is generated from README.Rmd. Please edit that file -->

# minifyhtml

<!-- badges: start -->
<!-- badges: end -->

The goal of minifyhtml is to …

## Installation

You can install the development version of minifyhtml from
[GitHub](https://github.com/) with:

``` r
# install.packages("devtools")
devtools::install_github("hrbrmstr/minifyhtml")
```

## Example

This is a basic example which shows you how to solve a common problem:

``` r
library(minifyhtml)

src <- httr::content(httr::GET("https://rud.is"), as = "text")

cat(substr(src, 1, 300))
#> <html xmlns="http://www.w3.org/1999/xhtml">
#> <head>
#> <meta charset="UTF-8"/>
#> <meta name="viewport" content="width=device-width, initial-scale=1"/>
#> <meta name="theme-color" content="#4575b4"/>
#> <!-- meta name="viewport" content="user-scalable=false" / -->
#> <meta name="wot-verification" content="682597b61

cat(substr(minify(src), 1, 300))
#> <html xmlns=http://www.w3.org/1999/xhtml><meta charset=UTF-8><meta content=width=device-width,initial-scale=1 name=viewport><meta content=#4575b4 name=theme-color><meta content=682597b61c21873545ca name=wot-verification><meta content=ltsLJXWktlxKHpTw8xKoHsF2MnAjV9o7O0FmtGk1c_Y name=google-site-verif
```
