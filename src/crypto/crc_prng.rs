#[doc = "Reader of register CRC_PRNG"]
pub type R = crate::R<u32, super::CRC_PRNG>;
#[doc = "Reader of field `PRNG`"]
pub type PRNG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pseudo Random Value. Output of the Galois Field Shift Register. This holds the resulting pseudo-random number if entropy is disabled or true random number if entropy is enabled."]
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
