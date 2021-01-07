#[doc = "Reader of register CTUCH"]
pub type R = crate::R<u16, super::CTUCH>;
#[doc = "Writer for register CTUCH"]
pub type W = crate::W<u16, super::CTUCH>;
#[doc = "Register CTUCH `reset()`'s with value 0"]
impl crate::ResetValue for super::CTUCH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `C_T_UCH`"]
pub type C_T_UCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `C_T_UCH`"]
pub struct C_T_UCH_W<'a> {
    w: &'a mut W,
}
impl<'a> C_T_UCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - HS Chirp Timeout Clock Cycles. This configures the chirp timeout used by this device to negotiate a HS connection with a FS Host."]
    #[inline(always)]
    pub fn c_t_uch(&self) -> C_T_UCH_R {
        C_T_UCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HS Chirp Timeout Clock Cycles. This configures the chirp timeout used by this device to negotiate a HS connection with a FS Host."]
    #[inline(always)]
    pub fn c_t_uch(&mut self) -> C_T_UCH_W {
        C_T_UCH_W { w: self }
    }
}
