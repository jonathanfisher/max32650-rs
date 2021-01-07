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
#[doc = "Slaves Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSEL_A {
    #[doc = "0: Slave 0 is selected."]
    SLAVE_0 = 0,
    #[doc = "1: Slave 1 is selected."]
    SLAVE_1 = 1,
}
impl From<SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSEL`"]
pub type SSEL_R = crate::R<u8, SSEL_A>;
impl SSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSEL_A::SLAVE_0),
            1 => Val(SSEL_A::SLAVE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_0`"]
    #[inline(always)]
    pub fn is_slave_0(&self) -> bool {
        *self == SSEL_A::SLAVE_0
    }
    #[doc = "Checks if the value of the field is `SLAVE_1`"]
    #[inline(always)]
    pub fn is_slave_1(&self) -> bool {
        *self == SSEL_A::SLAVE_1
    }
}
#[doc = "Write proxy for field `SSEL`"]
pub struct SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slave 0 is selected."]
    #[inline(always)]
    pub fn slave_0(self) -> &'a mut W {
        self.variant(SSEL_A::SLAVE_0)
    }
    #[doc = "Slave 1 is selected."]
    #[inline(always)]
    pub fn slave_1(self) -> &'a mut W {
        self.variant(SSEL_A::SLAVE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Defines SPI Mode, Only valid values are 0 and 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: SPIX Mode 0. CLK Polarity = 0, CLK Phase = 0."]
    SPIX_MODE_0 = 0,
    #[doc = "3: SPIX Mode 3. CLK Polarity = 1, CLK Phase = 1."]
    SPIX_MODE_3 = 3,
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
            0 => Val(MODE_A::SPIX_MODE_0),
            3 => Val(MODE_A::SPIX_MODE_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPIX_MODE_0`"]
    #[inline(always)]
    pub fn is_spix_mode_0(&self) -> bool {
        *self == MODE_A::SPIX_MODE_0
    }
    #[doc = "Checks if the value of the field is `SPIX_MODE_3`"]
    #[inline(always)]
    pub fn is_spix_mode_3(&self) -> bool {
        *self == MODE_A::SPIX_MODE_3
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
    #[doc = "SPIX Mode 0. CLK Polarity = 0, CLK Phase = 0."]
    #[inline(always)]
    pub fn spix_mode_0(self) -> &'a mut W {
        self.variant(MODE_A::SPIX_MODE_0)
    }
    #[doc = "SPIX Mode 3. CLK Polarity = 1, CLK Phase = 1."]
    #[inline(always)]
    pub fn spix_mode_3(self) -> &'a mut W {
        self.variant(MODE_A::SPIX_MODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Page Size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAGE_SIZE_A {
    #[doc = "0: 4 bytes."]
    _4_BYTES = 0,
    #[doc = "1: 8 bytes."]
    _8_BYTES = 1,
    #[doc = "2: 16 bytes."]
    _16_BYTES = 2,
    #[doc = "3: 32 bytes."]
    _32_BYTES = 3,
}
impl From<PAGE_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PAGE_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAGE_SIZE`"]
pub type PAGE_SIZE_R = crate::R<u8, PAGE_SIZE_A>;
impl PAGE_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGE_SIZE_A {
        match self.bits {
            0 => PAGE_SIZE_A::_4_BYTES,
            1 => PAGE_SIZE_A::_8_BYTES,
            2 => PAGE_SIZE_A::_16_BYTES,
            3 => PAGE_SIZE_A::_32_BYTES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4_BYTES`"]
    #[inline(always)]
    pub fn is_4_bytes(&self) -> bool {
        *self == PAGE_SIZE_A::_4_BYTES
    }
    #[doc = "Checks if the value of the field is `_8_BYTES`"]
    #[inline(always)]
    pub fn is_8_bytes(&self) -> bool {
        *self == PAGE_SIZE_A::_8_BYTES
    }
    #[doc = "Checks if the value of the field is `_16_BYTES`"]
    #[inline(always)]
    pub fn is_16_bytes(&self) -> bool {
        *self == PAGE_SIZE_A::_16_BYTES
    }
    #[doc = "Checks if the value of the field is `_32_BYTES`"]
    #[inline(always)]
    pub fn is_32_bytes(&self) -> bool {
        *self == PAGE_SIZE_A::_32_BYTES
    }
}
#[doc = "Write proxy for field `PAGE_SIZE`"]
pub struct PAGE_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE_SIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 bytes."]
    #[inline(always)]
    pub fn _4_bytes(self) -> &'a mut W {
        self.variant(PAGE_SIZE_A::_4_BYTES)
    }
    #[doc = "8 bytes."]
    #[inline(always)]
    pub fn _8_bytes(self) -> &'a mut W {
        self.variant(PAGE_SIZE_A::_8_BYTES)
    }
    #[doc = "16 bytes."]
    #[inline(always)]
    pub fn _16_bytes(self) -> &'a mut W {
        self.variant(PAGE_SIZE_A::_16_BYTES)
    }
    #[doc = "32 bytes."]
    #[inline(always)]
    pub fn _32_bytes(self) -> &'a mut W {
        self.variant(PAGE_SIZE_A::_32_BYTES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "SCLK High Clocks. Number of system clocks that SCLK will be high when SCLK pulses are generated. 0 Correspond to 16 system clocks and, all other values defines the number of system clock taht SCLK will be held high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HI_CLK_A {
    #[doc = "0: 16 system clocks."]
    _16_SCLK = 0,
}
impl From<HI_CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: HI_CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HI_CLK`"]
pub type HI_CLK_R = crate::R<u8, HI_CLK_A>;
impl HI_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HI_CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HI_CLK_A::_16_SCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16_SCLK`"]
    #[inline(always)]
    pub fn is_16_sclk(&self) -> bool {
        *self == HI_CLK_A::_16_SCLK
    }
}
#[doc = "Write proxy for field `HI_CLK`"]
pub struct HI_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> HI_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HI_CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16 system clocks."]
    #[inline(always)]
    pub fn _16_sclk(self) -> &'a mut W {
        self.variant(HI_CLK_A::_16_SCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "SCLK low Clocks. Number of system clocks that SCLK will be low when SCLK pulses are generated. 0 correspond to 16 system clocks and, all other values defines the number of system clock taht SCLK will be held low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LO_CLK_A {
    #[doc = "0: 16 system clocks."]
    _16_SCLK = 0,
}
impl From<LO_CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: LO_CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LO_CLK`"]
pub type LO_CLK_R = crate::R<u8, LO_CLK_A>;
impl LO_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LO_CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LO_CLK_A::_16_SCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16_SCLK`"]
    #[inline(always)]
    pub fn is_16_sclk(&self) -> bool {
        *self == LO_CLK_A::_16_SCLK
    }
}
#[doc = "Write proxy for field `LO_CLK`"]
pub struct LO_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LO_CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16 system clocks."]
    #[inline(always)]
    pub fn _16_sclk(self) -> &'a mut W {
        self.variant(LO_CLK_A::_16_SCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Slaves Select Activate Timing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SS_ACT_A {
    #[doc = "0: 0 sytem clocks."]
    _0_CLKS = 0,
    #[doc = "1: 2 sytem clocks."]
    _2_CLKS = 1,
    #[doc = "2: 4 sytem clocks."]
    _4_CLKS = 2,
    #[doc = "3: 8 sytem clocks."]
    _8_CLKS = 3,
}
impl From<SS_ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_ACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SS_ACT`"]
pub type SS_ACT_R = crate::R<u8, SS_ACT_A>;
impl SS_ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_ACT_A {
        match self.bits {
            0 => SS_ACT_A::_0_CLKS,
            1 => SS_ACT_A::_2_CLKS,
            2 => SS_ACT_A::_4_CLKS,
            3 => SS_ACT_A::_8_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_CLKS`"]
    #[inline(always)]
    pub fn is_0_clks(&self) -> bool {
        *self == SS_ACT_A::_0_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_CLKS`"]
    #[inline(always)]
    pub fn is_2_clks(&self) -> bool {
        *self == SS_ACT_A::_2_CLKS
    }
    #[doc = "Checks if the value of the field is `_4_CLKS`"]
    #[inline(always)]
    pub fn is_4_clks(&self) -> bool {
        *self == SS_ACT_A::_4_CLKS
    }
    #[doc = "Checks if the value of the field is `_8_CLKS`"]
    #[inline(always)]
    pub fn is_8_clks(&self) -> bool {
        *self == SS_ACT_A::_8_CLKS
    }
}
#[doc = "Write proxy for field `SS_ACT`"]
pub struct SS_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_ACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0 sytem clocks."]
    #[inline(always)]
    pub fn _0_clks(self) -> &'a mut W {
        self.variant(SS_ACT_A::_0_CLKS)
    }
    #[doc = "2 sytem clocks."]
    #[inline(always)]
    pub fn _2_clks(self) -> &'a mut W {
        self.variant(SS_ACT_A::_2_CLKS)
    }
    #[doc = "4 sytem clocks."]
    #[inline(always)]
    pub fn _4_clks(self) -> &'a mut W {
        self.variant(SS_ACT_A::_4_CLKS)
    }
    #[doc = "8 sytem clocks."]
    #[inline(always)]
    pub fn _8_clks(self) -> &'a mut W {
        self.variant(SS_ACT_A::_8_CLKS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Slaves Select Inactive Timing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SS_INACT_A {
    #[doc = "0: 4 sytem clocks."]
    _4_CLKS = 0,
    #[doc = "1: 6 sytem clocks."]
    _6_CLKS = 1,
    #[doc = "2: 8 sytem clocks."]
    _8_CLKS = 2,
    #[doc = "3: 12 sytem clocks."]
    _12_CLKS = 3,
}
impl From<SS_INACT_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_INACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SS_INACT`"]
pub type SS_INACT_R = crate::R<u8, SS_INACT_A>;
impl SS_INACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_INACT_A {
        match self.bits {
            0 => SS_INACT_A::_4_CLKS,
            1 => SS_INACT_A::_6_CLKS,
            2 => SS_INACT_A::_8_CLKS,
            3 => SS_INACT_A::_12_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4_CLKS`"]
    #[inline(always)]
    pub fn is_4_clks(&self) -> bool {
        *self == SS_INACT_A::_4_CLKS
    }
    #[doc = "Checks if the value of the field is `_6_CLKS`"]
    #[inline(always)]
    pub fn is_6_clks(&self) -> bool {
        *self == SS_INACT_A::_6_CLKS
    }
    #[doc = "Checks if the value of the field is `_8_CLKS`"]
    #[inline(always)]
    pub fn is_8_clks(&self) -> bool {
        *self == SS_INACT_A::_8_CLKS
    }
    #[doc = "Checks if the value of the field is `_12_CLKS`"]
    #[inline(always)]
    pub fn is_12_clks(&self) -> bool {
        *self == SS_INACT_A::_12_CLKS
    }
}
#[doc = "Write proxy for field `SS_INACT`"]
pub struct SS_INACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_INACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_INACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 sytem clocks."]
    #[inline(always)]
    pub fn _4_clks(self) -> &'a mut W {
        self.variant(SS_INACT_A::_4_CLKS)
    }
    #[doc = "6 sytem clocks."]
    #[inline(always)]
    pub fn _6_clks(self) -> &'a mut W {
        self.variant(SS_INACT_A::_6_CLKS)
    }
    #[doc = "8 sytem clocks."]
    #[inline(always)]
    pub fn _8_clks(self) -> &'a mut W {
        self.variant(SS_INACT_A::_8_CLKS)
    }
    #[doc = "12 sytem clocks."]
    #[inline(always)]
    pub fn _12_clks(self) -> &'a mut W {
        self.variant(SS_INACT_A::_12_CLKS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Slaves Select."]
    #[inline(always)]
    pub fn ssel(&self) -> SSEL_R {
        SSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Defines SPI Mode, Only valid values are 0 and 3."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Page Size."]
    #[inline(always)]
    pub fn page_size(&self) -> PAGE_SIZE_R {
        PAGE_SIZE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - SCLK High Clocks. Number of system clocks that SCLK will be high when SCLK pulses are generated. 0 Correspond to 16 system clocks and, all other values defines the number of system clock taht SCLK will be held high."]
    #[inline(always)]
    pub fn hi_clk(&self) -> HI_CLK_R {
        HI_CLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SCLK low Clocks. Number of system clocks that SCLK will be low when SCLK pulses are generated. 0 correspond to 16 system clocks and, all other values defines the number of system clock taht SCLK will be held low."]
    #[inline(always)]
    pub fn lo_clk(&self) -> LO_CLK_R {
        LO_CLK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Slaves Select Activate Timing."]
    #[inline(always)]
    pub fn ss_act(&self) -> SS_ACT_R {
        SS_ACT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Slaves Select Inactive Timing."]
    #[inline(always)]
    pub fn ss_inact(&self) -> SS_INACT_R {
        SS_INACT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slaves Select."]
    #[inline(always)]
    pub fn ssel(&mut self) -> SSEL_W {
        SSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Defines SPI Mode, Only valid values are 0 and 3."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Page Size."]
    #[inline(always)]
    pub fn page_size(&mut self) -> PAGE_SIZE_W {
        PAGE_SIZE_W { w: self }
    }
    #[doc = "Bits 8:11 - SCLK High Clocks. Number of system clocks that SCLK will be high when SCLK pulses are generated. 0 Correspond to 16 system clocks and, all other values defines the number of system clock taht SCLK will be held high."]
    #[inline(always)]
    pub fn hi_clk(&mut self) -> HI_CLK_W {
        HI_CLK_W { w: self }
    }
    #[doc = "Bits 12:15 - SCLK low Clocks. Number of system clocks that SCLK will be low when SCLK pulses are generated. 0 correspond to 16 system clocks and, all other values defines the number of system clock taht SCLK will be held low."]
    #[inline(always)]
    pub fn lo_clk(&mut self) -> LO_CLK_W {
        LO_CLK_W { w: self }
    }
    #[doc = "Bits 16:17 - Slaves Select Activate Timing."]
    #[inline(always)]
    pub fn ss_act(&mut self) -> SS_ACT_W {
        SS_ACT_W { w: self }
    }
    #[doc = "Bits 18:19 - Slaves Select Inactive Timing."]
    #[inline(always)]
    pub fn ss_inact(&mut self) -> SS_INACT_W {
        SS_INACT_W { w: self }
    }
}
