use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

pub trait SubIf<'a> {
    fn sub_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + Sub + Clone;
}

impl<'a, T> SubIf<'a> for T
where
    T: Sized + Sub + Clone + 'a,
    &'a T: Sub<&'a T, Output = T>,
{
    fn sub_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self - other
        } else {
            self.clone()
        }
    }
}

pub trait AddIf<'a> {
    fn add_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + Add + Clone;
}

impl<'a, T> AddIf<'a> for T
where
    T: Sized + Add + Clone + 'a,
    &'a T: Add<&'a T, Output = T>,
{
    fn add_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self + other
        } else {
            self.clone()
        }
    }
}

pub trait DivIf<'a> {
    fn div_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + Div + Clone;
}

impl<'a, T> DivIf<'a> for T
where
    T: Sized + Div + Clone + 'a,
    &'a T: Div<&'a T, Output = T>,
{
    fn div_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self / other
        } else {
            self.clone()
        }
    }
}

pub trait MulIf<'a> {
    fn mul_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + Mul + Clone;
}

impl<'a, T> MulIf<'a> for T
where
    T: Sized + Mul + Clone + 'a,
    &'a T: Mul<&'a T, Output = T>,
{
    fn mul_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self * other
        } else {
            self.clone()
        }
    }
}

pub trait NegIf<'a> {
    fn neg_if(&'a self, condition: bool) -> Self
    where
        Self: Sized + Neg + Clone;
}

impl<'a, T> NegIf<'a> for T
where
    T: Sized + Neg + Clone + 'a,
    &'a T: Neg<Output = T>,
{
    fn neg_if(&'a self, condition: bool) -> Self {
        if condition {
            -self
        } else {
            self.clone()
        }
    }
}

pub trait RemIf<'a> {
    fn rem_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + Rem + Clone;
}

impl<'a, T> RemIf<'a> for T
where
    T: Sized + Rem + Clone + 'a,
    &'a T: Rem<&'a T, Output = T>,
{
    fn rem_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self % other
        } else {
            self.clone()
        }
    }
}

pub trait BitAndIf<'a> {
    fn bitand_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + BitAnd + Clone;
}

impl<'a, T> BitAndIf<'a> for T
where
    T: Sized + BitAnd + Clone + 'a,
    &'a T: BitAnd<&'a T, Output = T>,
{
    fn bitand_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self & other
        } else {
            self.clone()
        }
    }
}

pub trait BitOrIf<'a> {
    fn bitor_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + BitOr + Clone;
}

impl<'a, T> BitOrIf<'a> for T
where
    T: Sized + BitOr + Clone + 'a,
    &'a T: BitOr<&'a T, Output = T>,
{
    fn bitor_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self | other
        } else {
            self.clone()
        }
    }
}

pub trait BitXorIf<'a> {
    fn bitxor_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + BitXor + Clone;
}

impl<'a, T> BitXorIf<'a> for T
where
    T: Sized + BitXor + Clone + 'a,
    &'a T: BitXor<&'a T, Output = T>,
{
    fn bitxor_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self ^ other
        } else {
            self.clone()
        }
    }
}

pub trait NotIf<'a> {
    fn not_if(&'a self, condition: bool) -> Self
    where
        Self: Sized + Not + Clone;
}

impl<'a, T> NotIf<'a> for T
where
    T: Sized + Not + Clone + 'a,
    &'a T: Not<Output = T>,
{
    fn not_if(&'a self, condition: bool) -> Self {
        if condition {
            !self
        } else {
            self.clone()
        }
    }
}

pub trait ShlIf<'a> {
    fn shl_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + Shl + Clone;
}

impl<'a, T> ShlIf<'a> for T
where
    T: Sized + Shl + Clone + 'a,
    &'a T: Shl<&'a T, Output = T>,
{
    fn shl_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self << other
        } else {
            self.clone()
        }
    }
}

pub trait ShrIf<'a> {
    fn shr_if(&'a self, condition: bool, other: &'a Self) -> Self
    where
        Self: Sized + Shr + Clone;
}

impl<'a, T> ShrIf<'a> for T
where
    T: Sized + Shr + Clone + 'a,
    &'a T: Shr<&'a T, Output = T>,
{
    fn shr_if(&'a self, condition: bool, other: &'a Self) -> Self {
        if condition {
            self >> other
        } else {
            self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_if_i32() {
        assert_eq!(3, 5.sub_if(true, &2));
        assert_eq!(5, 5.sub_if(false, &2));
    }

    #[test]
    fn add_if_i32() {
        assert_eq!(7, 5.add_if(true, &2));
        assert_eq!(5, 5.add_if(false, &2));
    }

    #[test]
    fn div_if_i32() {
        assert_eq!(2, 5.div_if(true, &2));
        assert_eq!(5, 5.div_if(false, &2));
    }

    #[test]
    fn mul_if_i32() {
        assert_eq!(10, 5.mul_if(true, &2));
        assert_eq!(5, 5.mul_if(false, &2));
    }

    #[test]
    fn neg_if_i32() {
        assert_eq!(-5, 5.neg_if(true));
        assert_eq!(5, 5.neg_if(false));
    }

    #[test]
    fn rem_if_i32() {
        assert_eq!(1, 5.rem_if(true, &2));
        assert_eq!(5, 5.rem_if(false, &2));
    }

    #[test]
    fn bitand_if_i32() {
        assert_eq!(0, 5.bitand_if(true, &2));
        assert_eq!(5, 5.bitand_if(false, &2));
    }

    #[test]
    fn bitor_if_i32() {
        assert_eq!(7, 5.bitor_if(true, &2));
        assert_eq!(5, 5.bitor_if(false, &2));
    }

    #[test]
    fn bitxor_if_i32() {
        assert_eq!(7, 5.bitxor_if(true, &2));
        assert_eq!(5, 5.bitxor_if(false, &2));
    }

    #[test]
    fn not_if_i32() {
        assert_eq!(-6, 5.not_if(true));
        assert_eq!(5, 5.not_if(false));
    }

    #[test]
    fn shl_if_i32() {
        assert_eq!(20, 5.shl_if(true, &2));
        assert_eq!(5, 5.shl_if(false, &2));
    }

    #[test]
    fn shr_if_i32() {
        assert_eq!(1, 5.shr_if(true, &2));
        assert_eq!(5, 5.shr_if(false, &2));
    }
}
