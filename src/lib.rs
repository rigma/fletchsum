// This file is part of fletchsum.
//
//  fletchsum is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  fletchsum is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Foobar.  If not, see <https://www.gnu.org/licenses/>

use std::ops::Range;

/// Performs the Fletcher-64 checksum onto the given vector u32
/// 
/// ### Examples
/// ```
/// let input = vec![1, 2, 3, 4];
/// 
/// assert_eq!(85899345930, fletchsum::fletcher_checksum(&input));
/// ```
fn fletcher_checksum(data: &Vec<u32>) -> u64 {
    // Setting up the working variables
    let mut c0: u64 = 0;
    let mut c1: u64 = 0;
    let mut start = 0;

    // While we haven't reach the end of the vector
    while data.len() > start {
        // Computing the current working range (we slice the vector into 360 elements chunks)
        let indexes: Range<usize>;
        if data.len() - start >= 360 {
            indexes = 0..360;
        } else {
            indexes = 0..data.len();
        }

        // For each element in the chunk, we'll compute the simple checksum and the Fletcher one
        for i in indexes {
            c0 += data[start + i] as u64;
            c1 += c0;
        }

        // Applying the modulus on each checksum
        c0 %= 0xffffffff;
        c1 %= 0xffffffff;

        // Moving to the next chunk
        start += 360;
    }

    // Returning the concatenation of the two checksums
    c1 << 32 | c0
}

/// Performs the Fletcher-64 checksum on the given string slice
/// 
/// ### Examples
/// ```
/// let input = "hello, world";
/// 
/// assert_eq!(32195074851976, fletchsum::fletcher_checksum_str(input));
/// ```
pub fn fletcher_checksum_str(data: &str) -> u64 {
    // First, we'll parse the string slice into a u32 vector
    let bytes: Vec<_> = data.as_bytes()
                            .iter()
                            .map(|c| *c as u32)
                            .collect();
    
    // Then, we'll return the value of the checksum of the produced vector
    fletcher_checksum(&bytes)
}

/// Performs the Fletcher-64 checksum on the given byte slice
/// 
/// ### Examples
/// ```
/// let input = vec![b'h', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd'];
/// 
/// assert_eq!(32195074851976, fletchsum::fletcher_checksum_bytes(&input));
/// ```
pub fn fletcher_checksum_bytes(data: &[u8]) -> u64 {
    // First, we'll parse the byte slice into a u32 vector
    let bytes: Vec<_> = data.iter()
                            .map(|b| *b as u32)
                            .collect();

    // Then, we'll return the value of the checksum of the produced vector
    fletcher_checksum(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fletcher_checksum() {
        let input = vec![1, 2, 3, 4];
        
        assert_eq!(85899345930, fletcher_checksum(&input));
    }
    
    #[test]
    fn test_fletcher_checksum_str() {
        let input = "hello, world";
        
        assert_eq!(32195074851976, fletcher_checksum_str(input));
    }

    #[test]
    fn test_fletcher_checksum_bytes() {
        let input = vec![b'h', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd'];

        assert_eq!(32195074851976, fletcher_checksum_bytes(&input));
    }
}
