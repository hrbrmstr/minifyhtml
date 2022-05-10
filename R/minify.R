#' Minify UTF-8 HTML code
#'
#' @param html length 1 character vector of HTML
#' @param do_not_minify_doctype Do not minify DOCTYPEs. Minified DOCTYPEs may not be spec compliant.
#' @param ensure_spec_compliant_unquoted_attribute_values Ensure all unquoted attribute values in the output do not contain any characters prohibited by the WHATWG spec
#' @param keep_closing_tags Do not omit closing tags when possible. Do not omit closing tags when possible.
#' @param keep_html_and_head_opening_tags Do not omit html and head opening tags when they don't have attributes.
#' @param keep_spaces_between_attributes Keep spaces between attributes when possible to conform to HTML standards.
#' @param keep_comments Keep all comments.
#' @param minify_css TODO (this requires another Rust module and adding it with features enabled doesn't seem to work on macOS)
#' @param minify_js TODO (this requires another Rust module and adding it with features enabled doesn't seem to work on macOS)
#' @param remove_bangs Remove all bangs.
#' @param remove_processing_instructions  Remove all processing_instructions.
#' @return length 1 character vector of minified HTML code
#' @note All logical parameters default to `FALSE`
#' @export
minify <- function(
    html,
    do_not_minify_doctype = FALSE,
    ensure_spec_compliant_unquoted_attribute_values = FALSE,
    keep_closing_tags = FALSE,
    keep_html_and_head_opening_tags = FALSE,
    keep_spaces_between_attributes = FALSE,
    keep_comments = FALSE,
    minify_css = FALSE,
    minify_js = FALSE,
    remove_bangs = FALSE,
    remove_processing_instructions = FALSE
) {

  internal_minify(
    html = html,
    do_not_minify_doctype = do_not_minify_doctype,
    ensure_spec_compliant_unquoted_attribute_values = ensure_spec_compliant_unquoted_attribute_values,
    keep_closing_tags = keep_closing_tags,
    keep_html_and_head_opening_tags = keep_html_and_head_opening_tags,
    keep_spaces_between_attributes = keep_spaces_between_attributes,
    keep_comments = keep_comments,
    minify_css = minify_css,
    minify_js = minify_js,
    remove_bangs = remove_bangs,
    remove_processing_instructions = remove_processing_instructions
  )

}
