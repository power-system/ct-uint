use crate::limb::Limb;
use crate::uint::Uint;

impl<const LIMBS: usize> Uint<LIMBS> {
    pub fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != LIMBS * 8 {
            return None;
        }
        let mut out = [Limb::ZERO; LIMBS];
        let mut i = 0;
        while i < LIMBS {
            let mut v: u64 = 0;
            let mut j = 0;
            while j < 8 {
                v |= (bytes[i * 8 + j] as u64) << (j * 8);
                j += 1;
            }
            out[i] = Limb::from_u64(v);
            i += 1;
        }
        Some(Self::from_limbs(out))
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != LIMBS * 8 {
            return None;
        }
        let mut out = [Limb::ZERO; LIMBS];
        let mut i = 0;
        while i < LIMBS {
            let base = (LIMBS - 1 - i) * 8;
            let mut v: u64 = 0;
            let mut j = 0;
            while j < 8 {
                v = (v << 8) | (bytes[base + j] as u64);
                j += 1;
            }
            out[i] = Limb::from_u64(v);
            i += 1;
        }
        Some(Self::from_limbs(out))
    }

    pub fn write_le_bytes(&self, out: &mut [u8]) {
        assert_eq!(out.len(), LIMBS * 8);
        let limbs = self.limbs();
        let mut i = 0;
        while i < LIMBS {
            let v = limbs[i].to_u64();
            let mut j = 0;
            while j < 8 {
                out[i * 8 + j] = (v >> (j * 8)) as u8;
                j += 1;
            }
            i += 1;
        }
    }

    pub fn write_be_bytes(&self, out: &mut [u8]) {
        assert_eq!(out.len(), LIMBS * 8);
        let limbs = self.limbs();
        let mut i = 0;
        while i < LIMBS {
            let v = limbs[i].to_u64();
            let base = (LIMBS - 1 - i) * 8;
            let mut j = 0;
            while j < 8 {
                out[base + j] = (v >> (8 * (7 - j))) as u8;
                j += 1;
            }
            i += 1;
        }
    }
}
