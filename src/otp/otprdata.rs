#[doc = "Reader of register OTPRDATA"]
pub type R = crate::R<u32, super::OTPRDATA>;
#[doc = "Reader of field `RDATA`"]
pub type RDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP Data Output from read operation."]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
