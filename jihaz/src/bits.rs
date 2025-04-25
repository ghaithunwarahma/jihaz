//! 
//! Bit manipuation functions as well as the maximum values that can be stored in a specified number of bits
//! 
pub use jihaz_primal::bits::*;

/// Bit manipuation functions as well
pub trait StashInBits {
    /// Adds to right, when we know the maximum value.
    fn add_to_right_given_value(&mut self, value: u64, right_max_value: u64);
    /// Adds to right, when we know the maximum bits.
    fn add_to_right_given_bits(&mut self, value: u64, bits_of_right_max_value: usize);
    /// Remove from right, when we know the maximum value
    fn take_from_right_given_value(&mut self, right_max_value: u64) -> u64;
    /// Remove from right, when we know the maximum bits
    fn take_from_right_given_bits(&mut self, bits_of_right_max_value: usize) -> u64;
    /// Read a certain value stored in specific bits.
    /// 
    /// The shift can be zero when we're reading the right most bits in self.
    fn read_given_bits(&self, bits_of_max_value: usize, shift_from_right: usize) -> u64;
    /// Read a certain value stored in specific bits; given the maximum bitwise value of these bits.
    /// 
    /// The shift can be zero when we're reading the right most bits in self.
    fn read_given_bitwise_max_value(&self, bitwise_max_value: u64, shift_from_right: usize) -> u64;
}

impl StashInBits for u64 {
    fn add_to_right_given_value(&mut self, value: u64, right_max_value: u64) {
        *self = const_stash_in_bits::add_to_right_given_value(*self, value, right_max_value);
    }

    fn add_to_right_given_bits(&mut self, value: u64, bits_of_right_max_value: usize) {
        *self = const_stash_in_bits::add_to_right_given_bits(*self, value, bits_of_right_max_value);
    }

    fn take_from_right_given_value(&mut self, right_max_value: u64) -> u64 {
        let (this, res) = const_stash_in_bits::take_from_right_given_value(*self, right_max_value);
        *self = this;
        res
    }

    fn take_from_right_given_bits(&mut self, bits_of_right_max_value: usize) -> u64 {
        let (this, res) = const_stash_in_bits::take_from_right_given_bits(*self, bits_of_right_max_value);
        *self = this;
        res
    }
    
    fn read_given_bits(&self, bits_of_max_value: usize, shift_from_right: usize) -> u64 {
        const_stash_in_bits::read_given_bits(*self, bits_of_max_value, shift_from_right)
    }

    fn read_given_bitwise_max_value(&self, bitwise_max_value: u64, shift_from_right: usize) -> u64 {
        const_stash_in_bits::read_given_bitwise_max_value(*self, bitwise_max_value, shift_from_right)
    }
}

pub mod const_stash_in_bits {
    use jihaz_primal::bits::{max_bitwise_value_for_bits, number_of_bits};

    pub const fn add_to_right_given_value(this: u64, value: u64, right_max_value: u64) -> u64{
        let bits_of_right_max_value = number_of_bits(right_max_value);
        add_to_right_given_bits(this, value, bits_of_right_max_value)
    }

    pub const fn add_to_right_given_bits(mut this: u64, value: u64, bits_of_right_max_value: usize) -> u64 {
        // For example, 1101 with 10

        // shift by the number of bits needed to save the new value
        // 1101 => 110100
        this = this << bits_of_right_max_value;

        // add the new value to the right
        // 110100 | 10 => 110110
        this = this | value;
        this
    }

    pub const fn take_from_right_given_value(this: u64, right_max_value: u64) -> (u64, u64) {
        let bits_of_right_max_value = number_of_bits(right_max_value);
        take_from_right_given_bits(this, bits_of_right_max_value)
    }

    pub const fn take_from_right_given_bits(this: u64, bits_of_right_max_value: usize) -> (u64, u64) {
        // For example, 10 from 110110

        let mut left = this;

        // right_bitwise_max_value => 11
        let right_bitwise_max_value = max_bitwise_value_for_bits(bits_of_right_max_value);

        // left & right_bitwise_max_value => 10 (right value)
        let right = left & right_bitwise_max_value;

        let copy_of_self = left;

        // remove the right bits from self
        left = copy_of_self >> bits_of_right_max_value;
        
        (left, right)
    }

    pub const fn read_given_bits(this: u64, bits_of_max_value: usize, shift_from_right: usize) -> u64 {
        
        let bitwise_max_value = max_bitwise_value_for_bits(bits_of_max_value);
        read_given_bitwise_max_value(this, bitwise_max_value, shift_from_right)
    }

