//! Simple library that can be used to represent a grow-able bit array.
//!
//! Functionality includes `FIFO`, concatenation and setting bits `ON` and `OFF`.
//!
//! # Usage
//!
//! This crate is not published on `crates.io` and can be used by adding `bit_array_list` under the
//! `dependencies` section name in your project's `Cargo.toml` as follows:
//!
//! ```toml
//! [dependencies]
//! bit_array_list = { git = "https://github.com/konstantindt/bit-array-list" }
//! ```
//!
//! and the following to your crate root:
//!
//! ```rust
//! extern crate bit_array_list;
//! ```
//!
//! # Examples
//!
//! The following example shows how we can implement boolean flags.
//!
//! ```{.rust}
//! extern crate bit_array_list;
//!
//! use bit_array_list::BitArrayList;
//!
//! fn main() {
//!     // Light 16 LEDs in the pattern 0000101010100101.
//!     let leds = BitArrayList::from(vec![10, 165], 16);
//!     // Lit LEDs are:
//!     assert!(leds.is_set(4));
//!     assert!(leds.is_set(6));
//!     assert!(leds.is_set(8));
//!     assert!(leds.is_set(10));
//!     assert!(leds.is_set(13));
//!     assert!(leds.is_set(15));
//!     // Off LEDs are:
//!     assert!(!leds.is_set(0));
//!     assert!(!leds.is_set(1));
//!     assert!(!leds.is_set(2));
//!     assert!(!leds.is_set(3));
//!     assert!(!leds.is_set(5));
//!     assert!(!leds.is_set(7));
//!     assert!(!leds.is_set(9));
//!     assert!(!leds.is_set(11));
//!     assert!(!leds.is_set(12));
//!     assert!(!leds.is_set(14));
//! }
//! ```
use std::fmt;

/// A contiguous grow-able array type consisting of a list of bits.
///
/// Internally the array stores an array of bytes and so the bit array length depics wasted space
/// up to 7 bits---no more than a byte. Most computer architectures perceive the byte as the
/// smallest unit of data.
///
/// # Examples
///
/// Calculate wasted space:
///
/// ```
/// use bit_array_list::BitArrayList;
/// let bit_array = BitArrayList::from(vec![255, 32], 11);
///
/// let wasted_space = bit_array.bytes().len() * 8 - bit_array.len();
/// assert_eq!(wasted_space, 5);
/// ```
#[derive(Clone, Debug)]
pub struct BitArrayList {
    bytes: Vec<u8>,
    length: usize,
}

impl BitArrayList {
    /// Returns the bit at a given index `i`.
    ///
    /// * Returns ```true``` if bit at `i` is `1`.
    /// * Returns ```false``` if bit at `i` is `0`.
    ///
    /// # Panics
    ///
    /// If `i` is not within bounds (`i >= bit_array.len()`).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let bit_array = BitArrayList::from(vec![64, 32], 11);
    ///
    /// assert!(bit_array.is_set(1));
    /// assert!(bit_array.is_set(10));
    /// assert!(!bit_array.is_set(0));
    /// ```
    pub fn is_set(&self, bit_index: usize) -> bool {
        if bit_index < self.length {
            let (byte_index, bit_position) = split_index(bit_index);

            self.zero_testing(byte_index, bit_position)
        } else {
            panic!("BitArrayList index out of bounds: index is {} but array length is {}.",
                   bit_index,
                   self.length);
        }
    }

    /// Returns the number of elements in the bit array.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let bit_array = BitArrayList::from(vec![12, 16], 12);
    ///
    /// assert_eq!(bit_array.len(), 12);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }

