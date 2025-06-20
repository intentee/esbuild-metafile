use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::Arc;

use actix_utils::future::Ready;
use actix_utils::future::ok;
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::dev;

use super::EsbuildMetaFile;
use super::asset::Asset;
use super::instance::get_esbuild_metafile;
use super::preloadable_asset::PreloadableAsset;

pub struct HttpPreloader {
    pub includes: RefCell<BTreeSet<Asset>>,
    pub preloads: RefCell<BTreeSet<PreloadableAsset>>,

    esbuild_metafile: Arc<EsbuildMetaFile>,
}

impl HttpPreloader {
    pub fn new(esbuild_metafile: Arc<EsbuildMetaFile>) -> Self {
        Self {
            esbuild_metafile,
            includes: RefCell::new(BTreeSet::new()),
            preloads: RefCell::new(BTreeSet::new()),
        }
    }

    pub fn register_input(&self, input_path: &str) -> Option<()> {
        match self.esbuild_metafile.find_outputs_for_input(input_path) {
            None => None,
            Some(output_paths) => {
                let mut includes = self.includes.borrow_mut();
                let mut preloads = self.preloads.borrow_mut();

                for output_path in output_paths {
                    if includes.insert(Asset::from_path(output_path.clone())) {
                        for preload in self.esbuild_metafile.get_preloads(&output_path) {
                            preloads.insert(PreloadableAsset::from_path(preload));
                        }
                    }
                }

                Some(())
            }
        }
    }

    pub fn register_preload(&self, preload_path: &str) -> Option<()> {
        match self.esbuild_metafile.find_outputs_for_input(preload_path) {
            None => None,
            Some(output_paths) => {
                let mut preloads = self.preloads.borrow_mut();

                for output_path in output_paths {
                    preloads.insert(PreloadableAsset::from_path(output_path.clone()));

                    for preload in self.esbuild_metafile.get_preloads(&output_path) {
                        preloads.insert(PreloadableAsset::from_path(preload));
                    }
                }

                Some(())
            }
        }
    }
}

impl FromRequest for HttpPreloader {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        ok(HttpPreloader::new(get_esbuild_metafile()))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;
    use crate::test::get_metafile_basic;

    #[test]
    fn test_unique_includes_and_preloads() -> Result<()> {
        let metafile = get_metafile_basic()?;
        let preloader = HttpPreloader::new(metafile);

        preloader.register_input("src/main.ts");
        preloader.register_input("src/main.ts");

        let includes = preloader.includes.borrow();

        assert_eq!(includes.len(), 2);
        assert!(includes.contains(&Asset::from_path("dist/main.js".to_string())));
        assert!(includes.contains(&Asset::from_path("dist/main.css".to_string())));

        let preloads = preloader.preloads.borrow();

        assert_eq!(preloads.len(), 3);
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/chunk1.js".to_string())));
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/chunk2.js".to_string())));
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/style1.css".to_string())));

        Ok(())
    }

    #[test]
    fn test_css_preloads_uniqueness() -> Result<()> {
        let metafile = get_metafile_basic()?;
        let preloader = HttpPreloader::new(metafile);

        preloader.register_input("src/style.css");

        let preloads = preloader.preloads.borrow();

        assert_eq!(preloads.len(), 1);
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/style1.css".to_string())));

        Ok(())
    }
}
