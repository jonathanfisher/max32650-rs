#[doc = "Reader of register EXTSCN"]
pub type R = crate::R<u32, super::EXTSCN>;
#[doc = "Writer for register EXTSCN"]
pub type W = crate::W<u32, super::EXTSCN>;
#[doc = "Register EXTSCN `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTSCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External Sensor Enable for input/output pair 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTS_EN0_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EXTS_EN0_A> for bool {
    #[inline(always)]
    fn from(variant: EXTS_EN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTS_EN0`"]
pub type EXTS_EN0_R = crate::R<bool, EXTS_EN0_A>;
impl EXTS_EN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTS_EN0_A {
        match self.bits {
            false => EXTS_EN0_A::DIS,
            true => EXTS_EN0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EXTS_EN0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EXTS_EN0_A::EN
    }
}
#[doc = "Write proxy for field `EXTS_EN0`"]
pub struct EXTS_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTS_EN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTS_EN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EXTS_EN0_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EXTS_EN0_A::EN)
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
#[doc = "External Sensor Enable for input/output pair 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTS_EN1_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EXTS_EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTS_EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTS_EN1`"]
pub type EXTS_EN1_R = crate::R<bool, EXTS_EN1_A>;
impl EXTS_EN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTS_EN1_A {
        match self.bits {
            false => EXTS_EN1_A::DIS,
            true => EXTS_EN1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EXTS_EN1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EXTS_EN1_A::EN
    }
}
#[doc = "Write proxy for field `EXTS_EN1`"]
pub struct EXTS_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTS_EN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTS_EN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EXTS_EN1_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EXTS_EN1_A::EN)
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
#[doc = "External Sensor Enable for input/output pair 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTS_EN2_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EXTS_EN2_A> for bool {
    #[inline(always)]
    fn from(variant: EXTS_EN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTS_EN2`"]
pub type EXTS_EN2_R = crate::R<bool, EXTS_EN2_A>;
impl EXTS_EN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTS_EN2_A {
        match self.bits {
            false => EXTS_EN2_A::DIS,
            true => EXTS_EN2_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EXTS_EN2_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EXTS_EN2_A::EN
    }
}
#[doc = "Write proxy for field `EXTS_EN2`"]
pub struct EXTS_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTS_EN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTS_EN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EXTS_EN2_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EXTS_EN2_A::EN)
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
#[doc = "External Sensor Enable for input/output pair 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTS_EN3_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EXTS_EN3_A> for bool {
    #[inline(always)]
    fn from(variant: EXTS_EN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTS_EN3`"]
pub type EXTS_EN3_R = crate::R<bool, EXTS_EN3_A>;
impl EXTS_EN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTS_EN3_A {
        match self.bits {
            false => EXTS_EN3_A::DIS,
            true => EXTS_EN3_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EXTS_EN3_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EXTS_EN3_A::EN
    }
}
#[doc = "Write proxy for field `EXTS_EN3`"]
pub struct EXTS_EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTS_EN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTS_EN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EXTS_EN3_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EXTS_EN3_A::EN)
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
#[doc = "External Sensor Enable for input/output pair 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTS_EN4_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EXTS_EN4_A> for bool {
    #[inline(always)]
    fn from(variant: EXTS_EN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTS_EN4`"]
pub type EXTS_EN4_R = crate::R<bool, EXTS_EN4_A>;
impl EXTS_EN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTS_EN4_A {
        match self.bits {
            false => EXTS_EN4_A::DIS,
            true => EXTS_EN4_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EXTS_EN4_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EXTS_EN4_A::EN
    }
}
#[doc = "Write proxy for field `EXTS_EN4`"]
pub struct EXTS_EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTS_EN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTS_EN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EXTS_EN4_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EXTS_EN4_A::EN)
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
#[doc = "External Sensor Enable for input/output pair 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTS_EN5_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EXTS_EN5_A> for bool {
    #[inline(always)]
    fn from(variant: EXTS_EN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTS_EN5`"]
pub type EXTS_EN5_R = crate::R<bool, EXTS_EN5_A>;
impl EXTS_EN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTS_EN5_A {
        match self.bits {
            false => EXTS_EN5_A::DIS,
            true => EXTS_EN5_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EXTS_EN5_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EXTS_EN5_A::EN
    }
}
#[doc = "Write proxy for field `EXTS_EN5`"]
pub struct EXTS_EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTS_EN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTS_EN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EXTS_EN5_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EXTS_EN5_A::EN)
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
#[doc = "Reader of field `EXTCNT`"]
pub type EXTCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTCNT`"]
pub struct EXTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "External Sensor Frequency. These bits define the frequency at which the external sensors are clocked to/from the EXTS_IN and EXTS_OUT pair.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTFRQ_A {
    #[doc = "0: Div 4 (2000Hz)."]
    FREQ2000HZ = 0,
    #[doc = "1: Div 8 (1000Hz)."]
    FREQ1000HZ = 1,
    #[doc = "2: Div 16 (500Hz)."]
    FREQ500HZ = 2,
    #[doc = "3: Div 32 (250Hz)."]
    FREQ250HZ = 3,
    #[doc = "4: Div 64 (125Hz)."]
    FREQ125HZ = 4,
    #[doc = "5: Div 128 (63Hz)."]
    FREQ63HZ = 5,
    #[doc = "6: Div 256 (31Hz)."]
    FREQ31HZ = 6,
    #[doc = "7: Reserved. Do not use."]
    RFU = 7,
}
impl From<EXTFRQ_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTFRQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTFRQ`"]
pub type EXTFRQ_R = crate::R<u8, EXTFRQ_A>;
impl EXTFRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTFRQ_A {
        match self.bits {
            0 => EXTFRQ_A::FREQ2000HZ,
            1 => EXTFRQ_A::FREQ1000HZ,
            2 => EXTFRQ_A::FREQ500HZ,
            3 => EXTFRQ_A::FREQ250HZ,
            4 => EXTFRQ_A::FREQ125HZ,
            5 => EXTFRQ_A::FREQ63HZ,
            6 => EXTFRQ_A::FREQ31HZ,
            7 => EXTFRQ_A::RFU,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREQ2000HZ`"]
    #[inline(always)]
    pub fn is_freq2000hz(&self) -> bool {
        *self == EXTFRQ_A::FREQ2000HZ
    }
    #[doc = "Checks if the value of the field is `FREQ1000HZ`"]
    #[inline(always)]
    pub fn is_freq1000hz(&self) -> bool {
        *self == EXTFRQ_A::FREQ1000HZ
    }
    #[doc = "Checks if the value of the field is `FREQ500HZ`"]
    #[inline(always)]
    pub fn is_freq500hz(&self) -> bool {
        *self == EXTFRQ_A::FREQ500HZ
    }
    #[doc = "Checks if the value of the field is `FREQ250HZ`"]
    #[inline(always)]
    pub fn is_freq250hz(&self) -> bool {
        *self == EXTFRQ_A::FREQ250HZ
    }
    #[doc = "Checks if the value of the field is `FREQ125HZ`"]
    #[inline(always)]
    pub fn is_freq125hz(&self) -> bool {
        *self == EXTFRQ_A::FREQ125HZ
    }
    #[doc = "Checks if the value of the field is `FREQ63HZ`"]
    #[inline(always)]
    pub fn is_freq63hz(&self) -> bool {
        *self == EXTFRQ_A::FREQ63HZ
    }
    #[doc = "Checks if the value of the field is `FREQ31HZ`"]
    #[inline(always)]
    pub fn is_freq31hz(&self) -> bool {
        *self == EXTFRQ_A::FREQ31HZ
    }
    #[doc = "Checks if the value of the field is `RFU`"]
    #[inline(always)]
    pub fn is_rfu(&self) -> bool {
        *self == EXTFRQ_A::RFU
    }
}
#[doc = "Write proxy for field `EXTFRQ`"]
pub struct EXTFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTFRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTFRQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Div 4 (2000Hz)."]
    #[inline(always)]
    pub fn freq2000hz(self) -> &'a mut W {
        self.variant(EXTFRQ_A::FREQ2000HZ)
    }
    #[doc = "Div 8 (1000Hz)."]
    #[inline(always)]
    pub fn freq1000hz(self) -> &'a mut W {
        self.variant(EXTFRQ_A::FREQ1000HZ)
    }
    #[doc = "Div 16 (500Hz)."]
    #[inline(always)]
    pub fn freq500hz(self) -> &'a mut W {
        self.variant(EXTFRQ_A::FREQ500HZ)
    }
    #[doc = "Div 32 (250Hz)."]
    #[inline(always)]
    pub fn freq250hz(self) -> &'a mut W {
        self.variant(EXTFRQ_A::FREQ250HZ)
    }
    #[doc = "Div 64 (125Hz)."]
    #[inline(always)]
    pub fn freq125hz(self) -> &'a mut W {
        self.variant(EXTFRQ_A::FREQ125HZ)
    }
    #[doc = "Div 128 (63Hz)."]
    #[inline(always)]
    pub fn freq63hz(self) -> &'a mut W {
        self.variant(EXTFRQ_A::FREQ63HZ)
    }
    #[doc = "Div 256 (31Hz)."]
    #[inline(always)]
    pub fn freq31hz(self) -> &'a mut W {
        self.variant(EXTFRQ_A::FREQ31HZ)
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(EXTFRQ_A::RFU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Clock Divide. These bits are used to divide the 8KHz input clock. The resulting divided clock is used for all logic within the Security Monitor Block. Note: If the input clock is divided with these bits, the error count threshold table and output frequency will be affected accordingly with the same divide factor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVCLK_A {
    #[doc = "0: Divide by 1 (8000 Hz)."]
    DIV1 = 0,
    #[doc = "1: Divide by 2 (4000 Hz)."]
    DIV2 = 1,
    #[doc = "2: Divide by 4 (2000 Hz)."]
    DIV4 = 2,
    #[doc = "3: Divide by 8 (1000 Hz)."]
    DIV8 = 3,
    #[doc = "4: Divide by 16 (500 Hz)."]
    DIV16 = 4,
    #[doc = "5: Divide by 32 (250 Hz)."]
    DIV32 = 5,
    #[doc = "6: Divide by 64 (125 Hz)."]
    DIV64 = 6,
}
impl From<DIVCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVCLK`"]
pub type DIVCLK_R = crate::R<u8, DIVCLK_A>;
impl DIVCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVCLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVCLK_A::DIV1),
            1 => Val(DIVCLK_A::DIV2),
            2 => Val(DIVCLK_A::DIV4),
            3 => Val(DIVCLK_A::DIV8),
            4 => Val(DIVCLK_A::DIV16),
            5 => Val(DIVCLK_A::DIV32),
            6 => Val(DIVCLK_A::DIV64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVCLK_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIVCLK_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIVCLK_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIVCLK_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DIVCLK_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DIVCLK_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DIVCLK_A::DIV64
    }
}
#[doc = "Write proxy for field `DIVCLK`"]
pub struct DIVCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1 (8000 Hz)."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIVCLK_A::DIV1)
    }
    #[doc = "Divide by 2 (4000 Hz)."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIVCLK_A::DIV2)
    }
    #[doc = "Divide by 4 (2000 Hz)."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DIVCLK_A::DIV4)
    }
    #[doc = "Divide by 8 (1000 Hz)."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DIVCLK_A::DIV8)
    }
    #[doc = "Divide by 16 (500 Hz)."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DIVCLK_A::DIV16)
    }
    #[doc = "Divide by 32 (250 Hz)."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DIVCLK_A::DIV32)
    }
    #[doc = "Divide by 64 (125 Hz)."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(DIVCLK_A::DIV64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Busy. This bit is set to 1 by hardware after EXTSCN register is written to. This bit is automatically cleared to 0 after this register information has been transferred to the security monitor domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Idle."]
    IDLE = 0,
    #[doc = "1: Update in Progress."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Lock Register. Once locked, the EXTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register.\n\nValue on reset: 0"]
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
    #[doc = "Bit 0 - External Sensor Enable for input/output pair 0."]
    #[inline(always)]
    pub fn exts_en0(&self) -> EXTS_EN0_R {
        EXTS_EN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Sensor Enable for input/output pair 1."]
    #[inline(always)]
    pub fn exts_en1(&self) -> EXTS_EN1_R {
        EXTS_EN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Sensor Enable for input/output pair 2."]
    #[inline(always)]
    pub fn exts_en2(&self) -> EXTS_EN2_R {
        EXTS_EN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Sensor Enable for input/output pair 3."]
    #[inline(always)]
    pub fn exts_en3(&self) -> EXTS_EN3_R {
        EXTS_EN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Sensor Enable for input/output pair 4."]
    #[inline(always)]
    pub fn exts_en4(&self) -> EXTS_EN4_R {
        EXTS_EN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Sensor Enable for input/output pair 5."]
    #[inline(always)]
    pub fn exts_en5(&self) -> EXTS_EN5_R {
        EXTS_EN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - External Sensor Error Counter. These bits set the number of external sensor accepted mismatches that have to occur within a single bit period before an external sensor alarm is triggered."]
    #[inline(always)]
    pub fn extcnt(&self) -> EXTCNT_R {
        EXTCNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - External Sensor Frequency. These bits define the frequency at which the external sensors are clocked to/from the EXTS_IN and EXTS_OUT pair."]
    #[inline(always)]
    pub fn extfrq(&self) -> EXTFRQ_R {
        EXTFRQ_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Clock Divide. These bits are used to divide the 8KHz input clock. The resulting divided clock is used for all logic within the Security Monitor Block. Note: If the input clock is divided with these bits, the error count threshold table and output frequency will be affected accordingly with the same divide factor."]
    #[inline(always)]
    pub fn divclk(&self) -> DIVCLK_R {
        DIVCLK_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Busy. This bit is set to 1 by hardware after EXTSCN register is written to. This bit is automatically cleared to 0 after this register information has been transferred to the security monitor domain."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Register. Once locked, the EXTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Sensor Enable for input/output pair 0."]
    #[inline(always)]
    pub fn exts_en0(&mut self) -> EXTS_EN0_W {
        EXTS_EN0_W { w: self }
    }
    #[doc = "Bit 1 - External Sensor Enable for input/output pair 1."]
    #[inline(always)]
    pub fn exts_en1(&mut self) -> EXTS_EN1_W {
        EXTS_EN1_W { w: self }
    }
    #[doc = "Bit 2 - External Sensor Enable for input/output pair 2."]
    #[inline(always)]
    pub fn exts_en2(&mut self) -> EXTS_EN2_W {
        EXTS_EN2_W { w: self }
    }
    #[doc = "Bit 3 - External Sensor Enable for input/output pair 3."]
    #[inline(always)]
    pub fn exts_en3(&mut self) -> EXTS_EN3_W {
        EXTS_EN3_W { w: self }
    }
    #[doc = "Bit 4 - External Sensor Enable for input/output pair 4."]
    #[inline(always)]
    pub fn exts_en4(&mut self) -> EXTS_EN4_W {
        EXTS_EN4_W { w: self }
    }
    #[doc = "Bit 5 - External Sensor Enable for input/output pair 5."]
    #[inline(always)]
    pub fn exts_en5(&mut self) -> EXTS_EN5_W {
        EXTS_EN5_W { w: self }
    }
    #[doc = "Bits 16:20 - External Sensor Error Counter. These bits set the number of external sensor accepted mismatches that have to occur within a single bit period before an external sensor alarm is triggered."]
    #[inline(always)]
    pub fn extcnt(&mut self) -> EXTCNT_W {
        EXTCNT_W { w: self }
    }
    #[doc = "Bits 21:23 - External Sensor Frequency. These bits define the frequency at which the external sensors are clocked to/from the EXTS_IN and EXTS_OUT pair."]
    #[inline(always)]
    pub fn extfrq(&mut self) -> EXTFRQ_W {
        EXTFRQ_W { w: self }
    }
    #[doc = "Bits 24:26 - Clock Divide. These bits are used to divide the 8KHz input clock. The resulting divided clock is used for all logic within the Security Monitor Block. Note: If the input clock is divided with these bits, the error count threshold table and output frequency will be affected accordingly with the same divide factor."]
    #[inline(always)]
    pub fn divclk(&mut self) -> DIVCLK_W {
        DIVCLK_W { w: self }
    }
    #[doc = "Bit 31 - Lock Register. Once locked, the EXTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
