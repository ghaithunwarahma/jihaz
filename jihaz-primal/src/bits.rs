//! 
//! The maximum values that can be stored in a specified number of bits
//! 

// ----- ----- Methods ----- -----

pub const fn max_u8(bits: usize) -> u8 {
    u8::MAX >> (8 - bits)
}

pub const fn max_u16(bits: usize) -> u16 {
    u16::MAX >> (16 - bits)
}

pub const fn max_u32(bits: usize) -> u32 {
    u32::MAX >> (32 - bits)
}

pub const fn max_u64(bits: usize) -> u64 {
    u64::MAX >> (64 - bits)
}

// ----- ----- Macros ----- -----

macro_rules! max_u8 {
    ($bits:expr) => ( u8::MAX >> (8 - $bits) )
}

macro_rules! max_u16 {
    ($bits:expr) => ( u16::MAX >> (16 - $bits) )
}

macro_rules! max_u32 {
    ($bits:expr) => ( u32::MAX >> (32 - $bits) )
}

macro_rules! max_u64 {
    ($bits:expr) => ( u64::MAX >> (64 - $bits) )
}

/// Converts a maximum value given by user to its storing bits number; 
/// and then to the maximum value these bits can store.
pub const fn max_bitwise_value(max_user_value: u64) -> u64 {
    max_bitwise_value_for_bits(number_of_bits(max_user_value))
}

/// The maximum value that can be stored in the given number of bits
pub const fn max_bitwise_value_for_bits(number_of_bits: usize) -> u64 {
    assert!(0 < number_of_bits && number_of_bits <= 64);
    if number_of_bits <= 32 {
        if number_of_bits <= 16 {
            if number_of_bits <= 8 {
                max_u8!(number_of_bits) as u64
            } else {
                max_u16!(number_of_bits) as u64
            }
        } else {
            max_u32!(number_of_bits) as u64
        }
    } else {
        max_u64!(number_of_bits) as u64
    }
}

pub const MAX_U1: u8 = max_u8!(1);
pub const MAX_U2: u8 = max_u8!(2);
pub const MAX_U3: u8 = max_u8!(3);
pub const MAX_U4: u8 = max_u8!(4);
pub const MAX_U5: u8 = max_u8!(5);
pub const MAX_U6: u8 = max_u8!(6);
pub const MAX_U7: u8 = max_u8!(7);
pub const MAX_U8: u8 = u8::MAX;
pub const MAX_U9: u16 = max_u16!(9);

pub const MAX_U10: u16 = max_u16!(10);
pub const MAX_U11: u16 = max_u16!(11);
pub const MAX_U12: u16 = max_u16!(12);
pub const MAX_U13: u16 = max_u16!(13);
pub const MAX_U14: u16 = max_u16!(14);
pub const MAX_U15: u16 = max_u16!(15);
pub const MAX_U16: u16 = u16::MAX;
pub const MAX_U17: u32 = max_u32!(17);
pub const MAX_U18: u32 = max_u32!(18);
pub const MAX_U19: u32 = max_u32!(19);

pub const MAX_U20: u32 = max_u32!(20);
pub const MAX_U21: u32 = max_u32!(21);
pub const MAX_U22: u32 = max_u32!(22);
pub const MAX_U23: u32 = max_u32!(23);
pub const MAX_U24: u32 = max_u32!(24);
pub const MAX_U25: u32 = max_u32!(25);
pub const MAX_U26: u32 = max_u32!(26);
pub const MAX_U27: u32 = max_u32!(27);
pub const MAX_U28: u32 = max_u32!(28);
pub const MAX_U29: u32 = max_u32!(29);

pub const MAX_U30: u32 = max_u32!(30);
pub const MAX_U31: u32 = max_u32!(31);
pub const MAX_U32: u32 = u32::MAX;
pub const MAX_U33: u64 = max_u64!(33);
pub const MAX_U34: u64 = max_u64!(34);
pub const MAX_U35: u64 = max_u64!(35);
pub const MAX_U36: u64 = max_u64!(36);
pub const MAX_U37: u64 = max_u64!(37);
pub const MAX_U38: u64 = max_u64!(38);
pub const MAX_U39: u64 = max_u64!(39);

pub const MAX_U40: u64 = max_u64!(40);
pub const MAX_U41: u64 = max_u64!(41);
pub const MAX_U42: u64 = max_u64!(42);
pub const MAX_U43: u64 = max_u64!(43);
pub const MAX_U44: u64 = max_u64!(44);
pub const MAX_U45: u64 = max_u64!(45);
pub const MAX_U46: u64 = max_u64!(46);
pub const MAX_U47: u64 = max_u64!(47);
pub const MAX_U48: u64 = max_u64!(48);
pub const MAX_U49: u64 = max_u64!(49);

