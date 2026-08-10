/// Parse a single option byte from an OM response.
///
/// Evaluates to `true` if the byte matches `$present`, `false` if it is `b'-'`,
/// or returns `Err(RigError::InvalidResponseData)` for any other value.
macro_rules! parse_option {
    ($name:ident => $bytes:ident [ $index:literal ] == $present:literal) => {
        match $bytes[$index] {
            $present => true,
            b'-' => false,
            _ => {
                ::tracing::error!(
                    "byte value {:02X?} not valid for option {}",
                    $bytes[$index],
                    stringify!($name)
                );
                return Err($crate::error::RigError::InvalidResponseData {
                    data: $bytes.to_vec(),
                });
            }
        }
    };
}

/// Extract a boolean flag from a packed byte in an IC-style response.
///
/// Forms:
/// - `parse_flag!(byte_idx:bit_idx == state in bytes)` — explicit value
/// - `parse_flag!(byte_idx:bit_idx ON in bytes)` — bit must be set
/// - `parse_flag!(byte_idx:bit_idx OFF in bytes)` — bit must be clear
/// - `parse_flag!(byte_idx:bit_idx in bytes)` — shorthand for ON
macro_rules! parse_flag {
    ($byte:literal : $bit:literal == $state:literal in $bytes:expr) => {
        $bytes[$byte] & 1 << $bit == $state
    };
    ($byte:literal : $bit:literal ON in $bytes:expr) => {
        parse_flag!($byte:$bit == 1 in $bytes)
    };
    ($byte:literal : $bit:literal OFF in $bytes:expr) => {
        parse_flag!($byte:$bit == 0 in $bytes)
    };
    ($byte:literal : $bit:literal in $bytes:expr) => {
        parse_flag!($byte:$bit ON in $bytes)
    };
}
