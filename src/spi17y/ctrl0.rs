#[doc = "Reader of register CTRL0"]
pub type R = crate::R<u32, super::CTRL0>;
#[doc = "Writer for register CTRL0"]
pub type W = crate::W<u32, super::CTRL0>;
#[doc = "Register CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SPI Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: SPI is disabled."]
    DIS = 0,
    #[doc = "1: SPI is enabled."]
    EN = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, EN_A>;
impl EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DIS,
            true => EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN_A::EN
    }
}
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN_A::DIS)
    }
    #[doc = "SPI is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN_A::EN)
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
pub enum MASTER_A {
    #[doc = "0: SPI is Slave mode."]
    DIS = 0,
    #[doc = "1: SPI is Master mode."]
    EN = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, MASTER_A>;
impl MASTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::DIS,
            true => MASTER_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MASTER_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MASTER_A::EN
    }
}
#[doc = "Write proxy for field `MASTER`"]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI is Slave mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MASTER_A::DIS)
    }
    #[doc = "SPI is Master mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MASTER_A::EN)
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
#[doc = "Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode. This bit has no effect in slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_IO_A {
    #[doc = "0: Slave select 0 is output."]
    OUTPUT = 0,
    #[doc = "1: Slave Select 0 is input, only valid if MMEN=1."]
    INPUT = 1,
}
impl From<SS_IO_A> for bool {
    #[inline(always)]
    fn from(variant: SS_IO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SS_IO`"]
pub type SS_IO_R = crate::R<bool, SS_IO_A>;
impl SS_IO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_IO_A {
        match self.bits {
            false => SS_IO_A::OUTPUT,
            true => SS_IO_A::INPUT,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == SS_IO_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == SS_IO_A::INPUT
    }
}
#[doc = "Write proxy for field `SS_IO`"]
pub struct SS_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_IO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_IO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave select 0 is output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(SS_IO_A::OUTPUT)
    }
    #[doc = "Slave Select 0 is input, only valid if MMEN=1."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(SS_IO_A::INPUT)
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
pub enum START_A {
    #[doc = "1: Master Initiates a transaction, this bit is self clearing when transactions are done. If a transaction cimpletes, and the TX FIFO is empty, the Master halts, if a transaction completes, and the TX FIFO is not empty, the Master initiates another transaction."]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, START_A>;
impl START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, START_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(START_A::START),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master Initiates a transaction, this bit is self clearing when transactions are done. If a transaction cimpletes, and the TX FIFO is empty, the Master halts, if a transaction completes, and the TX FIFO is not empty, the Master initiates another transaction."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
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
#[doc = "Start Select Control. Used in Master mode to control the behavior of the Slave Select signal at the end of a transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_CTRL_A {
    #[doc = "0: SPI De-asserts Slave Select at the end of a transaction."]
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
    #[doc = "SPI De-asserts Slave Select at the end of a transaction."]
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode. This bit has no effect in slave mode."]
    #[inline(always)]
    pub fn ss_io(&self) -> SS_IO_R {
        SS_IO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start Transmit."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Start Select Control. Used in Master mode to control the behavior of the Slave Select signal at the end of a transaction."]
    #[inline(always)]
    pub fn ss_ctrl(&self) -> SS_CTRL_R {
        SS_CTRL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    #[doc = "Bit 4 - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode. This bit has no effect in slave mode."]
    #[inline(always)]
    pub fn ss_io(&mut self) -> SS_IO_W {
        SS_IO_W { w: self }
    }
    #[doc = "Bit 5 - Start Transmit."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 8 - Start Select Control. Used in Master mode to control the behavior of the Slave Select signal at the end of a transaction."]
    #[inline(always)]
    pub fn ss_ctrl(&mut self) -> SS_CTRL_W {
        SS_CTRL_W { w: self }
    }
    #[doc = "Bits 16:19 - Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected."]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
}
