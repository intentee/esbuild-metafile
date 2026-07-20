use std::fmt::Display;
use std::io;

use askama::Result;
use askama::Values;

use crate::http_preloader::HttpPreloader;

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

#[cfg(test)]
mod tests {
    use askama::NO_VALUES;

    use super::*;
    use crate::test::get_metafile_fonts;

    #[test]
    fn test_asset_filter_errors_for_unknown_input() {
        let preloader = HttpPreloader::new(get_metafile_fonts());

        assert!(asset("resources/ts/unknown.tsx", NO_VALUES, &preloader).is_err());
    }
}
