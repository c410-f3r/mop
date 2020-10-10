use crate::{
  gp::GpDefinitionsBuilderError,
  dr_matrix::DrMatrixError
};
use core::fmt;
use num_traits::{NumCast, ToPrimitive};

#[derive(Debug)]
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
  GDBE(GpDefinitionsBuilderError),
  /// Error from external ndsparse dependency
  #[cfg(feature = "with-ndsparse")]
  NdsparseError(ndsparse::Error),
  /// An unspecified error occurred
  Other(&'static str),
  /// Unsupported conversion
  UnsupportedConversion
}

impl Error {
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
  fn from(_: core::convert::Infallible) -> Self {
    Self::BadCast
  }
}

impl From<GpDefinitionsBuilderError> for Error {
  fn from(from: GpDefinitionsBuilderError) -> Self {
    Self::GDBE(from)
  }
}


impl From<DrMatrixError> for Error {
  fn from(from: DrMatrixError) -> Self {
    Self::DrMatrixError(from)
  }
}

#[cfg(feature = "with-ndsparse")]
impl From<ndsparse::Error> for Error {
  fn from(from: ndsparse::Error) -> Self {
    Self::NdsparseError(from)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::BadCast => write!(f, "BadCast"),
      Self::DrMatrixError(ref x) => write!(f, "DrMatrixError({})", x),
      Self::EmptyElement => write!(f, "EmptyElement"),
      Self::InsufficientCapacity => write!(f, "Insufficient capacity"),
      Self::GDBE(ref x) => write!(f, "GDBE({})", x),
      #[cfg(feature = "with-ndsparse")]
      Self::NdsparseError(ref x) => write!(f, "NdsparseError({})", x),
      Self::Other(ref x) => write!(f, "Other({})", x),
      Self::UnsupportedConversion => write!(f, "Unsupported conversion"),
    }
  }
}
