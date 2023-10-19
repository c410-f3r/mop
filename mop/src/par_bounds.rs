/// Marker that imposes different bounds depending on the selected feature.
#[cfg(not(feature = "rayon"))]
pub trait ParBounds {}

#[cfg(not(feature = "rayon"))]
impl<T> ParBounds for T where T: ?Sized {}

/// Marker that imposes different bounds depending on the selected feature.
#[cfg(feature = "rayon")]
pub trait ParBounds: Send + Sync {}

#[cfg(feature = "rayon")]
impl<T> ParBounds for T where T: Send + Sync + ?Sized {}
