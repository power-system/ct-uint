use crate::limb::Limb;
use crate::uint::Uint;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

impl<const LIMBS: usize> ConstantTimeEq for Uint<LIMBS> {
    #[inline(always)]
    fn ct_eq(&self, other: &Self) -> Choice {
        let mut acc = Choice::from(1u8);
        let mut i = 0;
        let a = self.limbs();
        let b = other.limbs();
        while i < LIMBS {
            acc &= a[i].ct_eq(&b[i]);
            i += 1;
        }
        acc
    }
}

impl<const LIMBS: usize> ConditionallySelectable for Uint<LIMBS> {
    #[inline(always)]
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mut out = [Limb::ZERO; LIMBS];
        let mut i = 0;
        let al = a.limbs();
        let bl = b.limbs();
        while i < LIMBS {
            out[i] = Limb::conditional_select(&al[i], &bl[i], choice);
            i += 1;
        }
        Uint::from_limbs(out)
    }
}

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub fn ct_lt(&self, other: &Self) -> Choice {
        let mut borrow = Limb::ZERO;
        let mut i = 0;
        let a = self.limbs();
        let b = other.limbs();
        while i < LIMBS {
            let (_, next) = a[i].sbb(b[i], borrow);
            borrow = next;
            i += 1;
        }
        borrow.ct_eq(&Limb::ONE)
    }

    #[inline(always)]
    pub fn ct_is_zero(&self) -> Choice {
        self.ct_eq(&Self::ZERO)
    }

    #[inline(always)]
    pub fn conditional_negate(&self, choice: Choice) -> Self {
        let neg = Self::ZERO.wrapping_sub(self);
        Self::conditional_select(self, &neg, choice)
    }
}
