#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut collector: Vec<u8> = Vec::new();
    for to_encode in values {
        // Given 128 = 10000000 => [0x81, 0x0] = [10000001, 00000000]

        let mut count = 0u32;
        let mut little_endian_collection = Vec::new();
        loop {
            let first_bit  = if count == 0 { 0x0 } else { 0x80 };
            let next_byte = first_bit | (0x7F & (to_encode >> (7 * count))) as u8;
            little_endian_collection.push(next_byte);
            count += 1;
            if to_encode.unbounded_shr(7*count) == 0 {
                break;
            }
        }

        little_endian_collection.reverse();
        collector.append(&mut little_endian_collection);
    }

    collector
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut collector: Vec<u32> = Vec::new();

    let mut accumulator = 0u32;
    let mut is_continuing = false;
    for byte in bytes {
        accumulator = (accumulator << 7) | ((*byte as u32) & 0x7F);
        is_continuing = byte & 0x80 > 0;
        
        if !is_continuing {
            // reset the accumulator, since no more bits exist
            collector.push(accumulator);
            accumulator = 0;
        }
    }

    if is_continuing {
        // We started a continuation bit but never reached a terminating bit
        Err(Error::IncompleteNumber)
    } else {
        Ok(collector)
    }
}