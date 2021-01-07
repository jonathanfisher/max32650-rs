#[doc = "Reader of register IP_ADDR"]
pub type R = crate::R<u32, super::IP_ADDR>;
#[doc = "Writer for register IP_ADDR"]
pub type W = crate::W<u32, super::IP_ADDR>;
#[doc = "Register IP_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::IP_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START_IP_ADDR`"]
pub type START_IP_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `START_IP_ADDR`"]
pub struct START_IP_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> START_IP_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Starting IP address for Q30E"]
    #[inline(always)]
    pub fn start_ip_addr(&self) -> START_IP_ADDR_R {
        START_IP_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Starting IP address for Q30E"]
    #[inline(always)]
    pub fn start_ip_addr(&mut self) -> START_IP_ADDR_W {
        START_IP_ADDR_W { w: self }
    }
}
