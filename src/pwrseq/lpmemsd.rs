#[doc = "Reader of register LPMEMSD"]
pub type R = crate::R<u32, super::LPMEMSD>;
#[doc = "Writer for register LPMEMSD"]
pub type W = crate::W<u32, super::LPMEMSD>;
#[doc = "Register LPMEMSD `reset()`'s with value 0"]
impl crate::ResetValue for super::LPMEMSD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System RAM block 0 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM0SD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM0SD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM0SD`"]
pub type SRAM0SD_R = crate::R<bool, SRAM0SD_A>;
impl SRAM0SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM0SD_A {
        match self.bits {
            false => SRAM0SD_A::NORMAL,
            true => SRAM0SD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM0SD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM0SD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `SRAM0SD`"]
pub struct SRAM0SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM0SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM0SD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM0SD_A::SHUTDOWN)
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
#[doc = "System RAM block 1 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM1SD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM1SD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM1SD`"]
pub type SRAM1SD_R = crate::R<bool, SRAM1SD_A>;
impl SRAM1SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1SD_A {
        match self.bits {
            false => SRAM1SD_A::NORMAL,
            true => SRAM1SD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM1SD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM1SD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `SRAM1SD`"]
pub struct SRAM1SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM1SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM1SD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM1SD_A::SHUTDOWN)
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
#[doc = "System RAM block 2 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM2SD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM2SD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM2SD`"]
pub type SRAM2SD_R = crate::R<bool, SRAM2SD_A>;
impl SRAM2SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM2SD_A {
        match self.bits {
            false => SRAM2SD_A::NORMAL,
            true => SRAM2SD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM2SD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM2SD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `SRAM2SD`"]
pub struct SRAM2SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM2SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM2SD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM2SD_A::SHUTDOWN)
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
#[doc = "System RAM block 3 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM3SD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM3SD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM3SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM3SD`"]
pub type SRAM3SD_R = crate::R<bool, SRAM3SD_A>;
impl SRAM3SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM3SD_A {
        match self.bits {
            false => SRAM3SD_A::NORMAL,
            true => SRAM3SD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM3SD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM3SD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `SRAM3SD`"]
pub struct SRAM3SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM3SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM3SD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM3SD_A::SHUTDOWN)
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
#[doc = "System RAM block 4 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM4SD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM4SD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM4SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM4SD`"]
pub type SRAM4SD_R = crate::R<bool, SRAM4SD_A>;
impl SRAM4SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM4SD_A {
        match self.bits {
            false => SRAM4SD_A::NORMAL,
            true => SRAM4SD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM4SD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM4SD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `SRAM4SD`"]
pub struct SRAM4SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM4SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM4SD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM4SD_A::SHUTDOWN)
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
#[doc = "System RAM block 5 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM5SD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM5SD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM5SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM5SD`"]
pub type SRAM5SD_R = crate::R<bool, SRAM5SD_A>;
impl SRAM5SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM5SD_A {
        match self.bits {
            false => SRAM5SD_A::NORMAL,
            true => SRAM5SD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM5SD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM5SD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `SRAM5SD`"]
pub struct SRAM5SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM5SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM5SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM5SD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM5SD_A::SHUTDOWN)
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
#[doc = "System RAM block 6 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM6SD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM6SD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM6SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM6SD`"]
pub type SRAM6SD_R = crate::R<bool, SRAM6SD_A>;
impl SRAM6SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM6SD_A {
        match self.bits {
            false => SRAM6SD_A::NORMAL,
            true => SRAM6SD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM6SD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SRAM6SD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `SRAM6SD`"]
pub struct SRAM6SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM6SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM6SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM6SD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM6SD_A::SHUTDOWN)
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
#[doc = "Instruction Cache RAM Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHESD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<ICACHESD_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHESD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICACHESD`"]
pub type ICACHESD_R = crate::R<bool, ICACHESD_A>;
impl ICACHESD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHESD_A {
        match self.bits {
            false => ICACHESD_A::NORMAL,
            true => ICACHESD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ICACHESD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == ICACHESD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `ICACHESD`"]
pub struct ICACHESD_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHESD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHESD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ICACHESD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(ICACHESD_A::SHUTDOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "XiP Instruction Cache RAM Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHEXIPSD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<ICACHEXIPSD_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHEXIPSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICACHEXIPSD`"]
pub type ICACHEXIPSD_R = crate::R<bool, ICACHEXIPSD_A>;
impl ICACHEXIPSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHEXIPSD_A {
        match self.bits {
            false => ICACHEXIPSD_A::NORMAL,
            true => ICACHEXIPSD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ICACHEXIPSD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == ICACHEXIPSD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `ICACHEXIPSD`"]
pub struct ICACHEXIPSD_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEXIPSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHEXIPSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ICACHEXIPSD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(ICACHEXIPSD_A::SHUTDOWN)
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
#[doc = "System Cache RAM Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCACHESD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SCACHESD_A> for bool {
    #[inline(always)]
    fn from(variant: SCACHESD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCACHESD`"]
pub type SCACHESD_R = crate::R<bool, SCACHESD_A>;
impl SCACHESD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCACHESD_A {
        match self.bits {
            false => SCACHESD_A::NORMAL,
            true => SCACHESD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SCACHESD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == SCACHESD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `SCACHESD`"]
pub struct SCACHESD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCACHESD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCACHESD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SCACHESD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SCACHESD_A::SHUTDOWN)
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
#[doc = "Crypto MAA RAM Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTOSD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<CRYPTOSD_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPTOSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYPTOSD`"]
pub type CRYPTOSD_R = crate::R<bool, CRYPTOSD_A>;
impl CRYPTOSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPTOSD_A {
        match self.bits {
            false => CRYPTOSD_A::NORMAL,
            true => CRYPTOSD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CRYPTOSD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == CRYPTOSD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `CRYPTOSD`"]
pub struct CRYPTOSD_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTOSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTOSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CRYPTOSD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(CRYPTOSD_A::SHUTDOWN)
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
#[doc = "USB FIFO Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBFIFOSD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<USBFIFOSD_A> for bool {
    #[inline(always)]
    fn from(variant: USBFIFOSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBFIFOSD`"]
pub type USBFIFOSD_R = crate::R<bool, USBFIFOSD_A>;
impl USBFIFOSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBFIFOSD_A {
        match self.bits {
            false => USBFIFOSD_A::NORMAL,
            true => USBFIFOSD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == USBFIFOSD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == USBFIFOSD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `USBFIFOSD`"]
pub struct USBFIFOSD_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFIFOSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBFIFOSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(USBFIFOSD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(USBFIFOSD_A::SHUTDOWN)
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
#[doc = "ROM Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROMSD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<ROMSD_A> for bool {
    #[inline(always)]
    fn from(variant: ROMSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROMSD`"]
pub type ROMSD_R = crate::R<bool, ROMSD_A>;
impl ROMSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROMSD_A {
        match self.bits {
            false => ROMSD_A::NORMAL,
            true => ROMSD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ROMSD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == ROMSD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `ROMSD`"]
pub struct ROMSD_W<'a> {
    w: &'a mut W,
}
impl<'a> ROMSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROMSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ROMSD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(ROMSD_A::SHUTDOWN)
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
#[doc = "ROM1 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM1SD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<ROM1SD_A> for bool {
    #[inline(always)]
    fn from(variant: ROM1SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROM1SD`"]
pub type ROM1SD_R = crate::R<bool, ROM1SD_A>;
impl ROM1SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM1SD_A {
        match self.bits {
            false => ROM1SD_A::NORMAL,
            true => ROM1SD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ROM1SD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == ROM1SD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `ROM1SD`"]
pub struct ROM1SD_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM1SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROM1SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ROM1SD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(ROM1SD_A::SHUTDOWN)
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
#[doc = "ICache 1 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC1SD_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<IC1SD_A> for bool {
    #[inline(always)]
    fn from(variant: IC1SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IC1SD`"]
pub type IC1SD_R = crate::R<bool, IC1SD_A>;
impl IC1SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC1SD_A {
        match self.bits {
            false => IC1SD_A::NORMAL,
            true => IC1SD_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IC1SD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == IC1SD_A::SHUTDOWN
    }
}
#[doc = "Write proxy for field `IC1SD`"]
pub struct IC1SD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC1SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IC1SD_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(IC1SD_A::SHUTDOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - System RAM block 0 Shut Down."]
    #[inline(always)]
    pub fn sram0sd(&self) -> SRAM0SD_R {
        SRAM0SD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - System RAM block 1 Shut Down."]
    #[inline(always)]
    pub fn sram1sd(&self) -> SRAM1SD_R {
        SRAM1SD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System RAM block 2 Shut Down."]
    #[inline(always)]
    pub fn sram2sd(&self) -> SRAM2SD_R {
        SRAM2SD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - System RAM block 3 Shut Down."]
    #[inline(always)]
    pub fn sram3sd(&self) -> SRAM3SD_R {
        SRAM3SD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - System RAM block 4 Shut Down."]
    #[inline(always)]
    pub fn sram4sd(&self) -> SRAM4SD_R {
        SRAM4SD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System RAM block 5 Shut Down."]
    #[inline(always)]
    pub fn sram5sd(&self) -> SRAM5SD_R {
        SRAM5SD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - System RAM block 6 Shut Down."]
    #[inline(always)]
    pub fn sram6sd(&self) -> SRAM6SD_R {
        SRAM6SD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Instruction Cache RAM Shut Down."]
    #[inline(always)]
    pub fn icachesd(&self) -> ICACHESD_R {
        ICACHESD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XiP Instruction Cache RAM Shut Down."]
    #[inline(always)]
    pub fn icachexipsd(&self) -> ICACHEXIPSD_R {
        ICACHEXIPSD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - System Cache RAM Shut Down."]
    #[inline(always)]
    pub fn scachesd(&self) -> SCACHESD_R {
        SCACHESD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Crypto MAA RAM Shut Down."]
    #[inline(always)]
    pub fn cryptosd(&self) -> CRYPTOSD_R {
        CRYPTOSD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB FIFO Shut Down."]
    #[inline(always)]
    pub fn usbfifosd(&self) -> USBFIFOSD_R {
        USBFIFOSD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ROM Shut Down."]
    #[inline(always)]
    pub fn romsd(&self) -> ROMSD_R {
        ROMSD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ROM1 Shut Down."]
    #[inline(always)]
    pub fn rom1sd(&self) -> ROM1SD_R {
        ROM1SD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ICache 1 Shut Down."]
    #[inline(always)]
    pub fn ic1sd(&self) -> IC1SD_R {
        IC1SD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM block 0 Shut Down."]
    #[inline(always)]
    pub fn sram0sd(&mut self) -> SRAM0SD_W {
        SRAM0SD_W { w: self }
    }
    #[doc = "Bit 1 - System RAM block 1 Shut Down."]
    #[inline(always)]
    pub fn sram1sd(&mut self) -> SRAM1SD_W {
        SRAM1SD_W { w: self }
    }
    #[doc = "Bit 2 - System RAM block 2 Shut Down."]
    #[inline(always)]
    pub fn sram2sd(&mut self) -> SRAM2SD_W {
        SRAM2SD_W { w: self }
    }
    #[doc = "Bit 3 - System RAM block 3 Shut Down."]
    #[inline(always)]
    pub fn sram3sd(&mut self) -> SRAM3SD_W {
        SRAM3SD_W { w: self }
    }
    #[doc = "Bit 4 - System RAM block 4 Shut Down."]
    #[inline(always)]
    pub fn sram4sd(&mut self) -> SRAM4SD_W {
        SRAM4SD_W { w: self }
    }
    #[doc = "Bit 5 - System RAM block 5 Shut Down."]
    #[inline(always)]
    pub fn sram5sd(&mut self) -> SRAM5SD_W {
        SRAM5SD_W { w: self }
    }
    #[doc = "Bit 6 - System RAM block 6 Shut Down."]
    #[inline(always)]
    pub fn sram6sd(&mut self) -> SRAM6SD_W {
        SRAM6SD_W { w: self }
    }
    #[doc = "Bit 7 - Instruction Cache RAM Shut Down."]
    #[inline(always)]
    pub fn icachesd(&mut self) -> ICACHESD_W {
        ICACHESD_W { w: self }
    }
    #[doc = "Bit 8 - XiP Instruction Cache RAM Shut Down."]
    #[inline(always)]
    pub fn icachexipsd(&mut self) -> ICACHEXIPSD_W {
        ICACHEXIPSD_W { w: self }
    }
    #[doc = "Bit 9 - System Cache RAM Shut Down."]
    #[inline(always)]
    pub fn scachesd(&mut self) -> SCACHESD_W {
        SCACHESD_W { w: self }
    }
    #[doc = "Bit 10 - Crypto MAA RAM Shut Down."]
    #[inline(always)]
    pub fn cryptosd(&mut self) -> CRYPTOSD_W {
        CRYPTOSD_W { w: self }
    }
    #[doc = "Bit 11 - USB FIFO Shut Down."]
    #[inline(always)]
    pub fn usbfifosd(&mut self) -> USBFIFOSD_W {
        USBFIFOSD_W { w: self }
    }
    #[doc = "Bit 12 - ROM Shut Down."]
    #[inline(always)]
    pub fn romsd(&mut self) -> ROMSD_W {
        ROMSD_W { w: self }
    }
    #[doc = "Bit 13 - ROM1 Shut Down."]
    #[inline(always)]
    pub fn rom1sd(&mut self) -> ROM1SD_W {
        ROM1SD_W { w: self }
    }
    #[doc = "Bit 14 - ICache 1 Shut Down."]
    #[inline(always)]
    pub fn ic1sd(&mut self) -> IC1SD_W {
        IC1SD_W { w: self }
    }
}
