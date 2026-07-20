use std::sync::Arc;

use actix_utils::future::Ready;
use actix_utils::future::ok;
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::dev;
use dashmap::DashSet;

use super::EsbuildMetaFile;
use super::asset::Asset;
use super::instance::get_esbuild_metafile;
use super::preloadable_asset::PreloadableAsset;

pub struct HttpPreloader {
    pub includes: DashSet<Asset>,
    pub preloads: DashSet<PreloadableAsset>,

    esbuild_metafile: Arc<EsbuildMetaFile>,
}

impl HttpPreloader {
    pub fn new(esbuild_metafile: Arc<EsbuildMetaFile>) -> Self {
        Self {
            esbuild_metafile,
            includes: DashSet::new(),
            preloads: DashSet::new(),
        }
    }

    pub fn register_input(&self, input_path: &str) -> Option<()> {
        match self.esbuild_metafile.find_outputs_for_input(input_path) {
            None => None,
            Some(output_paths) => {
                for output_path in output_paths {
                    if self.includes.insert(Asset::from_path(output_path.clone())) {
                        for preload in self.esbuild_metafile.get_preloads(&output_path) {
                            self.preloads.insert(PreloadableAsset::from_path(preload));
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
                for output_path in output_paths {
                    self.preloads
                        .insert(PreloadableAsset::from_path(output_path.clone()));

                    for preload in self.esbuild_metafile.get_preloads(&output_path) {
                        self.preloads.insert(PreloadableAsset::from_path(preload));
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
    use super::*;
    use crate::test::get_metafile_basic;

    #[test]
    fn test_unique_includes_and_preloads() {
        let metafile = get_metafile_basic();
        let preloader = HttpPreloader::new(metafile);

        preloader.register_input("src/main.ts");
        preloader.register_input("src/main.ts");

        let includes = &preloader.includes;

        assert_eq!(includes.len(), 2);
        assert!(includes.contains(&Asset::from_path("dist/main.js".to_string())));
        assert!(includes.contains(&Asset::from_path("dist/main.css".to_string())));

        let preloads = &preloader.preloads;

        assert_eq!(preloads.len(), 3);
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/chunk1.js".to_string())));
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/chunk2.js".to_string())));
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/style1.css".to_string())));
    }

    #[test]
    fn test_css_preloads_uniqueness() {
        let metafile = get_metafile_basic();
        let preloader = HttpPreloader::new(metafile);

        preloader.register_input("src/style.css");

        let preloads = preloader.preloads;

        assert_eq!(preloads.len(), 1);
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/style1.css".to_string())));
    }

    #[test]
    fn test_register_input_returns_none_for_unknown_input() {
        let preloader = HttpPreloader::new(get_metafile_basic());

        assert!(preloader.register_input("src/unknown.ts").is_none());
        assert!(preloader.includes.is_empty());
        assert!(preloader.preloads.is_empty());
    }

    #[test]
    fn test_register_preload_registers_mapped_outputs_and_their_preloads() {
        let preloader = HttpPreloader::new(get_metafile_basic());

        assert!(preloader.register_preload("src/main.ts").is_some());

        let preloads = &preloader.preloads;

        assert_eq!(preloads.len(), 5);
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/main.js".to_string())));
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/main.css".to_string())));
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/style1.css".to_string())));
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/chunk1.js".to_string())));
        assert!(preloads.contains(&PreloadableAsset::from_path("dist/chunk2.js".to_string())));
    }

    #[test]
    fn test_register_preload_returns_none_for_unknown_input() {
        let preloader = HttpPreloader::new(get_metafile_basic());

        assert!(preloader.register_preload("src/unknown.ts").is_none());
        assert!(preloader.preloads.is_empty());
    }

    #[test]
    fn test_from_request_builds_preloader_from_global_instance() {
        use actix_web::FromRequest;
        use actix_web::dev::Payload;
        use actix_web::test::TestRequest;

        crate::instance::initialize_instance("{\"outputs\":{}}");

        let request = TestRequest::default().to_http_request();
        let mut payload = Payload::None;
        let preloader = HttpPreloader::from_request(&request, &mut payload).into_inner();

        assert!(preloader.is_ok());
    }
}
