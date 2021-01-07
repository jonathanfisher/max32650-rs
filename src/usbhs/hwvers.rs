#[doc = "Reader of register HWVERS"]
pub type R = crate::R<u16, super::HWVERS>;
#[doc = "Writer for register HWVERS"]
pub type W = crate::W<u16, super::HWVERS>;
#[doc = "Register HWVERS `reset()`'s with value 0"]
impl crate::ResetValue for super::HWVERS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBHS_HWVERS`"]
pub type USBHS_HWVERS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `USBHS_HWVERS`"]
pub struct USBHS_HWVERS_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHS_HWVERS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - USBHS Register."]
    #[inline(always)]
    pub fn usbhs_hwvers(&self) -> USBHS_HWVERS_R {
        USBHS_HWVERS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - USBHS Register."]
    #[inline(always)]
    pub fn usbhs_hwvers(&mut self) -> USBHS_HWVERS_W {
        USBHS_HWVERS_W { w: self }
    }
}
