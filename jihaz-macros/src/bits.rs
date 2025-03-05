//! 
//! Bit manipuation functions as well as the maximum values that can be stored in a specified number of bits
//! 

use jihaz_primal::bits::number_of_bits;

/// Returns the size or number of bits needed to hold maximum values possible for low and high
pub const fn needed_integer_size(low_max: u16, high_max: u16) -> usize {
    number_of_bits(low_max as u64) + number_of_bits(high_max as u64)
}

/// Combines two integers into one, shift as little bits as possible
pub const fn combine_as(low: u64, low_max: u64, mut high: u64, high_max: u64) -> u64 {
    let low_bits = number_of_bits(low_max);
    let high_bits = number_of_bits(high_max);
    high = high << low_bits + high_bits;
    high | low
}

/// Retrieve the two integers from the combined one
pub const fn retrieve_from(combined: u64, low_max: u64, high_max: u64) -> (u64, u64) {
    let low_bits = number_of_bits(low_max as u64);
    let high_bits = number_of_bits(high_max as u64);
    (
        (combined & low_bits as u64),
        ((combined >> low_bits) & high_bits as u64)
    )
}

macro_rules!  {
    () => {
        use ::jihaz_primal::bits::number_of_bits;

        // Returns the size or number of bits needed to hold maximum values possible for low and high
        pub const fn needed_integer_size(low_max: u16, high_max: u16) -> usize {
            number_of_bits(low_max as u64) + number_of_bits(high_max as u64)
        }

        /// Combines two integers into one, shift as little bits as possible
        pub const fn combine_as_u16(low: u16, low_max: u16, mut high: u16, high_max: u16) -> u16 {
            let low_bits = number_of_bits(low_max as u64);
            let high_bits = number_of_bits(high_max as u64);
            high = (high as u16) << low_bits + high_bits;
            high | low
        }

        /// Retrieve the two integers from the combined one
        pub const fn retrieve_from_u16(combined: u16, low_max: u16, high_max: u16) -> (u16, u16) {
            let low_bits = number_of_bits(low_max as u64);
            let high_bits = number_of_bits(high_max as u64);
            (
                (combined & low_bits as u16),
                ((combined >> low_bits) & high_bits as u16)
            )
        }
    };
}