use std::{ops::Deref, sync::Arc};

use dashmap::{DashMap, DashSet};

use crate::RespFrame;

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug)]
pub struct BackendInner {
    map: DashMap<String, RespFrame>,
    hmap: DashMap<String, DashMap<String, RespFrame>>,
    set: DashMap<String, DashSet<String>>,
}

impl Deref for Backend {
    type Target = BackendInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Backend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        self.hmap
            .get(key)
            .and_then(|v| v.get(field).map(|v| v.value().clone()))
    }

    pub fn hset(&self, key: String, field: String, value: RespFrame) {
        if !self.hmap.contains_key(&key) {
            self.hmap.insert(key.clone(), DashMap::new());
        }
        self.hmap.get(&key).unwrap().insert(field, value);
    }

    pub fn hgetall(&self, key: &str) -> Option<DashMap<String, RespFrame>> {
        self.hmap.get(key).map(|v| v.clone())
    }

    pub fn sadd(&self, key: String, members: Vec<String>) -> i64 {
        if !self.set.contains_key(&key) {
            self.set.insert(key.clone(), DashSet::new());
        }
        let mut count = 0;
        for member in members {
            if self.set.get(&key).unwrap().insert(member.clone()) {
                count += 1;
            }
        }
        count
    }

    pub fn sismember(&self, key: &str, member: &str) -> Option<i64> {
        self.set
            .get(key)
            .map(|v| if v.contains(member) { 1 } else { 0 })
    }
}

impl Default for Backend {
    fn default() -> Self {
        Backend(Arc::new(BackendInner {
            map: DashMap::new(),
            hmap: DashMap::new(),
            set: DashMap::new(),
        }))
    }
}

impl Default for BackendInner {
    fn default() -> Self {
        BackendInner {
            map: DashMap::new(),
            hmap: DashMap::new(),
            set: DashMap::new(),
        }
    }
}
