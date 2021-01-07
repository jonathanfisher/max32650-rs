#[doc = "Reader of register CTHSRTN"]
pub type R = crate::R<u16, super::CTHSRTN>;
#[doc = "Writer for register CTHSRTN"]
pub type W = crate::W<u16, super::CTHSRTN>;
#[doc = "Register CTHSRTN `reset()`'s with value 0"]
impl crate::ResetValue for super::CTHSRTN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `C_T_HSTRN`"]
pub type C_T_HSTRN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `C_T_HSTRN`"]
pub struct C_T_HSTRN_W<'a> {
    w: &'a mut W,
}
impl<'a> C_T_HSTRN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - High Speed Resume Delay Clock Cycles. This configures the delay from when the RESUME state on the bus ends, the when the USBHS resumes normal operation."]
    #[inline(always)]
    pub fn c_t_hstrn(&self) -> C_T_HSTRN_R {
        C_T_HSTRN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - High Speed Resume Delay Clock Cycles. This configures the delay from when the RESUME state on the bus ends, the when the USBHS resumes normal operation."]
    #[inline(always)]
    pub fn c_t_hstrn(&mut self) -> C_T_HSTRN_W {
        C_T_HSTRN_W { w: self }
    }
}
