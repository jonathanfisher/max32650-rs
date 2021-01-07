#[doc = "Reader of register FRAME"]
pub type R = crate::R<u16, super::FRAME>;
#[doc = "Reader of field `FRAMENUM`"]
pub type FRAMENUM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Read the last received frame number, that is the 11-bit frame number received in the SOF packet."]
    #[inline(always)]
    pub fn framenum(&self) -> FRAMENUM_R {
        FRAMENUM_R::new((self.bits & 0x07ff) as u16)
    }
}