    pub const fn read_given_bitwise_max_value(this: u64, bitwise_max_value: u64, shift_from_right: usize) -> u64 {
        
        match shift_from_right {
            0 => this & bitwise_max_value,
            _ => (this >> shift_from_right) & bitwise_max_value,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_VALUES: [u64; 4] = [
        MAX_U10 as u64, 
        MAX_U7 as u64, 
        MAX_U11 as u64,
        MAX_U13 as u64,
    ];
    const VALUES: [u64; 4] = [843, 120, 2000, 7777];

    fn steps_add(mut left: u64, right: u64, right_max: u64) -> u64 {
        println!("> left {:#b}", left);

        // shift by the number of bits needed to save the new value
        println!("shift by the number of bits needed to save the new value");
        left = left << number_of_bits(right_max);
        println!("> left {:#b}", left);

        println!("> + right {:#b}", right);

        // add the new value to the right
        println!("add the new value to the right");
        left = left | right;
        println!("> left {:#b}", left);

        left
    }

    fn steps_take(mut left: u64, right_max: u64) -> (u64, u64) {
        // For example, 10 from 110110

        let right_bitwise_max_value = max_bitwise_value(right_max);
        println!("> right_bitwise_max_value => {:#b}", right_bitwise_max_value);

        let right = left & right_bitwise_max_value;
        println!("> left AND right_bitwise_max_value === right => {:#b}", right );

        let copy_of_self = left;

        // remove the right bits from self
        println!("remove the right bits from self:");
        left = copy_of_self >> number_of_bits(right_max);
        println!("> left => {:#b}", left);

        (left, right)
    }

    #[test]
    fn bits_steps() {

        // [ - ] adding

        let left = steps_add(2000, 3, 12);

        // year 2000 (max 3000), month 3 (max 12)

        // [ - ] taking

        let (left, res) = steps_take(left, 12);

        assert!(res == 3 as u64);
        assert!(left == 2000 as u64)
    }

    #[test]
    fn bits_steps2() {

        // [ - ] adding

        let left = steps_add(2000, 3, 12);
        println!();
        let left = steps_add(left, 27, 31);
        println!();

        // year 2000 (max 3000), month 3 (max 12), day 27 (max 31)

        // [ - ] taking

        let (left, res) = steps_take(left, 31);
        println!();

        assert!(res == 27);

        let (left, res) = steps_take(left, 12);
        println!();

        assert!(res == 3);
        assert!(left == 2000)
    }

    #[test]
    fn bits() {
        // adds

        let mut current_value = VALUES[0];
        
        current_value.add_to_right_given_value(VALUES[1], MAX_VALUES[1]);
        
        // takes
        
        let res = current_value.take_from_right_given_value(MAX_VALUES[1]);
        
        println!("res: {:?}", res);
        assert!(res == VALUES[1]);

        println!("res: {:?}", res);
        assert!(current_value == VALUES[0]);
    }

    #[test]
    fn bits2() {
        // adds

        let mut current_value = VALUES[0];

        current_value.add_to_right_given_value(VALUES[1], MAX_VALUES[1]);
        
        current_value.add_to_right_given_value(VALUES[2], MAX_VALUES[2]);
        
        // takes

        let res = current_value.take_from_right_given_value(MAX_VALUES[2]);
        assert!(res == VALUES[2]);

        let res = current_value.take_from_right_given_value(MAX_VALUES[1]);
        assert!(res == VALUES[1]);

        assert!(current_value == VALUES[0]);
    }

    #[test]
    fn bits3() {
        // adds

        let mut current_value = VALUES[0];

        current_value.add_to_right_given_value(VALUES[1], MAX_VALUES[1]);
        
        current_value.add_to_right_given_value(VALUES[2], MAX_VALUES[2]);

        current_value.add_to_right_given_value(VALUES[3], MAX_VALUES[3]);
        
        // takes

        let res = current_value.take_from_right_given_value(MAX_VALUES[3]);

        assert!(res == VALUES[3]);

        let res = current_value.take_from_right_given_value(MAX_VALUES[2]);

        assert!(res == VALUES[2]);

        let res = current_value.take_from_right_given_value(MAX_VALUES[1]);

        assert!(res == VALUES[1]);

        assert!(current_value == VALUES[0]);
    }
}

// /// Returns the size or number of bits needed to hold two integers
// pub const fn needed_integer_size(low_max: u64, high_max: u64) -> usize {
//     number_of_bits(low_max) + number_of_bits(high_max)
// }

// /// Combines two integers into one, shifts as little bits as possible
// pub const fn combine_integers_given_value_limits(low: u64, low_max: u64, high: u64, high_max: u64) -> u64 {
//     let low_bits = number_of_bits(low_max);
//     let high_bits = number_of_bits(high_max);
//     combine_integers_given_bit_limits(low, low_bits, high, high_bits)
// }

// /// Retrieve the two integers from a combined integer
// pub const fn retrieve_integers_given_value_limits(combined: u64, low_max: u64, high_max: u64) -> (u64, u64) {
//     let low_bits = number_of_bits(low_max);
//     let high_bits = number_of_bits(high_max);
//     retrieve_integers_given_bit_limits(combined, low_bits, high_bits)
// }

// /// Combines two integers into one
// pub const fn combine_integers_given_bit_limits(low: u64, low_bits: usize, mut high: u64, high_bits: usize) -> u64 {
//     high = high << low_bits;
//     high | low
// }

// /// Retrieve the two integers from a combined integer
// pub const fn retrieve_integers_given_bit_limits(combined: u64, low_bits: usize, high_bits: usize) -> (u64, u64) {
//     (
//         combined & low_bits as u64,
//         (combined >> low_bits) & high_bits as u64
//     )
// }