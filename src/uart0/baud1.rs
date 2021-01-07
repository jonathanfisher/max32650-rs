#[doc = "Reader of register BAUD1"]
pub type R = crate::R<u32, super::BAUD1>;
#[doc = "Writer for register BAUD1"]
pub type W = crate::W<u32, super::BAUD1>;
#[doc = "Register BAUD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBAUD`"]
pub type DBAUD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DBAUD`"]
pub struct DBAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Decimal portion of baud rate divisor value. DIV = InputClock/(factor*Baud Rate Frequency). DDIV=(DIV-IDIV)*128."]
    #[inline(always)]
    pub fn dbaud(&self) -> DBAUD_R {
        DBAUD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Decimal portion of baud rate divisor value. DIV = InputClock/(factor*Baud Rate Frequency). DDIV=(DIV-IDIV)*128."]
    #[inline(always)]
    pub fn dbaud(&mut self) -> DBAUD_W {
        DBAUD_W { w: self }
    }
}
