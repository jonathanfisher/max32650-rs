#[doc = "Reader of register FIFO7"]
pub type R = crate::R<u32, super::FIFO7>;
#[doc = "Writer for register FIFO7"]
pub type W = crate::W<u32, super::FIFO7>;
#[doc = "Register FIFO7 `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBHS_FIFO7`"]
pub type USBHS_FIFO7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `USBHS_FIFO7`"]
pub struct USBHS_FIFO7_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHS_FIFO7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo7(&self) -> USBHS_FIFO7_R {
        USBHS_FIFO7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo7(&mut self) -> USBHS_FIFO7_W {
        USBHS_FIFO7_W { w: self }
    }
}
