#[doc = "Reader of register ctrl1"]
pub type R = crate::R<u32, super::CTRL1>;
#[doc = "Writer for register ctrl1"]
pub type W = crate::W<u32, super::CTRL1>;
#[doc = "Register ctrl1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SPI Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIEN_A {
    #[doc = "0: SPI is disabled."]
    DIS = 0,
    #[doc = "1: SPI is enabled."]
    EN = 1,
}
impl From<SPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIEN`"]
pub type SPIEN_R = crate::R<bool, SPIEN_A>;
impl SPIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIEN_A {
        match self.bits {
            false => SPIEN_A::DIS,
            true => SPIEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPIEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPIEN_A::EN
    }
}
#[doc = "Write proxy for field `SPIEN`"]
pub struct SPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPIEN_A::DIS)
    }
    #[doc = "SPI is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPIEN_A::EN)
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
#[doc = "Master Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMEN_A {
    #[doc = "0: SPI is Slave mode."]
    DIS = 0,
    #[doc = "1: SPI is Master mode."]
    EN = 1,
}
impl From<MMEN_A> for bool {
    #[inline(always)]
    fn from(variant: MMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MMEN`"]
pub type MMEN_R = crate::R<bool, MMEN_A>;
impl MMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMEN_A {
        match self.bits {
            false => MMEN_A::DIS,
            true => MMEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MMEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MMEN_A::EN
    }
}
#[doc = "Write proxy for field `MMEN`"]
pub struct MMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI is Slave mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MMEN_A::DIS)
    }
    #[doc = "SPI is Master mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MMEN_A::EN)
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
#[doc = "Timer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_A {
    #[doc = "0: Timer is disabled."]
    DIS = 0,
    #[doc = "1: Timer is enabled, only valid if SPIEN=0."]
    EN = 1,
}
impl From<TIMER_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<bool, TIMER_A>;
impl TIMER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_A {
        match self.bits {
            false => TIMER_A::DIS,
            true => TIMER_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TIMER_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TIMER_A::EN
    }
}
#[doc = "Write proxy for field `TIMER`"]
pub struct TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TIMER_A::DIS)
    }
    #[doc = "Timer is enabled, only valid if SPIEN=0."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TIMER_A::EN)
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
#[doc = "Flow Control Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FL_EN_A {
    #[doc = "0: Flow Control mode is disabled."]
    DIS = 0,
    #[doc = "1: Flow Control Mode is enabled."]
    EN = 1,
}
impl From<FL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FL_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FL_EN`"]
pub type FL_EN_R = crate::R<bool, FL_EN_A>;
impl FL_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FL_EN_A {
        match self.bits {
            false => FL_EN_A::DIS,
            true => FL_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FL_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FL_EN_A::EN
    }
}
#[doc = "Write proxy for field `FL_EN`"]
pub struct FL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FL_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flow Control mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FL_EN_A::DIS)
    }
    #[doc = "Flow Control Mode is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FL_EN_A::EN)
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
#[doc = "Slave Select 0, IO direction, to support Multi-Master mode, Slave Select 0 can be input in Master mode. This bit has no effect in slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIO_A {
    #[doc = "0: Slave select 0 is output."]
    OUTPUT = 0,
    #[doc = "1: Slave Select 0 is input, only valid if MMEN=1."]
    INPUT = 1,
}
impl From<SSIO_A> for bool {
    #[inline(always)]
    fn from(variant: SSIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSIO`"]
