use crate::datatype::LercDataType;
use crate::error::LercError;
use lerc_sys::ffi;

/// Encodes typed raster data into a LERC-compressed byte buffer.
///
/// # Parameters
/// - `data`: A flat slice of typed values in row-major order, including all bands and depth values.
/// - `valid_mask`: Optional pixel validity mask (1 = valid, 0 = invalid). Should be `width * height * n_masks` in size.
/// - `width`, `height`: Dimensions of the raster image.
/// - `depth`: Number of values per pixel (e.g. 1 for scalar, >1 for vector data).
/// - `n_bands`: Number of spectral or channel bands.
/// - `n_masks`: Number of masks (usually 1).
/// - `max_z_error`: Maximum allowable error for lossy compression (0.0 for lossless).
///
/// # Returns
/// A compressed byte buffer as `Vec<u8>` on success.
///
/// # Errors
/// Returns `LercError` if validation or LERC encoding fails.
pub fn encode<T: LercDataType>(
    data: &[T],
    valid_mask: Option<&[u8]>,
    width: usize,
    height: usize,
    depth: usize,
    n_bands: usize,
    n_masks: usize,
    max_z_error: f64,
) -> Result<Vec<u8>, LercError> {
    let total_values = width * height * depth * n_bands;

    if data.len() != total_values {
        return Err(LercError::InvalidArgument);
    }

    if let Some(mask) = valid_mask {
        let expected_mask_size = width * height * n_masks;

        if mask.len() != expected_mask_size {
            return Err(LercError::InvalidArgument);
        }
    }

    let mut compressed_size: u32 = 0;

    let status = unsafe {
        ffi::lerc_computeCompressedSize(
            data.as_ptr() as _,
            T::LERC_TYPE,
            depth as i32,
            width as i32,
            height as i32,
            n_bands as i32,
            n_masks as i32,
            valid_mask.map_or(std::ptr::null(), <[u8]>::as_ptr),
            max_z_error,
            &mut compressed_size,
        )
    };

    if status != 0 {
        return Err(LercError::from_status(status));
    }

    let mut output = vec![0u8; compressed_size as usize];

    let mut n_bytes_written: u32 = 0;

    let status = unsafe {
        ffi::lerc_encode(
            data.as_ptr().cast(),
            T::LERC_TYPE,
            depth as i32,
            width as i32,
            height as i32,
            n_bands as i32,
            n_masks as i32,
            valid_mask.map_or(std::ptr::null(), <[u8]>::as_ptr),
            max_z_error,
            output.as_mut_ptr(),
            output.len() as u32,
            &mut n_bytes_written,
        )
    };

    if status != 0 {
        return Err(LercError::from_status(status));
    }

    output.truncate(n_bytes_written as usize);

    Ok(output)
}
