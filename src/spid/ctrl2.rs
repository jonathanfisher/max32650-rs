#[doc = "Reader of register ctrl2"]
pub type R = crate::R<u32, super::CTRL2>;
#[doc = "Writer for register ctrl2"]
pub type W = crate::W<u32, super::CTRL2>;
#[doc = "Register ctrl2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_NUM_CHAR`"]
pub type TX_NUM_CHAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_NUM_CHAR`"]
pub struct TX_NUM_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_NUM_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `RX_NUM_CHAR`"]
pub type RX_NUM_CHAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_NUM_CHAR`"]
pub struct RX_NUM_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_NUM_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Nubmer of Characters to transmit."]
    #[inline(always)]
    pub fn tx_num_char(&self) -> TX_NUM_CHAR_R {
        TX_NUM_CHAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Nubmer of Characters to receive."]
    #[inline(always)]
    pub fn rx_num_char(&self) -> RX_NUM_CHAR_R {
        RX_NUM_CHAR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Nubmer of Characters to transmit."]
    #[inline(always)]
    pub fn tx_num_char(&mut self) -> TX_NUM_CHAR_W {
        TX_NUM_CHAR_W { w: self }
    }
    #[doc = "Bits 16:31 - Nubmer of Characters to receive."]
    #[inline(always)]
    pub fn rx_num_char(&mut self) -> RX_NUM_CHAR_W {
        RX_NUM_CHAR_W { w: self }
    }
}
