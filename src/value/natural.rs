

#[derive(Clone)]
pub struct Natural {
    bytes: Vec<u8>,
}


impl Natural {
    pub fn new(bytes: Vec<u8>) -> Self {
        Natural { bytes }
    }
}

impl std::ops::Add for Natural {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for (a, b) in self.bytes.iter().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_add(*b);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::Sub for Natural {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for (a, b) in self.bytes.iter().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_sub(*b);
            let (result, new_carry2) = result.overflowing_sub(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::Mul for Natural {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for (a, b) in self.bytes.iter().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_mul(*b);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::Div for Natural {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for (a, b) in self.bytes.iter().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_div(*b);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::Rem for Natural {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for (a, b) in self.bytes.iter().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_rem(*b);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::AddAssign for Natural {
    fn add_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        for (a, b) in self.bytes.iter_mut().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_add(*b);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            *a = result;
        }
    }
}

impl std::ops::SubAssign for Natural {
    fn sub_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        for (a, b) in self.bytes.iter_mut().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_sub(*b);
            let (result, new_carry2) = result.overflowing_sub(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            *a = result;
        }
    }
}

impl std::ops::MulAssign for Natural {
    fn mul_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        for (a, b) in self.bytes.iter_mut().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_mul(*b);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            *a = result;
        }
    }
}

impl std::ops::DivAssign for Natural {
    fn div_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        for (a, b) in self.bytes.iter_mut().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_div(*b);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            *a = result;
        }
    }
}

impl std::ops::RemAssign for Natural {
    fn rem_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        for (a, b) in self.bytes.iter_mut().zip(rhs.bytes.iter()) {
            let (result, new_carry) = a.overflowing_rem(*b);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 {
                1
            } else {
                0
            };
            *a = result;
        }
    }
}


impl std::ops::BitAnd for Natural {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for (a, b) in self.bytes.iter().zip(rhs.bytes.iter()) {
            let result = a & b;
            let (result, new_carry) = result.overflowing_add(carry);
            carry = if new_carry {
                1
            } else {
                0
            };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::BitOr for Natural {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for (a, b) in self.bytes.iter().zip(rhs.bytes.iter()) {
            let result = a | b;
            let (result, new_carry) = result.overflowing_add(carry);
            carry = if new_carry {
                1
            } else {
                0
            };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::BitXor for Natural {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for (a, b) in self.bytes.iter().zip(rhs.bytes.iter()) {
            let result = a ^ b;
            let (result, new_carry) = result.overflowing_add(carry);
            carry = if new_carry {
                1
            } else {
                0
            };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::BitAndAssign for Natural {
    fn bitand_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        for (a, b) in self.bytes.iter_mut().zip(rhs.bytes.iter()) {
            let result =  *a & b;
            let (result, new_carry) = result.overflowing_add(carry);
            carry = if new_carry {
                1
            } else {
                0
            };

            *a = result;
        }
    }
}

impl std::ops::BitOrAssign for Natural {
    fn bitor_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        for (a, b) in self.bytes.iter_mut().zip(rhs.bytes.iter()) {
            let result = *a | b;
            let (result, new_carry) = result.overflowing_add(carry);
            carry = if new_carry {
                1
            } else {
                0
            };

            *a = result;
        }
    }
}

impl std::ops::BitXorAssign for Natural {
    fn bitxor_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        for (a, b) in self.bytes.iter_mut().zip(rhs.bytes.iter()) {
            let result = *a ^ b;
            let (result, new_carry) = result.overflowing_add(carry);
            carry = if new_carry  {
                1
            } else {
                0
            };

            *a = result;
        }
    }
}

impl std::ops::Shl<u32> for Natural {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for a in self.bytes.iter() {
            let (result, new_carry) = a.overflowing_shl(rhs);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 { 1 } else { 0 };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::Shr<u32> for Natural {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        let mut bytes = Vec::new();
        let mut carry = 0;
        for a in self.bytes.iter() {
            let (result, new_carry) = a.overflowing_shr(rhs);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 { 1 } else { 0 };
            bytes.push(result);
        }
        Natural { bytes }
    }
}

impl std::ops::ShlAssign<u32> for Natural {
    fn shl_assign(&mut self, rhs: u32) {
        let mut carry = 0;
        for a in self.bytes.iter_mut() {
            let (result, new_carry) = a.overflowing_shl(rhs);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 { 1 } else { 0 };
            *a = result;
        }
    }
}

impl std::ops::ShrAssign<u32> for Natural {
    fn shr_assign(&mut self, rhs: u32) {
        let mut carry = 0;
        for a in self.bytes.iter_mut() {
            let (result, new_carry) = a.overflowing_shr(rhs);
            let (result, new_carry2) = result.overflowing_add(carry);
            carry = if new_carry || new_carry2 { 1 } else { 0 };
            *a = result;
        }
    }
}

impl std::cmp::PartialEq for Natural {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl std::cmp::PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.bytes.len() > other.bytes.len() {
            Some(std::cmp::Ordering::Greater)
        } else if self.bytes.len() < other.bytes.len() {
            Some(std::cmp::Ordering::Less)
        } else {
            for (a, b) in self.bytes.iter().zip(other.bytes.iter()).rev() {
                if a > b {
                    return Some(std::cmp::Ordering::Greater);
                } else if a < b {
                    return Some(std::cmp::Ordering::Less);
                }
            }
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl std::fmt::Display for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        let mut digit: Vec<&str> = Vec::new();
        let digits = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        for byte in self.bytes.iter().rev() {

        }



        write!(f, "{}", string)
    }
}

impl std::fmt::Debug for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.bytes)
    }
}








impl From<u8> for Natural {
    fn from(value: u8) -> Self {
        Natural {
            bytes: vec![value],
        }
    }
}

impl From<u16> for Natural {
    fn from(value: u16) -> Self {
        Natural {
            bytes: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<u32> for Natural {
    fn from(value: u32) -> Self {
        Natural {
            bytes: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<u64> for Natural {
    fn from(value: u64) -> Self {
        Natural {
            bytes: value.to_le_bytes().to_vec(),
        }
    }
}

impl From<usize> for Natural {
    fn from(value: usize) -> Self {
        Natural {
            bytes: value.to_le_bytes().to_vec(),
        }
    }
}

impl Into<u8> for Natural {
    fn into(self) -> u8 {
        self.bytes[0]
    }
}

impl Into<u16> for Natural {
    fn into(self) -> u16 {
        u16::from_le_bytes(self.bytes[0..2].try_into().unwrap())
    }
}

impl Into<u32> for Natural {
    fn into(self) -> u32 {
        u32::from_le_bytes(self.bytes[0..4].try_into().unwrap())
    }
}

impl Into<u64> for Natural {
    fn into(self) -> u64 {
        u64::from_le_bytes(self.bytes[0..8].try_into().unwrap())
    }
}

impl Into<usize> for Natural {
    fn into(self) -> usize {
        usize::from_le_bytes(self.bytes[0..std::mem::size_of::<usize>()].try_into().unwrap())
    }
}