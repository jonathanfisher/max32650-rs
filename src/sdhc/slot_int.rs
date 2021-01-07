#[doc = "Reader of register SLOT_INT"]
pub type R = crate::R<u16, super::SLOT_INT>;
#[doc = "Reader of field `INT_SIGNALS`"]
pub type INT_SIGNALS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Signal For Each Slot."]
    #[inline(always)]
    pub fn int_signals(&self) -> INT_SIGNALS_R {
        INT_SIGNALS_R::new((self.bits & 0x01) != 0)
    }
}