    /// Helper method to perform `self.length < 1 i.e returns` `true` if bit array has no
    /// elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let mut bit_array = BitArrayList::new();
    ///
    /// assert!(bit_array.is_empty());
    ///
    /// bit_array.push(0);
    /// assert!(!bit_array.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.length < 1
    }

    /// Returns the raw underlying array data structure of bytes.
    ///
    /// Note that this array may contain wasted space and without knowing the length of the bit
    /// array at the time we cannot successfully recreate the bit array from this operation.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let bit_array = BitArrayList::from(vec![12, 70], 15);
    ///
    /// assert_eq!(bit_array.bytes(), &vec![12, 70]);
    /// ```
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    /// Helper method for turing [is_set()][1] output to
    /// string slice.
    ///
    /// [1]: struct.BitArrayList.html#method.is_set
    ///
    /// # Panics
    ///
    /// see [is_set()][2].
    ///
    /// [2]: struct.BitArrayList.html#panics
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let bit_array = BitArrayList::from(vec![64, 32], 11);
    ///
    /// assert_eq!(bit_array.bit_to_str(1), "1");
    /// assert_eq!(bit_array.bit_to_str(10), "1");
    /// assert_eq!(bit_array.bit_to_str(0), "0");
    /// ```
    pub fn bit_to_str(&self, bit_index: usize) -> &str {
        match self.is_set(bit_index) {
            true => "1",
            false => "0",
        }
    }

    /// Set a pre-exising bit (at index `i`) to the specified value.
    ///
    /// # Panics
    ///
    /// If `i` is not within bounds (`i >= bit_array.len()`).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let mut bit_array = BitArrayList::from(vec![64, 128], 9);
    ///
    /// bit_array.set_bit_to(1, 0);
    /// bit_array.set_bit_to(2, 1);
    /// assert_eq!(bit_array.bytes(), &vec![32, 128]);
    /// ```
    pub fn set_bit_to(&mut self, bit_index: usize, bit: u8) {
        if bit_index < self.length {
            let (byte_index, bit_position) = split_index(bit_index);
            // Set bit to user's preferences.
            match bit {
                1 => self.bytes[byte_index] |= bitmask(bit_position),
                0 => self.bytes[byte_index] &= !bitmask(bit_position),
                _ => panic!("Mismatched types: expected a u8 equal to 1 or 0."),
            }
        } else {
            panic!("BitArrayList index out of bounds: index is {} but array length is {}.",
                   bit_index,
                   self.length);
        }
    }

    /// Appends a given bit to the back of the collection of bits.
    ///
    /// # Panics
    ///
    /// If the number of internal bytes representing the bits overflows a `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let mut bit_array = BitArrayList::from(vec![200, 128], 15);
    ///
    /// bit_array.push(1);
    /// bit_array.push(1);
    /// bit_array.push(0);
    /// assert_eq!(bit_array.bytes(), &vec![200, 129, 128]);
    /// ```
    pub fn push(&mut self, bit: u8) {
        let (byte_index, bit_position) = split_index(self.length);

        if byte_index > self.bytes.len() - 1 {
            // Add new empty byte.
            self.bytes.push(0);
        }
        // Add the user's bit.
        match bit {
            1 => self.bytes[byte_index] |= bitmask(bit_position),
            0 => { /* Position at bit index is already initialised as 0 */ }
            _ => panic!("Mismatched types: expected a u8 equal to 1 or 0."),
        }
        // Update data structure length.
        self.length += 1;
    }

    /// Removes and returns the last bit in the collection unless collection is empty where
    /// `None` is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let mut bit_array = BitArrayList::from(vec![128], 2);
    ///
    /// assert_eq!(bit_array.pop(), Some(false));
    /// assert_eq!(bit_array.pop(), Some(true));
    /// assert_eq!(bit_array.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<bool> {
        if self.is_empty() {
            // Also avoids subtract with overflow.
            return None;
        }
        // Initialise  indexes.
        let last_bit_index = self.length - 1;
        let (byte_index, bit_position) = split_index(last_bit_index);

        let to_return = Some(self.zero_testing(byte_index, bit_position));

        if bit_position == 0 && self.bytes.len() > 2 && byte_index == self.bytes.len() - 1 {
            // Remove last empty byte unless we only have one byte.
            self.bytes.pop();
        } else {
            // Remove old data.
            self.bytes[byte_index] &= !bitmask(bit_position);
        }
        // Update data structure length.
        self.length -= 1;

        to_return
    }

    /// Appends two bit arrays together (self followed by other bits).
    ///
    /// Leaves other bits array unusable (dropped by Rust).
    ///
    /// Operation is fast if self has no wasted space in the last byte otherwise we push each bit
    /// from the other bits into self one by one.
    ///
    /// # Panics
    ///
    /// see [push()][3].
    ///
    /// [3]: struct.BitArrayList.html#panics-3
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let mut bit_array = BitArrayList::from(vec![213, 128], 9);
    ///
    /// bit_array.concatenate(BitArrayList::from(vec![48], 4));
    /// assert_eq!(bit_array.bytes(), &vec![213, 152]);
    /// assert_eq!(bit_array.len(), 9 + 4);
    /// ```
    pub fn concatenate(&mut self, other_bits: BitArrayList) {
        if self.length % 8 != 0 {
            // Push each bit in self from BitArrayList we are extending self with.
            for bit_index in 0..other_bits.length {
                match other_bits.is_set(bit_index) {
                    true => self.push(1),
                    false => self.push(0),
                }
            }
        } else {
            // No need to fill empty space at last byte.
            if self.is_empty() {
                self.bytes = other_bits.bytes;
            } else {
                self.bytes.extend(other_bits.bytes.into_iter());
            }
            // Extend self's length.
            self.length += other_bits.length;
        }
    }

    /// Constructs a new empty `BitArrayList`.
    ///
    /// The new bit array will not allocate bits until bits are pushed into it.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let bit_array = BitArrayList::new();
    ///
    /// assert!(bit_array.is_empty());
    /// ```
    pub fn new() -> BitArrayList {
        BitArrayList {
            bytes: vec![0],
            length: 0,
        }
    }

    /// This helper method converts a `Vec` of bytes and a length to a `BitArrayList`.
    ///
    /// # Safety
    ///
    /// Specifying a shorter length will not drop the elements after the last index and they may be
    /// read with [bytes][4].
    ///
    /// [4]: struct.BitArrayList.html#method.bytes
    ///
    /// # Examples
    ///
    /// Information left behind:
    ///
    /// ```
    /// use bit_array_list::BitArrayList;
    /// let mut bit_array = BitArrayList::from(vec![128, 224], 10);
    ///
    /// assert_eq!(bit_array.pop(), Some(true));
    /// // 11th bit is still set.
    /// assert_eq!(bit_array.bytes(), &vec![128, 128 + 32]);
    /// ```
    pub fn from(b: Vec<u8>, l: usize) -> BitArrayList {
        BitArrayList {
            bytes: b,
            length: l,
        }
    }

    /// Use this method to determine if bit at index `i` is set or not.
    fn zero_testing(&self, byte_index: usize, bit_position: u8) -> bool {
        if self.bytes[byte_index] & bitmask(bit_position) != 0 {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for BitArrayList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.length {
            0 => write!(f, "[]"),
            1 => write!(f, "[{}]", self.bit_to_str(0)),
            n => {
                write!(f, "[").unwrap();

                for i in 0..n - 1 {
                    write!(f, "{}, ", self.bit_to_str(i)).unwrap();
                }

                write!(f, "{}]", self.bit_to_str(n - 1))
            }
        }
    }
}

/// Use this method to convert a bit index to a byte index and the relevant index of the bit within
/// that byte.
fn split_index(to_split: usize) -> (usize, u8) {
    (to_split / 8, (to_split % 8) as u8)
}

/// Use this method to get the byte which singles out the the bit with given index `i` within some
/// byte.
///
/// # Panics
///
/// If `i >= 8`.
fn bitmask(bit_position: u8) -> u8 {
    if bit_position < 8 {
        128 >> bit_position
    } else {
        panic!("Bit index within some byte is out of bounds.");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bitmask_generation() {
        use std::panic::catch_unwind;

        assert_eq!(super::bitmask(0), 128);
        assert_eq!(super::bitmask(5), 4);
        assert!(catch_unwind(|| super::bitmask(9)).is_err());
    }

    #[test]
    fn index_splitting() {
        assert_eq!(super::split_index(5), (0, 5));
        assert_eq!(super::split_index(8), (1, 0));
        assert_eq!(super::split_index(19), (2, 3));
    }
}
