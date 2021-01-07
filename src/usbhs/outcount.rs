#[doc = "Reader of register OUTCOUNT"]
pub type R = crate::R<u16, super::OUTCOUNT>;
#[doc = "Reader of field `OUTCOUNT`"]
pub type OUTCOUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - Read Number of Data Bytes in OUT FIFO. Returns the number of data bytes in the packet that are read next in the OUT FIFO."]
    #[inline(always)]
    pub fn outcount(&self) -> OUTCOUNT_R {
        OUTCOUNT_R::new((self.bits & 0x1fff) as u16)
    }
}
