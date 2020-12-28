/// Trait configuration
#[cfg(not(feature = "with-futures"))]
pub trait TraitCfg {}
#[cfg(not(feature = "with-futures"))]
impl<T> TraitCfg for T {}

/// Trait configuration
#[cfg(feature = "with-futures")]
pub trait TraitCfg: Send + Sync {}
#[cfg(feature = "with-futures")]
impl<T> TraitCfg for T where T: Send + Sync {}
