#[doc = "Reader of register NOLCMP"]
pub type R = crate::R<u32, super::NOLCMP>;
#[doc = "Writer for register NOLCMP"]
pub type W = crate::W<u32, super::NOLCMP>;
#[doc = "Register NOLCMP `reset()`'s with value 0"]
impl crate::ResetValue for super::NOLCMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NOLLCMP`"]
pub type NOLLCMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NOLLCMP`"]
pub struct NOLLCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOLLCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NOLHCMP`"]
pub type NOLHCMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NOLHCMP`"]
pub struct NOLHCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOLHCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
    #[inline(always)]
    pub fn nollcmp(&self) -> NOLLCMP_R {
        NOLLCMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
    #[inline(always)]
    pub fn nolhcmp(&self) -> NOLHCMP_R {
        NOLHCMP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
    #[inline(always)]
    pub fn nollcmp(&mut self) -> NOLLCMP_W {
        NOLLCMP_W { w: self }
    }
    #[doc = "Bits 8:15 - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
    #[inline(always)]
    pub fn nolhcmp(&mut self) -> NOLHCMP_W {
        NOLHCMP_W { w: self }
    }
}
