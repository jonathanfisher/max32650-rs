#[doc = "Reader of register IRQ"]
pub type R = crate::R<u32, super::IRQ>;
#[doc = "Writer for register IRQ"]
pub type W = crate::W<u32, super::IRQ>;
#[doc = "Register IRQ `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_THRESH_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TX_THRESH_A> for bool {
    #[inline(always)]
    fn from(variant: TX_THRESH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_THRESH`"]
pub type TX_THRESH_R = crate::R<bool, TX_THRESH_A>;
impl TX_THRESH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TX_THRESH_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TX_THRESH_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_THRESH_A::CLEAR
    }
}
#[doc = "Write proxy for field `TX_THRESH`"]
pub struct TX_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_THRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_THRESH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TX_THRESH_A::CLEAR)
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
#[doc = "TX FIFO Empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_EMPTY_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, TX_EMPTY_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TX_EMPTY_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_EMPTY_A::CLEAR
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
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TX_EMPTY_A::CLEAR)
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
#[doc = "RX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_THRESH_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<RX_THRESH_A> for bool {
    #[inline(always)]
    fn from(variant: RX_THRESH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_THRESH`"]
pub type RX_THRESH_R = crate::R<bool, RX_THRESH_A>;
impl RX_THRESH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RX_THRESH_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RX_THRESH_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_THRESH_A::CLEAR
    }
}
#[doc = "Write proxy for field `RX_THRESH`"]
pub struct RX_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_THRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_THRESH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RX_THRESH_A::CLEAR)
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
#[doc = "RX FIFO FULL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FULL_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
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
    pub fn variant(&self) -> crate::Variant<bool, RX_FULL_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RX_FULL_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_FULL_A::CLEAR
    }
}
#[doc = "Write proxy for field `RX_FULL`"]
pub struct RX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RX_FULL_A::CLEAR)
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
#[doc = "Slave Select Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSA_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SSA_A> for bool {
    #[inline(always)]
    fn from(variant: SSA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSA`"]
pub type SSA_R = crate::R<bool, SSA_A>;
impl SSA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SSA_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SSA_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SSA_A::CLEAR
    }
}
#[doc = "Write proxy for field `SSA`"]
pub struct SSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SSA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SSA_A::CLEAR)
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
#[doc = "Slave Select Deasserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSD_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SSD_A> for bool {
    #[inline(always)]
    fn from(variant: SSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSD`"]
pub type SSD_R = crate::R<bool, SSD_A>;
impl SSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SSD_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SSD_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SSD_A::CLEAR
    }
}
#[doc = "Write proxy for field `SSD`"]
pub struct SSD_W<'a> {
    w: &'a mut W,
}
impl<'a> SSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SSD_A::CLEAR)
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
#[doc = "Multi-Master Mode Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULT`"]
pub type FAULT_R = crate::R<bool, FAULT_A>;
impl FAULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FAULT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(FAULT_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FAULT_A::CLEAR
    }
}
#[doc = "Write proxy for field `FAULT`"]
pub struct FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FAULT_A::CLEAR)
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
#[doc = "Slave Abort Detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABORT`"]
pub type ABORT_R = crate::R<bool, ABORT_A>;
impl ABORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ABORT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ABORT_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ABORT_A::CLEAR
    }
}
#[doc = "Write proxy for field `ABORT`"]
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABORT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ABORT_A::CLEAR)
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
#[doc = "Timeout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TIMEOUT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TIMEOUT_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TIMEOUT_A::CLEAR
    }
}
#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIMEOUT_A::CLEAR)
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
#[doc = "Master Done, set when SPI Master has completed any transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M_DONE_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<M_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: M_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `M_DONE`"]
pub type M_DONE_R = crate::R<bool, M_DONE_A>;
impl M_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, M_DONE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(M_DONE_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == M_DONE_A::CLEAR
    }
}
#[doc = "Write proxy for field `M_DONE`"]
pub struct M_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> M_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(M_DONE_A::CLEAR)
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
#[doc = "Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_OVR_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TX_OVR_A> for bool {
    #[inline(always)]
    fn from(variant: TX_OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_OVR`"]
pub type TX_OVR_R = crate::R<bool, TX_OVR_A>;
impl TX_OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TX_OVR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TX_OVR_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_OVR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TX_OVR`"]
pub struct TX_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_OVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_OVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TX_OVR_A::CLEAR)
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
#[doc = "Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_UND_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TX_UND_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_UND`"]
pub type TX_UND_R = crate::R<bool, TX_UND_A>;
impl TX_UND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TX_UND_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TX_UND_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_UND_A::CLEAR
    }
}
#[doc = "Write proxy for field `TX_UND`"]
pub struct TX_UND_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_UND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_UND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TX_UND_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_OVR_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<RX_OVR_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_OVR`"]
pub type RX_OVR_R = crate::R<bool, RX_OVR_A>;
impl RX_OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RX_OVR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RX_OVR_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_OVR_A::CLEAR
    }
}
#[doc = "Write proxy for field `RX_OVR`"]
pub struct RX_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_OVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RX_OVR_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_UND_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<RX_UND_A> for bool {
    #[inline(always)]
    fn from(variant: RX_UND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_UND`"]
