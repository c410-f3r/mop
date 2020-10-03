use crate::gp::GpDefinitionsBuilderError;
use core::fmt;
#[cfg(feature = "with-ndsparse")]
use ndsparse::csl::CslError;
use num_traits::{NumCast, ToPrimitive};

#[derive(Debug)]
pub enum Error {
  /// It wasn't possible to cast a value
  BadCast,
  /// Error from external ndsparse dependency
  #[cfg(feature = "with-ndsparse")]
  CslError(CslError),
  /// It wasn't possible to extract optional element
  EmptyElement,
  /// GpDefinitionsBuilder error
  GDBE(GpDefinitionsBuilderError),
  /// An unspecified error occurred
  Other(&'static str),
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

#[cfg(feature = "with-ndsparse")]
impl From<CslError> for Error {
  fn from(from: CslError) -> Self {
    Self::CslError(from)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::BadCast => write!(f, "BadCast"),
      #[cfg(feature = "with-ndsparse")]
      Self::CslError(ref x) => write!(f, "CslError({})", x),
      Self::EmptyElement => write!(f, "EmptyElement"),
      Self::GDBE(ref x) => write!(f, "GDBE({})", x),
      Self::Other(ref x) => write!(f, "Other({})", x),
    }
  }
}
