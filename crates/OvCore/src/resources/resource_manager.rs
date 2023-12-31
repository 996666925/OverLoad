use std::cell::Cell;

use rust_embed::{EmbeddedFile, RustEmbed};
use OvRender::resources::Resource;

pub trait ResourceTrait {
    fn get(&self, file_path: &str) -> Resource;
}

impl<T> ResourceTrait for T
where
    T: RustEmbed,
{
    fn get(&self, file_path: &str) -> Resource {
        Resource {
            name: file_path.to_string(),
            file: Self::get(file_path).unwrap(),
        }
    }
}

pub struct ResourceManager {
    value: Cell<Option<Box<dyn ResourceTrait>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            value: Cell::new(None),
        }
    }

    pub fn setPath(&self, value: Box<dyn ResourceTrait + 'static>) {
        self.value.set(Some(value));
    }

    pub fn get(&self, name: &str) -> Option<Resource> {
        let res = self.value.take()?;
        Some(res.get(name))
    }
}
