//! Minimal escaping for the markup the build scripts generate.
//!
//! `build.rs` interpolates post values into `sitemap.xml` and
//! `src/bin/post_build.rs` interpolates them into the static share-card HTML.
//! Both need the same five predefined XML/HTML entities, so one tested
//! implementation lives here instead of two that can drift apart.
//!
//! Slugs are already restricted to `[a-z0-9-]` (see [`crate::frontmatter::Slug`]);
//! titles and descriptions are author-controlled free text, so escaping here
//! is the actual guard, not just defense in depth.

/// Escape `&`, `<`, `>`, `"`, and `'` as their predefined entities.
pub fn escape(input: &str) -> String {
  let mut out = String::with_capacity(input.len() + 16);
  for c in input.chars() {
    match c {
      '&' => out.push_str("&amp;"),
      '<' => out.push_str("&lt;"),
      '>' => out.push_str("&gt;"),
      '"' => out.push_str("&quot;"),
      '\'' => out.push_str("&apos;"),
      _ => out.push(c),
    }
  }
  out
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn escapes_the_five_predefined_entities() {
    assert_eq!(escape(r#"& < > " '"#), "&amp; &lt; &gt; &quot; &apos;");
  }

  #[test]
  fn leaves_plain_text_and_unicode_alone() {
    assert_eq!(escape("rust-blog ภาษาไทย 2026"), "rust-blog ภาษาไทย 2026");
    assert_eq!(escape(""), "");
  }
}
