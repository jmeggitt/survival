pub mod body;

pub mod item;

pub mod loader;
#[allow(unused_imports)]
use loader::AssetLoader;

use amethyst::{
    assets::{Asset, AssetStorage, Handle, Loader, Source},
    ecs::World,
    error::{format_err, Error, ResultExt},
};
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock, RwLockReadGuard};

pub type StorageWrapper<T> = Arc<RwLock<Storage<T>>>;

pub use item::Details as Item;
pub type ItemStorage = StorageWrapper<Item>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Storage<T> {
    pub data: HashMap<String, T>,
    #[serde(skip_serializing, skip_deserializing)]
    pub handles: HashMap<String, Handle<T>>,
}

pub trait GetStorage<T> {
    fn borrow(&self) -> RwLockReadGuard<Storage<T>>;
}
impl<T> GetStorage<T> for Arc<RwLock<Storage<T>>> {
    fn borrow(&self) -> RwLockReadGuard<Storage<T>> {
        self.read().unwrap()
    }
}

pub struct StorageSource<T> {
    storage: Arc<RwLock<Storage<T>>>,
    source: PathBuf,
}
impl<T> StorageSource<T>
where
    T: for<'a> serde::Deserialize<'a> + serde::Serialize + Send + Sync + Asset + Sized + Default,
    <T as Asset>::Data: for<'a> serde::Deserialize<'a>,
{
    pub fn apply(source: &Path, world: &mut World) -> Result<Arc<RwLock<Storage<T>>>, Error> {
        let file = File::open(&source)
            .with_context(|_| format_err!("Failed to open file {:?}", source))?;

        let storage: Arc<RwLock<Storage<T>>> = Arc::new(RwLock::new(ron::de::from_reader(file)?));

        {
            world.add_resource(AssetStorage::<T>::default());
            let mut loader = world.write_resource::<Loader>();
            let asset_storage = world.read_resource::<AssetStorage<T>>();

            // Start loading all our own assets..lol
            // TODO: This method prevents us from dynamically loading NEW items
            // As the handles will stay the same, but we cant add actual new entires because of the clone...we'd have to wrap in
            // RwLock Instead...?
            let copy = Self {
                storage: storage.clone(),
                source: source.to_path_buf(),
            };
            loader.add_source("items", copy);

            println!("enter");
            {
                let mut borrow = storage.write().unwrap();
                let keys = borrow.data.keys().cloned().collect::<Vec<_>>();
                for key in &keys {
                    let handle = loader.load_from(
                        key.as_str(),
                        amethyst::assets::RonFormat,
                        (),
                        "items",
                        (),
                        &asset_storage,
                    );
                    println!("Loading: {} -> {:?}", key, handle);
                    borrow.handles.insert(key.to_string(), handle);
                }
            }
        }
        world.add_resource(storage.clone());

        Ok(storage)
    }
}
impl<T> Source for StorageSource<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + Asset + Sized + Default,
{
    fn modified(&self, path: &str) -> Result<u64, Error> {
        use std::fs::metadata;

        metadata(&self.source)
            .with_context(|_| format_err!("Failed to fetch metadata for {:?}", path))?
            .modified()
            .with_context(|_| format_err!("Could not get modification time"))?
            .duration_since(std::time::UNIX_EPOCH)
            .with_context(|_| {
                format_err!("Anomalies with the system clock caused `duration_since` to fail")
            })
            .map(|d| d.as_secs())
    }

    fn load(&self, path: &str) -> Result<Vec<u8>, Error> {
        let borrow = self.storage.borrow();
        let data = borrow
            .data
            .get(path)
            .ok_or_else(|| format_err!("Failed to fetch item {:?}", path))?;
        Ok(ron::ser::to_string(&data)?.as_bytes().to_vec())
    }

    fn load_with_metadata(&self, path: &str) -> Result<(Vec<u8>, u64), Error> {
        let m = self.modified(path)?;
        let b = self.load(path)?;

        Ok((b, m))
    }
}
