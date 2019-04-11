use std::collections::HashMap;
use std::fs;
use std::path::Path;

use amethyst::assets::{Asset, AssetStorage, Format, Handle, Loader};

/// Loads asset from the so-called asset packs
/// It caches assets which you can manually load or unload on demand.
///
/// Example:
/// If the folder structure looks like this
/// /assets/base/sprites/player.png
/// /assets/base/sounds/click.ogg
/// /assets/base/models/cube.obj
/// /assets/mod1/sprites/player.png
/// /assets/mod1/sounds/click.ogg
/// /assets/mod2/sounds/click.ogg
///
/// `resolve_path("sprites/player.png")` -> /assets/mod1/sprites/player.png
/// `resolve_path("models/cube.obj")` -> /assets/base/models/cube.obj
/// `resolve_path("sounds/click.ogg")` -> Unknown.
///
#[allow(clippy::module_name_repetitions)]
pub struct AssetLoader {
    base_path: String,
    default_pack: String,
    asset_packs: Vec<String>,
}

impl AssetLoader {
    pub fn new(base_path: &str, default_pack: &str) -> Self {
        let mut al = Self {
            base_path: Self::sanitize_path_trail_only(&base_path),
            default_pack: Self::sanitize_path(&default_pack),
            asset_packs: Vec::new(),
        };
        al.get_asset_packs();
        al
    }

    fn sanitize_path_trail_only(path: &str) -> String {
        let mut out = path.to_string();
        let chars = path.chars();
        let last = chars.last().unwrap();
        if last == '/' {
            let idx = out.len() - 1;
            out.remove(idx);
        }
        out
    }

    fn sanitize_path(path: &str) -> String {
        let mut out = path.to_string();
        let mut chars = path.chars();
        let first = chars.next().expect("An empty path was specified!");
        let last = chars.last().unwrap();
        out = out.replace("\\", "/").replace("\\\\", "/");
        if first == '/' {
            out.remove(0);
        }
        if out.starts_with('?') {
            out.remove(0);
            out.remove(0);
        }
        if last == '/' {
            let idx = out.len() - 1;
            out.remove(idx);
        }
        out
    }

    pub fn resolve_path(&self, path: &str) -> Option<String> {
        // Try to get from default path
        let mut res = self.resolve_path_for_pack(path, &self.default_pack);

        // Try to find overrides
        for p in &self.asset_packs {
            if p != &self.default_pack {
                if let Some(r) = self.resolve_path_for_pack(path, &p) {
                    res = Some(r);
                }
            }
        }

        res
    }

    fn resolve_path_for_pack(&self, path: &str, pack: &str) -> Option<String> {
        let mut abs = self.base_path.to_owned() + "/" + pack + "/" + &path.to_owned();
        if cfg!(windows) {
            abs = abs.replace("/", "\\").replace("\\\\?\\", "");
        }

        let final_path = Path::new(&abs);
        if final_path.exists() {
            Some(abs.clone())
        } else {
            // TODO: log switch warn!("Failed to find file at path: {}", abs);
            None
        }
    }

    pub fn get_asset_packs(&mut self) -> &Vec<String> {
        let mut buf: Option<Vec<String>> = None;
        if self.asset_packs.is_empty() {
            if let Ok(elems) = fs::read_dir(&self.base_path) {
                buf = Some(
                    elems
                        .map(|e| {
                            let path = &e.unwrap().path();
                            let tmp = &path.to_str().unwrap()[self.base_path.len()..];
                            Self::sanitize_path(&tmp)
                        })
                        .collect(),
                );
            } else {
                // TODO: log switch error!("Failed to find base_path directory for asset loading: {}",self.base_path);
            }
        }

        if let Some(v) = buf {
            self.asset_packs = v;
        }

        &self.asset_packs
    }

    pub fn get_asset_handle<T>(path: &str, ali: &AssetLoaderInternal<T>) -> Option<Handle<T>> {
        ali.assets.get(path).cloned()
    }

    pub fn get_asset<'a, T>(
        path: &str,
        ali: &AssetLoaderInternal<T>,
        storage: &'a AssetStorage<T>,
    ) -> Option<&'a T>
    where
        T: Asset,
    {
        if let Some(h) = Self::get_asset_handle::<T>(path, ali) {
            storage.get(&h)
        } else {
            None
        }
    }

    pub fn get_asset_or_load<'a, T, F>(
        &mut self,
        path: &str,
        format: F,
        options: F::Options,
        ali: &mut AssetLoaderInternal<T>,
        storage: &'a mut AssetStorage<T>,
        loader: &Loader,
    ) -> Option<&'a T>
    where
        T: Asset,
        F: Format<T> + 'static,
    {
        if let Some(h) = Self::get_asset_handle::<T>(path, ali) {
            return storage.get(&h);
            //return Some(a);
        }
        if let Some(h) = self.load::<T, F>(path, format, options, ali, storage, loader) {
            return storage.get(&h);
        }
        None
    }

    pub fn load<T, F>(
        &self,
        path: &str,
        format: F,
        options: F::Options,
        ali: &mut AssetLoaderInternal<T>,
        storage: &mut AssetStorage<T>,
        loader: &Loader,
    ) -> Option<Handle<T>>
    where
        T: Asset,
        F: Format<T> + 'static,
    {
        if let Some(handle) = Self::get_asset_handle(path, ali) {
            return Some(handle);
        }
        if let Some(p) = self.resolve_path(path) {
            let handle: Handle<T> = loader.load(p, format, options, (), storage);
            ali.assets.insert(String::from(path), handle.clone());
            return Some(handle);
        }
        None
    }

    /// Only removes the internal Handle<T>. To truly unload the asset, you need to drop all handles that you have to it.
    pub fn unload<T>(path: &str, ali: &mut AssetLoaderInternal<T>) {
        ali.assets.remove(path);
    }
}

pub struct AssetLoaderInternal<T> {
    /// Map path to asset handle.
    pub assets: HashMap<String, Handle<T>>,
}

impl<T> Default for AssetLoaderInternal<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AssetLoaderInternal<T> {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn load_asset_loader() -> AssetLoader {
        AssetLoader::new(
            &format!("{}/tests/assets", env!("CARGO_MANIFEST_DIR")),
            "main",
        )
    }

    #[test]
    fn path_sanitisation() {
        AssetLoader::new(
            &format!("{}/tests/assets/", env!("CARGO_MANIFEST_DIR")),
            "/base/",
        );
    }

    #[test]
    fn asset_loader_resolve_unique_other() {
        let asset_loader = load_asset_loader();
        assert_eq!(
            asset_loader.resolve_path("config/uniqueother"),
            Some(
                format!(
                    "{}/tests/assets/mod1/config/uniqueother",
                    env!("CARGO_MANIFEST_DIR")
                )
                .to_string()
            )
        )
    }

    #[test]
    fn asset_loader_resolve_path_override_single() {
        let asset_loader = load_asset_loader();
        assert_eq!(
            asset_loader.resolve_path("config/ov1"),
            Some(
                format!(
                    "{}/tests/assets/mod1/config/ov1",
                    env!("CARGO_MANIFEST_DIR")
                )
                .to_string()
            )
        )
    }
}
