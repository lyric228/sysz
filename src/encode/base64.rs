use crate::{Error, Result};

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static DECODE_TABLE: [u8; 256] = build_decode_table();

/// Builds base64 decoding table for fast character lookup
const fn build_decode_table() -> [u8; 256] {
    let mut table = [0xFF; 256];
    let mut i = 0;

    while i < BASE64_CHARS.len() {
        table[BASE64_CHARS[i] as usize] = i as u8;
        i += 1;
    }

    table
}

/// Encodes UTF-8 string to base64 formatted string
pub fn encode(data: &str) -> String {
    encode_bytes(data.as_bytes())
}

/// Encodes raw bytes to base64 formatted string
pub fn encode_bytes(data: &[u8]) -> String {
    let len = data.len();
    let mut result = Vec::with_capacity(4 * ((len + 2) / 3));
    let mut i = 0;
    
    while i + 3 <= len {
        let chunk = &data[i..i+3];
        let indices = [
            (chunk[0] >> 2) as usize,
            (((chunk[0] & 0x03) << 4) | (chunk[1] >> 4)) as usize,
            (((chunk[1] & 0x0F) << 2) | (chunk[2] >> 6)) as usize,
            (chunk[2] & 0x3F) as usize,
        ];
        
        result.extend(indices.iter().map(|&idx| BASE64_CHARS[idx]));
        i += 3;
    }

    match len - i {
        1 => {
            let b0 = data[i];
            
            result.push(BASE64_CHARS[(b0 >> 2) as usize]);
            result.push(BASE64_CHARS[((b0 & 0x03) << 4) as usize]);
            result.push(b'=');
            result.push(b'=');
        }
        2 => {
            let b0 = data[i];
            let b1 = data[i+1];

            result.push(BASE64_CHARS[(b0 >> 2) as usize]);
            result.push(BASE64_CHARS[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize]);
            result.push(BASE64_CHARS[((b1 & 0x0F) << 2) as usize]);
            result.push(b'=');
        }
        _ => {}
    }

    unsafe { String::from_utf8_unchecked(result) }
}

/// Decodes base64 string to UTF-8 string with validation
pub fn decode(s: &str) -> Result<String> {
    let bytes = decode_bytes(s)?;

    String::from_utf8(bytes).map_err(|e| Error::InvalidSyntax(format!("Invalid UTF-8: {e}")))
}

/// Decodes base64 string to raw bytes with full validation
pub fn decode_bytes(s: &str) -> Result<Vec<u8>> {
    let bytes = s.as_bytes();
    let len = bytes.len();

    if len % 4 != 0 {
        return Err(Error::InvalidSyntax(
            "Base64 input length must be multiple of 4".to_string()
        ));
    }

    let num_blocks = len / 4;

    if num_blocks == 0 {
        return Ok(Vec::new());
    }

    let mut result = Vec::with_capacity(3 * num_blocks);
    
    for i in 0..num_blocks {
        let start = i * 4;
        let end = start + 4;
        let group = &bytes[start..end];
        
        let a0 = DECODE_TABLE[group[0] as usize];
        let a1 = DECODE_TABLE[group[1] as usize];
        let a2 = DECODE_TABLE[group[2] as usize];
        let a3 = DECODE_TABLE[group[3] as usize];

        if a0 == 0xFF {
            return Err(Error::InvalidSyntax(
                format!("Invalid base64 character: '{}'", group[0] as char)
            ));
        }
        if a1 == 0xFF {
            return Err(Error::InvalidSyntax(
                format!("Invalid base64 character: '{}'", group[1] as char)
            ));
        }
        if a2 == 0xFF && group[2] != b'=' {
            return Err(Error::InvalidSyntax(
                format!("Invalid base64 character: '{}'", group[2] as char)
            ));
        }
        if a3 == 0xFF && group[3] != b'=' {
            return Err(Error::InvalidSyntax(
                format!("Invalid base64 character: '{}'", group[3] as char)
            ));
        }

        if group[2] == b'=' {
            if group[3] != b'=' {
                return Err(Error::InvalidSyntax(
                    "Invalid padding: expected '=' at position 4".to_string()
                ));
            }

            result.push((a0 << 2) | (a1 >> 4));
        } else if group[3] == b'=' {
            result.push((a0 << 2) | (a1 >> 4));
            result.push((a1 << 4) | (a2 >> 2));
        } else {
            result.push((a0 << 2) | (a1 >> 4));
            result.push((a1 << 4) | (a2 >> 2));
            result.push((a2 << 6) | a3);
        }
    }

    Ok(result)
}

/// Checks if string contains only valid base64 characters
pub fn is_valid(base64: &str) -> bool {
    let bytes = base64.as_bytes();
    let len = bytes.len();
    
    if len % 4 != 0 {
        return false;
    }
    
    for &b in bytes {
        if !(b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'=') {
            return false;
        }
    }
    
    if len >= 4 {
        let padding_start = len - 2;

        for i in 0..len {
            if bytes[i] == b'=' {
                if i < padding_start {
                    return false;
                }
            }
        }
    }
    
    true
}
