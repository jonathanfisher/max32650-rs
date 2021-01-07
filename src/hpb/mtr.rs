#[doc = "Reader of register MTR[%s]"]
pub type R = crate::R<u32, super::MTR>;
#[doc = "Writer for register MTR[%s]"]
pub type W = crate::W<u32, super::MTR>;
#[doc = "Register MTR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::MTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCSHI`"]
pub type RCSHI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCSHI`"]
pub struct RCSHI_W<'a> {
    w: &'a mut W,
}
impl<'a> RCSHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `WCSHI`"]
pub type WCSHI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WCSHI`"]
pub struct WCSHI_W<'a> {
    w: &'a mut W,
}
impl<'a> WCSHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RCSS`"]
pub type RCSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCSS`"]
pub struct RCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `WCSS`"]
pub type WCSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WCSS`"]
pub struct WCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> WCSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RCSH`"]
pub type RCSH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCSH`"]
pub struct RCSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RCSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `WCSH`"]
pub type WCSH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WCSH`"]
pub struct WCSH_W<'a> {
    w: &'a mut W,
}
impl<'a> WCSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Latency Cycle for HyperRAM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LTCY_A {
    #[doc = "0: 5 clock latency."]
    _5CLOCK = 0,
    #[doc = "1: 6 clock latency."]
    _6CLOCK = 1,
    #[doc = "14: 3 clock latency."]
    _3CLOCK = 14,
    #[doc = "15: 4 clock latency."]
    _4CLOCK = 15,
}
impl From<LTCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LTCY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LTCY`"]
pub type LTCY_R = crate::R<u8, LTCY_A>;
impl LTCY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LTCY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LTCY_A::_5CLOCK),
            1 => Val(LTCY_A::_6CLOCK),
            14 => Val(LTCY_A::_3CLOCK),
            15 => Val(LTCY_A::_4CLOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_5CLOCK`"]
    #[inline(always)]
    pub fn is_5clock(&self) -> bool {
        *self == LTCY_A::_5CLOCK
    }
    #[doc = "Checks if the value of the field is `_6CLOCK`"]
    #[inline(always)]
    pub fn is_6clock(&self) -> bool {
        *self == LTCY_A::_6CLOCK
    }
    #[doc = "Checks if the value of the field is `_3CLOCK`"]
    #[inline(always)]
    pub fn is_3clock(&self) -> bool {
        *self == LTCY_A::_3CLOCK
    }
    #[doc = "Checks if the value of the field is `_4CLOCK`"]
    #[inline(always)]
    pub fn is_4clock(&self) -> bool {
        *self == LTCY_A::_4CLOCK
    }
}
#[doc = "Write proxy for field `LTCY`"]
pub struct LTCY_W<'a> {
    w: &'a mut W,
}
impl<'a> LTCY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTCY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "5 clock latency."]
    #[inline(always)]
    pub fn _5clock(self) -> &'a mut W {
        self.variant(LTCY_A::_5CLOCK)
    }
    #[doc = "6 clock latency."]
    #[inline(always)]
    pub fn _6clock(self) -> &'a mut W {
        self.variant(LTCY_A::_6CLOCK)
    }
    #[doc = "3 clock latency."]
    #[inline(always)]
    pub fn _3clock(self) -> &'a mut W {
        self.variant(LTCY_A::_3CLOCK)
    }
    #[doc = "4 clock latency."]
    #[inline(always)]
    pub fn _4clock(self) -> &'a mut W {
        self.variant(LTCY_A::_4CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Read Chip Select High Between Operations."]
    #[inline(always)]
    pub fn rcshi(&self) -> RCSHI_R {
        RCSHI_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Write Chip Select High Between Operations."]
    #[inline(always)]
    pub fn wcshi(&self) -> WCSHI_R {
        WCSHI_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Read Chip Select Setup To Next CK Rising Edge."]
    #[inline(always)]
    pub fn rcss(&self) -> RCSS_R {
        RCSS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write Chip Select Setup To Next CK Rising Edge."]
    #[inline(always)]
    pub fn wcss(&self) -> WCSS_R {
        WCSS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Read Chip Select Hold After CK falling Edge."]
    #[inline(always)]
    pub fn rcsh(&self) -> RCSH_R {
        RCSH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Write Chip Select Hold After CK falling Edge."]
    #[inline(always)]
    pub fn wcsh(&self) -> WCSH_R {
        WCSH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Latency Cycle for HyperRAM mode."]
    #[inline(always)]
    pub fn ltcy(&self) -> LTCY_R {
        LTCY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Read Chip Select High Between Operations."]
    #[inline(always)]
    pub fn rcshi(&mut self) -> RCSHI_W {
        RCSHI_W { w: self }
    }
    #[doc = "Bits 24:27 - Write Chip Select High Between Operations."]
    #[inline(always)]
    pub fn wcshi(&mut self) -> WCSHI_W {
        WCSHI_W { w: self }
    }
    #[doc = "Bits 20:23 - Read Chip Select Setup To Next CK Rising Edge."]
    #[inline(always)]
    pub fn rcss(&mut self) -> RCSS_W {
        RCSS_W { w: self }
    }
    #[doc = "Bits 16:19 - Write Chip Select Setup To Next CK Rising Edge."]
    #[inline(always)]
    pub fn wcss(&mut self) -> WCSS_W {
        WCSS_W { w: self }
    }
    #[doc = "Bits 12:15 - Read Chip Select Hold After CK falling Edge."]
    #[inline(always)]
    pub fn rcsh(&mut self) -> RCSH_W {
        RCSH_W { w: self }
    }
    #[doc = "Bits 8:11 - Write Chip Select Hold After CK falling Edge."]
    #[inline(always)]
    pub fn wcsh(&mut self) -> WCSH_W {
        WCSH_W { w: self }
    }
    #[doc = "Bits 0:3 - Latency Cycle for HyperRAM mode."]
    #[inline(always)]
    pub fn ltcy(&mut self) -> LTCY_W {
        LTCY_W { w: self }
    }
}
