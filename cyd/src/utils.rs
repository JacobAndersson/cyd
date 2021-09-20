use dashmap::DashMap;
use std::sync::Arc;
use crate::search::TtEntry;

pub fn new_tt_table() -> Arc<DashMap<u64, TtEntry>> {
    Arc::new(DashMap::<u64, TtEntry>::new())
}
