// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;
#[derive(Debug)]
pub struct SaturatingU16{
    value : u16,
}

impl From<u16> for SaturatingU16{
    fn from(val : u16) -> Self{
       SaturatingU16 {value : val} 
    }
}

impl From<u8> for SaturatingU16{
    fn from(val : u8) -> Self{
        SaturatingU16::from(val as u16)
    }   
}

impl From<&u16> for SaturatingU16{
    fn from(val : &u16) -> Self{
        SaturatingU16{value : *val}
    }
}

impl From<&u8> for SaturatingU16{
    fn from(val : &u8) -> Self{
        (*value).into
    }   
}

impl PartialEq for SaturatingU16{
    fn eq(&self, _other : &Self) ->bool{
        &self.value == &_other.value
    }
}

impl Add for SaturatingU16{ 
    type Output = Self;
    fn add(self, other : Self) -> Self{
        self + other.value
    }
}

/*impl Add for SaturatingU16{
    type Output = Self;
    fn add(self, other : Self)-> Self{
        SaturatingU16{value: self.value.wrapping_add(other.value)}
    }
}*/

impl Add<&SaturatingU16> for SaturatingU16{
    type Output = Self;
    fn add(self, other : &SaturatingU16) -> Self{
        SaturatingU16{value : self.value + other.value}
    }
}

impl Add<u16> for SaturatingU16{
    type Output = Self;
    fn add(self, other : u16) -> Self{
        self + Self{value: other}
    }
} 

impl Add<&u16> for SaturatingU16{
    type Output = Self;
    fn add(self, other : &u16) -> Self{
        self + Self{value: *other}
    }
}



