#[doc = "Reader of register HAM_ECC"]
pub type R = crate::R<u32, super::HAM_ECC>;
#[doc = "Writer for register HAM_ECC"]
pub type W = crate::W<u32, super::HAM_ECC>;
#[doc = "Register HAM_ECC `reset()`'s with value 0"]
impl crate::ResetValue for super::HAM_ECC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ECC`"]
pub type ECC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ECC`"]
pub struct ECC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Parity. This is the parity of the entire array.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAR_A {
    #[doc = "0: Even."]
    EVEN = 0,
    #[doc = "1: Odd."]
    ODD = 1,
}
impl From<PAR_A> for bool {
    #[inline(always)]
    fn from(variant: PAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAR`"]
pub type PAR_R = crate::R<bool, PAR_A>;
impl PAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAR_A {
        match self.bits {
            false => PAR_A::EVEN,
            true => PAR_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PAR_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PAR_A::ODD
    }
}
#[doc = "Write proxy for field `PAR`"]
pub struct PAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Even."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PAR_A::EVEN)
    }
    #[doc = "Odd."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PAR_A::ODD)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Hamming ECC Value. These bits are the even parity of their corresponding bit groups."]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Parity. This is the parity of the entire array."]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Hamming ECC Value. These bits are the even parity of their corresponding bit groups."]
    #[inline(always)]
    pub fn ecc(&mut self) -> ECC_W {
        ECC_W { w: self }
    }
    #[doc = "Bit 16 - Parity. This is the parity of the entire array."]
    #[inline(always)]
    pub fn par(&mut self) -> PAR_W {
        PAR_W { w: self }
    }
}
