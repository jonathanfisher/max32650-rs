#[doc = "Reader of register TRIM"]
pub type R = crate::R<u32, super::TRIM>;
#[doc = "Writer for register TRIM"]
pub type W = crate::W<u32, super::TRIM>;
#[doc = "Register TRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIM`"]
pub type TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM`"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `VBATTMR`"]
pub type VBATTMR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VBATTMR`"]
pub struct VBATTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATTMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
    #[inline(always)]
    pub fn vbattmr(&self) -> VBATTMR_R {
        VBATTMR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Bits 8:31 - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
    #[inline(always)]
    pub fn vbattmr(&mut self) -> VBATTMR_W {
        VBATTMR_W { w: self }
    }
}
