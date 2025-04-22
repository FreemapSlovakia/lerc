use lerc_sys::ffi;

use crate::{LercError, datatype::LercDataType};

/// Decodes a LERC-compressed byte buffer into typed raster values.
///
/// # Parameters
/// - `blob`: Compressed LERC byte buffer.
/// - `width`, `height`: Dimensions of the raster image.
/// - `depth`: Number of values per pixel.
/// - `n_bands`: Number of spectral or channel bands.
/// - `n_masks`: Number of masks (usually 1).
///
/// # Returns
/// A tuple `(Vec<T>, Option<Vec<u8>>)` with the decoded data and an optional pixel validity mask.
///
/// # Errors
/// Returns `LercError` if decoding fails.
pub fn decode<T: LercDataType>(
    blob: &[u8],
    width: usize,
    height: usize,
    depth: usize,
    n_bands: usize,
    n_masks: usize,
) -> Result<(Vec<T>, Option<Vec<u8>>), LercError> {
    let total_values = width * height * depth * n_bands;

    let mut valid_mask: Option<Vec<u8>> = None;

    let p_valid_bytes = if n_masks > 0 {
        let mut mask = vec![0u8; width * height * n_masks];

        let ptr = mask.as_mut_ptr();

        valid_mask = Some(mask);

        ptr
    } else {
        std::ptr::null_mut()
    };

    let mut data = vec![T::default(); total_values];

    let status = unsafe {
        ffi::lerc_decode(
            blob.as_ptr(),
            blob.len() as u32,
            n_masks as i32,
            p_valid_bytes,
            depth as i32,
            width as i32,
            height as i32,
            n_bands as i32,
            T::LERC_TYPE,
            data.as_mut_ptr().cast(),
        )
    };

    if status != 0 {
        return Err(LercError::from_status(status));
    }

    Ok((data, valid_mask))
}
