use extendr_api::prelude::*;
use minify_html::{minify as minify_html_native, Cfg};
use std::str::from_utf8;

#[extendr]
fn internal_minify(html: &str,
          do_not_minify_doctype: Option<bool>,
          ensure_spec_compliant_unquoted_attribute_values: Option<bool>,
          keep_closing_tags: Option<bool>,
          keep_html_and_head_opening_tags: Option<bool>,
          keep_spaces_between_attributes: Option<bool>,
          keep_comments: Option<bool>,
          minify_css: Option<bool>,
          minify_js: Option<bool>,
          remove_bangs: Option<bool>,
          remove_processing_instructions: Option<bool>) -> String {

  let code = html.as_bytes();

  let mut cfg = Cfg::new();

  if do_not_minify_doctype.unwrap_or(false) { cfg.do_not_minify_doctype = true; }
  if ensure_spec_compliant_unquoted_attribute_values.unwrap_or(false) { cfg.ensure_spec_compliant_unquoted_attribute_values = true; }
  if keep_closing_tags.unwrap_or(false) { cfg.keep_closing_tags = true; }
  if keep_html_and_head_opening_tags.unwrap_or(false) { cfg.keep_html_and_head_opening_tags = true; }
  if keep_spaces_between_attributes.unwrap_or(false) { cfg.keep_spaces_between_attributes = true; }
  if keep_comments.unwrap_or(false) { cfg.keep_comments = true; }
  if minify_css.unwrap_or(false) { cfg.minify_css = true; println!("minify css"); }
  if minify_js.unwrap_or(false) { cfg.minify_js = true; }
  if remove_bangs.unwrap_or(false) { cfg.remove_bangs = true; }
  if remove_processing_instructions.unwrap_or(false) { cfg.remove_processing_instructions = true; }

  let res = minify_html_native(&code, &cfg);

  String::from(from_utf8(&res).unwrap())

}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod minifyhtml;
    fn internal_minify;
}

