use crate::search::transposition_table::TtEntry;
use evmap;
use fnv;

use std::clone::Clone;
use std::hash::BuildHasherDefault;
use std::sync::{Arc, Mutex};

pub struct TranspositionTable {
    reader: evmap::ReadHandle<u64, TtEntry, (), BuildHasherDefault<fnv::FnvHasher>>,
    #[allow(clippy::type_complexity)]
    writer: Arc<Mutex<evmap::WriteHandle<u64, TtEntry, (), BuildHasherDefault<fnv::FnvHasher>>>>,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        let opt = evmap::Options::default();
        let mp = opt.with_hasher(BuildHasherDefault::<fnv::FnvHasher>::default());
        let (r, w) = mp.construct();
        TranspositionTable {
            reader: r,
            writer: Arc::new(Mutex::new(w)),
        }
    }

    pub fn get(&self, val: &u64) -> Option<evmap::ReadGuard<TtEntry>> {
        self.reader.get_one(val)
    }

    fn convert(&self, value: evmap::ReadGuard<TtEntry>) -> TtEntry {
        TtEntry {
            mv: value.mv,
            depth: value.depth,
            flag: value.flag,
            value: value.value
        }
    }

    fn get_and_convert(&self, val: &u64) -> Option<TtEntry> {
        self.get(val).map(|value| self.convert(value))
    }

    pub fn insert(&mut self, key: u64, val: TtEntry) {
        match self.get_and_convert(&key) {
            Some(value) if value != val => {} //If the value is already present, no need to insert it again
            _ => {
                let mut writer = self.writer.lock().unwrap();
                writer.insert(key, val);
                writer.refresh();
            },
        }
    }

    pub fn insert_no_refresh(&mut self, key: u64, val: TtEntry) {
        let mut writer = self.writer.lock().unwrap();
        writer.insert(key, val);
    }

    pub fn refresh(&mut self) {
        let mut writer = self.writer.lock().unwrap();
        writer.refresh();
    }

    #[allow(clippy::len_without_is_empty)] //is_empty never used
    pub fn len(&self) -> usize {
        self.reader.len()
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TranspositionTable {
    fn clone(&self) -> Self {
        Self {
            writer: self.writer.clone(),
            reader: self.reader.clone(),
        }
    }
}
