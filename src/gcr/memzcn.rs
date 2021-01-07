#[doc = "Reader of register MEMZCN"]
pub type R = crate::R<u32, super::MEMZCN>;
#[doc = "Writer for register MEMZCN"]
pub type W = crate::W<u32, super::MEMZCN>;
#[doc = "Register MEMZCN `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMZCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System RAM Block 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM0Z_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SRAM0Z_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0Z_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM0Z`"]
pub type SRAM0Z_R = crate::R<bool, SRAM0Z_A>;
impl SRAM0Z_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM0Z_A {
        match self.bits {
            false => SRAM0Z_A::NOP,
            true => SRAM0Z_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SRAM0Z_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SRAM0Z_A::START
    }
}
#[doc = "Write proxy for field `SRAM0Z`"]
pub struct SRAM0Z_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0Z_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM0Z_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SRAM0Z_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SRAM0Z_A::START)
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
#[doc = "System RAM Block 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM1Z_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SRAM1Z_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1Z_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM1Z`"]
pub type SRAM1Z_R = crate::R<bool, SRAM1Z_A>;
impl SRAM1Z_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1Z_A {
        match self.bits {
            false => SRAM1Z_A::NOP,
            true => SRAM1Z_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SRAM1Z_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SRAM1Z_A::START
    }
}
#[doc = "Write proxy for field `SRAM1Z`"]
pub struct SRAM1Z_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1Z_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM1Z_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SRAM1Z_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SRAM1Z_A::START)
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
#[doc = "System RAM Block 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM2Z_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SRAM2Z_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2Z_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM2Z`"]
pub type SRAM2Z_R = crate::R<bool, SRAM2Z_A>;
impl SRAM2Z_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM2Z_A {
        match self.bits {
            false => SRAM2Z_A::NOP,
            true => SRAM2Z_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SRAM2Z_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SRAM2Z_A::START
    }
}
#[doc = "Write proxy for field `SRAM2Z`"]
pub struct SRAM2Z_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2Z_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM2Z_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SRAM2Z_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SRAM2Z_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "System RAM Block 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM3Z_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SRAM3Z_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM3Z_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM3Z`"]
pub type SRAM3Z_R = crate::R<bool, SRAM3Z_A>;
impl SRAM3Z_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM3Z_A {
        match self.bits {
            false => SRAM3Z_A::NOP,
            true => SRAM3Z_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SRAM3Z_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SRAM3Z_A::START
    }
}
#[doc = "Write proxy for field `SRAM3Z`"]
pub struct SRAM3Z_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3Z_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM3Z_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SRAM3Z_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SRAM3Z_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "System RAM Block 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM4Z_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SRAM4Z_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM4Z_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM4Z`"]
pub type SRAM4Z_R = crate::R<bool, SRAM4Z_A>;
impl SRAM4Z_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM4Z_A {
        match self.bits {
            false => SRAM4Z_A::NOP,
            true => SRAM4Z_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SRAM4Z_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SRAM4Z_A::START
    }
}
#[doc = "Write proxy for field `SRAM4Z`"]
pub struct SRAM4Z_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4Z_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM4Z_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SRAM4Z_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SRAM4Z_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "System RAM Block 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM5Z_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SRAM5Z_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM5Z_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM5Z`"]
pub type SRAM5Z_R = crate::R<bool, SRAM5Z_A>;
impl SRAM5Z_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM5Z_A {
        match self.bits {
            false => SRAM5Z_A::NOP,
            true => SRAM5Z_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SRAM5Z_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SRAM5Z_A::START
    }
}
#[doc = "Write proxy for field `SRAM5Z`"]
pub struct SRAM5Z_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM5Z_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM5Z_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SRAM5Z_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SRAM5Z_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "System RAM Block 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM6Z_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SRAM6Z_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM6Z_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM6Z`"]
pub type SRAM6Z_R = crate::R<bool, SRAM6Z_A>;
impl SRAM6Z_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM6Z_A {
        match self.bits {
            false => SRAM6Z_A::NOP,
            true => SRAM6Z_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SRAM6Z_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SRAM6Z_A::START
    }
}
#[doc = "Write proxy for field `SRAM6Z`"]
pub struct SRAM6Z_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM6Z_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM6Z_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SRAM6Z_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SRAM6Z_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Instruction Cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHEZ_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<ICACHEZ_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHEZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICACHEZ`"]
pub type ICACHEZ_R = crate::R<bool, ICACHEZ_A>;
impl ICACHEZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHEZ_A {
        match self.bits {
            false => ICACHEZ_A::NOP,
            true => ICACHEZ_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ICACHEZ_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ICACHEZ_A::START
    }
}
#[doc = "Write proxy for field `ICACHEZ`"]
pub struct ICACHEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHEZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(ICACHEZ_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ICACHEZ_A::START)
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
#[doc = "Instruction Cache XIP Data and Tag Ram zeroizatoin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHEXIPZ_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<ICACHEXIPZ_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHEXIPZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICACHEXIPZ`"]
pub type ICACHEXIPZ_R = crate::R<bool, ICACHEXIPZ_A>;
impl ICACHEXIPZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHEXIPZ_A {
        match self.bits {
            false => ICACHEXIPZ_A::NOP,
            true => ICACHEXIPZ_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ICACHEXIPZ_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ICACHEXIPZ_A::START
    }
}
#[doc = "Write proxy for field `ICACHEXIPZ`"]
pub struct ICACHEXIPZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEXIPZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHEXIPZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(ICACHEXIPZ_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ICACHEXIPZ_A::START)
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
#[doc = "System Cache Data Ram Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCACHEDATAZ_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SCACHEDATAZ_A> for bool {
    #[inline(always)]
    fn from(variant: SCACHEDATAZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCACHEDATAZ`"]
pub type SCACHEDATAZ_R = crate::R<bool, SCACHEDATAZ_A>;
impl SCACHEDATAZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCACHEDATAZ_A {
        match self.bits {
            false => SCACHEDATAZ_A::NOP,
            true => SCACHEDATAZ_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SCACHEDATAZ_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SCACHEDATAZ_A::START
    }
}
#[doc = "Write proxy for field `SCACHEDATAZ`"]
pub struct SCACHEDATAZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SCACHEDATAZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCACHEDATAZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SCACHEDATAZ_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SCACHEDATAZ_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "System Cache Tag Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCACHETAGZ_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SCACHETAGZ_A> for bool {
    #[inline(always)]
    fn from(variant: SCACHETAGZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCACHETAGZ`"]
pub type SCACHETAGZ_R = crate::R<bool, SCACHETAGZ_A>;
impl SCACHETAGZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCACHETAGZ_A {
        match self.bits {
            false => SCACHETAGZ_A::NOP,
            true => SCACHETAGZ_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SCACHETAGZ_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SCACHETAGZ_A::START
    }
}
#[doc = "Write proxy for field `SCACHETAGZ`"]
pub struct SCACHETAGZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SCACHETAGZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCACHETAGZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SCACHETAGZ_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SCACHETAGZ_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Crypto (MAA) Memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTOZ_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<CRYPTOZ_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPTOZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYPTOZ`"]
pub type CRYPTOZ_R = crate::R<bool, CRYPTOZ_A>;
impl CRYPTOZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPTOZ_A {
        match self.bits {
            false => CRYPTOZ_A::NOP,
            true => CRYPTOZ_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == CRYPTOZ_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CRYPTOZ_A::START
    }
}
#[doc = "Write proxy for field `CRYPTOZ`"]
pub struct CRYPTOZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTOZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTOZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(CRYPTOZ_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CRYPTOZ_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "USB FIFO Zeroizatoin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBFIFOZ_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<USBFIFOZ_A> for bool {
    #[inline(always)]
    fn from(variant: USBFIFOZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBFIFOZ`"]
pub type USBFIFOZ_R = crate::R<bool, USBFIFOZ_A>;
impl USBFIFOZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBFIFOZ_A {
        match self.bits {
            false => USBFIFOZ_A::NOP,
            true => USBFIFOZ_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == USBFIFOZ_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == USBFIFOZ_A::START
    }
}
#[doc = "Write proxy for field `USBFIFOZ`"]
pub struct USBFIFOZ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFIFOZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBFIFOZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(USBFIFOZ_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(USBFIFOZ_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - System RAM Block 0."]
    #[inline(always)]
    pub fn sram0z(&self) -> SRAM0Z_R {
        SRAM0Z_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - System RAM Block 1."]
    #[inline(always)]
    pub fn sram1z(&self) -> SRAM1Z_R {
        SRAM1Z_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System RAM Block 2."]
    #[inline(always)]
    pub fn sram2z(&self) -> SRAM2Z_R {
        SRAM2Z_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - System RAM Block 3."]
    #[inline(always)]
    pub fn sram3z(&self) -> SRAM3Z_R {
        SRAM3Z_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - System RAM Block 4."]
    #[inline(always)]
    pub fn sram4z(&self) -> SRAM4Z_R {
        SRAM4Z_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System RAM Block 5."]
    #[inline(always)]
    pub fn sram5z(&self) -> SRAM5Z_R {
        SRAM5Z_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - System RAM Block 6."]
    #[inline(always)]
    pub fn sram6z(&self) -> SRAM6Z_R {
        SRAM6Z_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Instruction Cache."]
    #[inline(always)]
    pub fn icachez(&self) -> ICACHEZ_R {
        ICACHEZ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Instruction Cache XIP Data and Tag Ram zeroizatoin."]
    #[inline(always)]
    pub fn icachexipz(&self) -> ICACHEXIPZ_R {
        ICACHEXIPZ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - System Cache Data Ram Zeroization."]
    #[inline(always)]
    pub fn scachedataz(&self) -> SCACHEDATAZ_R {
        SCACHEDATAZ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - System Cache Tag Zeroization."]
    #[inline(always)]
    pub fn scachetagz(&self) -> SCACHETAGZ_R {
        SCACHETAGZ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Crypto (MAA) Memory."]
    #[inline(always)]
    pub fn cryptoz(&self) -> CRYPTOZ_R {
        CRYPTOZ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USB FIFO Zeroizatoin."]
    #[inline(always)]
    pub fn usbfifoz(&self) -> USBFIFOZ_R {
        USBFIFOZ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM Block 0."]
    #[inline(always)]
    pub fn sram0z(&mut self) -> SRAM0Z_W {
        SRAM0Z_W { w: self }
    }
    #[doc = "Bit 1 - System RAM Block 1."]
    #[inline(always)]
    pub fn sram1z(&mut self) -> SRAM1Z_W {
        SRAM1Z_W { w: self }
    }
    #[doc = "Bit 2 - System RAM Block 2."]
    #[inline(always)]
    pub fn sram2z(&mut self) -> SRAM2Z_W {
        SRAM2Z_W { w: self }
    }
    #[doc = "Bit 3 - System RAM Block 3."]
    #[inline(always)]
    pub fn sram3z(&mut self) -> SRAM3Z_W {
        SRAM3Z_W { w: self }
    }
    #[doc = "Bit 4 - System RAM Block 4."]
    #[inline(always)]
    pub fn sram4z(&mut self) -> SRAM4Z_W {
        SRAM4Z_W { w: self }
    }
    #[doc = "Bit 5 - System RAM Block 5."]
    #[inline(always)]
    pub fn sram5z(&mut self) -> SRAM5Z_W {
        SRAM5Z_W { w: self }
    }
    #[doc = "Bit 6 - System RAM Block 6."]
    #[inline(always)]
    pub fn sram6z(&mut self) -> SRAM6Z_W {
        SRAM6Z_W { w: self }
    }
    #[doc = "Bit 8 - Instruction Cache."]
    #[inline(always)]
    pub fn icachez(&mut self) -> ICACHEZ_W {
        ICACHEZ_W { w: self }
    }
    #[doc = "Bit 9 - Instruction Cache XIP Data and Tag Ram zeroizatoin."]
    #[inline(always)]
    pub fn icachexipz(&mut self) -> ICACHEXIPZ_W {
        ICACHEXIPZ_W { w: self }
    }
    #[doc = "Bit 10 - System Cache Data Ram Zeroization."]
    #[inline(always)]
    pub fn scachedataz(&mut self) -> SCACHEDATAZ_W {
        SCACHEDATAZ_W { w: self }
    }
    #[doc = "Bit 11 - System Cache Tag Zeroization."]
    #[inline(always)]
    pub fn scachetagz(&mut self) -> SCACHETAGZ_W {
        SCACHETAGZ_W { w: self }
    }
    #[doc = "Bit 12 - Crypto (MAA) Memory."]
    #[inline(always)]
    pub fn cryptoz(&mut self) -> CRYPTOZ_W {
        CRYPTOZ_W { w: self }
    }
    #[doc = "Bit 13 - USB FIFO Zeroizatoin."]
    #[inline(always)]
    pub fn usbfifoz(&mut self) -> USBFIFOZ_W {
        USBFIFOZ_W { w: self }
    }
}
