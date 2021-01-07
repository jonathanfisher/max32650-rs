#[doc = "Reader of register OCNTL"]
pub type R = crate::R<u32, super::OCNTL>;
#[doc = "Writer for register OCNTL"]
pub type W = crate::W<u32, super::OCNTL>;
#[doc = "Register OCNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::OCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PA`"]
pub type PA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PA`"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Setting this bit to a 1 initiates a read of the OTP location specified in the PA field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_OP_A {
    #[doc = "0: No Operation."]
    NO_OPERATION = 0,
    #[doc = "1: Initiate Read Operation."]
    INITIATE_READ_OP = 1,
}
impl From<RD_OP_A> for bool {
    #[inline(always)]
    fn from(variant: RD_OP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RD_OP`"]
pub type RD_OP_R = crate::R<bool, RD_OP_A>;
impl RD_OP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_OP_A {
        match self.bits {
            false => RD_OP_A::NO_OPERATION,
            true => RD_OP_A::INITIATE_READ_OP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == RD_OP_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `INITIATE_READ_OP`"]
    #[inline(always)]
    pub fn is_initiate_read_op(&self) -> bool {
        *self == RD_OP_A::INITIATE_READ_OP
    }
}
#[doc = "Write proxy for field `RD_OP`"]
pub struct RD_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_OP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RD_OP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Operation."]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(RD_OP_A::NO_OPERATION)
    }
    #[doc = "Initiate Read Operation."]
    #[inline(always)]
    pub fn initiate_read_op(self) -> &'a mut W {
        self.variant(RD_OP_A::INITIATE_READ_OP)
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
#[doc = "Setting this bit to a 1 initiates a write of the OTP location specified in the PA field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROG_OP_A {
    #[doc = "0: No Operation."]
    NO_OPERATION = 0,
    #[doc = "1: Initiate Write Operation."]
    INITIATE_WRITE_OP = 1,
}
impl From<PROG_OP_A> for bool {
    #[inline(always)]
    fn from(variant: PROG_OP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROG_OP`"]
pub type PROG_OP_R = crate::R<bool, PROG_OP_A>;
impl PROG_OP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROG_OP_A {
        match self.bits {
            false => PROG_OP_A::NO_OPERATION,
            true => PROG_OP_A::INITIATE_WRITE_OP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == PROG_OP_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `INITIATE_WRITE_OP`"]
    #[inline(always)]
    pub fn is_initiate_write_op(&self) -> bool {
        *self == PROG_OP_A::INITIATE_WRITE_OP
    }
}
#[doc = "Write proxy for field `PROG_OP`"]
pub struct PROG_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_OP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROG_OP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Operation."]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(PROG_OP_A::NO_OPERATION)
    }
    #[doc = "Initiate Write Operation."]
    #[inline(always)]
    pub fn initiate_write_op(self) -> &'a mut W {
        self.variant(PROG_OP_A::INITIATE_WRITE_OP)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - This value is the address of the OTP 32-bit value."]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Setting this bit to a 1 initiates a read of the OTP location specified in the PA field."]
    #[inline(always)]
    pub fn rd_op(&self) -> RD_OP_R {
        RD_OP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Setting this bit to a 1 initiates a write of the OTP location specified in the PA field."]
    #[inline(always)]
    pub fn prog_op(&self) -> PROG_OP_R {
        PROG_OP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - This value is the address of the OTP 32-bit value."]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Bit 8 - Setting this bit to a 1 initiates a read of the OTP location specified in the PA field."]
    #[inline(always)]
    pub fn rd_op(&mut self) -> RD_OP_W {
        RD_OP_W { w: self }
    }
    #[doc = "Bit 9 - Setting this bit to a 1 initiates a write of the OTP location specified in the PA field."]
    #[inline(always)]
    pub fn prog_op(&mut self) -> PROG_OP_W {
        PROG_OP_W { w: self }
    }
}
