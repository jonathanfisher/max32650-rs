#[doc = "Reader of register ctrl4"]
pub type R = crate::R<u32, super::CTRL4>;
#[doc = "Writer for register ctrl4"]
pub type W = crate::W<u32, super::CTRL4>;
#[doc = "Register ctrl4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slave Select Action delay 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSACT1_A {
    #[doc = "0: 256 system clocks between SS active and first serial clock edge."]
    _256 = 0,
}
impl From<SSACT1_A> for u8 {
    #[inline(always)]
    fn from(variant: SSACT1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSACT1`"]
pub type SSACT1_R = crate::R<u8, SSACT1_A>;
impl SSACT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSACT1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSACT1_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SSACT1_A::_256
    }
}
#[doc = "Write proxy for field `SSACT1`"]
pub struct SSACT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACT1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(SSACT1_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Slave Select Action delay 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSACT2_A {
    #[doc = "0: 256 system clocks between last serial clock edge and SS inactive."]
    _256 = 0,
}
impl From<SSACT2_A> for u8 {
    #[inline(always)]
    fn from(variant: SSACT2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSACT2`"]
pub type SSACT2_R = crate::R<u8, SSACT2_A>;
impl SSACT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSACT2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSACT2_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SSACT2_A::_256
    }
}
#[doc = "Write proxy for field `SSACT2`"]
pub struct SSACT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACT2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(SSACT2_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Slave Select Inactive delay.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSINACT_A {
    #[doc = "0: 256 system clocks between transactions."]
    _256 = 0,
}
impl From<SSINACT_A> for u8 {
    #[inline(always)]
    fn from(variant: SSINACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSINACT`"]
pub type SSINACT_R = crate::R<u8, SSINACT_A>;
impl SSINACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSINACT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSINACT_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SSINACT_A::_256
    }
}
#[doc = "Write proxy for field `SSINACT`"]
pub struct SSINACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSINACT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(SSINACT_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Select Action delay 1."]
    #[inline(always)]
    pub fn ssact1(&self) -> SSACT1_R {
        SSACT1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Select Action delay 2."]
    #[inline(always)]
    pub fn ssact2(&self) -> SSACT2_R {
        SSACT2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn ssinact(&self) -> SSINACT_R {
        SSINACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Select Action delay 1."]
    #[inline(always)]
    pub fn ssact1(&mut self) -> SSACT1_W {
        SSACT1_W { w: self }
    }
    #[doc = "Bits 8:15 - Slave Select Action delay 2."]
    #[inline(always)]
    pub fn ssact2(&mut self) -> SSACT2_W {
        SSACT2_W { w: self }
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn ssinact(&mut self) -> SSINACT_W {
        SSINACT_W { w: self }
    }
}
