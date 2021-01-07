#[doc = "Reader of register GEN_CTRL"]
pub type R = crate::R<u32, super::GEN_CTRL>;
#[doc = "Writer for register GEN_CTRL"]
pub type W = crate::W<u32, super::GEN_CTRL>;
#[doc = "Register GEN_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::GEN_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SPI Master enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable SPI Master, putting a reset state."]
    DIS = 0,
    #[doc = "1: Enable SPI Master for processing transactions."]
    EN = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DIS,
            true => ENABLE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENABLE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ENABLE_A::EN
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable SPI Master, putting a reset state."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENABLE_A::DIS)
    }
    #[doc = "Enable SPI Master for processing transactions."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ENABLE_A::EN)
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
#[doc = "Transaction FIFO Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_EN_A {
    #[doc = "0: Disable Transaction FIFO."]
    DIS_TXFIFO = 0,
    #[doc = "1: Enable Transaction FIFO."]
    EN_TXFIFO = 1,
}
impl From<TX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_FIFO_EN`"]
pub type TX_FIFO_EN_R = crate::R<bool, TX_FIFO_EN_A>;
impl TX_FIFO_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFO_EN_A {
        match self.bits {
            false => TX_FIFO_EN_A::DIS_TXFIFO,
            true => TX_FIFO_EN_A::EN_TXFIFO,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_TXFIFO`"]
    #[inline(always)]
    pub fn is_dis_txfifo(&self) -> bool {
        *self == TX_FIFO_EN_A::DIS_TXFIFO
    }
    #[doc = "Checks if the value of the field is `EN_TXFIFO`"]
    #[inline(always)]
    pub fn is_en_txfifo(&self) -> bool {
        *self == TX_FIFO_EN_A::EN_TXFIFO
    }
}
#[doc = "Write proxy for field `TX_FIFO_EN`"]
pub struct TX_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FIFO_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Transaction FIFO."]
    #[inline(always)]
    pub fn dis_txfifo(self) -> &'a mut W {
        self.variant(TX_FIFO_EN_A::DIS_TXFIFO)
    }
    #[doc = "Enable Transaction FIFO."]
    #[inline(always)]
    pub fn en_txfifo(self) -> &'a mut W {
        self.variant(TX_FIFO_EN_A::EN_TXFIFO)
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
#[doc = "Result FIFO Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_EN_A {
    #[doc = "0: Disable Result FIFO."]
    DIS_RXFIFO = 0,
    #[doc = "1: Enable Result FIFO."]
    EN_RXFIFO = 1,
}
impl From<RX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_FIFO_EN`"]
pub type RX_FIFO_EN_R = crate::R<bool, RX_FIFO_EN_A>;
impl RX_FIFO_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_EN_A {
        match self.bits {
            false => RX_FIFO_EN_A::DIS_RXFIFO,
            true => RX_FIFO_EN_A::EN_RXFIFO,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_RXFIFO`"]
    #[inline(always)]
    pub fn is_dis_rxfifo(&self) -> bool {
        *self == RX_FIFO_EN_A::DIS_RXFIFO
    }
    #[doc = "Checks if the value of the field is `EN_RXFIFO`"]
    #[inline(always)]
    pub fn is_en_rxfifo(&self) -> bool {
        *self == RX_FIFO_EN_A::EN_RXFIFO
    }
}
#[doc = "Write proxy for field `RX_FIFO_EN`"]
pub struct RX_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Result FIFO."]
    #[inline(always)]
    pub fn dis_rxfifo(self) -> &'a mut W {
        self.variant(RX_FIFO_EN_A::DIS_RXFIFO)
    }
    #[doc = "Enable Result FIFO."]
    #[inline(always)]
    pub fn en_rxfifo(self) -> &'a mut W {
        self.variant(RX_FIFO_EN_A::EN_RXFIFO)
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
#[doc = "Bit-Bang Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBMODE_A {
    #[doc = "0: Disable Bit-Bang Mode."]
    DIS = 0,
    #[doc = "1: Enable Bit-Bang Mode."]
    EN = 1,
}
impl From<BBMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BBMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BBMODE`"]
pub type BBMODE_R = crate::R<bool, BBMODE_A>;
impl BBMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BBMODE_A {
        match self.bits {
            false => BBMODE_A::DIS,
            true => BBMODE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BBMODE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BBMODE_A::EN
    }
}
#[doc = "Write proxy for field `BBMODE`"]
pub struct BBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BBMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BBMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Bit-Bang Mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BBMODE_A::DIS)
    }
    #[doc = "Enable Bit-Bang Mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BBMODE_A::EN)
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
#[doc = "This bits reflects the state of the currently selected slave select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDR_A {
    #[doc = "0: Selected Slave select output = 0."]
    OUTPUT0 = 0,
    #[doc = "1: Selected Slave select output = 1."]
    OUTPUT1 = 1,
}
impl From<SSDR_A> for bool {
    #[inline(always)]
    fn from(variant: SSDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSDR`"]
