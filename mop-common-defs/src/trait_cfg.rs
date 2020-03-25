#[cfg(not(feature = "with_rayon"))]
pub trait TraitCfg {}
#[cfg(not(feature = "with_rayon"))]
impl<T> TraitCfg for T {}

#[cfg(feature = "with_rayon")]
pub trait TraitCfg: Send + Sync {}
#[cfg(feature = "with_rayon")]
impl<T> TraitCfg for T where T: Send + Sync {}
