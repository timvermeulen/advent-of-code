use num::traits::{One, Zero};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr};

pub trait Masked:
    Sized
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + Not<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
    + PartialEq
    + One
    + Zero
    + Copy
{
    fn trailing_zeros(self) -> u32;
    fn count_ones(self) -> u32;
}

impl Masked for u32 {
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }

    fn count_ones(self) -> u32 {
        self.count_ones()
    }
}

impl Masked for u64 {
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }

    fn count_ones(self) -> u32 {
        self.count_ones()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mask<T>(pub T);

impl<T> Mask<T>
where
    T: Masked,
{
    pub fn empty() -> Self {
        Self(T::zero())
    }

    pub fn insert(&mut self, i: u32) {
        self.0 |= T::one() << i;
    }

    pub fn remove(&mut self, i: u32) {
        self.0 &= !(T::one() << i);
    }

    pub fn add(&mut self, other: Self) {
        self.0 |= other.0;
    }

    pub fn contains(self, i: u32) -> bool {
        (self.0 >> i) & T::one() == T::one()
    }

    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != T::zero()
    }

    pub fn is_empty(self) -> bool {
        self == Self::empty()
    }

    pub fn len(self) -> u32 {
        self.0.count_ones()
    }

    pub fn iter(mut self) -> impl Iterator<Item = u32> {
        std::iter::from_fn(move || {
            if self.is_empty() {
                None
            } else {
                let i = self.0.trailing_zeros();
                self.remove(i);
                Some(i)
            }
        })
    }
}

impl<T> BitAnd for Mask<T>
where
    T: Masked,
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Self(self.0 & other.0)
    }
}

impl<T> BitAndAssign for Mask<T>
where
    T: Masked,
{
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
    }
}

impl<T> BitOr for Mask<T>
where
    T: Masked,
{
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Self(self.0 | other.0)
    }
}

impl<T> BitOrAssign for Mask<T>
where
    T: Masked,
{
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl<T> Not for Mask<T>
where
    T: Masked,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<Mask<u32>> for Mask<u64> {
    fn from(mask: Mask<u32>) -> Self {
        Self(u64::from(mask.0))
    }
}

impl<T> Shl<u32> for Mask<T>
where
    T: Masked,
{
    type Output = Self;

    fn shl(self, n: u32) -> Self::Output {
        Self(self.0 << n)
    }
}

impl<T> Shr<u32> for Mask<T>
where
    T: Masked,
{
    type Output = Self;

    fn shr(self, n: u32) -> Self::Output {
        Self(self.0 >> n)
    }
}
