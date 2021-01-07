#[doc = "Reader of register FIFO6"]
pub type R = crate::R<u32, super::FIFO6>;
#[doc = "Writer for register FIFO6"]
pub type W = crate::W<u32, super::FIFO6>;
#[doc = "Register FIFO6 `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBHS_FIFO6`"]
pub type USBHS_FIFO6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `USBHS_FIFO6`"]
pub struct USBHS_FIFO6_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHS_FIFO6_W<'a> {
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
    pub fn usbhs_fifo6(&self) -> USBHS_FIFO6_R {
        USBHS_FIFO6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo6(&mut self) -> USBHS_FIFO6_W {
        USBHS_FIFO6_W { w: self }
    }
}
