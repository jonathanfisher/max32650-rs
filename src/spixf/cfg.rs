#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Defines SPI Mode, Only valid values are 0 and 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Description not available."]
    SCLK_HI_SAMPLE_RISING = 0,
    #[doc = "3: Description not available."]
    SCLK_LO_SAMPLE_FAILLING = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::SCLK_HI_SAMPLE_RISING),
            3 => Val(MODE_A::SCLK_LO_SAMPLE_FAILLING),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_HI_SAMPLE_RISING`"]
    #[inline(always)]
    pub fn is_sclk_hi_sample_rising(&self) -> bool {
        *self == MODE_A::SCLK_HI_SAMPLE_RISING
    }
    #[doc = "Checks if the value of the field is `SCLK_LO_SAMPLE_FAILLING`"]
    #[inline(always)]
    pub fn is_sclk_lo_sample_failling(&self) -> bool {
        *self == MODE_A::SCLK_LO_SAMPLE_FAILLING
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Description not available."]
    #[inline(always)]
    pub fn sclk_hi_sample_rising(self) -> &'a mut W {
        self.variant(MODE_A::SCLK_HI_SAMPLE_RISING)
    }
    #[doc = "Description not available."]
    #[inline(always)]
    pub fn sclk_lo_sample_failling(self) -> &'a mut W {
        self.variant(MODE_A::SCLK_LO_SAMPLE_FAILLING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Slave Select Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSPOL_A {
    #[doc = "0: Slave Select is Active High."]
    ACTIVE_HIGH = 0,
    #[doc = "1: Slave Select is Active Low."]
    ACTIVE_LOW = 1,
}
impl From<SSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SSPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSPOL`"]
pub type SSPOL_R = crate::R<bool, SSPOL_A>;
impl SSPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSPOL_A {
        match self.bits {
            false => SSPOL_A::ACTIVE_HIGH,
            true => SSPOL_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == SSPOL_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == SSPOL_A::ACTIVE_LOW
    }
}
#[doc = "Write proxy for field `SSPOL`"]
pub struct SSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave Select is Active High."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(SSPOL_A::ACTIVE_HIGH)
    }
    #[doc = "Slave Select is Active Low."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(SSPOL_A::ACTIVE_LOW)
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
#[doc = "Reader of field `SSEL`"]
pub type SSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSEL`"]
pub struct SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `LO_CLK`"]
pub type LO_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LO_CLK`"]
pub struct LO_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HI_CLK`"]
pub type HI_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HI_CLK`"]
pub struct HI_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> HI_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Slave Select Active Timing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSACT_A {
    #[doc = "0: 0 system clocks."]
    OFF = 0,
    #[doc = "1: 2 System clocks."]
    FOR_2_MOD_CLK = 1,
    #[doc = "2: 4 System clocks."]
    FOR_4_MOD_CLK = 2,
    #[doc = "3: 8 System clocks."]
    FOR_8_MOD_CLK = 3,
}
impl From<SSACT_A> for u8 {
    #[inline(always)]
    fn from(variant: SSACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSACT`"]
pub type SSACT_R = crate::R<u8, SSACT_A>;
impl SSACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACT_A {
        match self.bits {
            0 => SSACT_A::OFF,
            1 => SSACT_A::FOR_2_MOD_CLK,
            2 => SSACT_A::FOR_4_MOD_CLK,
            3 => SSACT_A::FOR_8_MOD_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SSACT_A::OFF
    }
    #[doc = "Checks if the value of the field is `FOR_2_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_2_mod_clk(&self) -> bool {
        *self == SSACT_A::FOR_2_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_4_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_4_mod_clk(&self) -> bool {
        *self == SSACT_A::FOR_4_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_8_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_8_mod_clk(&self) -> bool {
        *self == SSACT_A::FOR_8_MOD_CLK
    }
}
#[doc = "Write proxy for field `SSACT`"]
pub struct SSACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0 system clocks."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SSACT_A::OFF)
    }
    #[doc = "2 System clocks."]
    #[inline(always)]
    pub fn for_2_mod_clk(self) -> &'a mut W {
        self.variant(SSACT_A::FOR_2_MOD_CLK)
    }
    #[doc = "4 System clocks."]
    #[inline(always)]
    pub fn for_4_mod_clk(self) -> &'a mut W {
        self.variant(SSACT_A::FOR_4_MOD_CLK)
    }
    #[doc = "8 System clocks."]
    #[inline(always)]
    pub fn for_8_mod_clk(self) -> &'a mut W {
        self.variant(SSACT_A::FOR_8_MOD_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Slave Select Inactive Timing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSIACT_A {
    #[doc = "0: 1 system clocks."]
    FOR_1_MOD_CLK = 0,
    #[doc = "1: 3 System clocks."]
    FOR_3_MOD_CLK = 1,
    #[doc = "2: 5 System clocks."]
    FOR_5_MOD_CLK = 2,
    #[doc = "3: 9 System clocks."]
    FOR_9_MOD_CLK = 3,
}
impl From<SSIACT_A> for u8 {
    #[inline(always)]
    fn from(variant: SSIACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSIACT`"]
pub type SSIACT_R = crate::R<u8, SSIACT_A>;
impl SSIACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIACT_A {
        match self.bits {
            0 => SSIACT_A::FOR_1_MOD_CLK,
            1 => SSIACT_A::FOR_3_MOD_CLK,
            2 => SSIACT_A::FOR_5_MOD_CLK,
            3 => SSIACT_A::FOR_9_MOD_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FOR_1_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_1_mod_clk(&self) -> bool {
        *self == SSIACT_A::FOR_1_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_3_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_3_mod_clk(&self) -> bool {
        *self == SSIACT_A::FOR_3_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_5_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_5_mod_clk(&self) -> bool {
        *self == SSIACT_A::FOR_5_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_9_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_9_mod_clk(&self) -> bool {
        *self == SSIACT_A::FOR_9_MOD_CLK
    }
}
#[doc = "Write proxy for field `SSIACT`"]
pub struct SSIACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 system clocks."]
    #[inline(always)]
    pub fn for_1_mod_clk(self) -> &'a mut W {
        self.variant(SSIACT_A::FOR_1_MOD_CLK)
    }
    #[doc = "3 System clocks."]
    #[inline(always)]
    pub fn for_3_mod_clk(self) -> &'a mut W {
        self.variant(SSIACT_A::FOR_3_MOD_CLK)
    }
    #[doc = "5 System clocks."]
    #[inline(always)]
    pub fn for_5_mod_clk(self) -> &'a mut W {
        self.variant(SSIACT_A::FOR_5_MOD_CLK)
    }
    #[doc = "9 System clocks."]
    #[inline(always)]
    pub fn for_9_mod_clk(self) -> &'a mut W {
        self.variant(SSIACT_A::FOR_9_MOD_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Defines SPI Mode, Only valid values are 0 and 3."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Slave Select Polarity."]
    #[inline(always)]
    pub fn sspol(&self) -> SSPOL_R {
        SSPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Slave Select. Only valid value is zero."]
    #[inline(always)]
    pub fn ssel(&self) -> SSEL_R {
        SSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Number of system clocks that SCLK will be low when SCLK pulses are generated."]
    #[inline(always)]
    pub fn lo_clk(&self) -> LO_CLK_R {
        LO_CLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of system clocks that SCLK will be high when SCLK pulses are generated."]
    #[inline(always)]
    pub fn hi_clk(&self) -> HI_CLK_R {
        HI_CLK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Slave Select Active Timing."]
    #[inline(always)]
    pub fn ssact(&self) -> SSACT_R {
        SSACT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Slave Select Inactive Timing."]
    #[inline(always)]
    pub fn ssiact(&self) -> SSIACT_R {
        SSIACT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines SPI Mode, Only valid values are 0 and 3."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 2 - Slave Select Polarity."]
    #[inline(always)]
    pub fn sspol(&mut self) -> SSPOL_W {
        SSPOL_W { w: self }
    }
    #[doc = "Bits 4:6 - Slave Select. Only valid value is zero."]
    #[inline(always)]
    pub fn ssel(&mut self) -> SSEL_W {
        SSEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Number of system clocks that SCLK will be low when SCLK pulses are generated."]
    #[inline(always)]
    pub fn lo_clk(&mut self) -> LO_CLK_W {
        LO_CLK_W { w: self }
    }
    #[doc = "Bits 12:15 - Number of system clocks that SCLK will be high when SCLK pulses are generated."]
    #[inline(always)]
    pub fn hi_clk(&mut self) -> HI_CLK_W {
        HI_CLK_W { w: self }
    }
    #[doc = "Bits 16:17 - Slave Select Active Timing."]
    #[inline(always)]
    pub fn ssact(&mut self) -> SSACT_W {
        SSACT_W { w: self }
    }
    #[doc = "Bits 18:19 - Slave Select Inactive Timing."]
    #[inline(always)]
    pub fn ssiact(&mut self) -> SSIACT_W {
        SSIACT_W { w: self }
    }
}
