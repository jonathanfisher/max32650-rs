#[doc = "Reader of register CTRL2"]
pub type R = crate::R<u32, super::CTRL2>;
#[doc = "Writer for register CTRL2"]
pub type W = crate::W<u32, super::CTRL2>;
#[doc = "Register CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    RISING_EDGE = 0,
    #[doc = "1: Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    FALLING_EDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::RISING_EDGE,
            true => CPHA_A::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CPHA_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CPHA_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CPHA_A::RISING_EDGE)
    }
    #[doc = "Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CPHA_A::FALLING_EDGE)
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
#[doc = "Clock Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    NORMAL = 0,
    #[doc = "1: Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    INVERTED = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::NORMAL,
            true => CPOL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CPOL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CPOL_A::INVERTED
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CPOL_A::NORMAL)
    }
    #[doc = "Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CPOL_A::INVERTED)
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
#[doc = "Reader of field `SCLK_INV`"]
pub type SCLK_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_INV`"]
pub struct SCLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_INV_W<'a> {
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
#[doc = "Number of Bits per character.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NUMBITS_A {
    #[doc = "0: 16 bits per character."]
    _0 = 0,
}
impl From<NUMBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMBITS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NUMBITS`"]
pub type NUMBITS_R = crate::R<u8, NUMBITS_A>;
impl NUMBITS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NUMBITS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NUMBITS_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NUMBITS_A::_0
    }
}
#[doc = "Write proxy for field `NUMBITS`"]
pub struct NUMBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMBITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NUMBITS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NUMBITS_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "SPI Data width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA_WIDTH_A {
    #[doc = "0: 1 data pin."]
    MONO = 0,
    #[doc = "1: 2 data pins."]
    DUAL = 1,
    #[doc = "2: 4 data pins."]
    QUAD = 2,
}
impl From<DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATA_WIDTH`"]
pub type DATA_WIDTH_R = crate::R<u8, DATA_WIDTH_A>;
impl DATA_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATA_WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATA_WIDTH_A::MONO),
            1 => Val(DATA_WIDTH_A::DUAL),
            2 => Val(DATA_WIDTH_A::QUAD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == DATA_WIDTH_A::MONO
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DATA_WIDTH_A::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == DATA_WIDTH_A::QUAD
    }
}
#[doc = "Write proxy for field `DATA_WIDTH`"]
pub struct DATA_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data pin."]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::MONO)
    }
    #[doc = "2 data pins."]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::DUAL)
    }
    #[doc = "4 data pins."]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::QUAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Three Wire mode. MOSI/MISO pin(s) shared. Only Mono mode suports Four-Wire.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREE_WIRE_A {
    #[doc = "0: Use four wire mode (Mono only)."]
    DIS = 0,
    #[doc = "1: Use three wire mode."]
    EN = 1,
}
impl From<THREE_WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: THREE_WIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `THREE_WIRE`"]
pub type THREE_WIRE_R = crate::R<bool, THREE_WIRE_A>;
impl THREE_WIRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREE_WIRE_A {
        match self.bits {
            false => THREE_WIRE_A::DIS,
            true => THREE_WIRE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == THREE_WIRE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == THREE_WIRE_A::EN
    }
}
#[doc = "Write proxy for field `THREE_WIRE`"]
pub struct THREE_WIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> THREE_WIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREE_WIRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use four wire mode (Mono only)."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(THREE_WIRE_A::DIS)
    }
    #[doc = "Use three wire mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(THREE_WIRE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Slave Select Polarity, each Slave Select can have unique polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SS_POL_A {
    #[doc = "1: SS0 active high."]
    SS0_HIGH = 1,
    #[doc = "2: SS1 active high."]
    SS1_HIGH = 2,
    #[doc = "4: SS2 active high."]
    SS2_HIGH = 4,
    #[doc = "8: SS3 active high."]
    SS3_HIGH = 8,
}
impl From<SS_POL_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_POL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SS_POL`"]
pub type SS_POL_R = crate::R<u8, SS_POL_A>;
impl SS_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SS_POL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SS_POL_A::SS0_HIGH),
            2 => Val(SS_POL_A::SS1_HIGH),
            4 => Val(SS_POL_A::SS2_HIGH),
            8 => Val(SS_POL_A::SS3_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SS0_HIGH`"]
    #[inline(always)]
    pub fn is_ss0_high(&self) -> bool {
        *self == SS_POL_A::SS0_HIGH
    }
    #[doc = "Checks if the value of the field is `SS1_HIGH`"]
    #[inline(always)]
    pub fn is_ss1_high(&self) -> bool {
        *self == SS_POL_A::SS1_HIGH
    }
    #[doc = "Checks if the value of the field is `SS2_HIGH`"]
    #[inline(always)]
    pub fn is_ss2_high(&self) -> bool {
        *self == SS_POL_A::SS2_HIGH
    }
    #[doc = "Checks if the value of the field is `SS3_HIGH`"]
    #[inline(always)]
    pub fn is_ss3_high(&self) -> bool {
        *self == SS_POL_A::SS3_HIGH
    }
}
#[doc = "Write proxy for field `SS_POL`"]
pub struct SS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_POL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SS0 active high."]
    #[inline(always)]
    pub fn ss0_high(self) -> &'a mut W {
        self.variant(SS_POL_A::SS0_HIGH)
    }
    #[doc = "SS1 active high."]
    #[inline(always)]
    pub fn ss1_high(self) -> &'a mut W {
        self.variant(SS_POL_A::SS1_HIGH)
    }
    #[doc = "SS2 active high."]
    #[inline(always)]
    pub fn ss2_high(self) -> &'a mut W {
        self.variant(SS_POL_A::SS2_HIGH)
    }
    #[doc = "SS3 active high."]
    #[inline(always)]
    pub fn ss3_high(self) -> &'a mut W {
        self.variant(SS_POL_A::SS3_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Slave Ready Polarity, each Slave Ready can have unique polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRPOL_A {
    #[doc = "1: SR0 active high."]
    SR0_HIGH = 1,
    #[doc = "2: SR1 active high."]
    SR1_HIGH = 2,
    #[doc = "4: SR2 active high."]
    SR2_HIGH = 4,
    #[doc = "8: SR3 active high."]
    SR3_HIGH = 8,
    #[doc = "16: SR4 active high."]
    SR4_HIGH = 16,
    #[doc = "32: SR5 active high."]
    SR5_HIGH = 32,
    #[doc = "64: SR6 active high."]
    SR6_HIGH = 64,
    #[doc = "128: SR7 active high."]
    SR7_HIGH = 128,
}
impl From<SRPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SRPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRPOL`"]
pub type SRPOL_R = crate::R<u8, SRPOL_A>;
impl SRPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRPOL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SRPOL_A::SR0_HIGH),
            2 => Val(SRPOL_A::SR1_HIGH),
            4 => Val(SRPOL_A::SR2_HIGH),
            8 => Val(SRPOL_A::SR3_HIGH),
            16 => Val(SRPOL_A::SR4_HIGH),
            32 => Val(SRPOL_A::SR5_HIGH),
            64 => Val(SRPOL_A::SR6_HIGH),
            128 => Val(SRPOL_A::SR7_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR0_HIGH`"]
    #[inline(always)]
    pub fn is_sr0_high(&self) -> bool {
        *self == SRPOL_A::SR0_HIGH
    }
    #[doc = "Checks if the value of the field is `SR1_HIGH`"]
    #[inline(always)]
    pub fn is_sr1_high(&self) -> bool {
        *self == SRPOL_A::SR1_HIGH
    }
    #[doc = "Checks if the value of the field is `SR2_HIGH`"]
    #[inline(always)]
    pub fn is_sr2_high(&self) -> bool {
        *self == SRPOL_A::SR2_HIGH
    }
    #[doc = "Checks if the value of the field is `SR3_HIGH`"]
    #[inline(always)]
    pub fn is_sr3_high(&self) -> bool {
        *self == SRPOL_A::SR3_HIGH
    }
    #[doc = "Checks if the value of the field is `SR4_HIGH`"]
    #[inline(always)]
    pub fn is_sr4_high(&self) -> bool {
        *self == SRPOL_A::SR4_HIGH
    }
    #[doc = "Checks if the value of the field is `SR5_HIGH`"]
    #[inline(always)]
    pub fn is_sr5_high(&self) -> bool {
        *self == SRPOL_A::SR5_HIGH
    }
    #[doc = "Checks if the value of the field is `SR6_HIGH`"]
    #[inline(always)]
    pub fn is_sr6_high(&self) -> bool {
        *self == SRPOL_A::SR6_HIGH
    }
    #[doc = "Checks if the value of the field is `SR7_HIGH`"]
    #[inline(always)]
    pub fn is_sr7_high(&self) -> bool {
        *self == SRPOL_A::SR7_HIGH
    }
}
#[doc = "Write proxy for field `SRPOL`"]
pub struct SRPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRPOL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SR0 active high."]
    #[inline(always)]
    pub fn sr0_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR0_HIGH)
    }
    #[doc = "SR1 active high."]
    #[inline(always)]
    pub fn sr1_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR1_HIGH)
    }
    #[doc = "SR2 active high."]
    #[inline(always)]
    pub fn sr2_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR2_HIGH)
    }
    #[doc = "SR3 active high."]
    #[inline(always)]
    pub fn sr3_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR3_HIGH)
    }
    #[doc = "SR4 active high."]
    #[inline(always)]
    pub fn sr4_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR4_HIGH)
    }
    #[doc = "SR5 active high."]
    #[inline(always)]
    pub fn sr5_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR5_HIGH)
    }
    #[doc = "SR6 active high."]
    #[inline(always)]
    pub fn sr6_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR6_HIGH)
    }
    #[doc = "SR7 active high."]
    #[inline(always)]
    pub fn sr7_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR7_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reserved - Must Always Be Cleared to 0."]
    #[inline(always)]
    pub fn sclk_inv(&self) -> SCLK_INV_R {
        SCLK_INV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    pub fn numbits(&self) -> NUMBITS_R {
        NUMBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Three Wire mode. MOSI/MISO pin(s) shared. Only Mono mode suports Four-Wire."]
    #[inline(always)]
    pub fn three_wire(&self) -> THREE_WIRE_R {
        THREE_WIRE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    pub fn ss_pol(&self) -> SS_POL_R {
        SS_POL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Slave Ready Polarity, each Slave Ready can have unique polarity."]
    #[inline(always)]
    pub fn srpol(&self) -> SRPOL_R {
        SRPOL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 4 - Reserved - Must Always Be Cleared to 0."]
    #[inline(always)]
    pub fn sclk_inv(&mut self) -> SCLK_INV_W {
        SCLK_INV_W { w: self }
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    pub fn numbits(&mut self) -> NUMBITS_W {
        NUMBITS_W { w: self }
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W { w: self }
    }
    #[doc = "Bit 15 - Three Wire mode. MOSI/MISO pin(s) shared. Only Mono mode suports Four-Wire."]
    #[inline(always)]
    pub fn three_wire(&mut self) -> THREE_WIRE_W {
        THREE_WIRE_W { w: self }
    }
    #[doc = "Bits 16:23 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    pub fn ss_pol(&mut self) -> SS_POL_W {
        SS_POL_W { w: self }
    }
    #[doc = "Bits 24:31 - Slave Ready Polarity, each Slave Ready can have unique polarity."]
    #[inline(always)]
    pub fn srpol(&mut self) -> SRPOL_W {
        SRPOL_W { w: self }
    }
}
