#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bus Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_A {
    #[doc = "0: I2C Bus Idle."]
    IDLE = 0,
    #[doc = "1: I2C Bus Busy."]
    BUSY = 1,
}
impl From<BUS_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUS`"]
pub type BUS_R = crate::R<bool, BUS_A>;
impl BUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_A {
        match self.bits {
            false => BUS_A::IDLE,
            true => BUS_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUS_A::BUSY
    }
}
#[doc = "RX empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_EMPTY_A {
    #[doc = "0: Not Empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Empty."]
    EMPTY = 1,
}
impl From<RX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_EMPTY`"]
pub type RX_EMPTY_R = crate::R<bool, RX_EMPTY_A>;
impl RX_EMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EMPTY_A {
        match self.bits {
            false => RX_EMPTY_A::NOT_EMPTY,
            true => RX_EMPTY_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RX_EMPTY_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RX_EMPTY_A::EMPTY
    }
}
#[doc = "RX Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FULL_A {
    #[doc = "0: Not Full."]
    NOT_FULL = 0,
    #[doc = "1: Full."]
    FULL = 1,
}
impl From<RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_FULL`"]
pub type RX_FULL_R = crate::R<bool, RX_FULL_A>;
impl RX_FULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FULL_A {
        match self.bits {
            false => RX_FULL_A::NOT_FULL,
            true => RX_FULL_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RX_FULL_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RX_FULL_A::FULL
    }
}
#[doc = "TX Empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_EMPTY_A {
    #[doc = "0: Not Empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Empty."]
    EMPTY = 1,
}
impl From<TX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_EMPTY`"]
pub type TX_EMPTY_R = crate::R<bool, TX_EMPTY_A>;
impl TX_EMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EMPTY_A {
        match self.bits {
            false => TX_EMPTY_A::NOT_EMPTY,
            true => TX_EMPTY_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TX_EMPTY_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TX_EMPTY_A::EMPTY
    }
}
#[doc = "Write proxy for field `TX_EMPTY`"]
pub struct TX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EMPTY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_EMPTY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TX_EMPTY_A::NOT_EMPTY)
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TX_EMPTY_A::EMPTY)
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
#[doc = "TX Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FULL_A {
    #[doc = "0: Not Empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Empty."]
    EMPTY = 1,
}
impl From<TX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_FULL`"]
pub type TX_FULL_R = crate::R<bool, TX_FULL_A>;
impl TX_FULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FULL_A {
        match self.bits {
            false => TX_FULL_A::NOT_EMPTY,
            true => TX_FULL_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TX_FULL_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TX_FULL_A::EMPTY
    }
}
#[doc = "Write proxy for field `TX_FULL`"]
pub struct TX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TX_FULL_A::NOT_EMPTY)
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TX_FULL_A::EMPTY)
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
#[doc = "Clock Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_MODE_A {
    #[doc = "0: Device not actively driving SCL clock cycles."]
    NOT_ACTIVELY_DRIVING_SCL_CLOCK = 0,
    #[doc = "1: Device operating as master and actively driving SCL clock cycles."]
    ACTIVELY_DRIVING_SCL_CLOCK = 1,
}
impl From<CLK_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_MODE`"]
pub type CLK_MODE_R = crate::R<bool, CLK_MODE_A>;
impl CLK_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_MODE_A {
        match self.bits {
            false => CLK_MODE_A::NOT_ACTIVELY_DRIVING_SCL_CLOCK,
            true => CLK_MODE_A::ACTIVELY_DRIVING_SCL_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVELY_DRIVING_SCL_CLOCK`"]
    #[inline(always)]
    pub fn is_not_actively_driving_scl_clock(&self) -> bool {
        *self == CLK_MODE_A::NOT_ACTIVELY_DRIVING_SCL_CLOCK
    }
    #[doc = "Checks if the value of the field is `ACTIVELY_DRIVING_SCL_CLOCK`"]
    #[inline(always)]
    pub fn is_actively_driving_scl_clock(&self) -> bool {
        *self == CLK_MODE_A::ACTIVELY_DRIVING_SCL_CLOCK
    }
}
#[doc = "Controller Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATUS_A {
    #[doc = "0: Controller Idle."]
    IDLE = 0,
    #[doc = "1: master Transmit address."]
    MTX_ADDR = 1,
    #[doc = "2: Master Receive address ACK."]
    MRX_ADDR_ACK = 2,
    #[doc = "3: Master Transmit extended address."]
    MTX_EX_ADDR = 3,
    #[doc = "4: Master Receive extended address ACK."]
    MRX_EX_ADDR = 4,
    #[doc = "5: Slave Receive address."]
    SRX_ADDR = 5,
    #[doc = "6: Slave Transmit address ACK."]
    STX_ADDR_ACK = 6,
    #[doc = "7: Slave Receive extended address."]
    SRX_EX_ADDR = 7,
    #[doc = "8: Slave Transmit extended address ACK."]
    STX_EX_ADDR_ACK = 8,
    #[doc = "9: Transmit data (master or slave)."]
    TX = 9,
    #[doc = "10: Receive data ACK (master or slave)."]
    RX_ACK = 10,
    #[doc = "11: Receive data (master or slave)."]
    RX = 11,
    #[doc = "12: Transmit data ACK (master or slave)."]
    TX_ACK = 12,
    #[doc = "13: NACK stage (master or slave)."]
    NACK = 13,
    #[doc = "15: Bystander state (ongoing transaction but not participant- another master addressing another slave)."]
    BY_ST = 15,
}
impl From<STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<u8, STATUS_A>;
impl STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATUS_A::IDLE),
            1 => Val(STATUS_A::MTX_ADDR),
            2 => Val(STATUS_A::MRX_ADDR_ACK),
            3 => Val(STATUS_A::MTX_EX_ADDR),
            4 => Val(STATUS_A::MRX_EX_ADDR),
            5 => Val(STATUS_A::SRX_ADDR),
            6 => Val(STATUS_A::STX_ADDR_ACK),
            7 => Val(STATUS_A::SRX_EX_ADDR),
            8 => Val(STATUS_A::STX_EX_ADDR_ACK),
            9 => Val(STATUS_A::TX),
            10 => Val(STATUS_A::RX_ACK),
            11 => Val(STATUS_A::RX),
            12 => Val(STATUS_A::TX_ACK),
            13 => Val(STATUS_A::NACK),
            15 => Val(STATUS_A::BY_ST),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATUS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `MTX_ADDR`"]
    #[inline(always)]
    pub fn is_mtx_addr(&self) -> bool {
        *self == STATUS_A::MTX_ADDR
    }
    #[doc = "Checks if the value of the field is `MRX_ADDR_ACK`"]
    #[inline(always)]
    pub fn is_mrx_addr_ack(&self) -> bool {
        *self == STATUS_A::MRX_ADDR_ACK
    }
    #[doc = "Checks if the value of the field is `MTX_EX_ADDR`"]
    #[inline(always)]
    pub fn is_mtx_ex_addr(&self) -> bool {
        *self == STATUS_A::MTX_EX_ADDR
    }
    #[doc = "Checks if the value of the field is `MRX_EX_ADDR`"]
    #[inline(always)]
    pub fn is_mrx_ex_addr(&self) -> bool {
        *self == STATUS_A::MRX_EX_ADDR
    }
    #[doc = "Checks if the value of the field is `SRX_ADDR`"]
    #[inline(always)]
    pub fn is_srx_addr(&self) -> bool {
        *self == STATUS_A::SRX_ADDR
    }
    #[doc = "Checks if the value of the field is `STX_ADDR_ACK`"]
    #[inline(always)]
    pub fn is_stx_addr_ack(&self) -> bool {
        *self == STATUS_A::STX_ADDR_ACK
    }
    #[doc = "Checks if the value of the field is `SRX_EX_ADDR`"]
    #[inline(always)]
    pub fn is_srx_ex_addr(&self) -> bool {
        *self == STATUS_A::SRX_EX_ADDR
    }
    #[doc = "Checks if the value of the field is `STX_EX_ADDR_ACK`"]
    #[inline(always)]
    pub fn is_stx_ex_addr_ack(&self) -> bool {
        *self == STATUS_A::STX_EX_ADDR_ACK
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == STATUS_A::TX
    }
    #[doc = "Checks if the value of the field is `RX_ACK`"]
    #[inline(always)]
    pub fn is_rx_ack(&self) -> bool {
        *self == STATUS_A::RX_ACK
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == STATUS_A::RX
    }
    #[doc = "Checks if the value of the field is `TX_ACK`"]
    #[inline(always)]
    pub fn is_tx_ack(&self) -> bool {
        *self == STATUS_A::TX_ACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == STATUS_A::NACK
    }
    #[doc = "Checks if the value of the field is `BY_ST`"]
    #[inline(always)]
    pub fn is_by_st(&self) -> bool {
        *self == STATUS_A::BY_ST
    }
}
#[doc = "Write proxy for field `STATUS`"]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Controller Idle."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(STATUS_A::IDLE)
    }
    #[doc = "master Transmit address."]
    #[inline(always)]
    pub fn mtx_addr(self) -> &'a mut W {
        self.variant(STATUS_A::MTX_ADDR)
    }
    #[doc = "Master Receive address ACK."]
    #[inline(always)]
    pub fn mrx_addr_ack(self) -> &'a mut W {
        self.variant(STATUS_A::MRX_ADDR_ACK)
    }
    #[doc = "Master Transmit extended address."]
    #[inline(always)]
    pub fn mtx_ex_addr(self) -> &'a mut W {
        self.variant(STATUS_A::MTX_EX_ADDR)
    }
    #[doc = "Master Receive extended address ACK."]
    #[inline(always)]
    pub fn mrx_ex_addr(self) -> &'a mut W {
        self.variant(STATUS_A::MRX_EX_ADDR)
    }
    #[doc = "Slave Receive address."]
    #[inline(always)]
    pub fn srx_addr(self) -> &'a mut W {
        self.variant(STATUS_A::SRX_ADDR)
    }
    #[doc = "Slave Transmit address ACK."]
    #[inline(always)]
    pub fn stx_addr_ack(self) -> &'a mut W {
        self.variant(STATUS_A::STX_ADDR_ACK)
    }
    #[doc = "Slave Receive extended address."]
    #[inline(always)]
    pub fn srx_ex_addr(self) -> &'a mut W {
        self.variant(STATUS_A::SRX_EX_ADDR)
    }
    #[doc = "Slave Transmit extended address ACK."]
    #[inline(always)]
    pub fn stx_ex_addr_ack(self) -> &'a mut W {
        self.variant(STATUS_A::STX_EX_ADDR_ACK)
    }
    #[doc = "Transmit data (master or slave)."]
    #[inline(always)]
    pub fn tx(self) -> &'a mut W {
        self.variant(STATUS_A::TX)
    }
    #[doc = "Receive data ACK (master or slave)."]
    #[inline(always)]
    pub fn rx_ack(self) -> &'a mut W {
        self.variant(STATUS_A::RX_ACK)
    }
    #[doc = "Receive data (master or slave)."]
    #[inline(always)]
    pub fn rx(self) -> &'a mut W {
        self.variant(STATUS_A::RX)
    }
    #[doc = "Transmit data ACK (master or slave)."]
    #[inline(always)]
    pub fn tx_ack(self) -> &'a mut W {
        self.variant(STATUS_A::TX_ACK)
    }
    #[doc = "NACK stage (master or slave)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(STATUS_A::NACK)
    }
    #[doc = "Bystander state (ongoing transaction but not participant- another master addressing another slave)."]
    #[inline(always)]
    pub fn by_st(self) -> &'a mut W {
        self.variant(STATUS_A::BY_ST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bus Status."]
    #[inline(always)]
    pub fn bus(&self) -> BUS_R {
        BUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX empty."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Full."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX Empty."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX Full."]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Mode."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Controller Status."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - TX Empty."]
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W {
        TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 4 - TX Full."]
    #[inline(always)]
    pub fn tx_full(&mut self) -> TX_FULL_W {
        TX_FULL_W { w: self }
    }
    #[doc = "Bits 8:11 - Controller Status."]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
}