pub type RX_UND_R = crate::R<bool, RX_UND_A>;
impl RX_UND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RX_UND_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RX_UND_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_UND_A::CLEAR
    }
}
#[doc = "Write proxy for field `RX_UND`"]
pub struct RX_UND_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_UND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_UND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RX_UND_A::CLEAR)
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
#[doc = "Slave Ready 0 Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR0A_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SR0A_A> for bool {
    #[inline(always)]
    fn from(variant: SR0A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR0A`"]
pub type SR0A_R = crate::R<bool, SR0A_A>;
impl SR0A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SR0A_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SR0A_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SR0A_A::CLEAR
    }
}
#[doc = "Write proxy for field `SR0A`"]
pub struct SR0A_W<'a> {
    w: &'a mut W,
}
impl<'a> SR0A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR0A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SR0A_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Slave Ready 1 Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR1A_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SR1A_A> for bool {
    #[inline(always)]
    fn from(variant: SR1A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR1A`"]
pub type SR1A_R = crate::R<bool, SR1A_A>;
impl SR1A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SR1A_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SR1A_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SR1A_A::CLEAR
    }
}
#[doc = "Write proxy for field `SR1A`"]
pub struct SR1A_W<'a> {
    w: &'a mut W,
}
impl<'a> SR1A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR1A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SR1A_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Slave Ready 2 Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR2A_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SR2A_A> for bool {
    #[inline(always)]
    fn from(variant: SR2A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR2A`"]
pub type SR2A_R = crate::R<bool, SR2A_A>;
impl SR2A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SR2A_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SR2A_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SR2A_A::CLEAR
    }
}
#[doc = "Write proxy for field `SR2A`"]
pub struct SR2A_W<'a> {
    w: &'a mut W,
}
impl<'a> SR2A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR2A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SR2A_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Slave Ready 3 Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR3A_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SR3A_A> for bool {
    #[inline(always)]
    fn from(variant: SR3A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR3A`"]
pub type SR3A_R = crate::R<bool, SR3A_A>;
impl SR3A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SR3A_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SR3A_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SR3A_A::CLEAR
    }
}
#[doc = "Write proxy for field `SR3A`"]
pub struct SR3A_W<'a> {
    w: &'a mut W,
}
impl<'a> SR3A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR3A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SR3A_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Slave Ready 4 Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR4A_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SR4A_A> for bool {
    #[inline(always)]
    fn from(variant: SR4A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR4A`"]
pub type SR4A_R = crate::R<bool, SR4A_A>;
impl SR4A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SR4A_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SR4A_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SR4A_A::CLEAR
    }
}
#[doc = "Write proxy for field `SR4A`"]
pub struct SR4A_W<'a> {
    w: &'a mut W,
}
impl<'a> SR4A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR4A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SR4A_A::CLEAR)
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
#[doc = "Slave Ready 5 Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR5A_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SR5A_A> for bool {
    #[inline(always)]
    fn from(variant: SR5A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR5A`"]
pub type SR5A_R = crate::R<bool, SR5A_A>;
impl SR5A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SR5A_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SR5A_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SR5A_A::CLEAR
    }
}
#[doc = "Write proxy for field `SR5A`"]
pub struct SR5A_W<'a> {
    w: &'a mut W,
}
impl<'a> SR5A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR5A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SR5A_A::CLEAR)
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
#[doc = "Slave Ready 6 Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR6A_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SR6A_A> for bool {
    #[inline(always)]
    fn from(variant: SR6A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR6A`"]
pub type SR6A_R = crate::R<bool, SR6A_A>;
impl SR6A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SR6A_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SR6A_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SR6A_A::CLEAR
    }
}
#[doc = "Write proxy for field `SR6A`"]
pub struct SR6A_W<'a> {
    w: &'a mut W,
}
impl<'a> SR6A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR6A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SR6A_A::CLEAR)
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
#[doc = "Slave Ready 7 Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR7A_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SR7A_A> for bool {
    #[inline(always)]
    fn from(variant: SR7A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR7A`"]
pub type SR7A_R = crate::R<bool, SR7A_A>;
impl SR7A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SR7A_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SR7A_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SR7A_A::CLEAR
    }
}
#[doc = "Write proxy for field `SR7A`"]
pub struct SR7A_W<'a> {
    w: &'a mut W,
}
impl<'a> SR7A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR7A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SR7A_A::CLEAR)
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
impl R {
    #[doc = "Bit 0 - TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_thresh(&self) -> TX_THRESH_R {
        TX_THRESH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_thresh(&self) -> RX_THRESH_R {
        RX_THRESH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX FIFO FULL."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Select Asserted."]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deasserted."]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Slave Abort Detected."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timeout."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Master Done, set when SPI Master has completed any transactions."]
    #[inline(always)]
    pub fn m_done(&self) -> M_DONE_R {
        M_DONE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
    #[inline(always)]
    pub fn tx_ovr(&self) -> TX_OVR_R {
        TX_OVR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
    #[inline(always)]
    pub fn tx_und(&self) -> TX_UND_R {
        TX_UND_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
    #[inline(always)]
    pub fn rx_ovr(&self) -> RX_OVR_R {
        RX_OVR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
    #[inline(always)]
    pub fn rx_und(&self) -> RX_UND_R {
        RX_UND_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slave Ready 0 Asserted."]
    #[inline(always)]
    pub fn sr0a(&self) -> SR0A_R {
        SR0A_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Slave Ready 1 Asserted."]
    #[inline(always)]
    pub fn sr1a(&self) -> SR1A_R {
        SR1A_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Slave Ready 2 Asserted."]
    #[inline(always)]
    pub fn sr2a(&self) -> SR2A_R {
        SR2A_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Slave Ready 3 Asserted."]
    #[inline(always)]
    pub fn sr3a(&self) -> SR3A_R {
        SR3A_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Slave Ready 4 Asserted."]
    #[inline(always)]
    pub fn sr4a(&self) -> SR4A_R {
        SR4A_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Slave Ready 5 Asserted."]
    #[inline(always)]
    pub fn sr5a(&self) -> SR5A_R {
        SR5A_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Slave Ready 6 Asserted."]
    #[inline(always)]
    pub fn sr6a(&self) -> SR6A_R {
        SR6A_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Slave Ready 7 Asserted."]
    #[inline(always)]
    pub fn sr7a(&self) -> SR7A_R {
        SR7A_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_thresh(&mut self) -> TX_THRESH_W {
        TX_THRESH_W { w: self }
    }
    #[doc = "Bit 1 - TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W {
        TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_thresh(&mut self) -> RX_THRESH_W {
        RX_THRESH_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFO FULL."]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RX_FULL_W {
        RX_FULL_W { w: self }
    }
    #[doc = "Bit 4 - Slave Select Asserted."]
    #[inline(always)]
    pub fn ssa(&mut self) -> SSA_W {
        SSA_W { w: self }
    }
    #[doc = "Bit 5 - Slave Select Deasserted."]
    #[inline(always)]
    pub fn ssd(&mut self) -> SSD_W {
        SSD_W { w: self }
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault."]
    #[inline(always)]
    pub fn fault(&mut self) -> FAULT_W {
        FAULT_W { w: self }
    }
    #[doc = "Bit 9 - Slave Abort Detected."]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bit 10 - Timeout."]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 11 - Master Done, set when SPI Master has completed any transactions."]
    #[inline(always)]
    pub fn m_done(&mut self) -> M_DONE_W {
        M_DONE_W { w: self }
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
    #[inline(always)]
    pub fn tx_ovr(&mut self) -> TX_OVR_W {
        TX_OVR_W { w: self }
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
    #[inline(always)]
    pub fn tx_und(&mut self) -> TX_UND_W {
        TX_UND_W { w: self }
    }
    #[doc = "Bit 14 - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
    #[inline(always)]
    pub fn rx_ovr(&mut self) -> RX_OVR_W {
        RX_OVR_W { w: self }
    }
    #[doc = "Bit 15 - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
    #[inline(always)]
    pub fn rx_und(&mut self) -> RX_UND_W {
        RX_UND_W { w: self }
    }
    #[doc = "Bit 16 - Slave Ready 0 Asserted."]
    #[inline(always)]
    pub fn sr0a(&mut self) -> SR0A_W {
        SR0A_W { w: self }
    }
    #[doc = "Bit 17 - Slave Ready 1 Asserted."]
    #[inline(always)]
    pub fn sr1a(&mut self) -> SR1A_W {
        SR1A_W { w: self }
    }
    #[doc = "Bit 18 - Slave Ready 2 Asserted."]
    #[inline(always)]
    pub fn sr2a(&mut self) -> SR2A_W {
        SR2A_W { w: self }
    }
    #[doc = "Bit 19 - Slave Ready 3 Asserted."]
    #[inline(always)]
    pub fn sr3a(&mut self) -> SR3A_W {
        SR3A_W { w: self }
    }
    #[doc = "Bit 20 - Slave Ready 4 Asserted."]
    #[inline(always)]
    pub fn sr4a(&mut self) -> SR4A_W {
        SR4A_W { w: self }
    }
    #[doc = "Bit 21 - Slave Ready 5 Asserted."]
    #[inline(always)]
    pub fn sr5a(&mut self) -> SR5A_W {
        SR5A_W { w: self }
    }
    #[doc = "Bit 22 - Slave Ready 6 Asserted."]
    #[inline(always)]
    pub fn sr6a(&mut self) -> SR6A_W {
        SR6A_W { w: self }
    }
    #[doc = "Bit 23 - Slave Ready 7 Asserted."]
    #[inline(always)]
    pub fn sr7a(&mut self) -> SR7A_W {
        SR7A_W { w: self }
    }
}
