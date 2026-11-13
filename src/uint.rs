use crate::limb::Limb;
use subtle::{Choice, ConstantTimeEq};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Uint<const LIMBS: usize>([Limb; LIMBS]);

impl<const LIMBS: usize> Uint<LIMBS> {
    pub const ZERO: Self = Self([Limb::ZERO; LIMBS]);

    pub const ONE: Self = {
        assert!(LIMBS >= 1);
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0] = Limb::ONE;
        Self(limbs)
    };

    #[inline(always)]
    pub const fn from_u64(v: u64) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        if LIMBS >= 1 {
            limbs[0] = Limb::from_u64(v);
        }
        Self(limbs)
    }

    pub fn to_u64(&self) -> Option<u64> {
        let mut i = 1;
        while i < LIMBS {
            if self.0[i].to_u64() != 0 {
                return None;
            }
            i += 1;
        }
        Some(self.0[0].to_u64())
    }

    #[inline(always)]
    pub fn checked_add(&self, rhs: &Self) -> (Self, Choice) {
        let mut out = [Limb::ZERO; LIMBS];
        let mut carry = Limb::ZERO;
        let mut i = 0;
        while i < LIMBS {
            let (v, c) = self.0[i].adc(rhs.0[i], carry);
            out[i] = v;
            carry = c;
            i += 1;
        }
        (Self(out), carry.ct_eq(&Limb::ONE))
    }

    #[inline(always)]
    pub fn wrapping_add(&self, rhs: &Self) -> Self {
        self.checked_add(rhs).0
    }

    #[inline(always)]
    pub fn checked_sub(&self, rhs: &Self) -> (Self, Choice) {
        let mut out = [Limb::ZERO; LIMBS];
        let mut borrow = Limb::ZERO;
        let mut i = 0;
        while i < LIMBS {
            let (v, b) = self.0[i].sbb(rhs.0[i], borrow);
            out[i] = v;
            borrow = b;
            i += 1;
        }
        (Self(out), borrow.ct_eq(&Limb::ONE))
    }

    #[inline(always)]
    pub fn wrapping_sub(&self, rhs: &Self) -> Self {
        self.checked_sub(rhs).0
    }

    #[inline]
    pub fn widening_mul(&self, rhs: &Self) -> (Self, Self) {
        let mut lo = [Limb::ZERO; LIMBS];
        let mut hi = [Limb::ZERO; LIMBS];

        let mut i = 0;
        while i < LIMBS {
            let mut carry: u64 = 0;
            let mut j = 0;
            while j < LIMBS {
                let k = i + j;
                let current = if k < LIMBS {
                    lo[k].to_u64()
                } else {
                    hi[k - LIMBS].to_u64()
                };
                let acc = (self.0[i].to_u64() as u128) * (rhs.0[j].to_u64() as u128)
                    + (current as u128)
                    + (carry as u128);
                if k < LIMBS {
                    lo[k] = Limb::from_u64(acc as u64);
                } else {
                    hi[k - LIMBS] = Limb::from_u64(acc as u64);
                }
                carry = (acc >> 64) as u64;
                j += 1;
            }
            hi[i] = Limb::from_u64(carry);
            i += 1;
        }
        (Self(lo), Self(hi))
    }

    #[inline(always)]
    pub fn limbs(&self) -> &[Limb; LIMBS] {
        &self.0
    }

    #[inline(always)]
    pub const fn from_limbs(limbs: [Limb; LIMBS]) -> Self {
        Self(limbs)
    }
}

impl<const LIMBS: usize> Default for Uint<LIMBS> {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(feature = "zeroize")]
impl<const LIMBS: usize> zeroize::Zeroize for Uint<LIMBS> {
    #[inline(always)]
    fn zeroize(&mut self) {
        for limb in self.0.iter_mut() {
            limb.zeroize();
        }
    }
}
