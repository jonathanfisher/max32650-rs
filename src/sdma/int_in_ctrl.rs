#[doc = "Reader of register INT_IN_CTRL"]
pub type R = crate::R<u32, super::INT_IN_CTRL>;
#[doc = "Writer for register INT_IN_CTRL"]
pub type W = crate::W<u32, super::INT_IN_CTRL>;
#[doc = "Register INT_IN_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_IN_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set Interrupt Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSET_A {
    #[doc = "0: Set interrupt Flag to 0."]
    DIS = 0,
    #[doc = "1: Set Interrupt Flag to 1."]
    SET = 1,
}
impl From<INTSET_A> for bool {
    #[inline(always)]
    fn from(variant: INTSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTSET`"]
pub type INTSET_R = crate::R<bool, INTSET_A>;
impl INTSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSET_A {
        match self.bits {
            false => INTSET_A::DIS,
            true => INTSET_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == INTSET_A::DIS
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == INTSET_A::SET
    }
}
#[doc = "Write proxy for field `INTSET`"]
pub struct INTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set interrupt Flag to 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INTSET_A::DIS)
    }
    #[doc = "Set Interrupt Flag to 1."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(INTSET_A::SET)
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
    #[doc = "Bit 0 - Set Interrupt Flag."]
    #[inline(always)]
    pub fn intset(&self) -> INTSET_R {
        INTSET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set Interrupt Flag."]
    #[inline(always)]
    pub fn intset(&mut self) -> INTSET_W {
        INTSET_W { w: self }
    }
}
