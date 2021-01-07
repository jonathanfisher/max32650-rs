#[doc = "Reader of register SS_TIME"]
pub type R = crate::R<u32, super::SS_TIME>;
#[doc = "Writer for register SS_TIME"]
pub type W = crate::W<u32, super::SS_TIME>;
#[doc = "Register SS_TIME `reset()`'s with value 0"]
impl crate::ResetValue for super::SS_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slave Select Pre delay 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRE_A {
    #[doc = "0: 256 system clocks between SS active and first serial clock edge."]
    _256 = 0,
}
impl From<PRE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRE`"]
pub type PRE_R = crate::R<u8, PRE_A>;
impl PRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRE_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PRE_A::_256
    }
}
#[doc = "Write proxy for field `PRE`"]
pub struct PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(PRE_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Slave Select Post delay 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POST_A {
    #[doc = "0: 256 system clocks between last serial clock edge and SS inactive."]
    _256 = 0,
}
impl From<POST_A> for u8 {
    #[inline(always)]
    fn from(variant: POST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `POST`"]
pub type POST_R = crate::R<u8, POST_A>;
impl POST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, POST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(POST_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == POST_A::_256
    }
}
#[doc = "Write proxy for field `POST`"]
pub struct POST_W<'a> {
    w: &'a mut W,
}
impl<'a> POST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(POST_A::_256)
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
pub enum INACT_A {
    #[doc = "0: 256 system clocks between transactions."]
    _256 = 0,
}
impl From<INACT_A> for u8 {
    #[inline(always)]
    fn from(variant: INACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INACT`"]
pub type INACT_R = crate::R<u8, INACT_A>;
impl INACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INACT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INACT_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == INACT_A::_256
    }
}
#[doc = "Write proxy for field `INACT`"]
pub struct INACT_W<'a> {
    w: &'a mut W,
}
impl<'a> INACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INACT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(INACT_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    pub fn post(&self) -> POST_R {
        POST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn inact(&self) -> INACT_R {
        INACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W {
        PRE_W { w: self }
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    pub fn post(&mut self) -> POST_W {
        POST_W { w: self }
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn inact(&mut self) -> INACT_W {
        INACT_W { w: self }
    }
}
