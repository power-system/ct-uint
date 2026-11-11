use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Limb(u64);

impl Limb {
    pub const ZERO: Self = Limb(0);
    pub const ONE: Self = Limb(1);
    pub const MAX: Self = Limb(u64::MAX);

    #[inline]
    pub const fn from_u64(v: u64) -> Self {
        Limb(v)
    }

    #[inline]
    pub const fn to_u64(self) -> u64 {
        self.0
    }

    #[inline]
    pub const fn adc(self, rhs: Self, carry: Self) -> (Self, Self) {
        let (a, c1) = self.0.overflowing_add(rhs.0);
        let (b, c2) = a.overflowing_add(carry.0);
        let carry_out = (c1 as u64) + (c2 as u64);
        (Limb(b), Limb(carry_out))
    }

    #[inline]
    pub const fn sbb(self, rhs: Self, borrow: Self) -> (Self, Self) {
        let (a, b1) = self.0.overflowing_sub(rhs.0);
        let (b, b2) = a.overflowing_sub(borrow.0);
        let borrow_out = (b1 as u64) + (b2 as u64);
        (Limb(b), Limb(borrow_out))
    }

    #[inline]
    pub const fn mac(self, rhs: Self, carry: Self) -> (Self, Self) {
        let wide = (self.0 as u128) * (rhs.0 as u128) + (carry.0 as u128);
        (Limb(wide as u64), Limb((wide >> 64) as u64))
    }
}

impl ConstantTimeEq for Limb {
    #[inline]
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl ConditionallySelectable for Limb {
    #[inline]
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Limb(u64::conditional_select(&a.0, &b.0, choice))
    }
}

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for Limb {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}
