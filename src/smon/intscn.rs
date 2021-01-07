#[doc = "Reader of register INTSCN"]
pub type R = crate::R<u32, super::INTSCN>;
#[doc = "Writer for register INTSCN"]
pub type W = crate::W<u32, super::INTSCN>;
#[doc = "Register INTSCN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Die Shield Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHIELD_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<SHIELD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SHIELD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SHIELD_EN`"]
pub type SHIELD_EN_R = crate::R<bool, SHIELD_EN_A>;
impl SHIELD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIELD_EN_A {
        match self.bits {
            false => SHIELD_EN_A::DIS,
            true => SHIELD_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SHIELD_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SHIELD_EN_A::EN
    }
}
#[doc = "Write proxy for field `SHIELD_EN`"]
pub struct SHIELD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIELD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHIELD_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SHIELD_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SHIELD_EN_A::EN)
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
#[doc = "Temperature Sensor Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMP_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<TEMP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TEMP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEMP_EN`"]
pub type TEMP_EN_R = crate::R<bool, TEMP_EN_A>;
impl TEMP_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMP_EN_A {
        match self.bits {
            false => TEMP_EN_A::DIS,
            true => TEMP_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TEMP_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TEMP_EN_A::EN
    }
}
#[doc = "Write proxy for field `TEMP_EN`"]
pub struct TEMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMP_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TEMP_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TEMP_EN_A::EN)
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
#[doc = "Battery Monitor Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBAT_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VBAT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VBAT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VBAT_EN`"]
pub type VBAT_EN_R = crate::R<bool, VBAT_EN_A>;
impl VBAT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBAT_EN_A {
        match self.bits {
            false => VBAT_EN_A::DIS,
            true => VBAT_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VBAT_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VBAT_EN_A::EN
    }
}
#[doc = "Write proxy for field `VBAT_EN`"]
pub struct VBAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBAT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBAT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VBAT_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VBAT_EN_A::EN)
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
#[doc = "Low Temperature Detection Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOTEMP_SEL_A {
    #[doc = "0: -50 degrees C."]
    NEG50C = 0,
    #[doc = "1: -30 degrees C."]
    NEG30C = 1,
}
impl From<LOTEMP_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LOTEMP_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOTEMP_SEL`"]
pub type LOTEMP_SEL_R = crate::R<bool, LOTEMP_SEL_A>;
impl LOTEMP_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOTEMP_SEL_A {
        match self.bits {
            false => LOTEMP_SEL_A::NEG50C,
            true => LOTEMP_SEL_A::NEG30C,
        }
    }
    #[doc = "Checks if the value of the field is `NEG50C`"]
    #[inline(always)]
    pub fn is_neg50c(&self) -> bool {
        *self == LOTEMP_SEL_A::NEG50C
    }
    #[doc = "Checks if the value of the field is `NEG30C`"]
    #[inline(always)]
    pub fn is_neg30c(&self) -> bool {
        *self == LOTEMP_SEL_A::NEG30C
    }
}
#[doc = "Write proxy for field `LOTEMP_SEL`"]
pub struct LOTEMP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTEMP_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOTEMP_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "-50 degrees C."]
    #[inline(always)]
    pub fn neg50c(self) -> &'a mut W {
        self.variant(LOTEMP_SEL_A::NEG50C)
    }
    #[doc = "-30 degrees C."]
    #[inline(always)]
    pub fn neg30c(self) -> &'a mut W {
        self.variant(LOTEMP_SEL_A::NEG30C)
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
#[doc = "VCORE Undervoltage Detect Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCORELOEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VCORELOEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCORELOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCORELOEN`"]
pub type VCORELOEN_R = crate::R<bool, VCORELOEN_A>;
impl VCORELOEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORELOEN_A {
        match self.bits {
            false => VCORELOEN_A::DIS,
            true => VCORELOEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VCORELOEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VCORELOEN_A::EN
    }
}
#[doc = "Write proxy for field `VCORELOEN`"]
pub struct VCORELOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCORELOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCORELOEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VCORELOEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VCORELOEN_A::EN)
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
#[doc = "VCORE Overvoltage Detect Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOREHIEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VCOREHIEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCOREHIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCOREHIEN`"]
pub type VCOREHIEN_R = crate::R<bool, VCOREHIEN_A>;
impl VCOREHIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOREHIEN_A {
        match self.bits {
            false => VCOREHIEN_A::DIS,
            true => VCOREHIEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VCOREHIEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VCOREHIEN_A::EN
    }
}
#[doc = "Write proxy for field `VCOREHIEN`"]
pub struct VCOREHIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOREHIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOREHIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VCOREHIEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VCOREHIEN_A::EN)
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
#[doc = "VDD Undervoltage Detect Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDLOEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VDDLOEN_A> for bool {
    #[inline(always)]
    fn from(variant: VDDLOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDLOEN`"]
pub type VDDLOEN_R = crate::R<bool, VDDLOEN_A>;
impl VDDLOEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDLOEN_A {
        match self.bits {
            false => VDDLOEN_A::DIS,
            true => VDDLOEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VDDLOEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VDDLOEN_A::EN
    }
}
#[doc = "Write proxy for field `VDDLOEN`"]
pub struct VDDLOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDLOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDLOEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDLOEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDLOEN_A::EN)
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
#[doc = "VDD Overvoltage Detect Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDHIEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VDDHIEN_A> for bool {
    #[inline(always)]
    fn from(variant: VDDHIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDHIEN`"]
pub type VDDHIEN_R = crate::R<bool, VDDHIEN_A>;
impl VDDHIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDHIEN_A {
        match self.bits {
            false => VDDHIEN_A::DIS,
            true => VDDHIEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VDDHIEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VDDHIEN_A::EN
    }
}
#[doc = "Write proxy for field `VDDHIEN`"]
pub struct VDDHIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDHIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDHIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDHIEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDHIEN_A::EN)
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
#[doc = "Voltage Glitch Detection Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VGLEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VGLEN_A> for bool {
    #[inline(always)]
    fn from(variant: VGLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VGLEN`"]
pub type VGLEN_R = crate::R<bool, VGLEN_A>;
impl VGLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VGLEN_A {
        match self.bits {
            false => VGLEN_A::DIS,
            true => VGLEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VGLEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VGLEN_A::EN
    }
}
#[doc = "Write proxy for field `VGLEN`"]
pub struct VGLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VGLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VGLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VGLEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VGLEN_A::EN)
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
#[doc = "Lock Register. Once locked, the INTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: Unlocked."]
    UNLOCKED = 0,
    #[doc = "1: Locked."]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Die Shield Enable."]
    #[inline(always)]
    pub fn shield_en(&self) -> SHIELD_EN_R {
        SHIELD_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Temperature Sensor Enable."]
    #[inline(always)]
    pub fn temp_en(&self) -> TEMP_EN_R {
        TEMP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Battery Monitor Enable."]
    #[inline(always)]
    pub fn vbat_en(&self) -> VBAT_EN_R {
        VBAT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Low Temperature Detection Select."]
    #[inline(always)]
    pub fn lotemp_sel(&self) -> LOTEMP_SEL_R {
        LOTEMP_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - VCORE Undervoltage Detect Enable."]
    #[inline(always)]
    pub fn vcoreloen(&self) -> VCORELOEN_R {
        VCORELOEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - VCORE Overvoltage Detect Enable."]
    #[inline(always)]
    pub fn vcorehien(&self) -> VCOREHIEN_R {
        VCOREHIEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - VDD Undervoltage Detect Enable."]
    #[inline(always)]
    pub fn vddloen(&self) -> VDDLOEN_R {
        VDDLOEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VDD Overvoltage Detect Enable."]
    #[inline(always)]
    pub fn vddhien(&self) -> VDDHIEN_R {
        VDDHIEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Voltage Glitch Detection Enable."]
    #[inline(always)]
    pub fn vglen(&self) -> VGLEN_R {
        VGLEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Register. Once locked, the INTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Die Shield Enable."]
    #[inline(always)]
    pub fn shield_en(&mut self) -> SHIELD_EN_W {
        SHIELD_EN_W { w: self }
    }
    #[doc = "Bit 1 - Temperature Sensor Enable."]
    #[inline(always)]
    pub fn temp_en(&mut self) -> TEMP_EN_W {
        TEMP_EN_W { w: self }
    }
    #[doc = "Bit 2 - Battery Monitor Enable."]
    #[inline(always)]
    pub fn vbat_en(&mut self) -> VBAT_EN_W {
        VBAT_EN_W { w: self }
    }
    #[doc = "Bit 16 - Low Temperature Detection Select."]
    #[inline(always)]
    pub fn lotemp_sel(&mut self) -> LOTEMP_SEL_W {
        LOTEMP_SEL_W { w: self }
    }
    #[doc = "Bit 18 - VCORE Undervoltage Detect Enable."]
    #[inline(always)]
    pub fn vcoreloen(&mut self) -> VCORELOEN_W {
        VCORELOEN_W { w: self }
    }
    #[doc = "Bit 19 - VCORE Overvoltage Detect Enable."]
    #[inline(always)]
    pub fn vcorehien(&mut self) -> VCOREHIEN_W {
        VCOREHIEN_W { w: self }
    }
    #[doc = "Bit 20 - VDD Undervoltage Detect Enable."]
    #[inline(always)]
    pub fn vddloen(&mut self) -> VDDLOEN_W {
        VDDLOEN_W { w: self }
    }
    #[doc = "Bit 21 - VDD Overvoltage Detect Enable."]
    #[inline(always)]
    pub fn vddhien(&mut self) -> VDDHIEN_W {
        VDDHIEN_W { w: self }
    }
    #[doc = "Bit 22 - Voltage Glitch Detection Enable."]
    #[inline(always)]
    pub fn vglen(&mut self) -> VGLEN_W {
        VGLEN_W { w: self }
    }
    #[doc = "Bit 31 - Lock Register. Once locked, the INTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
