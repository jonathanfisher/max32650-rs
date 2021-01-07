#[doc = "Reader of register DLRTC"]
pub type R = crate::R<u32, super::DLRTC>;
#[doc = "Reader of field `DLRTC`"]
pub type DLRTC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DRS Log RTC Value. This register contains the 32 bit value in the RTC second register when the last DRS event occured."]
    #[inline(always)]
    pub fn dlrtc(&self) -> DLRTC_R {
        DLRTC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
