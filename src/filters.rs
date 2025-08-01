use std::fmt::Display;
use std::io;

use askama::Result;
use askama::Values;

use super::http_preloader::HttpPreloader;

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

pub fn render_assets(http_preloader: &HttpPreloader, _values: &dyn Values) -> Result<String> {
    let mut rendered_assets: String = String::new();

    for path in http_preloader.preloads.borrow().iter() {
        rendered_assets.push_str(&path.to_string());
    }

    for path in http_preloader.includes.borrow().iter() {
        rendered_assets.push_str(&path.to_string());
    }

    Ok(rendered_assets)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use askama::Template;

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

        let _ = template.render()?;

        Ok(())
    }
}
