#[doc = "Reader of register COUNT0"]
pub type R = crate::R<u16, super::COUNT0>;
#[doc = "Reader of field `COUNT0`"]
pub type COUNT0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Read Number of Data Bytes in the Endpoint 0 FIFO. Returns the number of data bytes in the endpoint 0 FIFO. This value changes as contents of the FIFO change. The value is only valued when USBHS_OUTSCRL_outpktrdy = 1"]
    #[inline(always)]
    pub fn count0(&self) -> COUNT0_R {
        COUNT0_R::new((self.bits & 0x7f) as u8)
    }
}
