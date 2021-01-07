#[doc = "Reader of register ctrl3"]
pub type R = crate::R<u32, super::CTRL3>;
#[doc = "Writer for register ctrl3"]
pub type W = crate::W<u32, super::CTRL3>;
#[doc = "Register ctrl3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
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
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
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
#[doc = "Invert SCLK Feedback in Master Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_FB_INV_A {
    #[doc = "0: SCLK is not inverted to Line Receiver."]
    NON_INV = 0,
    #[doc = "1: SCLK is inverted to Line Receiver."]
    INV = 1,
}
impl From<SCLK_FB_INV_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_FB_INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLK_FB_INV`"]
pub type SCLK_FB_INV_R = crate::R<bool, SCLK_FB_INV_A>;
impl SCLK_FB_INV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_FB_INV_A {
        match self.bits {
            false => SCLK_FB_INV_A::NON_INV,
            true => SCLK_FB_INV_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NON_INV`"]
    #[inline(always)]
    pub fn is_non_inv(&self) -> bool {
        *self == SCLK_FB_INV_A::NON_INV
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == SCLK_FB_INV_A::INV
    }
}
#[doc = "Write proxy for field `SCLK_FB_INV`"]
pub struct SCLK_FB_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_FB_INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_FB_INV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SCLK is not inverted to Line Receiver."]
    #[inline(always)]
    pub fn non_inv(self) -> &'a mut W {
        self.variant(SCLK_FB_INV_A::NON_INV)
    }
    #[doc = "SCLK is inverted to Line Receiver."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(SCLK_FB_INV_A::INV)
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
pub enum SSPOL_A {
    #[doc = "1: SS0 active high."]
    SS0_HIGH = 1,
    #[doc = "2: SS1 active high."]
    SS1_HIGH = 2,
    #[doc = "4: SS2 active high."]
    SS2_HIGH = 4,
    #[doc = "8: SS3 active high."]
    SS3_HIGH = 8,
    #[doc = "16: SS4 active high."]
    SS4_HIGH = 16,
    #[doc = "32: SS5 active high."]
    SS5_HIGH = 32,
    #[doc = "64: SS6 active high."]
    SS6_HIGH = 64,
    #[doc = "128: SS7 active high."]
    SS7_HIGH = 128,
}
impl From<SSPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SSPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSPOL`"]
pub type SSPOL_R = crate::R<u8, SSPOL_A>;
impl SSPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSPOL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SSPOL_A::SS0_HIGH),
            2 => Val(SSPOL_A::SS1_HIGH),
            4 => Val(SSPOL_A::SS2_HIGH),
            8 => Val(SSPOL_A::SS3_HIGH),
            16 => Val(SSPOL_A::SS4_HIGH),
            32 => Val(SSPOL_A::SS5_HIGH),
            64 => Val(SSPOL_A::SS6_HIGH),
            128 => Val(SSPOL_A::SS7_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SS0_HIGH`"]
    #[inline(always)]
    pub fn is_ss0_high(&self) -> bool {
        *self == SSPOL_A::SS0_HIGH
    }
    #[doc = "Checks if the value of the field is `SS1_HIGH`"]
    #[inline(always)]
    pub fn is_ss1_high(&self) -> bool {
        *self == SSPOL_A::SS1_HIGH
    }
    #[doc = "Checks if the value of the field is `SS2_HIGH`"]
    #[inline(always)]
    pub fn is_ss2_high(&self) -> bool {
        *self == SSPOL_A::SS2_HIGH
    }
    #[doc = "Checks if the value of the field is `SS3_HIGH`"]
    #[inline(always)]
    pub fn is_ss3_high(&self) -> bool {
        *self == SSPOL_A::SS3_HIGH
    }
    #[doc = "Checks if the value of the field is `SS4_HIGH`"]
    #[inline(always)]
    pub fn is_ss4_high(&self) -> bool {
        *self == SSPOL_A::SS4_HIGH
    }
    #[doc = "Checks if the value of the field is `SS5_HIGH`"]
    #[inline(always)]
    pub fn is_ss5_high(&self) -> bool {
        *self == SSPOL_A::SS5_HIGH
    }
    #[doc = "Checks if the value of the field is `SS6_HIGH`"]
    #[inline(always)]
    pub fn is_ss6_high(&self) -> bool {
        *self == SSPOL_A::SS6_HIGH
    }
    #[doc = "Checks if the value of the field is `SS7_HIGH`"]
    #[inline(always)]
    pub fn is_ss7_high(&self) -> bool {
        *self == SSPOL_A::SS7_HIGH
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SS0 active high."]
    #[inline(always)]
    pub fn ss0_high(self) -> &'a mut W {
        self.variant(SSPOL_A::SS0_HIGH)
    }
    #[doc = "SS1 active high."]
    #[inline(always)]
    pub fn ss1_high(self) -> &'a mut W {
        self.variant(SSPOL_A::SS1_HIGH)
    }
    #[doc = "SS2 active high."]
    #[inline(always)]
    pub fn ss2_high(self) -> &'a mut W {
        self.variant(SSPOL_A::SS2_HIGH)
    }
    #[doc = "SS3 active high."]
    #[inline(always)]
    pub fn ss3_high(self) -> &'a mut W {
        self.variant(SSPOL_A::SS3_HIGH)
    }
    #[doc = "SS4 active high."]
    #[inline(always)]
    pub fn ss4_high(self) -> &'a mut W {
        self.variant(SSPOL_A::SS4_HIGH)
    }
    #[doc = "SS5 active high."]
    #[inline(always)]
    pub fn ss5_high(self) -> &'a mut W {
        self.variant(SSPOL_A::SS5_HIGH)
    }
    #[doc = "SS6 active high."]
    #[inline(always)]
    pub fn ss6_high(self) -> &'a mut W {
        self.variant(SSPOL_A::SS6_HIGH)
    }
    #[doc = "SS7 active high."]
    #[inline(always)]
    pub fn ss7_high(self) -> &'a mut W {
        self.variant(SSPOL_A::SS7_HIGH)
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
    #[doc = "Bit 4 - Invert SCLK Feedback in Master Mode."]
    #[inline(always)]
    pub fn sclk_fb_inv(&self) -> SCLK_FB_INV_R {
        SCLK_FB_INV_R::new(((self.bits >> 4) & 0x01) != 0)
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
    pub fn sspol(&self) -> SSPOL_R {
        SSPOL_R::new(((self.bits >> 16) & 0xff) as u8)
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
    #[doc = "Bit 4 - Invert SCLK Feedback in Master Mode."]
    #[inline(always)]
    pub fn sclk_fb_inv(&mut self) -> SCLK_FB_INV_W {
        SCLK_FB_INV_W { w: self }
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
    pub fn sspol(&mut self) -> SSPOL_W {
        SSPOL_W { w: self }
    }
    #[doc = "Bits 24:31 - Slave Ready Polarity, each Slave Ready can have unique polarity."]
    #[inline(always)]
    pub fn srpol(&mut self) -> SRPOL_W {
        SRPOL_W { w: self }
    }
}