pub type SSIO_R = crate::R<bool, SSIO_A>;
impl SSIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIO_A {
        match self.bits {
            false => SSIO_A::OUTPUT,
            true => SSIO_A::INPUT,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == SSIO_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == SSIO_A::INPUT
    }
}
#[doc = "Write proxy for field `SSIO`"]
pub struct SSIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave select 0 is output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(SSIO_A::OUTPUT)
    }
    #[doc = "Slave Select 0 is input, only valid if MMEN=1."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(SSIO_A::INPUT)
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
#[doc = "Start Transmit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_START_A {
    #[doc = "1: Master Initiates a transaction, this bit is self clearing when transactions are done. If a transaction completes, and the TX FIFO is empty, the Master halts, if a transaction completes, and the TX FIFO is not empty, the Master initiates another transaction."]
    START = 1,
}
impl From<TX_START_A> for bool {
    #[inline(always)]
    fn from(variant: TX_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_START`"]
pub type TX_START_R = crate::R<bool, TX_START_A>;
impl TX_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TX_START_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TX_START_A::START),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == TX_START_A::START
    }
}
#[doc = "Write proxy for field `TX_START`"]
pub struct TX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master Initiates a transaction, this bit is self clearing when transactions are done. If a transaction completes, and the TX FIFO is empty, the Master halts, if a transaction completes, and the TX FIFO is not empty, the Master initiates another transaction."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(TX_START_A::START)
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
#[doc = "Slave Select Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_CTRL_A {
    #[doc = "0: SPI de-asserts Slave Select at the end of a transaction."]
    DEASSERT = 0,
    #[doc = "1: SPI leaves Slave Select asserted at the end of a transaction."]
    ASSERT = 1,
}
impl From<SS_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: SS_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SS_CTRL`"]
pub type SS_CTRL_R = crate::R<bool, SS_CTRL_A>;
impl SS_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_CTRL_A {
        match self.bits {
            false => SS_CTRL_A::DEASSERT,
            true => SS_CTRL_A::ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == SS_CTRL_A::DEASSERT
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SS_CTRL_A::ASSERT
    }
}
#[doc = "Write proxy for field `SS_CTRL`"]
pub struct SS_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI de-asserts Slave Select at the end of a transaction."]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(SS_CTRL_A::DEASSERT)
    }
    #[doc = "SPI leaves Slave Select asserted at the end of a transaction."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(SS_CTRL_A::ASSERT)
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
#[doc = "Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SS_A {
    #[doc = "1: SS0 is selected."]
    SS0 = 1,
    #[doc = "2: SS1 is selected."]
    SS1 = 2,
    #[doc = "4: SS2 is selected."]
    SS2 = 4,
    #[doc = "8: SS3 is selected."]
    SS3 = 8,
    #[doc = "16: SS4 is selected."]
    SS4 = 16,
    #[doc = "32: SS5 is selected."]
    SS5 = 32,
    #[doc = "64: SS6 is selected."]
    SS6 = 64,
    #[doc = "128: SS7 is selected."]
    SS7 = 128,
}
impl From<SS_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SS`"]
pub type SS_R = crate::R<u8, SS_A>;
impl SS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SS_A::SS0),
            2 => Val(SS_A::SS1),
            4 => Val(SS_A::SS2),
            8 => Val(SS_A::SS3),
            16 => Val(SS_A::SS4),
            32 => Val(SS_A::SS5),
            64 => Val(SS_A::SS6),
            128 => Val(SS_A::SS7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SS0`"]
    #[inline(always)]
    pub fn is_ss0(&self) -> bool {
        *self == SS_A::SS0
    }
    #[doc = "Checks if the value of the field is `SS1`"]
    #[inline(always)]
    pub fn is_ss1(&self) -> bool {
        *self == SS_A::SS1
    }
    #[doc = "Checks if the value of the field is `SS2`"]
    #[inline(always)]
    pub fn is_ss2(&self) -> bool {
        *self == SS_A::SS2
    }
    #[doc = "Checks if the value of the field is `SS3`"]
    #[inline(always)]
    pub fn is_ss3(&self) -> bool {
        *self == SS_A::SS3
    }
    #[doc = "Checks if the value of the field is `SS4`"]
    #[inline(always)]
    pub fn is_ss4(&self) -> bool {
        *self == SS_A::SS4
    }
    #[doc = "Checks if the value of the field is `SS5`"]
    #[inline(always)]
    pub fn is_ss5(&self) -> bool {
        *self == SS_A::SS5
    }
    #[doc = "Checks if the value of the field is `SS6`"]
    #[inline(always)]
    pub fn is_ss6(&self) -> bool {
        *self == SS_A::SS6
    }
    #[doc = "Checks if the value of the field is `SS7`"]
    #[inline(always)]
    pub fn is_ss7(&self) -> bool {
        *self == SS_A::SS7
    }
}
#[doc = "Write proxy for field `SS`"]
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SS0 is selected."]
    #[inline(always)]
    pub fn ss0(self) -> &'a mut W {
        self.variant(SS_A::SS0)
    }
    #[doc = "SS1 is selected."]
    #[inline(always)]
    pub fn ss1(self) -> &'a mut W {
        self.variant(SS_A::SS1)
    }
    #[doc = "SS2 is selected."]
    #[inline(always)]
    pub fn ss2(self) -> &'a mut W {
        self.variant(SS_A::SS2)
    }
    #[doc = "SS3 is selected."]
    #[inline(always)]
    pub fn ss3(self) -> &'a mut W {
        self.variant(SS_A::SS3)
    }
    #[doc = "SS4 is selected."]
    #[inline(always)]
    pub fn ss4(self) -> &'a mut W {
        self.variant(SS_A::SS4)
    }
    #[doc = "SS5 is selected."]
    #[inline(always)]
    pub fn ss5(self) -> &'a mut W {
        self.variant(SS_A::SS5)
    }
    #[doc = "SS6 is selected."]
    #[inline(always)]
    pub fn ss6(self) -> &'a mut W {
        self.variant(SS_A::SS6)
    }
    #[doc = "SS7 is selected."]
    #[inline(always)]
    pub fn ss7(self) -> &'a mut W {
        self.variant(SS_A::SS7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mmen(&self) -> MMEN_R {
        MMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer Enable."]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flow Control Mode Enable."]
    #[inline(always)]
    pub fn fl_en(&self) -> FL_EN_R {
        FL_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Select 0, IO direction, to support Multi-Master mode, Slave Select 0 can be input in Master mode. This bit has no effect in slave mode."]
    #[inline(always)]
    pub fn ssio(&self) -> SSIO_R {
        SSIO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start Transmit."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Select Control."]
    #[inline(always)]
    pub fn ss_ctrl(&self) -> SS_CTRL_R {
        SS_CTRL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W {
        SPIEN_W { w: self }
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mmen(&mut self) -> MMEN_W {
        MMEN_W { w: self }
    }
    #[doc = "Bit 2 - Timer Enable."]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W { w: self }
    }
    #[doc = "Bit 3 - Flow Control Mode Enable."]
    #[inline(always)]
    pub fn fl_en(&mut self) -> FL_EN_W {
        FL_EN_W { w: self }
    }
    #[doc = "Bit 4 - Slave Select 0, IO direction, to support Multi-Master mode, Slave Select 0 can be input in Master mode. This bit has no effect in slave mode."]
    #[inline(always)]
    pub fn ssio(&mut self) -> SSIO_W {
        SSIO_W { w: self }
    }
    #[doc = "Bit 5 - Start Transmit."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W {
        TX_START_W { w: self }
    }
    #[doc = "Bit 8 - Slave Select Control."]
    #[inline(always)]
    pub fn ss_ctrl(&mut self) -> SS_CTRL_W {
        SS_CTRL_W { w: self }
    }
    #[doc = "Bits 16:23 - Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected."]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
}
