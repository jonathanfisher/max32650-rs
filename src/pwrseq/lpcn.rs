#[doc = "Reader of register LPCN"]
pub type R = crate::R<u32, super::LPCN>;
#[doc = "Writer for register LPCN"]
pub type W = crate::W<u32, super::LPCN>;
#[doc = "Register LPCN `reset()`'s with value 0"]
impl crate::ResetValue for super::LPCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMRET_A {
    #[doc = "0: Disable Ram Retention."]
    DIS = 0,
    #[doc = "1: Enable System RAM 0 retention."]
    EN1 = 1,
    #[doc = "2: Enable System RAM 0 and 1 retention."]
    EN2 = 2,
    #[doc = "3: Enable System RAM 0 and 1 retention, if RREGEN=0, Enable all System RAM retention."]
    EN3 = 3,
}
impl From<RAMRET_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMRET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAMRET`"]
pub type RAMRET_R = crate::R<u8, RAMRET_A>;
impl RAMRET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRET_A {
        match self.bits {
            0 => RAMRET_A::DIS,
            1 => RAMRET_A::EN1,
            2 => RAMRET_A::EN2,
            3 => RAMRET_A::EN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RAMRET_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN1`"]
    #[inline(always)]
    pub fn is_en1(&self) -> bool {
        *self == RAMRET_A::EN1
    }
    #[doc = "Checks if the value of the field is `EN2`"]
    #[inline(always)]
    pub fn is_en2(&self) -> bool {
        *self == RAMRET_A::EN2
    }
    #[doc = "Checks if the value of the field is `EN3`"]
    #[inline(always)]
    pub fn is_en3(&self) -> bool {
        *self == RAMRET_A::EN3
    }
}
#[doc = "Write proxy for field `RAMRET`"]
pub struct RAMRET_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMRET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMRET_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RAMRET_A::DIS)
    }
    #[doc = "Enable System RAM 0 retention."]
    #[inline(always)]
    pub fn en1(self) -> &'a mut W {
        self.variant(RAMRET_A::EN1)
    }
    #[doc = "Enable System RAM 0 and 1 retention."]
    #[inline(always)]
    pub fn en2(self) -> &'a mut W {
        self.variant(RAMRET_A::EN2)
    }
    #[doc = "Enable System RAM 0 and 1 retention, if RREGEN=0, Enable all System RAM retention."]
    #[inline(always)]
    pub fn en3(self) -> &'a mut W {
        self.variant(RAMRET_A::EN3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Operating Voltage Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVR_A {
    #[doc = "0: 0.9V 24MHz"]
    _0_9V = 0,
    #[doc = "1: 1.0V 48MHz"]
    _1_0V = 1,
    #[doc = "2: 1.1V 96MHz"]
    _1_1V = 2,
}
impl From<OVR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<u8, OVR_A>;
impl OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OVR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OVR_A::_0_9V),
            1 => Val(OVR_A::_1_0V),
            2 => Val(OVR_A::_1_1V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_9V`"]
    #[inline(always)]
    pub fn is_0_9v(&self) -> bool {
        *self == OVR_A::_0_9V
    }
    #[doc = "Checks if the value of the field is `_1_0V`"]
    #[inline(always)]
    pub fn is_1_0v(&self) -> bool {
        *self == OVR_A::_1_0V
    }
    #[doc = "Checks if the value of the field is `_1_1V`"]
    #[inline(always)]
    pub fn is_1_1v(&self) -> bool {
        *self == OVR_A::_1_1V
    }
}
#[doc = "Write proxy for field `OVR`"]
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.9V 24MHz"]
    #[inline(always)]
    pub fn _0_9v(self) -> &'a mut W {
        self.variant(OVR_A::_0_9V)
    }
    #[doc = "1.0V 48MHz"]
    #[inline(always)]
    pub fn _1_0v(self) -> &'a mut W {
        self.variant(OVR_A::_1_0V)
    }
    #[doc = "1.1V 96MHz"]
    #[inline(always)]
    pub fn _1_1v(self) -> &'a mut W {
        self.variant(OVR_A::_1_1V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Block Auto-Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKDET_A {
    #[doc = "0: enable"]
    ENABLED = 0,
    #[doc = "1: disable"]
    DISABLE = 1,
}
impl From<BLKDET_A> for bool {
    #[inline(always)]
    fn from(variant: BLKDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLKDET`"]
pub type BLKDET_R = crate::R<bool, BLKDET_A>;
impl BLKDET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLKDET_A {
        match self.bits {
            false => BLKDET_A::ENABLED,
            true => BLKDET_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BLKDET_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BLKDET_A::DISABLE
    }
}
#[doc = "Write proxy for field `BLKDET`"]
pub struct BLKDET_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLKDET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BLKDET_A::ENABLED)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BLKDET_A::DISABLE)
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
#[doc = "Reader of field `FVDDEN`"]
pub type FVDDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FVDDEN`"]
pub struct FVDDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FVDDEN_W<'a> {
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
#[doc = "Retention Regulator Enable. This bit controls the retention regulator in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RREGEN_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<RREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RREGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RREGEN`"]
pub type RREGEN_R = crate::R<bool, RREGEN_A>;
impl RREGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RREGEN_A {
        match self.bits {
            false => RREGEN_A::DIS,
            true => RREGEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RREGEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RREGEN_A::EN
    }
}
#[doc = "Write proxy for field `RREGEN`"]
pub struct RREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RREGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RREGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RREGEN_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RREGEN_A::EN)
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
#[doc = "Background Mode ENable. This bit allows low-power background mode operations, while the CPU is in DeepSleep.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCKGRND_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<BCKGRND_A> for bool {
    #[inline(always)]
    fn from(variant: BCKGRND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCKGRND`"]
pub type BCKGRND_R = crate::R<bool, BCKGRND_A>;
impl BCKGRND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCKGRND_A {
        match self.bits {
            false => BCKGRND_A::DIS,
            true => BCKGRND_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BCKGRND_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BCKGRND_A::EN
    }
}
#[doc = "Write proxy for field `BCKGRND`"]
pub struct BCKGRND_W<'a> {
    w: &'a mut W,
}
impl<'a> BCKGRND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCKGRND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BCKGRND_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BCKGRND_A::EN)
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
#[doc = "Fast Wake-Up Mode. This bit enables fast wake-up from DeepSleep mode. (5uS typical).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWKM_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<FWKM_A> for bool {
    #[inline(always)]
    fn from(variant: FWKM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FWKM`"]
pub type FWKM_R = crate::R<bool, FWKM_A>;
impl FWKM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWKM_A {
        match self.bits {
            false => FWKM_A::DIS,
            true => FWKM_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FWKM_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FWKM_A::EN
    }
}
#[doc = "Write proxy for field `FWKM`"]
pub struct FWKM_W<'a> {
    w: &'a mut W,
}
impl<'a> FWKM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWKM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FWKM_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FWKM_A::EN)
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
#[doc = "Bandgap OFF. This controls the System Bandgap in DeepSleep mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGOFF_A {
    #[doc = "0: Bandgap is always ON."]
    ON = 0,
    #[doc = "1: Bandgap is OFF in DeepSleep mode(default)."]
    OFF = 1,
}
impl From<BGOFF_A> for bool {
    #[inline(always)]
    fn from(variant: BGOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BGOFF`"]
pub type BGOFF_R = crate::R<bool, BGOFF_A>;
impl BGOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGOFF_A {
        match self.bits {
            false => BGOFF_A::ON,
            true => BGOFF_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == BGOFF_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BGOFF_A::OFF
    }
}
#[doc = "Write proxy for field `BGOFF`"]
pub struct BGOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BGOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(BGOFF_A::ON)
    }
    #[doc = "Bandgap is OFF in DeepSleep mode(default)."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(BGOFF_A::OFF)
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
#[doc = "VDDC(VCore) Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDC supply in DeepSleep and BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORVDDCMD_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<PORVDDCMD_A> for bool {
    #[inline(always)]
    fn from(variant: PORVDDCMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORVDDCMD`"]
pub type PORVDDCMD_R = crate::R<bool, PORVDDCMD_A>;
impl PORVDDCMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORVDDCMD_A {
        match self.bits {
            false => PORVDDCMD_A::DIS,
            true => PORVDDCMD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PORVDDCMD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PORVDDCMD_A::EN
    }
}
#[doc = "Write proxy for field `PORVDDCMD`"]
pub struct PORVDDCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PORVDDCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORVDDCMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PORVDDCMD_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PORVDDCMD_A::EN)
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
#[doc = "VDDC(Vcore) Monitor Disable. This bit controls the power monitor on the VCore supply in all operating modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDCMD_A {
    #[doc = "0: Enable if Bandgap is ON(default)"]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<VDDCMD_A> for bool {
    #[inline(always)]
    fn from(variant: VDDCMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDCMD`"]
pub type VDDCMD_R = crate::R<bool, VDDCMD_A>;
impl VDDCMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDCMD_A {
        match self.bits {
            false => VDDCMD_A::EN,
            true => VDDCMD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VDDCMD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VDDCMD_A::DIS
    }
}
#[doc = "Write proxy for field `VDDCMD`"]
pub struct VDDCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDCMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable if Bandgap is ON(default)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDCMD_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDCMD_A::DIS)
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
#[doc = "VRTC Monitor Disable. This bit controls the power monitor on the Always-On Supply in all operating modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRTCMD_A {
    #[doc = "0: Enable if Bandgap is ON(default)"]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<VRTCMD_A> for bool {
    #[inline(always)]
    fn from(variant: VRTCMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VRTCMD`"]
pub type VRTCMD_R = crate::R<bool, VRTCMD_A>;
impl VRTCMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRTCMD_A {
        match self.bits {
            false => VRTCMD_A::EN,
            true => VRTCMD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VRTCMD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VRTCMD_A::DIS
    }
}
#[doc = "Write proxy for field `VRTCMD`"]
pub struct VRTCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> VRTCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VRTCMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable if Bandgap is ON(default)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VRTCMD_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VRTCMD_A::DIS)
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
#[doc = "VDDA Monitor Disable. This bit controls the power monitor of the Analog Supply in all operating modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDAMD_A {
    #[doc = "0: Enable if Bandgap is ON(default)"]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<VDDAMD_A> for bool {
    #[inline(always)]
    fn from(variant: VDDAMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDAMD`"]
pub type VDDAMD_R = crate::R<bool, VDDAMD_A>;
impl VDDAMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDAMD_A {
        match self.bits {
            false => VDDAMD_A::EN,
            true => VDDAMD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VDDAMD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VDDAMD_A::DIS
    }
}
#[doc = "Write proxy for field `VDDAMD`"]
pub struct VDDAMD_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDAMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDAMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable if Bandgap is ON(default)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDAMD_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDAMD_A::DIS)
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
#[doc = "VDDIO Monitor Disable. This bit controls the power monitor of the Analog Supply in all operating modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDIOMD_A {
    #[doc = "0: Enable if Bandgap is ON(default)"]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<VDDIOMD_A> for bool {
    #[inline(always)]
    fn from(variant: VDDIOMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDIOMD`"]
pub type VDDIOMD_R = crate::R<bool, VDDIOMD_A>;
impl VDDIOMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDIOMD_A {
        match self.bits {
            false => VDDIOMD_A::EN,
            true => VDDIOMD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VDDIOMD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VDDIOMD_A::DIS
    }
}
#[doc = "Write proxy for field `VDDIOMD`"]
pub struct VDDIOMD_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDIOMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDIOMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable if Bandgap is ON(default)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDIOMD_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDIOMD_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "VFDDIOH Monitor Disable. This bit controls the power monitor of the Analog Supply in all operating modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDIOHMD_A {
    #[doc = "0: Enable if Bandgap is ON(default)"]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<VDDIOHMD_A> for bool {
    #[inline(always)]
    fn from(variant: VDDIOHMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDIOHMD`"]
pub type VDDIOHMD_R = crate::R<bool, VDDIOHMD_A>;
impl VDDIOHMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDIOHMD_A {
        match self.bits {
            false => VDDIOHMD_A::EN,
            true => VDDIOHMD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VDDIOHMD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VDDIOHMD_A::DIS
    }
}
#[doc = "Write proxy for field `VDDIOHMD`"]
pub struct VDDIOHMD_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDIOHMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDIOHMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable if Bandgap is ON(default)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDIOHMD_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDIOHMD_A::DIS)
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
#[doc = "VDDIO Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIO supply in all operating mods.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORVDDIOMD_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<PORVDDIOMD_A> for bool {
    #[inline(always)]
    fn from(variant: PORVDDIOMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORVDDIOMD`"]
pub type PORVDDIOMD_R = crate::R<bool, PORVDDIOMD_A>;
impl PORVDDIOMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORVDDIOMD_A {
        match self.bits {
            false => PORVDDIOMD_A::DIS,
            true => PORVDDIOMD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PORVDDIOMD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PORVDDIOMD_A::EN
    }
}
#[doc = "Write proxy for field `PORVDDIOMD`"]
pub struct PORVDDIOMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PORVDDIOMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORVDDIOMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PORVDDIOMD_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PORVDDIOMD_A::EN)
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
#[doc = "VDDIOH Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIOH supply in all operating mods.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORVDDIOHMD_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<PORVDDIOHMD_A> for bool {
    #[inline(always)]
    fn from(variant: PORVDDIOHMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORVDDIOHMD`"]
pub type PORVDDIOHMD_R = crate::R<bool, PORVDDIOHMD_A>;
impl PORVDDIOHMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORVDDIOHMD_A {
        match self.bits {
            false => PORVDDIOHMD_A::DIS,
            true => PORVDDIOHMD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PORVDDIOHMD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PORVDDIOHMD_A::EN
    }
}
#[doc = "Write proxy for field `PORVDDIOHMD`"]
pub struct PORVDDIOHMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PORVDDIOHMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORVDDIOHMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PORVDDIOHMD_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PORVDDIOHMD_A::EN)
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
#[doc = "VDDB Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDB supply in all operating mods.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDBMD_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<VDDBMD_A> for bool {
    #[inline(always)]
    fn from(variant: VDDBMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDBMD`"]
pub type VDDBMD_R = crate::R<bool, VDDBMD_A>;
impl VDDBMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDBMD_A {
        match self.bits {
            false => VDDBMD_A::DIS,
            true => VDDBMD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VDDBMD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VDDBMD_A::EN
    }
}
#[doc = "Write proxy for field `VDDBMD`"]
pub struct VDDBMD_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDBMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDBMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDBMD_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDBMD_A::EN)
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
impl R {
    #[doc = "Bits 0:1 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret(&self) -> RAMRET_R {
        RAMRET_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Operating Voltage Range"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Block Auto-Detect"]
    #[inline(always)]
    pub fn blkdet(&self) -> BLKDET_R {
        BLKDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flash VDD Enabled"]
    #[inline(always)]
    pub fn fvdden(&self) -> FVDDEN_R {
        FVDDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Retention Regulator Enable. This bit controls the retention regulator in BACKUP mode."]
    #[inline(always)]
    pub fn rregen(&self) -> RREGEN_R {
        RREGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Background Mode ENable. This bit allows low-power background mode operations, while the CPU is in DeepSleep."]
    #[inline(always)]
    pub fn bckgrnd(&self) -> BCKGRND_R {
        BCKGRND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast Wake-Up Mode. This bit enables fast wake-up from DeepSleep mode. (5uS typical)."]
    #[inline(always)]
    pub fn fwkm(&self) -> FWKM_R {
        FWKM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
    #[inline(always)]
    pub fn bgoff(&self) -> BGOFF_R {
        BGOFF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - VDDC(VCore) Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDC supply in DeepSleep and BACKUP mode."]
    #[inline(always)]
    pub fn porvddcmd(&self) -> PORVDDCMD_R {
        PORVDDCMD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 20 - VDDC(Vcore) Monitor Disable. This bit controls the power monitor on the VCore supply in all operating modes."]
    #[inline(always)]
    pub fn vddcmd(&self) -> VDDCMD_R {
        VDDCMD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VRTC Monitor Disable. This bit controls the power monitor on the Always-On Supply in all operating modes."]
    #[inline(always)]
    pub fn vrtcmd(&self) -> VRTCMD_R {
        VRTCMD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - VDDA Monitor Disable. This bit controls the power monitor of the Analog Supply in all operating modes."]
    #[inline(always)]
    pub fn vddamd(&self) -> VDDAMD_R {
        VDDAMD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - VDDIO Monitor Disable. This bit controls the power monitor of the Analog Supply in all operating modes."]
    #[inline(always)]
    pub fn vddiomd(&self) -> VDDIOMD_R {
        VDDIOMD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - VFDDIOH Monitor Disable. This bit controls the power monitor of the Analog Supply in all operating modes."]
    #[inline(always)]
    pub fn vddiohmd(&self) -> VDDIOHMD_R {
        VDDIOHMD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - VDDIO Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIO supply in all operating mods."]
    #[inline(always)]
    pub fn porvddiomd(&self) -> PORVDDIOMD_R {
        PORVDDIOMD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - VDDIOH Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIOH supply in all operating mods."]
    #[inline(always)]
    pub fn porvddiohmd(&self) -> PORVDDIOHMD_R {
        PORVDDIOHMD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - VDDB Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDB supply in all operating mods."]
    #[inline(always)]
    pub fn vddbmd(&self) -> VDDBMD_R {
        VDDBMD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret(&mut self) -> RAMRET_W {
        RAMRET_W { w: self }
    }
    #[doc = "Bits 4:5 - Operating Voltage Range"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    #[doc = "Bit 6 - Block Auto-Detect"]
    #[inline(always)]
    pub fn blkdet(&mut self) -> BLKDET_W {
        BLKDET_W { w: self }
    }
    #[doc = "Bit 7 - Flash VDD Enabled"]
    #[inline(always)]
    pub fn fvdden(&mut self) -> FVDDEN_W {
        FVDDEN_W { w: self }
    }
    #[doc = "Bit 8 - Retention Regulator Enable. This bit controls the retention regulator in BACKUP mode."]
    #[inline(always)]
    pub fn rregen(&mut self) -> RREGEN_W {
        RREGEN_W { w: self }
    }
    #[doc = "Bit 9 - Background Mode ENable. This bit allows low-power background mode operations, while the CPU is in DeepSleep."]
    #[inline(always)]
    pub fn bckgrnd(&mut self) -> BCKGRND_W {
        BCKGRND_W { w: self }
    }
    #[doc = "Bit 10 - Fast Wake-Up Mode. This bit enables fast wake-up from DeepSleep mode. (5uS typical)."]
    #[inline(always)]
    pub fn fwkm(&mut self) -> FWKM_W {
        FWKM_W { w: self }
    }
    #[doc = "Bit 11 - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
    #[inline(always)]
    pub fn bgoff(&mut self) -> BGOFF_W {
        BGOFF_W { w: self }
    }
    #[doc = "Bit 12 - VDDC(VCore) Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDC supply in DeepSleep and BACKUP mode."]
    #[inline(always)]
    pub fn porvddcmd(&mut self) -> PORVDDCMD_W {
        PORVDDCMD_W { w: self }
    }
    #[doc = "Bit 20 - VDDC(Vcore) Monitor Disable. This bit controls the power monitor on the VCore supply in all operating modes."]
    #[inline(always)]
    pub fn vddcmd(&mut self) -> VDDCMD_W {
        VDDCMD_W { w: self }
    }
    #[doc = "Bit 21 - VRTC Monitor Disable. This bit controls the power monitor on the Always-On Supply in all operating modes."]
    #[inline(always)]
    pub fn vrtcmd(&mut self) -> VRTCMD_W {
        VRTCMD_W { w: self }
    }
    #[doc = "Bit 22 - VDDA Monitor Disable. This bit controls the power monitor of the Analog Supply in all operating modes."]
    #[inline(always)]
    pub fn vddamd(&mut self) -> VDDAMD_W {
        VDDAMD_W { w: self }
    }
    #[doc = "Bit 23 - VDDIO Monitor Disable. This bit controls the power monitor of the Analog Supply in all operating modes."]
    #[inline(always)]
    pub fn vddiomd(&mut self) -> VDDIOMD_W {
        VDDIOMD_W { w: self }
    }
    #[doc = "Bit 24 - VFDDIOH Monitor Disable. This bit controls the power monitor of the Analog Supply in all operating modes."]
    #[inline(always)]
    pub fn vddiohmd(&mut self) -> VDDIOHMD_W {
        VDDIOHMD_W { w: self }
    }
    #[doc = "Bit 25 - VDDIO Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIO supply in all operating mods."]
    #[inline(always)]
    pub fn porvddiomd(&mut self) -> PORVDDIOMD_W {
        PORVDDIOMD_W { w: self }
    }
    #[doc = "Bit 26 - VDDIOH Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIOH supply in all operating mods."]
    #[inline(always)]
    pub fn porvddiohmd(&mut self) -> PORVDDIOHMD_W {
        PORVDDIOHMD_W { w: self }
    }
    #[doc = "Bit 27 - VDDB Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDB supply in all operating mods."]
    #[inline(always)]
    pub fn vddbmd(&mut self) -> VDDBMD_W {
        VDDBMD_W { w: self }
    }
}
