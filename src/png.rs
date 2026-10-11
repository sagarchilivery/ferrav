//! Packs RGBA pixels into a PNG file. Only built when the `png` feature is on.

/// Encodes RGBA pixels (4 bytes per pixel, rows top to bottom) as PNG bytes.
///
/// The input buffer must contain exactly `width * height * 4` bytes:
///
/// ```text
/// R G B A R G B A ...
/// ```
///
/// # Panics
///
/// Panics if:
/// - `width` or `height` is zero
/// - `width * height * 4` doesn't fit in memory (`usize`)
/// - `rgba.len()` is not exactly `width * height * 4`
pub fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    assert!(width > 0 && height > 0, "PNG images need at least 1x1 pixels");

    let row_len = (width as usize)
        .checked_mul(4)
        .expect("image width is too large");

    let expected_len = row_len
        .checked_mul(height as usize)
        .expect("image dimensions are too large");

    assert_eq!(rgba.len(), expected_len, "pixel buffer has wrong length");

    // Each row starts with a "filter type" byte. 0 means "no filtering".
    let mut raw = Vec::with_capacity(
        (row_len + 1)
            .checked_mul(height as usize)
            .expect("image is too large"),
    );

    for row in rgba.chunks_exact(row_len) {
        raw.push(0);
        raw.extend_from_slice(row);
    }

    // PNG stores the rows compressed with zlib (DEFLATE plus a small header).
    let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&raw, 6);

    // IHDR chunk data: width, height, then bit depth 8, color type 6 (RGBA),
    // and compression, filter and interlace methods (all 0).
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);

    // Every chunk costs 12 bytes on top of its data: length, type and CRC.
    let mut png = Vec::with_capacity(
        8 // signature
            + 12 + ihdr.len() // IHDR
            + 12 + compressed.len() // IDAT
            + 12, // IEND (no data)
    );

    // The PNG signature: 8 fixed bytes every PNG file starts with.
    png.extend_from_slice(b"\x89PNG\r\n\x1a\n");

    write_chunk(&mut png, b"IHDR", &ihdr);
    write_chunk(&mut png, b"IDAT", &compressed);
    write_chunk(&mut png, b"IEND", &[]);

    png
}

/// Writes one PNG chunk.
///
/// Layout:
///
/// ```text
/// length (4 bytes, big-endian)
/// type   (4 bytes)
/// data
/// crc32(type + data) (4 bytes, big-endian)
/// ```
fn write_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    let length = u32::try_from(data.len()).expect("PNG chunk exceeds maximum size");

    out.extend_from_slice(&length.to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    out.extend_from_slice(&crc32(&[kind, data]).to_be_bytes());
}

/// CRC-32, the checksum PNG uses, computed one bit at a time.
fn crc32(parts: &[&[u8]]) -> u32 {
    let mut crc = 0xffff_ffff_u32;

    for part in parts {
        for &byte in *part {
            crc ^= u32::from(byte);

            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xedb8_8320
                } else {
                    crc >> 1
                };
            }
        }
    }

    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_matches_standard_value() {
        assert_eq!(crc32(&[b"123456789"]), 0xcbf4_3926);
    }

    #[test]
    fn starts_with_png_signature() {
        let png = encode_png(1, 1, &[255, 0, 0, 255]);

        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    }

    #[test]
    fn writes_header_and_end_chunks() {
        // A 2×1 image: one red pixel, then one green pixel.
        let png = encode_png(2, 1, &[255, 0, 0, 255, 0, 255, 0, 255]);

        // Right after the signature: the IHDR chunk, with length 13,
        // width 2, height 1, then bit depth and color type.
        assert_eq!(&png[8..12], &13u32.to_be_bytes());
        assert_eq!(&png[12..16], b"IHDR");
        assert_eq!(&png[16..20], &2u32.to_be_bytes());
        assert_eq!(&png[20..24], &1u32.to_be_bytes());
        assert_eq!(&png[24..29], &[8, 6, 0, 0, 0]);

        // The file ends with the IEND chunk, which is always these 12 bytes:
        // length 0, type "IEND", and its CRC.
        let iend = [0, 0, 0, 0, b'I', b'E', b'N', b'D', 0xae, 0x42, 0x60, 0x82];
        assert_eq!(&png[png.len() - 12..], &iend);
    }

    #[test]
    #[should_panic(expected = "pixel buffer has wrong length")]
    fn rejects_wrong_buffer_length() {
        // A 2×2 image needs 16 bytes, not 4.
        encode_png(2, 2, &[0; 4]);
    }
}