pub const MAX_U50: u64 = max_u64!(50);
pub const MAX_U51: u64 = max_u64!(51);
pub const MAX_U52: u64 = max_u64!(52);
pub const MAX_U53: u64 = max_u64!(53);
pub const MAX_U54: u64 = max_u64!(54);
pub const MAX_U55: u64 = max_u64!(55);
pub const MAX_U56: u64 = max_u64!(56);
pub const MAX_U57: u64 = max_u64!(57);
pub const MAX_U58: u64 = max_u64!(58);
pub const MAX_U59: u64 = max_u64!(59);

pub const MAX_U60: u64 = max_u64!(60);
pub const MAX_U61: u64 = max_u64!(61);
pub const MAX_U62: u64 = max_u64!(62);
pub const MAX_U63: u64 = max_u64!(63);
pub const MAX_U64: u64 = u64::MAX;
// pub const MAX_U65: u8 = max_u16!(65);
// pub const MAX_U66: u8 = max_u16!(66);
// pub const MAX_U67: u8 = max_u16!(67);
// pub const MAX_U68: u8 = max_u16!(68);
// pub const MAX_U69: u8 = max_u16!(69);

/// The maximum value that can be stored in the given number of bits
pub const fn max_bitwise_value_for_bits_2(number_of_bits: usize) -> u64 {
    if number_of_bits <= 32 {
        if number_of_bits <= 16 {
            if number_of_bits <= 8 {
                if number_of_bits <= 4 {
                    if number_of_bits <= 2 {
                        if number_of_bits == 1 {
                            MAX_U1 as u64
                        } else {
                            MAX_U2 as u64
                        }
                    } else {
                        if number_of_bits == 3 {
                            MAX_U3 as u64
                        } else {
                            MAX_U4 as u64
                        }
                    }
                } else {
                    if number_of_bits <= 6 {
                        if number_of_bits == 5 {
                            MAX_U5 as u64
                        } else {
                            MAX_U6 as u64
                        }
                    } else {
                        if number_of_bits == 7 {
                            MAX_U7 as u64
                        } else {
                            MAX_U8 as u64
                        }
                    }
                }
            } else {
                if number_of_bits <= 12 {
                    if number_of_bits <= 10 {
                        if number_of_bits == 9 {
                            MAX_U9 as u64
                        } else {
                            MAX_U10 as u64
                        }
                    } else {
                        if number_of_bits == 11 {
                            MAX_U11 as u64
                        } else {
                            MAX_U12 as u64
                        }
                    }
                } else {
                    if number_of_bits <= 14 {
                        if number_of_bits == 13 {
                            MAX_U13 as u64
                        } else {
                            MAX_U14 as u64
                        }
                    } else {
                        if number_of_bits == 15 {
                            MAX_U15 as u64
                        } else {
                            MAX_U16 as u64
                        }
                    }
                }
            }
        } else {
            if number_of_bits <= 24 {
                if number_of_bits <= 20 {
                    if number_of_bits <= 18 {
                        if number_of_bits == 17 {
                            MAX_U17 as u64
                        } else {
                            MAX_U18 as u64
                        }
                    } else {
                        if number_of_bits == 19 {
                            MAX_U19 as u64
                        } else {
                            MAX_U20 as u64
                        }
                    }
                } else {
                    if number_of_bits <= 22 {
                        if number_of_bits == 21 {
                            MAX_U21 as u64
                        } else {
                            MAX_U22 as u64
                        }
                    } else {
                        if number_of_bits == 23 {
                            MAX_U23 as u64
                        } else {
                            MAX_U24 as u64
                        }
                    }
                }
            } else {
                if number_of_bits <= 28 {
                    if number_of_bits <= 26 {
                        if number_of_bits == 25 {
                            MAX_U25 as u64
                        } else {
                            MAX_U26 as u64
                        }
                    } else {
                        if number_of_bits == 27 {
                            MAX_U27 as u64
                        } else {
                            MAX_U28 as u64
                        }
                    }
                } else {
                    if number_of_bits <= 30 {
                        if number_of_bits == 29 {
                            MAX_U29 as u64
                        } else {
                            MAX_U30 as u64
                        }
                    } else {
                        if number_of_bits == 31 {
                            MAX_U31 as u64
                        } else {
                            MAX_U32 as u64
                        }
                    }
                }
            }
        }
    } else {
        if number_of_bits <= 48 {
            if number_of_bits <= 40 {
                if number_of_bits <= 36 {
                    if number_of_bits <= 34 {
                        if number_of_bits == 33 {
                            MAX_U33 as u64
                        } else {
                            MAX_U34 as u64
                        }
                    } else {
                        if number_of_bits == 35 {
                            MAX_U35 as u64
                        } else {
                            MAX_U36 as u64
                        }
                    }
                } else {
                    if number_of_bits <= 38 {
                        if number_of_bits == 37 {
                            MAX_U37 as u64
                        } else {
                            MAX_U38 as u64
                        }
                    } else {
                        if number_of_bits == 39 {
                            MAX_U39 as u64
                        } else {
                            MAX_U40 as u64
                        }
                    }
                }
            } else {
                if number_of_bits <= 44 {
                    if number_of_bits <= 42 {
                        if number_of_bits == 41 {
                            MAX_U41 as u64
                        } else {
                            MAX_U42 as u64
                        }
                    } else {
                        if number_of_bits == 43 {
                            MAX_U43 as u64
                        } else {
                            MAX_U44 as u64
                        }
                    }
                } else {
                    if number_of_bits <= 46 {
                        if number_of_bits == 45 {
                            MAX_U45 as u64
                        } else {
                            MAX_U46 as u64
                        }
                    } else {
                        if number_of_bits == 47 {
                            MAX_U47 as u64
                        } else {
                            MAX_U48 as u64
                        }
                    }
                }
            }
        } else {
            if number_of_bits <= 56 {
                if number_of_bits <= 52 {
                    if number_of_bits <= 50 {
                        if number_of_bits == 49 {
                            MAX_U49 as u64
                        } else {
                            MAX_U50 as u64
                        }
                    } else {
                        if number_of_bits == 51 {
                            MAX_U51 as u64
                        } else {
                            MAX_U52 as u64
                        }
                    }
                } else {
                    if number_of_bits <= 54 {
                        if number_of_bits == 53 {
                            MAX_U53 as u64
                        } else {
                            MAX_U54 as u64
                        }
                    } else {
                        if number_of_bits == 55 {
                            MAX_U55 as u64
                        } else {
                            MAX_U56 as u64
                        }
                    }
                }
            } else {
                if number_of_bits <= 60 {
                    if number_of_bits <= 58 {
                        if number_of_bits == 57 {
                            MAX_U57 as u64
                        } else {
                            MAX_U58 as u64
                        }
                    } else {
                        if number_of_bits == 59 {
                            MAX_U59 as u64
                        } else {
                            MAX_U60 as u64
                        }
                    }
                } else {
                    if number_of_bits <= 62 {
                        if number_of_bits == 61 {
                            MAX_U61 as u64
                        } else {
                            MAX_U62 as u64
                        }
                    } else {
                        if number_of_bits == 63 {
                            MAX_U63 as u64
                        } else {
                            MAX_U64 as u64
                        }
                    }
                }
            }
        }
    }
}

