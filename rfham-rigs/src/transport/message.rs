//!
//! Provides a protocol-specific implementation for various radio communication protocols.
//!
//! Currently, this module supports the following protocols:
//!
//! - CAT (Computer Aided Transceiver)
//! - CI-V (Icom's Computer Interface)
//!
//!

use std::borrow::Cow;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Message<'a>(Cow<'a, [u8]>);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl std::fmt::Display for Message<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "Message [{}]",
                self.as_bytes()
                    .iter()
                    .map(|b| if b.is_ascii_graphic() {
                        format!(" {}", *b as char)
                    } else {
                        format!("{:02X?}", b)
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        } else {
            write!(f, "Message {:02X?}", self.as_bytes())
        }
    }
}

impl<'a> From<Cow<'a, [u8]>> for Message<'a> {
    fn from(cow: Cow<'a, [u8]>) -> Self {
        Message(cow)
    }
}

impl From<Vec<u8>> for Message<'_> {
    fn from(vec: Vec<u8>) -> Self {
        Message(Cow::Owned(vec))
    }
}

impl<'a> From<&'a [u8]> for Message<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Message(Cow::Borrowed(slice))
    }
}

impl<'a> From<&'a Message<'a>> for Vec<u8> {
    fn from(message: &'a Message<'a>) -> Self {
        message.to_vec()
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for Message<'_> {
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.as_bytes() == *other
    }
}

impl PartialEq<&[u8]> for Message<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.as_bytes() == *other
    }
}

impl PartialEq<Vec<u8>> for Message<'_> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.as_bytes() == other.as_slice()
    }
}

impl<'a> Message<'a> {
    ///
    /// Construct a new message with the specified length, initialized with zeros.
    ///
    pub fn new(length: usize) -> Self {
        Message(Cow::Owned(vec![0u8; length]))
    }

    ///
    /// Return the underlying byte slice of the message.
    ///
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    ///
    /// Return the underlying `Cow` of the message.
    ///
    pub fn as_cow(&self) -> &Cow<'a, [u8]> {
        &self.0
    }

    ///
    /// Return the length of the underlying byte slice of the message.
    ///
    pub fn len(&self) -> usize {
        self.0.len()
    }

    ///
    /// Return `true` if the underlying byte slice of the message is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    /// Return the underlying data as a vector of bytes.
    ///
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