pub type SSDR_R = crate::R<bool, SSDR_A>;
impl SSDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSDR_A {
        match self.bits {
            false => SSDR_A::OUTPUT0,
            true => SSDR_A::OUTPUT1,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT0`"]
    #[inline(always)]
    pub fn is_output0(&self) -> bool {
        *self == SSDR_A::OUTPUT0
    }
    #[doc = "Checks if the value of the field is `OUTPUT1`"]
    #[inline(always)]
    pub fn is_output1(&self) -> bool {
        *self == SSDR_A::OUTPUT1
    }
}
#[doc = "Write proxy for field `SSDR`"]
pub struct SSDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected Slave select output = 0."]
    #[inline(always)]
    pub fn output0(self) -> &'a mut W {
        self.variant(SSDR_A::OUTPUT0)
    }
    #[doc = "Selected Slave select output = 1."]
    #[inline(always)]
    pub fn output1(self) -> &'a mut W {
        self.variant(SSDR_A::OUTPUT1)
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
#[doc = "SSCLK Drive and State.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_DR_A {
    #[doc = "0: SCLK is 0."]
    SCLK_0 = 0,
    #[doc = "1: SCLK is 1."]
    SCLK_1 = 1,
}
impl From<SCLK_DR_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_DR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLK_DR`"]
pub type SCLK_DR_R = crate::R<bool, SCLK_DR_A>;
impl SCLK_DR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_DR_A {
        match self.bits {
            false => SCLK_DR_A::SCLK_0,
            true => SCLK_DR_A::SCLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_0`"]
    #[inline(always)]
    pub fn is_sclk_0(&self) -> bool {
        *self == SCLK_DR_A::SCLK_0
    }
    #[doc = "Checks if the value of the field is `SCLK_1`"]
    #[inline(always)]
    pub fn is_sclk_1(&self) -> bool {
        *self == SCLK_DR_A::SCLK_1
    }
}
#[doc = "Write proxy for field `SCLK_DR`"]
pub struct SCLK_DR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_DR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SCLK is 0."]
    #[inline(always)]
    pub fn sclk_0(self) -> &'a mut W {
        self.variant(SCLK_DR_A::SCLK_0)
    }
    #[doc = "SCLK is 1."]
    #[inline(always)]
    pub fn sclk_1(self) -> &'a mut W {
        self.variant(SCLK_DR_A::SCLK_1)
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
#[doc = "SDIO Input Data Value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDIO_DATA_IN_A {
    #[doc = "0: SDIO\\[0\\]"]
    SDIO0 = 0,
    #[doc = "1: SDIO\\[1\\]"]
    SDIO1 = 1,
    #[doc = "2: SDIO\\[2\\]"]
    SDIO2 = 2,
    #[doc = "3: SDIO\\[3\\]"]
    SDIO3 = 3,
}
impl From<SDIO_DATA_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIO_DATA_IN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SDIO_DATA_IN`"]
pub type SDIO_DATA_IN_R = crate::R<u8, SDIO_DATA_IN_A>;
impl SDIO_DATA_IN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDIO_DATA_IN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SDIO_DATA_IN_A::SDIO0),
            1 => Val(SDIO_DATA_IN_A::SDIO1),
            2 => Val(SDIO_DATA_IN_A::SDIO2),
            3 => Val(SDIO_DATA_IN_A::SDIO3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDIO0`"]
    #[inline(always)]
    pub fn is_sdio0(&self) -> bool {
        *self == SDIO_DATA_IN_A::SDIO0
    }
    #[doc = "Checks if the value of the field is `SDIO1`"]
    #[inline(always)]
    pub fn is_sdio1(&self) -> bool {
        *self == SDIO_DATA_IN_A::SDIO1
    }
    #[doc = "Checks if the value of the field is `SDIO2`"]
    #[inline(always)]
    pub fn is_sdio2(&self) -> bool {
        *self == SDIO_DATA_IN_A::SDIO2
    }
    #[doc = "Checks if the value of the field is `SDIO3`"]
    #[inline(always)]
    pub fn is_sdio3(&self) -> bool {
        *self == SDIO_DATA_IN_A::SDIO3
    }
}
#[doc = "Write proxy for field `SDIO_DATA_IN`"]
pub struct SDIO_DATA_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DATA_IN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_DATA_IN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDIO\\[0\\]"]
    #[inline(always)]
    pub fn sdio0(self) -> &'a mut W {
        self.variant(SDIO_DATA_IN_A::SDIO0)
    }
    #[doc = "SDIO\\[1\\]"]
    #[inline(always)]
    pub fn sdio1(self) -> &'a mut W {
        self.variant(SDIO_DATA_IN_A::SDIO1)
    }
    #[doc = "SDIO\\[2\\]"]
    #[inline(always)]
    pub fn sdio2(self) -> &'a mut W {
        self.variant(SDIO_DATA_IN_A::SDIO2)
    }
    #[doc = "SDIO\\[3\\]"]
    #[inline(always)]
    pub fn sdio3(self) -> &'a mut W {
        self.variant(SDIO_DATA_IN_A::SDIO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "No description available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BB_DATA_A {
    #[doc = "0: SDIO\\[0\\]"]
    SDIO0 = 0,
    #[doc = "1: SDIO\\[1\\]"]
    SDIO1 = 1,
    #[doc = "2: SDIO\\[2\\]"]
    SDIO2 = 2,
    #[doc = "3: SDIO\\[3\\]"]
    SDIO3 = 3,
}
impl From<BB_DATA_A> for u8 {
    #[inline(always)]
    fn from(variant: BB_DATA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BB_DATA`"]
pub type BB_DATA_R = crate::R<u8, BB_DATA_A>;
impl BB_DATA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BB_DATA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BB_DATA_A::SDIO0),
            1 => Val(BB_DATA_A::SDIO1),
            2 => Val(BB_DATA_A::SDIO2),
            3 => Val(BB_DATA_A::SDIO3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDIO0`"]
    #[inline(always)]
    pub fn is_sdio0(&self) -> bool {
        *self == BB_DATA_A::SDIO0
    }
    #[doc = "Checks if the value of the field is `SDIO1`"]
    #[inline(always)]
    pub fn is_sdio1(&self) -> bool {
        *self == BB_DATA_A::SDIO1
    }
    #[doc = "Checks if the value of the field is `SDIO2`"]
    #[inline(always)]
    pub fn is_sdio2(&self) -> bool {
        *self == BB_DATA_A::SDIO2
    }
    #[doc = "Checks if the value of the field is `SDIO3`"]
    #[inline(always)]
    pub fn is_sdio3(&self) -> bool {
        *self == BB_DATA_A::SDIO3
    }
}
#[doc = "Write proxy for field `BB_DATA`"]
pub struct BB_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DATA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDIO\\[0\\]"]
    #[inline(always)]
    pub fn sdio0(self) -> &'a mut W {
        self.variant(BB_DATA_A::SDIO0)
    }
    #[doc = "SDIO\\[1\\]"]
    #[inline(always)]
    pub fn sdio1(self) -> &'a mut W {
        self.variant(BB_DATA_A::SDIO1)
    }
    #[doc = "SDIO\\[2\\]"]
    #[inline(always)]
    pub fn sdio2(self) -> &'a mut W {
        self.variant(BB_DATA_A::SDIO2)
    }
    #[doc = "SDIO\\[3\\]"]
    #[inline(always)]
    pub fn sdio3(self) -> &'a mut W {
        self.variant(BB_DATA_A::SDIO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Bit Bang SDIO Output Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BB_DATA_OUT_EN_A {
    #[doc = "0: SDIO\\[0\\]"]
    SDIO0 = 0,
    #[doc = "1: SDIO\\[1\\]"]
    SDIO1 = 1,
    #[doc = "2: SDIO\\[2\\]"]
    SDIO2 = 2,
    #[doc = "3: SDIO\\[3\\]"]
    SDIO3 = 3,
}
impl From<BB_DATA_OUT_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: BB_DATA_OUT_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BB_DATA_OUT_EN`"]
pub type BB_DATA_OUT_EN_R = crate::R<u8, BB_DATA_OUT_EN_A>;
impl BB_DATA_OUT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BB_DATA_OUT_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BB_DATA_OUT_EN_A::SDIO0),
            1 => Val(BB_DATA_OUT_EN_A::SDIO1),
            2 => Val(BB_DATA_OUT_EN_A::SDIO2),
            3 => Val(BB_DATA_OUT_EN_A::SDIO3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDIO0`"]
    #[inline(always)]
    pub fn is_sdio0(&self) -> bool {
        *self == BB_DATA_OUT_EN_A::SDIO0
    }
    #[doc = "Checks if the value of the field is `SDIO1`"]
    #[inline(always)]
    pub fn is_sdio1(&self) -> bool {
        *self == BB_DATA_OUT_EN_A::SDIO1
    }
    #[doc = "Checks if the value of the field is `SDIO2`"]
    #[inline(always)]
    pub fn is_sdio2(&self) -> bool {
        *self == BB_DATA_OUT_EN_A::SDIO2
    }
    #[doc = "Checks if the value of the field is `SDIO3`"]
    #[inline(always)]
    pub fn is_sdio3(&self) -> bool {
        *self == BB_DATA_OUT_EN_A::SDIO3
    }
}
#[doc = "Write proxy for field `BB_DATA_OUT_EN`"]
pub struct BB_DATA_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DATA_OUT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DATA_OUT_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDIO\\[0\\]"]
    #[inline(always)]
    pub fn sdio0(self) -> &'a mut W {
        self.variant(BB_DATA_OUT_EN_A::SDIO0)
    }
    #[doc = "SDIO\\[1\\]"]
    #[inline(always)]
    pub fn sdio1(self) -> &'a mut W {
        self.variant(BB_DATA_OUT_EN_A::SDIO1)
    }
    #[doc = "SDIO\\[2\\]"]
    #[inline(always)]
    pub fn sdio2(self) -> &'a mut W {
        self.variant(BB_DATA_OUT_EN_A::SDIO2)
    }
    #[doc = "SDIO\\[3\\]"]
    #[inline(always)]
    pub fn sdio3(self) -> &'a mut W {
        self.variant(BB_DATA_OUT_EN_A::SDIO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Enable SCLK Feedback Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_FB_A {
    #[doc = "0: `0`"]
    DIS = 0,
    #[doc = "1: `1`"]
    EN = 1,
}
impl From<SCLK_FB_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_FB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLK_FB`"]
pub type SCLK_FB_R = crate::R<bool, SCLK_FB_A>;
impl SCLK_FB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_FB_A {
        match self.bits {
            false => SCLK_FB_A::DIS,
            true => SCLK_FB_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SCLK_FB_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SCLK_FB_A::EN
    }
}
#[doc = "Write proxy for field `SCLK_FB`"]
pub struct SCLK_FB_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_FB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_FB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SCLK_FB_A::DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SCLK_FB_A::EN)
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
impl R {
    #[doc = "Bit 0 - SPI Master enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transaction FIFO Enable."]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TX_FIFO_EN_R {
        TX_FIFO_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Result FIFO Enable."]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RX_FIFO_EN_R {
        RX_FIFO_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bit-Bang Mode."]
    #[inline(always)]
    pub fn bbmode(&self) -> BBMODE_R {
        BBMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bits reflects the state of the currently selected slave select."]
    #[inline(always)]
    pub fn ssdr(&self) -> SSDR_R {
        SSDR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SSCLK Drive and State."]
    #[inline(always)]
    pub fn sclk_dr(&self) -> SCLK_DR_R {
        SCLK_DR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - SDIO Input Data Value."]
    #[inline(always)]
    pub fn sdio_data_in(&self) -> SDIO_DATA_IN_R {
        SDIO_DATA_IN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - No description available."]
    #[inline(always)]
    pub fn bb_data(&self) -> BB_DATA_R {
        BB_DATA_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Bit Bang SDIO Output Enable."]
    #[inline(always)]
    pub fn bb_data_out_en(&self) -> BB_DATA_OUT_EN_R {
        BB_DATA_OUT_EN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enable SCLK Feedback Mode."]
    #[inline(always)]
    pub fn sclk_fb(&self) -> SCLK_FB_R {
        SCLK_FB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Master enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Transaction FIFO Enable."]
    #[inline(always)]
    pub fn tx_fifo_en(&mut self) -> TX_FIFO_EN_W {
        TX_FIFO_EN_W { w: self }
    }
    #[doc = "Bit 2 - Result FIFO Enable."]
    #[inline(always)]
    pub fn rx_fifo_en(&mut self) -> RX_FIFO_EN_W {
        RX_FIFO_EN_W { w: self }
    }
    #[doc = "Bit 3 - Bit-Bang Mode."]
    #[inline(always)]
    pub fn bbmode(&mut self) -> BBMODE_W {
        BBMODE_W { w: self }
    }
    #[doc = "Bit 4 - This bits reflects the state of the currently selected slave select."]
    #[inline(always)]
    pub fn ssdr(&mut self) -> SSDR_W {
        SSDR_W { w: self }
    }
    #[doc = "Bit 6 - SSCLK Drive and State."]
    #[inline(always)]
    pub fn sclk_dr(&mut self) -> SCLK_DR_W {
        SCLK_DR_W { w: self }
    }
    #[doc = "Bits 8:11 - SDIO Input Data Value."]
    #[inline(always)]
    pub fn sdio_data_in(&mut self) -> SDIO_DATA_IN_W {
        SDIO_DATA_IN_W { w: self }
    }
    #[doc = "Bits 12:15 - No description available."]
    #[inline(always)]
    pub fn bb_data(&mut self) -> BB_DATA_W {
        BB_DATA_W { w: self }
    }
    #[doc = "Bits 16:19 - Bit Bang SDIO Output Enable."]
    #[inline(always)]
    pub fn bb_data_out_en(&mut self) -> BB_DATA_OUT_EN_W {
        BB_DATA_OUT_EN_W { w: self }
    }
    #[doc = "Bit 24 - Enable SCLK Feedback Mode."]
    #[inline(always)]
    pub fn sclk_fb(&mut self) -> SCLK_FB_W {
        SCLK_FB_W { w: self }
    }
}
