use lerc_sys::ffi;

use crate::{BlobInfo, LercError, datatype::LercDataType, get_blob_info};

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

/// Decodes a LERC blob using the provided [`BlobInfo`] metadata.
///
/// This is a convenience wrapper over [`decode`] that extracts all
/// necessary dimensions and layout parameters from a `BlobInfo` instance.
///
/// # Parameters
/// - `blob`: The LERC-compressed input byte slice.
/// - `info`: Metadata describing the compressed layout.
///
/// # Returns
/// A tuple of decoded data and optional validity mask.
pub fn decode_with_info<T: LercDataType>(
    blob: &[u8],
    info: &BlobInfo,
) -> Result<(Vec<T>, Option<Vec<u8>>), LercError> {
    decode::<T>(
        blob,
        info.width as usize,
        info.height as usize,
        info.depth as usize,
        info.bands as usize,
        info.masks as usize,
    )
}

/// Decodes a LERC blob by first inspecting its metadata with [`get_blob_info`].
///
/// This is the simplest way to decode a LERC blob when you don’t know its
/// layout ahead of time. Internally calls [`get_blob_info`] and then [`decode_with_info`].
///
/// # Parameters
/// - `blob`: The LERC-compressed input byte slice.
///
/// # Returns
/// A tuple of decoded data and optional validity mask.
pub fn decode_auto<T: LercDataType>(blob: &[u8]) -> Result<(Vec<T>, Option<Vec<u8>>), LercError> {
    let info = get_blob_info(blob)?;

    decode_with_info::<T>(blob, &info)
}
