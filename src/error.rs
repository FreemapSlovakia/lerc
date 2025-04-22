use std::fmt;

/// Represents possible errors returned by LERC operations.
#[derive(Debug)]
pub enum LercError {
    /// The input arguments were invalid, such as mismatched dimensions or null pointers.
    InvalidArgument,

    /// The LERC C API returned a known failure code.
    LercStatus(LercStatus),

    /// The LERC C API returned an unknown status code.
    UnknownStatus(u32),
}

impl fmt::Display for LercError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument => write!(
                f,
                "Invalid argument: mismatched dimensions or null pointers"
            ),
            Self::LercStatus(status) => write!(f, "LERC C API error: {status}"),
            Self::UnknownStatus(code) => {
                write!(f, "Unknown LERC status code returned: {code}")
            }
        }
    }
}

impl std::error::Error for LercError {}

#[derive(Debug)]
pub enum LercStatus {
    Failed,
    WrongParam,
    BufferTooSmall,
    NaN,
    HasNoData,
}

impl LercError {
    /// Converts a raw status code returned from the LERC C API into a `LercError`.
    ///
    /// Known values are mapped to [`LercStatus`] variants inside `LercError::LercStatus`,
    /// and unknown codes are wrapped in `LercError::UnknownStatus`.
    ///
    /// # Parameters
    /// - `code`: The numeric status code returned by a LERC function.
    ///
    /// # Returns
    /// A corresponding `LercError` variant.
    #[must_use]
    pub const fn from_status(code: u32) -> Self {
        match code {
            1 => Self::LercStatus(LercStatus::Failed),
            2 => Self::LercStatus(LercStatus::WrongParam),
            3 => Self::LercStatus(LercStatus::BufferTooSmall),
            4 => Self::LercStatus(LercStatus::NaN),
            5 => Self::LercStatus(LercStatus::HasNoData),
            _ => Self::UnknownStatus(code),
        }
    }
}

/// Represents specific error codes returned by the LERC C API.
///
/// These codes correspond to values in the internal `ErrCode` enum used by LERC.
/// They are returned when an encoding or decoding operation fails.
///
/// See also: [`LercError::LercStatus`]
impl fmt::Display for LercStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Failed => "Operation failed",
            Self::WrongParam => "Invalid parameter",
            Self::BufferTooSmall => "Buffer too small",
            Self::NaN => "Input data contains NaN values",
            Self::HasNoData => "Blob contains only no-data values",
        };
        write!(f, "{msg}")
    }
}
