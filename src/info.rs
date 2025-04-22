use lerc_sys::ffi;

use crate::LercError;

/// Contains metadata extracted from a LERC blob.
///
/// This includes image dimensions, number of bands, depth, data type, version info,
/// and optionally the global data range.
pub struct BlobInfo {
    pub version: u32,
    pub data_type: u32,
    pub depth: usize,
    pub width: usize,
    pub height: usize,
    pub bands: usize,
    pub masks: usize,
    pub data_range: Option<(f64, f64)>, // optional
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
    let mut info = [0u32; 10];
    let mut range = [0.0f64; 2];

    let status = unsafe {
        ffi::lerc_getBlobInfo(
            blob.as_ptr(),
            blob.len() as u32,
            info.as_mut_ptr(),
            range.as_mut_ptr(),
            info.len() as i32,
            range.len() as i32,
        )
    };

    if status != 0 {
        return Err(LercError::from_status(status));
    }

    Ok(BlobInfo {
        version: info[0],
        data_type: info[1],
        height: info[2] as usize,
        width: info[3] as usize,
        bands: info[4] as usize,
        depth: info[5] as usize,
        masks: info[6] as usize,
        data_range: Some(range.into()),
    })
}
