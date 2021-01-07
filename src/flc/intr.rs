#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: No interrupt is pending"]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending"]
    PENDING = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, DONE_A>;
impl DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::INACTIVE,
            true => DONE_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DONE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DONE_A::PENDING
    }
}
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt is pending"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DONE_A::INACTIVE)
    }
    #[doc = "An interrupt is pending"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DONE_A::PENDING)
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
#[doc = "Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AF_A {
    #[doc = "0: No Failure."]
    NOERROR = 0,
    #[doc = "1: Failure occurs."]
    ERROR = 1,
}
impl From<AF_A> for bool {
    #[inline(always)]
    fn from(variant: AF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AF`"]
pub type AF_R = crate::R<bool, AF_A>;
impl AF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AF_A {
        match self.bits {
            false => AF_A::NOERROR,
            true => AF_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AF_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == AF_A::ERROR
    }
}
#[doc = "Write proxy for field `AF`"]
pub struct AF_W<'a> {
    w: &'a mut W,
}
impl<'a> AF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Failure."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(AF_A::NOERROR)
    }
    #[doc = "Failure occurs."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(AF_A::ERROR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Flash Done Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONEIE_A {
    #[doc = "0: Disable."]
    DISABLE = 0,
    #[doc = "1: Enable."]
    ENABLE = 1,
}
impl From<DONEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DONEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DONEIE`"]
pub type DONEIE_R = crate::R<bool, DONEIE_A>;
impl DONEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONEIE_A {
        match self.bits {
            false => DONEIE_A::DISABLE,
            true => DONEIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DONEIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DONEIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `DONEIE`"]
pub struct DONEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DONEIE_A::DISABLE)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DONEIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `AFIE`"]
pub type AFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AFIE`"]
pub struct AFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn doneie(&self) -> DONEIE_R {
        DONEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn afie(&self) -> AFIE_R {
        AFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 1 - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
    #[inline(always)]
    pub fn af(&mut self) -> AF_W {
        AF_W { w: self }
    }
    #[doc = "Bit 8 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn doneie(&mut self) -> DONEIE_W {
        DONEIE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn afie(&mut self) -> AFIE_W {
        AFIE_W { w: self }
    }
}
