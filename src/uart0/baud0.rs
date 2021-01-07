#[doc = "Reader of register BAUD0"]
pub type R = crate::R<u32, super::BAUD0>;
#[doc = "Writer for register BAUD0"]
pub type W = crate::W<u32, super::BAUD0>;
#[doc = "Register BAUD0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUD0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IBAUD`"]
pub type IBAUD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IBAUD`"]
pub struct IBAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> IBAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "FACTOR must be chosen to have IDIV>0. factor used in calculation = 128 >> FACTOR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FACTOR_A {
    #[doc = "0: Baud Factor 128"]
    _128 = 0,
    #[doc = "1: Baud Factor 64"]
    _64 = 1,
    #[doc = "2: Baud Factor 32"]
    _32 = 2,
    #[doc = "3: Baud Factor 16"]
    _16 = 3,
}
impl From<FACTOR_A> for u8 {
    #[inline(always)]
    fn from(variant: FACTOR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FACTOR`"]
pub type FACTOR_R = crate::R<u8, FACTOR_A>;
impl FACTOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FACTOR_A {
        match self.bits {
            0 => FACTOR_A::_128,
            1 => FACTOR_A::_64,
            2 => FACTOR_A::_32,
            3 => FACTOR_A::_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == FACTOR_A::_128
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == FACTOR_A::_64
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == FACTOR_A::_32
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == FACTOR_A::_16
    }
}
#[doc = "Write proxy for field `FACTOR`"]
pub struct FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> FACTOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FACTOR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Baud Factor 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(FACTOR_A::_128)
    }
    #[doc = "Baud Factor 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(FACTOR_A::_64)
    }
    #[doc = "Baud Factor 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(FACTOR_A::_32)
    }
    #[doc = "Baud Factor 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(FACTOR_A::_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Integer portion of baud rate divisor value. IBAUD = InputClock / (factor * Baud Rate Frequency)."]
    #[inline(always)]
    pub fn ibaud(&self) -> IBAUD_R {
        IBAUD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:17 - FACTOR must be chosen to have IDIV>0. factor used in calculation = 128 >> FACTOR."]
    #[inline(always)]
    pub fn factor(&self) -> FACTOR_R {
        FACTOR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Integer portion of baud rate divisor value. IBAUD = InputClock / (factor * Baud Rate Frequency)."]
    #[inline(always)]
    pub fn ibaud(&mut self) -> IBAUD_W {
        IBAUD_W { w: self }
    }
    #[doc = "Bits 16:17 - FACTOR must be chosen to have IDIV>0. factor used in calculation = 128 >> FACTOR."]
    #[inline(always)]
    pub fn factor(&mut self) -> FACTOR_W {
        FACTOR_W { w: self }
    }
}
