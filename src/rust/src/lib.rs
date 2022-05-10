use extendr_api::prelude::*;
use minify_html::{minify as minify_html_native, Cfg};
use std::str::from_utf8;

/// Return string `"Hello world!"` to R.
/// @param text text
/// @export
#[extendr]
fn minify(text: &str) -> String {
  let code = text.as_bytes();
  let mut cfg = Cfg::new();
  cfg.keep_comments = false;
  let res = minify_html_native(&code, &cfg);
  String::from(from_utf8(&res).unwrap())
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod minifyhtml;
    fn minify;
}


// fn hello_world(text: &str) -> &'static str {
