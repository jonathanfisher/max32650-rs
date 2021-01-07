#[doc = "Reader of register DOUT[%s]"]
pub type R = crate::R<u32, super::DOUT>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Crypto Data Output. Resulting data from cipher calculation. Data is placed in the lower words of these four registers depending on algorithm."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
