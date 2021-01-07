#[doc = "Writer for register RST"]
pub type W = crate::W<u32, super::RST>;
#[doc = "Register RST `reset()`'s with value 0"]
impl crate::ResetValue for super::RST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD then a watchdog reset will occur, if enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDT_RST_AW {
    #[doc = "165: The first value to be written to reset the WDT."]
    SEQ0 = 165,
    #[doc = "90: The second value to be written to reset the WDT."]
    SEQ1 = 90,
}
impl From<WDT_RST_AW> for u8 {
    #[inline(always)]
    fn from(variant: WDT_RST_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `WDT_RST`"]
pub struct WDT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_RST_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The first value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq0(self) -> &'a mut W {
        self.variant(WDT_RST_AW::SEQ0)
    }
    #[doc = "The second value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq1(self) -> &'a mut W {
        self.variant(WDT_RST_AW::SEQ1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD then a watchdog reset will occur, if enabled."]
    #[inline(always)]
    pub fn wdt_rst(&mut self) -> WDT_RST_W {
        WDT_RST_W { w: self }
    }
}
