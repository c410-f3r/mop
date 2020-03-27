#[cfg(not(feature = "with_futures"))]
pub trait TraitCfg {}
#[cfg(not(feature = "with_futures"))]
impl<T> TraitCfg for T {}

#[cfg(feature = "with_futures")]
pub trait TraitCfg: Send + Sync {}
#[cfg(feature = "with_futures")]
impl<T> TraitCfg for T where T: Send + Sync {}
