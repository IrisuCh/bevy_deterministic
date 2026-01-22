#[allow(clippy::disallowed_types)]
pub type Map<K, V> = indexmap::IndexMap<K, V, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
