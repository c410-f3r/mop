use crate::{dr_matrix::DrMatrixError, gp::GpDefinitionsBuilderError};
use core::fmt;
use num_traits::{NumCast, ToPrimitive};

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
  /// It wasn't possible to cast a value
  BadCast,
  /// Error from DrMatrix
  DrMatrixError(DrMatrixError),
  /// It wasn't possible to extract optional element
  EmptyElement,
  /// Insufficient capacity
  InsufficientCapacity,
  /// GpDefinitionsBuilder error
  Gdbe(GpDefinitionsBuilderError),
  /// Error from external ndstruct dependency
  #[cfg(feature = "ndstruct")]
  NdsparseError(ndstruct::Error),
  /// An unspecified error occurred
  Other(&'static str),
  /// Unsupported conversion
  UnsupportedConversion,
}

impl Error {
  #[inline]
  pub fn cast_rslt<T, U>(value: T) -> Result<U, Error>
  where
    T: ToPrimitive,
    U: NumCast,
  {
    if let Some(r) = NumCast::from(value) {
      Ok(r)
    } else {
      Err(Self::BadCast)
    }
  }

  #[inline]
  pub fn opt_rslt<T>(opt: Option<T>) -> Result<T, Error> {
    if let Some(r) = opt {
      Ok(r)
    } else {
      Err(Self::EmptyElement)
    }
  }
}

// Theoretically, it doesn't matter which Error is given for Infallible
impl From<core::convert::Infallible> for Error {
  #[inline]
  fn from(_: core::convert::Infallible) -> Self {
    Self::BadCast
  }
}

impl From<GpDefinitionsBuilderError> for Error {
  #[inline]
  fn from(from: GpDefinitionsBuilderError) -> Self {
    Self::Gdbe(from)
  }
}

impl From<DrMatrixError> for Error {
  #[inline]
  fn from(from: DrMatrixError) -> Self {
    Self::DrMatrixError(from)
  }
}

#[cfg(feature = "ndstruct")]
impl From<ndstruct::Error> for Error {
  #[inline]
  fn from(from: ndstruct::Error) -> Self {
    Self::NdsparseError(from)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::BadCast => write!(f, "BadCast"),
      Self::DrMatrixError(x) => write!(f, "DrMatrixError({x})"),
      Self::EmptyElement => write!(f, "EmptyElement"),
      Self::InsufficientCapacity => write!(f, "Insufficient capacity"),
      Self::Gdbe(x) => write!(f, "GDBE({x})"),
      #[cfg(feature = "ndstruct")]
      Self::NdsparseError(x) => write!(f, "NdsparseError({x})"),
      Self::Other(x) => write!(f, "Other({x})"),
      Self::UnsupportedConversion => write!(f, "Unsupported conversion"),
    }
  }
}
