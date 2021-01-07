#[doc = "Reader of register MEMCKCN"]
pub type R = crate::R<u32, super::MEMCKCN>;
#[doc = "Writer for register MEMCKCN"]
pub type W = crate::W<u32, super::MEMCKCN>;
#[doc = "Register MEMCKCN `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMCKCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FWS`"]
pub type FWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FWS`"]
pub struct FWS_W<'a> {
    w: &'a mut W,
}
impl<'a> FWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "System RAM 0 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM0LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM0LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM0LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRAM0LS`"]
pub type SYSRAM0LS_R = crate::R<bool, SYSRAM0LS_A>;
impl SYSRAM0LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM0LS_A {
        match self.bits {
            false => SYSRAM0LS_A::ACTIVE,
            true => SYSRAM0LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SYSRAM0LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == SYSRAM0LS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `SYSRAM0LS`"]
pub struct SYSRAM0LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM0LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM0LS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM0LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM0LS_A::LIGHT_SLEEP)
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
#[doc = "System RAM 1 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM1LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM1LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM1LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRAM1LS`"]
pub type SYSRAM1LS_R = crate::R<bool, SYSRAM1LS_A>;
impl SYSRAM1LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM1LS_A {
        match self.bits {
            false => SYSRAM1LS_A::ACTIVE,
            true => SYSRAM1LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SYSRAM1LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == SYSRAM1LS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `SYSRAM1LS`"]
pub struct SYSRAM1LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM1LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM1LS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM1LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM1LS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "System RAM 2 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM2LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM2LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM2LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRAM2LS`"]
pub type SYSRAM2LS_R = crate::R<bool, SYSRAM2LS_A>;
impl SYSRAM2LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM2LS_A {
        match self.bits {
            false => SYSRAM2LS_A::ACTIVE,
            true => SYSRAM2LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SYSRAM2LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == SYSRAM2LS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `SYSRAM2LS`"]
pub struct SYSRAM2LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM2LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM2LS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM2LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM2LS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "System RAM 3 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM3LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM3LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM3LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRAM3LS`"]
pub type SYSRAM3LS_R = crate::R<bool, SYSRAM3LS_A>;
impl SYSRAM3LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM3LS_A {
        match self.bits {
            false => SYSRAM3LS_A::ACTIVE,
            true => SYSRAM3LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SYSRAM3LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == SYSRAM3LS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `SYSRAM3LS`"]
pub struct SYSRAM3LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM3LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM3LS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM3LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM3LS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "System RAM 4 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM4LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM4LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM4LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRAM4LS`"]
pub type SYSRAM4LS_R = crate::R<bool, SYSRAM4LS_A>;
impl SYSRAM4LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM4LS_A {
        match self.bits {
            false => SYSRAM4LS_A::ACTIVE,
            true => SYSRAM4LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SYSRAM4LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == SYSRAM4LS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `SYSRAM4LS`"]
pub struct SYSRAM4LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM4LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM4LS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM4LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM4LS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "System RAM 5 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM5LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM5LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM5LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRAM5LS`"]
pub type SYSRAM5LS_R = crate::R<bool, SYSRAM5LS_A>;
impl SYSRAM5LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM5LS_A {
        match self.bits {
            false => SYSRAM5LS_A::ACTIVE,
            true => SYSRAM5LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SYSRAM5LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == SYSRAM5LS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `SYSRAM5LS`"]
pub struct SYSRAM5LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM5LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM5LS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM5LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM5LS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "System RAM 6 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM6LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM6LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM6LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRAM6LS`"]
pub type SYSRAM6LS_R = crate::R<bool, SYSRAM6LS_A>;
impl SYSRAM6LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM6LS_A {
        match self.bits {
            false => SYSRAM6LS_A::ACTIVE,
            true => SYSRAM6LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SYSRAM6LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == SYSRAM6LS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `SYSRAM6LS`"]
pub struct SYSRAM6LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM6LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM6LS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM6LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM6LS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "ICache RAM Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHELS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<ICACHELS_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHELS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICACHELS`"]
pub type ICACHELS_R = crate::R<bool, ICACHELS_A>;
impl ICACHELS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHELS_A {
        match self.bits {
            false => ICACHELS_A::ACTIVE,
            true => ICACHELS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ICACHELS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == ICACHELS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `ICACHELS`"]
pub struct ICACHELS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHELS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHELS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(ICACHELS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(ICACHELS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "ICACHE-XIP RAM Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHEXIPLS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<ICACHEXIPLS_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHEXIPLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICACHEXIPLS`"]
pub type ICACHEXIPLS_R = crate::R<bool, ICACHEXIPLS_A>;
impl ICACHEXIPLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHEXIPLS_A {
        match self.bits {
            false => ICACHEXIPLS_A::ACTIVE,
            true => ICACHEXIPLS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ICACHEXIPLS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == ICACHEXIPLS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `ICACHEXIPLS`"]
pub struct ICACHEXIPLS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEXIPLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHEXIPLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(ICACHEXIPLS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(ICACHEXIPLS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "SysCache RAM Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCACHELS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SCACHELS_A> for bool {
    #[inline(always)]
    fn from(variant: SCACHELS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCACHELS`"]
pub type SCACHELS_R = crate::R<bool, SCACHELS_A>;
impl SCACHELS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCACHELS_A {
        match self.bits {
            false => SCACHELS_A::ACTIVE,
            true => SCACHELS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SCACHELS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == SCACHELS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `SCACHELS`"]
pub struct SCACHELS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCACHELS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCACHELS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SCACHELS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SCACHELS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "CRYPTO RAM Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTOLS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<CRYPTOLS_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPTOLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYPTOLS`"]
pub type CRYPTOLS_R = crate::R<bool, CRYPTOLS_A>;
impl CRYPTOLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPTOLS_A {
        match self.bits {
            false => CRYPTOLS_A::ACTIVE,
            true => CRYPTOLS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CRYPTOLS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == CRYPTOLS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `CRYPTOLS`"]
pub struct CRYPTOLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTOLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTOLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(CRYPTOLS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(CRYPTOLS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "USB FIFO Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBLS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<USBLS_A> for bool {
    #[inline(always)]
    fn from(variant: USBLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBLS`"]
pub type USBLS_R = crate::R<bool, USBLS_A>;
impl USBLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBLS_A {
        match self.bits {
            false => USBLS_A::ACTIVE,
            true => USBLS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == USBLS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == USBLS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `USBLS`"]
pub struct USBLS_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(USBLS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(USBLS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "ROM Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROMLS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<ROMLS_A> for bool {
    #[inline(always)]
    fn from(variant: ROMLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROMLS`"]
pub type ROMLS_R = crate::R<bool, ROMLS_A>;
impl ROMLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROMLS_A {
        match self.bits {
            false => ROMLS_A::ACTIVE,
            true => ROMLS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ROMLS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        *self == ROMLS_A::LIGHT_SLEEP
    }
}
#[doc = "Write proxy for field `ROMLS`"]
pub struct ROMLS_W<'a> {
    w: &'a mut W,
}
impl<'a> ROMLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROMLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(ROMLS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(ROMLS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 16 - System RAM 0 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram0ls(&self) -> SYSRAM0LS_R {
        SYSRAM0LS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - System RAM 1 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram1ls(&self) -> SYSRAM1LS_R {
        SYSRAM1LS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - System RAM 2 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram2ls(&self) -> SYSRAM2LS_R {
        SYSRAM2LS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - System RAM 3 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram3ls(&self) -> SYSRAM3LS_R {
        SYSRAM3LS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - System RAM 4 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram4ls(&self) -> SYSRAM4LS_R {
        SYSRAM4LS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - System RAM 5 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram5ls(&self) -> SYSRAM5LS_R {
        SYSRAM5LS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - System RAM 6 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram6ls(&self) -> SYSRAM6LS_R {
        SYSRAM6LS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ICache RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn icachels(&self) -> ICACHELS_R {
        ICACHELS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ICACHE-XIP RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn icachexipls(&self) -> ICACHEXIPLS_R {
        ICACHEXIPLS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SysCache RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn scachels(&self) -> SCACHELS_R {
        SCACHELS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CRYPTO RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn cryptols(&self) -> CRYPTOLS_R {
        CRYPTOLS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - USB FIFO Light Sleep Mode."]
    #[inline(always)]
    pub fn usbls(&self) -> USBLS_R {
        USBLS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ROM Light Sleep Mode."]
    #[inline(always)]
    pub fn romls(&self) -> ROMLS_R {
        ROMLS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    pub fn fws(&mut self) -> FWS_W {
        FWS_W { w: self }
    }
    #[doc = "Bit 16 - System RAM 0 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram0ls(&mut self) -> SYSRAM0LS_W {
        SYSRAM0LS_W { w: self }
    }
    #[doc = "Bit 17 - System RAM 1 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram1ls(&mut self) -> SYSRAM1LS_W {
        SYSRAM1LS_W { w: self }
    }
    #[doc = "Bit 18 - System RAM 2 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram2ls(&mut self) -> SYSRAM2LS_W {
        SYSRAM2LS_W { w: self }
    }
    #[doc = "Bit 19 - System RAM 3 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram3ls(&mut self) -> SYSRAM3LS_W {
        SYSRAM3LS_W { w: self }
    }
    #[doc = "Bit 20 - System RAM 4 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram4ls(&mut self) -> SYSRAM4LS_W {
        SYSRAM4LS_W { w: self }
    }
    #[doc = "Bit 21 - System RAM 5 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram5ls(&mut self) -> SYSRAM5LS_W {
        SYSRAM5LS_W { w: self }
    }
    #[doc = "Bit 22 - System RAM 6 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram6ls(&mut self) -> SYSRAM6LS_W {
        SYSRAM6LS_W { w: self }
    }
    #[doc = "Bit 24 - ICache RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn icachels(&mut self) -> ICACHELS_W {
        ICACHELS_W { w: self }
    }
    #[doc = "Bit 25 - ICACHE-XIP RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn icachexipls(&mut self) -> ICACHEXIPLS_W {
        ICACHEXIPLS_W { w: self }
    }
    #[doc = "Bit 26 - SysCache RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn scachels(&mut self) -> SCACHELS_W {
        SCACHELS_W { w: self }
    }
    #[doc = "Bit 27 - CRYPTO RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn cryptols(&mut self) -> CRYPTOLS_W {
        CRYPTOLS_W { w: self }
    }
    #[doc = "Bit 28 - USB FIFO Light Sleep Mode."]
    #[inline(always)]
    pub fn usbls(&mut self) -> USBLS_W {
        USBLS_W { w: self }
    }
    #[doc = "Bit 29 - ROM Light Sleep Mode."]
    #[inline(always)]
    pub fn romls(&mut self) -> ROMLS_W {
        ROMLS_W { w: self }
    }
}
