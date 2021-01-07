#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u32, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tx_rx`"]
pub type TX_RX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_rx`"]
pub struct TX_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TX/RX Buffer."]
    #[inline(always)]
    pub fn tx_rx(&self) -> TX_RX_R {
        TX_RX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX/RX Buffer."]
    #[inline(always)]
    pub fn tx_rx(&mut self) -> TX_RX_W {
        TX_RX_W { w: self }
    }
}
