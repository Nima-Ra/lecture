use crate::common_types::*;
use crate::management::*;

use ic_kit::macros::*;
use ic_kit::*;
use ic_kit::ic::spawn;
use std::collections::HashMap;

#[derive(Default)]
pub struct HashMapDB(HashMap<Principal, Metadata>);

impl HashMapDB {
    pub fn archive(&mut self) -> Vec<(Principal, Metadata)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, Metadata)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn add(&mut self, metadata: Metadata) -> Result<(), Error> {
        let id: Principal = metadata.id;
        self.0.insert(metadata.id, metadata);
        Ok(())
    }

    pub fn remove(&mut self, id: &Principal) -> Result<(), Error> {
        if !self.0.contains_key(id) {
            return Err(Error::NonExistentItem);
        }
        self.0.remove(id);
        Ok(())
    }

    pub fn get(&mut self, id: Principal) -> Option<&Metadata> {
        self.0.get(&id)
    }

    pub fn get_all(&self) -> Vec<&Metadata> {
        self.0.values().collect()
    }
}

#[update]
pub fn add(metadata: Metadata) -> Result<(), Error> {
    let db = ic::get_mut::<HashMapDB>();
    db.add(metadata)
}

#[update]
pub fn remove(id: Principal) -> Result<(), Error> {
    let db = ic::get_mut::<HashMapDB>();
    db.remove(id)
}

#[query]
pub fn get(id: Principal) -> Option<&'static Metadata> {
    let db = ic::get::<HashMapDB>();
    db.get(id)
}

#[query]
pub fn get_all() -> Vec<&'static Metadata> {
    let db = ic::get::<HashMapDB>();
    db.get_all()
}