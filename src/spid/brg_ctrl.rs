#[doc = "Reader of register BRG_CTRL"]
pub type R = crate::R<u32, super::BRG_CTRL>;
#[doc = "Writer for register BRG_CTRL"]
pub type W = crate::W<u32, super::BRG_CTRL>;
#[doc = "Register BRG_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BRG_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Low duty cycle control. In timer mode, reload\\[7:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOW_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DIS = 0,
}
impl From<LOW_A> for u8 {
    #[inline(always)]
    fn from(variant: LOW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOW`"]
pub type LOW_R = crate::R<u8, LOW_A>;
impl LOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOW_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LOW_A::DIS
    }
}
#[doc = "Write proxy for field `LOW`"]
pub struct LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(LOW_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "High duty cycle control. In timer mode, reload\\[15:8\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HI_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DIS = 0,
}
impl From<HI_A> for u8 {
    #[inline(always)]
    fn from(variant: HI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HI`"]
pub type HI_R = crate::R<u8, HI_A>;
impl HI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HI_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HI_A::DIS
    }
}
#[doc = "Write proxy for field `HI`"]
pub struct HI_W<'a> {
    w: &'a mut W,
}
impl<'a> HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HI_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCALE`"]
pub type SCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE`"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn low(&mut self) -> LOW_W {
        LOW_W { w: self }
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    pub fn hi(&mut self) -> HI_W {
        HI_W { w: self }
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
}
