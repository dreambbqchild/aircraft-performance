#[derive(Clone, Copy)]
pub struct AtmosphereDef<T> {
    pub altitude: i16,
    pub temperature: i16,
    pub indexer: T
}

pub struct AtmosphereBounds<T> {
    pub lower: AtmosphereDef<T>,
    pub upper: AtmosphereDef<T>
}