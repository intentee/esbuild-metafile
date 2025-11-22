use std::fmt::Display;
use std::io;

use askama::Result;
use askama::Values;

use super::http_preloader::HttpPreloader;
use super::path_renderer::PathRenderer;

pub fn asset<TDisplay>(
    input_path: TDisplay,
    _values: &dyn Values,
    http_preloader: &HttpPreloader,
) -> Result<String>
where
    TDisplay: Display,
{
    match http_preloader.register_input(&input_path.to_string()) {
        Some(_) => Ok(String::new()),
        None => Err(askama::Error::Custom(Box::new(io::Error::other(format!(
            "esbuild input path not found: {input_path}"
        ))))),
    }
}

pub fn preload<TDisplay>(
    preload_path: TDisplay,
    _values: &dyn Values,
    http_preloader: &HttpPreloader,
) -> Result<String>
where
    TDisplay: Display,
{
    match http_preloader.register_preload(&preload_path.to_string()) {
        Some(_) => Ok(String::new()),
        None => Err(askama::Error::Custom(Box::new(io::Error::other(format!(
            "esbuild preload path not found: {preload_path}"
        ))))),
    }
}

/// Keeps rendered assets sorted for the sake of browser/proxy caching.
pub fn render_assets(http_preloader: &HttpPreloader, _values: &dyn Values) -> Result<String> {
    let path_renderer = PathRenderer {};
    let mut rendered_assets: String = String::new();
    let mut sorted_includes: Vec<_> = Vec::with_capacity(http_preloader.includes.len());
    let mut sorted_preloads: Vec<_> = Vec::with_capacity(http_preloader.preloads.len());

    for path in http_preloader.preloads.iter() {
        sorted_preloads.push(path.render(&path_renderer));
    }

    sorted_preloads.sort();

    for html in sorted_preloads.into_iter() {
        rendered_assets.push_str(&html);
        rendered_assets.push('\n');
    }

    for path in http_preloader.includes.iter() {
        sorted_includes.push(path.render(&path_renderer));
    }

    sorted_includes.sort();

    for html in sorted_includes.into_iter() {
        rendered_assets.push_str(&html);
        rendered_assets.push('\n');
    }

    Ok(rendered_assets)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use askama::Template;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::filters;
    use crate::test::get_metafile_fonts;

    #[derive(Template)]
    #[template(path = "fixtures/template.html")]
    struct WorkbenchTemplate {
        preloads: HttpPreloader,
    }

    #[test]
    fn test_asset_filter() -> Result<()> {
        let preloads = HttpPreloader::new(get_metafile_fonts()?);
        let template = WorkbenchTemplate {
            preloads,
        };

        let rendered = template.render()?;

        assert_eq!(
            rendered.trim(),
            indoc! {r#"
                <link rel="modulepreload" href="/static/chunk-EMZKCXNJ.js">
                <link rel="modulepreload" href="/static/chunk-PI4ZFSEL.js">
                <link rel="preload" href="/static/logo_XSTJPNLH.png" as="image">
                <link rel="preload" href="https://fonts/font1.woff2" as="font" crossorigin>
                <link rel="preload" href="https://fonts/font3.woff2" as="font" crossorigin>
                <link rel="stylesheet" href="/static/controller_foo_CX2Z63ZH.css">
                <script async src="/static/controller_foo_CTJMZK66.js" type="module"></script>
            "#}
            .trim()
        );

        Ok(())
    }
}