/// The number of bits that are needed to store the given maximum integer value
// pub const fn number_of_bits(max_value: u16) -> usize {
pub const fn number_of_bits(max_value: u64) -> usize {
    if max_value as u64 <= MAX_U32 as u64 {
        if max_value as u32 <= MAX_U16 as u32 {
            if max_value as u16 <= MAX_U8 as u16 {
                if max_value as u8 <= MAX_U4 as u8 {
                    if max_value as u8 <= MAX_U2 as u8 {
                        if max_value as u8 <= MAX_U1 as u8 {
                            1
                        } else {
                            2
                        }
                    } else {
                        if max_value as u8 <= MAX_U3 as u8 {
                            3
                        } else {
                            4
                        }
                    }
                } else {
                    if max_value as u8 <= MAX_U6 as u8 {
                        if max_value as u8 <= MAX_U5 as u8 {
                            5
                        } else {
                            6
                        }
                    } else {
                        if max_value as u8 <= MAX_U7 as u8 {
                            7
                        } else {
                            8
                        }
                    }
                }
            } else {
                if max_value as u16 <= MAX_U12 as u16 {
                    if max_value as u16 <= MAX_U10 as u16 {
                        if max_value as u16 <= MAX_U9 as u16 {
                            9
                        } else {
                            10
                        }
                    } else {
                        if max_value as u16 <= MAX_U11 as u16 {
                            11
                        } else {
                            12
                        }
                    }
                } else {
                    if max_value as u16 <= MAX_U14 as u16 {
                        if max_value as u16 <= MAX_U13 as u16 {
                            13
                        } else {
                            14
                        }
                    } else {
                        if max_value as u16 <= MAX_U15 as u16 {
                            15
                        } else {
                            16
                        }
                    }
                }
            }
        } else {
            if max_value as u32 <= MAX_U24 as u32 {
                if max_value as u32 <= MAX_U20 as u32 {
                    if max_value as u32 <= MAX_U18 as u32 {
                        if max_value as u32 <= MAX_U17 as u32 {
                            17
                        } else {
                            18
                        }
                    } else {
                        if max_value as u32 <= MAX_U19 as u32 {
                            19
                        } else {
                            20
                        }
                    }
                } else {
                    if max_value as u32 <= MAX_U22 as u32 {
                        if max_value as u32 <= MAX_U21 as u32 {
                            21
                        } else {
                            22
                        }
                    } else {
                        if max_value as u32 <= MAX_U23 as u32 {
                            23
                        } else {
                            24
                        }
                    }
                }
            } else {
                if max_value as u32 <= MAX_U28 as u32 {
                    if max_value as u32 <= MAX_U26 as u32 {
                        if max_value as u32 <= MAX_U25 as u32 {
                            25
                        } else {
                            26
                        }
                    } else {
                        if max_value as u32 <= MAX_U27 as u32 {
                            27
                        } else {
                            28
                        }
                    }
                } else {
                    if max_value as u32 <= MAX_U30 as u32 {
                        if max_value as u32 <= MAX_U29 as u32 {
                            29
                        } else {
                            30
                        }
                    } else {
                        if max_value as u32 <= MAX_U31 as u32 {
                            31
                        } else {
                            32
                        }
                    }
                }
            }
        }
    } else {
        if max_value as u64 <= MAX_U48 as u64 {
            if max_value as u64 <= MAX_U40 as u64 {
                if max_value as u64 <= MAX_U36 as u64 {
                    if max_value as u64 <= MAX_U34 as u64 {
                        if max_value as u64 <= MAX_U33 as u64 {
                            33
                        } else {
                            34
                        }
                    } else {
                        if max_value as u64 <= MAX_U35 as u64 {
                            35
                        } else {
                            36
                        }
                    }
                } else {
                    if max_value as u64 <= MAX_U38 as u64 {
                        if max_value as u64 <= MAX_U37 as u64 {
                            37
                        } else {
                            38
                        }
                    } else {
                        if max_value as u64 <= MAX_U39 as u64 {
                            39
                        } else {
                            40
                        }
                    }
                }
            } else {
                if max_value as u64 <= MAX_U44 as u64 {
                    if max_value as u64 <= MAX_U42 as u64 {
                        if max_value as u64 <= MAX_U41 as u64 {
                            41
                        } else {
                            42
                        }
                    } else {
                        if max_value as u64 <= MAX_U43 as u64 {
                            43
                        } else {
                            44
                        }
                    }
                } else {
                    if max_value as u64 <= MAX_U46 as u64 {
                        if max_value as u64 <= MAX_U45 as u64 {
                            45
                        } else {
                            46
                        }
                    } else {
                        if max_value as u64 <= MAX_U47 as u64 {
                            47
                        } else {
                            48
                        }
                    }
                }
            }
        } else {
            if max_value as u64 <= MAX_U56 as u64 {
                if max_value as u64 <= MAX_U52 as u64 {
                    if max_value as u64 <= MAX_U50 as u64 {
                        if max_value as u64 <= MAX_U49 as u64 {
                            49
                        } else {
                            50
                        }
                    } else {
                        if max_value as u64 <= MAX_U51 as u64 {
                            51
                        } else {
                            52
                        }
                    }
                } else {
                    if max_value as u64 <= MAX_U54 as u64 {
                        if max_value as u64 <= MAX_U53 as u64 {
                            53
                        } else {
                            54
                        }
                    } else {
                        if max_value as u64 <= MAX_U55 as u64 {
                            55
                        } else {
                            56
                        }
                    }
                }
            } else {
                if max_value as u64 <= MAX_U60 as u64 {
                    if max_value as u64 <= MAX_U58 as u64 {
                        if max_value as u64 <= MAX_U57 as u64 {
                            57
                        } else {
                            58
                        }
                    } else {
                        if max_value as u64 <= MAX_U59 as u64 {
                            59
                        } else {
                            60
                        }
                    }
                } else {
                    if max_value as u64 <= MAX_U62 as u64 {
                        if max_value as u64 <= MAX_U61 as u64 {
                            61
                        } else {
                            62
                        }
                    } else {
                        if max_value as u64 <= MAX_U63 as u64 {
                            63
                        } else {
                            64
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits() {
        for bits in 1..=64 {
            assert_eq!(bits, number_of_bits(max_bitwise_value_for_bits(bits)));
        }
    }
}