//! RF-Musette's PLAY and CONFIG surfaces: a Rust WebAssembly client of RackForge's
//! `rackforge.plugin.web@1` bridge (RackForge `docs/WEB_PLUGIN_API.md`),
//! after RF-5's (`rackforge-plugin-rf-5/docs/UI_ARCHITECTURE.md`).
//!
//! What can be tested off the browser is plain Rust: the panel map
//! ([`panel`]), the knobs' travel and readouts ([`dial`]) and the register
//! symbols ([`symbols`]), and the `.rfmusette` files CONFIG exports and
//! imports ([`archive`]). The pages themselves are `browser` (PLAY) and
//! `config` (CONFIG), built for wasm32 only; `rf-musette-lab web-ui` turns
//! them into `package/web/app.js`, which each page's HTML starts.

pub mod archive;
pub mod dial;
pub mod help;
pub mod light;
pub mod panel;
pub mod symbols;
pub mod texture;

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
mod config;

/// Text made safe to place in HTML, as content or a quoted attribute.
pub fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    #[test]
    fn html_is_escaped() {
        assert_eq!(
            super::escape_html("<a href=\"x\">16'&8'</a>"),
            "&lt;a href=&quot;x&quot;&gt;16&#39;&amp;8&#39;&lt;/a&gt;"
        );
    }
}
