#[doc = "Reader of register INT_IN_FLAG"]
pub type R = crate::R<u32, super::INT_IN_FLAG>;
#[doc = "Writer for register INT_IN_FLAG"]
pub type W = crate::W<u32, super::INT_IN_FLAG>;
#[doc = "Register INT_IN_FLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_IN_FLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTFLAG_A {
    #[doc = "0: No Effect."]
    NO_EFF = 0,
    #[doc = "1: INT_IN_FLAG =0"]
    CLEAR = 1,
}
impl From<INTFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: INTFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTFLAG`"]
pub type INTFLAG_R = crate::R<bool, INTFLAG_A>;
impl INTFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTFLAG_A {
        match self.bits {
            false => INTFLAG_A::NO_EFF,
            true => INTFLAG_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFF`"]
    #[inline(always)]
    pub fn is_no_eff(&self) -> bool {
        *self == INTFLAG_A::NO_EFF
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == INTFLAG_A::CLEAR
    }
}
#[doc = "Write proxy for field `INTFLAG`"]
pub struct INTFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTFLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no_eff(self) -> &'a mut W {
        self.variant(INTFLAG_A::NO_EFF)
    }
    #[doc = "INT_IN_FLAG =0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INTFLAG_A::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Flag."]
    #[inline(always)]
    pub fn intflag(&self) -> INTFLAG_R {
        INTFLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Flag."]
    #[inline(always)]
    pub fn intflag(&mut self) -> INTFLAG_W {
        INTFLAG_W { w: self }
    }
}
