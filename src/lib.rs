//! Safe and idiomatic Rust wrapper for the [Esri LERC](https://github.com/Esri/lerc) compression library.
//!
//! This crate provides high-level Rust bindings over the [`lerc-sys`] crate, which links to the native LERC C++ library.
//! It supports encoding and decoding 2D, 3D, and multi-band raster data with optional pixel validity masks.
//!
//! # Features
//! - Safe, generic encoding and decoding using type parameters (`u8`, `f32`, `f64`, etc.)
//! - Optional validity mask support
//! - Access to LERC blob metadata without full decompression
//! - Rich error handling with specific LERC error codes
//!
//! # Example: Encode
//! ```no_run
//! use lerc::encode;
//!
//! let width = 256;
//! let height = 256;
//! let data = vec![0.0f32; width * height];
//! let compressed = encode(&data, None, width, height, 1, 1, 1, 0.001).unwrap();
//! ```
//!
//! # Example: Decode
//! ```no_run
//! use lerc::{decode, get_blob_info};
//!
//! let compressed: Vec<u8> = /* read from file or network */ vec![];
//! let info = get_blob_info(&compressed).unwrap();
//! let (data, mask) = decode::<f32>(&compressed, info.width, info.height, info.depth, info.bands, info.masks).unwrap();
//! ```
//!
//! # Minimum Supported Rust Version
//! - Rust 1.60+ (due to `bindgen` and FFI support)
//!
//! # See also
//! - [Esri LERC GitHub](https://github.com/Esri/lerc)
//! - [`lerc-sys`] crate: low-level raw bindings

mod datatype;
mod decode;
mod encode;
mod error;
mod info;

pub use datatype::LercDataType;
pub use decode::decode;
pub use decode::decode_auto;
pub use decode::decode_with_info;
pub use encode::encode;
pub use error::LercError;
pub use info::{BlobInfo, get_blob_info};
