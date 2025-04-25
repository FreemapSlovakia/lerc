use lerc_sys::ffi;

use crate::LercError;

/// Metadata extracted from a LERC blob using `lerc_getBlobInfo`.
///
/// This structure provides information about the dimensions, data type, and encoding
/// details of the compressed raster data, without fully decoding the blob.
#[derive(Debug, Clone, Copy)]
pub struct BlobInfo {
    /// Codec version used to create the blob.
    pub version: u32,

    /// Encoded data type (see LERC C API `DataType` enum).
    pub data_type: u32,

    /// Number of values per pixel (aka depth or dimensionality).
    pub depth: u32,

    /// Width of the image in pixels (columns).
    pub width: u32,

    /// Height of the image in pixels (rows).
    pub height: u32,

    /// Number of bands (layers) per pixel.
    pub bands: u32,

    /// Number of valid pixels in the first band (others may differ if multiple masks).
    pub valid_pixel_count: u32,

    /// Size of the compressed blob in bytes.
    pub blob_size: u32,

    /// Number of validity masks:
    /// - `0`: all pixels valid
    /// - `1`: same mask for all bands
    /// - `nBands`: per-band masks
    pub masks: u32,

    /// Indicates how many bands use a no-data value (only applies if `depth > 1`).
    pub uses_nodata: u32,
}

/// Parses metadata from a LERC-compressed blob without decoding the data.
///
/// This includes raster dimensions, number of bands, depth, data type, and version info.
///
/// # Parameters
/// - `blob`: Compressed LERC byte buffer.
///
/// # Returns
/// A `BlobInfo` struct with metadata about the LERC blob.
///
/// # Errors
/// Returns `LercError` if the blob is invalid or info extraction fails.
pub fn get_blob_info(blob: &[u8]) -> Result<BlobInfo, LercError> {
    const INFO_LEN: usize = 12;
    let mut info_array = [0u32; INFO_LEN];
    let mut range_array = [0f64; 2]; // not used here

    let status = unsafe {
        ffi::lerc_getBlobInfo(
            blob.as_ptr(),
            blob.len() as u32,
            info_array.as_mut_ptr(),
            range_array.as_mut_ptr(),
            INFO_LEN as i32,
            range_array.len() as i32,
        )
    };

    if status != 0 {
        return Err(LercError::from_status(status));
    }

    Ok(BlobInfo {
        version: info_array[0],
        data_type: info_array[1],
        depth: info_array[2],
        width: info_array[3],
        height: info_array[4],
        bands: info_array[5],
        valid_pixel_count: info_array[6],
        blob_size: info_array[7],
        masks: info_array[8],
        uses_nodata: info_array[10],
    })
}
