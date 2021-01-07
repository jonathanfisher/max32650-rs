#[doc = "Reader of register INT_FL0"]
pub type R = crate::R<u32, super::INT_FL0>;
#[doc = "Writer for register INT_FL0"]
pub type W = crate::W<u32, super::INT_FL0>;
#[doc = "Register INT_FL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_FL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transfer Done Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, DONE_A>;
impl DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::INACTIVE,
            true => DONE_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DONE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DONE_A::PENDING
    }
}
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DONE_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DONE_A::PENDING)
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
#[doc = "Interactive Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_MODE_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<RX_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_MODE`"]
pub type RX_MODE_R = crate::R<bool, RX_MODE_A>;
impl RX_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_MODE_A {
        match self.bits {
            false => RX_MODE_A::INACTIVE,
            true => RX_MODE_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RX_MODE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_MODE_A::PENDING
    }
}
#[doc = "Write proxy for field `RX_MODE`"]
pub struct RX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(RX_MODE_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RX_MODE_A::PENDING)
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
#[doc = "Slave General Call Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEN_CALL_ADDR_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<GEN_CALL_ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: GEN_CALL_ADDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GEN_CALL_ADDR`"]
pub type GEN_CALL_ADDR_R = crate::R<bool, GEN_CALL_ADDR_A>;
impl GEN_CALL_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEN_CALL_ADDR_A {
        match self.bits {
            false => GEN_CALL_ADDR_A::INACTIVE,
            true => GEN_CALL_ADDR_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == GEN_CALL_ADDR_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == GEN_CALL_ADDR_A::PENDING
    }
}
#[doc = "Write proxy for field `GEN_CALL_ADDR`"]
pub struct GEN_CALL_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_CALL_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GEN_CALL_ADDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(GEN_CALL_ADDR_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(GEN_CALL_ADDR_A::PENDING)
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
#[doc = "Slave Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_MATCH_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ADDR_MATCH_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_MATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR_MATCH`"]
pub type ADDR_MATCH_R = crate::R<bool, ADDR_MATCH_A>;
impl ADDR_MATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_MATCH_A {
        match self.bits {
            false => ADDR_MATCH_A::INACTIVE,
            true => ADDR_MATCH_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ADDR_MATCH_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADDR_MATCH_A::PENDING
    }
}
#[doc = "Write proxy for field `ADDR_MATCH`"]
pub struct ADDR_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_MATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR_MATCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(ADDR_MATCH_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ADDR_MATCH_A::PENDING)
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
#[doc = "Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_THRESH_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending. RX_FIFO equal or more bytes than the threshold."]
    PENDING = 1,
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
    pub fn variant(&self) -> RX_THRESH_A {
        match self.bits {
            false => RX_THRESH_A::INACTIVE,
            true => RX_THRESH_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RX_THRESH_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_THRESH_A::PENDING
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
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(RX_THRESH_A::INACTIVE)
    }
    #[doc = "An interrupt is pending. RX_FIFO equal or more bytes than the threshold."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RX_THRESH_A::PENDING)
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
#[doc = "Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_THRESH_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    PENDING = 1,
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
    pub fn variant(&self) -> TX_THRESH_A {
        match self.bits {
            false => TX_THRESH_A::INACTIVE,
            true => TX_THRESH_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TX_THRESH_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_THRESH_A::PENDING
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
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(TX_THRESH_A::INACTIVE)
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TX_THRESH_A::PENDING)
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
#[doc = "STOP Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    PENDING = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, STOP_A>;
impl STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::INACTIVE,
            true => STOP_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == STOP_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STOP_A::PENDING
    }
}
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(STOP_A::INACTIVE)
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(STOP_A::PENDING)
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
#[doc = "Address Acknowledge Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_ACK_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ADDR_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR_ACK`"]
pub type ADDR_ACK_R = crate::R<bool, ADDR_ACK_A>;
impl ADDR_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_ACK_A {
        match self.bits {
            false => ADDR_ACK_A::INACTIVE,
            true => ADDR_ACK_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ADDR_ACK_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADDR_ACK_A::PENDING
    }
}
#[doc = "Write proxy for field `ADDR_ACK`"]
pub struct ADDR_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR_ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(ADDR_ACK_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ADDR_ACK_A::PENDING)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Arbritation error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_ER_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ARB_ER_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_ER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARB_ER`"]
pub type ARB_ER_R = crate::R<bool, ARB_ER_A>;
impl ARB_ER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_ER_A {
        match self.bits {
            false => ARB_ER_A::INACTIVE,
            true => ARB_ER_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ARB_ER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ARB_ER_A::PENDING
    }
}
#[doc = "Write proxy for field `ARB_ER`"]
pub struct ARB_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_ER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARB_ER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(ARB_ER_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ARB_ER_A::PENDING)
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
#[doc = "timeout Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TO_ER_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<TO_ER_A> for bool {
    #[inline(always)]
    fn from(variant: TO_ER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TO_ER`"]
pub type TO_ER_R = crate::R<bool, TO_ER_A>;
impl TO_ER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_ER_A {
        match self.bits {
            false => TO_ER_A::INACTIVE,
            true => TO_ER_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TO_ER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TO_ER_A::PENDING
    }
}
#[doc = "Write proxy for field `TO_ER`"]
pub struct TO_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_ER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TO_ER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(TO_ER_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TO_ER_A::PENDING)
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
#[doc = "Address NACK Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_NACK_ER_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ADDR_NACK_ER_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_NACK_ER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR_NACK_ER`"]
pub type ADDR_NACK_ER_R = crate::R<bool, ADDR_NACK_ER_A>;
impl ADDR_NACK_ER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_NACK_ER_A {
        match self.bits {
            false => ADDR_NACK_ER_A::INACTIVE,
            true => ADDR_NACK_ER_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ADDR_NACK_ER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADDR_NACK_ER_A::PENDING
    }
}
#[doc = "Write proxy for field `ADDR_NACK_ER`"]
pub struct ADDR_NACK_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_NACK_ER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR_NACK_ER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(ADDR_NACK_ER_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ADDR_NACK_ER_A::PENDING)
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
#[doc = "Data NACK Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_ER_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DATA_ER_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_ER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA_ER`"]
pub type DATA_ER_R = crate::R<bool, DATA_ER_A>;
impl DATA_ER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_ER_A {
        match self.bits {
            false => DATA_ER_A::INACTIVE,
            true => DATA_ER_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DATA_ER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DATA_ER_A::PENDING
    }
}
#[doc = "Write proxy for field `DATA_ER`"]
pub struct DATA_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_ER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DATA_ER_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DATA_ER_A::PENDING)
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
#[doc = "Do Not Respond Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DO_NOT_RESP_ER_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DO_NOT_RESP_ER_A> for bool {
    #[inline(always)]
    fn from(variant: DO_NOT_RESP_ER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DO_NOT_RESP_ER`"]
pub type DO_NOT_RESP_ER_R = crate::R<bool, DO_NOT_RESP_ER_A>;
impl DO_NOT_RESP_ER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DO_NOT_RESP_ER_A {
        match self.bits {
            false => DO_NOT_RESP_ER_A::INACTIVE,
            true => DO_NOT_RESP_ER_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DO_NOT_RESP_ER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DO_NOT_RESP_ER_A::PENDING
    }
}
#[doc = "Write proxy for field `DO_NOT_RESP_ER`"]
pub struct DO_NOT_RESP_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_NOT_RESP_ER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DO_NOT_RESP_ER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DO_NOT_RESP_ER_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DO_NOT_RESP_ER_A::PENDING)
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
#[doc = "Start Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_ER_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<START_ER_A> for bool {
    #[inline(always)]
    fn from(variant: START_ER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START_ER`"]
pub type START_ER_R = crate::R<bool, START_ER_A>;
impl START_ER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_ER_A {
        match self.bits {
            false => START_ER_A::INACTIVE,
            true => START_ER_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == START_ER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == START_ER_A::PENDING
    }
}
#[doc = "Write proxy for field `START_ER`"]
pub struct START_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> START_ER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_ER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(START_ER_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(START_ER_A::PENDING)
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
#[doc = "Stop Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_ER_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<STOP_ER_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_ER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOP_ER`"]
pub type STOP_ER_R = crate::R<bool, STOP_ER_A>;
impl STOP_ER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_ER_A {
        match self.bits {
            false => STOP_ER_A::INACTIVE,
            true => STOP_ER_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == STOP_ER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STOP_ER_A::PENDING
    }
}
#[doc = "Write proxy for field `STOP_ER`"]
pub struct STOP_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_ER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_ER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(STOP_ER_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(STOP_ER_A::PENDING)
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
#[doc = "Reader of field `TX_LOCK_OUT`"]
pub type TX_LOCK_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_LOCK_OUT`"]
pub struct TX_LOCK_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LOCK_OUT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer Done Interrupt."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interactive Receive Interrupt."]
    #[inline(always)]
    pub fn rx_mode(&self) -> RX_MODE_R {
        RX_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave General Call Address Match Interrupt."]
    #[inline(always)]
    pub fn gen_call_addr(&self) -> GEN_CALL_ADDR_R {
        GEN_CALL_ADDR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&self) -> ADDR_MATCH_R {
        ADDR_MATCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
    #[inline(always)]
    pub fn rx_thresh(&self) -> RX_THRESH_R {
        RX_THRESH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
    #[inline(always)]
    pub fn tx_thresh(&self) -> TX_THRESH_R {
        TX_THRESH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STOP Interrupt."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Address Acknowledge Interrupt."]
    #[inline(always)]
    pub fn addr_ack(&self) -> ADDR_ACK_R {
        ADDR_ACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Arbritation error Interrupt."]
    #[inline(always)]
    pub fn arb_er(&self) -> ARB_ER_R {
        ARB_ER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - timeout Error Interrupt."]
    #[inline(always)]
    pub fn to_er(&self) -> TO_ER_R {
        TO_ER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Address NACK Error Interrupt."]
    #[inline(always)]
    pub fn addr_nack_er(&self) -> ADDR_NACK_ER_R {
        ADDR_NACK_ER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Data NACK Error Interrupt."]
    #[inline(always)]
    pub fn data_er(&self) -> DATA_ER_R {
        DATA_ER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Do Not Respond Error Interrupt."]
    #[inline(always)]
    pub fn do_not_resp_er(&self) -> DO_NOT_RESP_ER_R {
        DO_NOT_RESP_ER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Start Error Interrupt."]
    #[inline(always)]
    pub fn start_er(&self) -> START_ER_R {
        START_ER_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stop Error Interrupt."]
    #[inline(always)]
    pub fn stop_er(&self) -> STOP_ER_R {
        STOP_ER_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmit Lock Out Interrupt."]
    #[inline(always)]
    pub fn tx_lock_out(&self) -> TX_LOCK_OUT_R {
        TX_LOCK_OUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Done Interrupt."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 1 - Interactive Receive Interrupt."]
    #[inline(always)]
    pub fn rx_mode(&mut self) -> RX_MODE_W {
        RX_MODE_W { w: self }
    }
    #[doc = "Bit 2 - Slave General Call Address Match Interrupt."]
    #[inline(always)]
    pub fn gen_call_addr(&mut self) -> GEN_CALL_ADDR_W {
        GEN_CALL_ADDR_W { w: self }
    }
    #[doc = "Bit 3 - Slave Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&mut self) -> ADDR_MATCH_W {
        ADDR_MATCH_W { w: self }
    }
    #[doc = "Bit 4 - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
    #[inline(always)]
    pub fn rx_thresh(&mut self) -> RX_THRESH_W {
        RX_THRESH_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
    #[inline(always)]
    pub fn tx_thresh(&mut self) -> TX_THRESH_W {
        TX_THRESH_W { w: self }
    }
    #[doc = "Bit 6 - STOP Interrupt."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 7 - Address Acknowledge Interrupt."]
    #[inline(always)]
    pub fn addr_ack(&mut self) -> ADDR_ACK_W {
        ADDR_ACK_W { w: self }
    }
    #[doc = "Bit 8 - Arbritation error Interrupt."]
    #[inline(always)]
    pub fn arb_er(&mut self) -> ARB_ER_W {
        ARB_ER_W { w: self }
    }
    #[doc = "Bit 9 - timeout Error Interrupt."]
    #[inline(always)]
    pub fn to_er(&mut self) -> TO_ER_W {
        TO_ER_W { w: self }
    }
    #[doc = "Bit 10 - Address NACK Error Interrupt."]
    #[inline(always)]
    pub fn addr_nack_er(&mut self) -> ADDR_NACK_ER_W {
        ADDR_NACK_ER_W { w: self }
    }
    #[doc = "Bit 11 - Data NACK Error Interrupt."]
    #[inline(always)]
    pub fn data_er(&mut self) -> DATA_ER_W {
        DATA_ER_W { w: self }
    }
    #[doc = "Bit 12 - Do Not Respond Error Interrupt."]
    #[inline(always)]
    pub fn do_not_resp_er(&mut self) -> DO_NOT_RESP_ER_W {
        DO_NOT_RESP_ER_W { w: self }
    }
    #[doc = "Bit 13 - Start Error Interrupt."]
    #[inline(always)]
    pub fn start_er(&mut self) -> START_ER_W {
        START_ER_W { w: self }
    }
    #[doc = "Bit 14 - Stop Error Interrupt."]
    #[inline(always)]
    pub fn stop_er(&mut self) -> STOP_ER_W {
        STOP_ER_W { w: self }
    }
    #[doc = "Bit 15 - Transmit Lock Out Interrupt."]
    #[inline(always)]
    pub fn tx_lock_out(&mut self) -> TX_LOCK_OUT_W {
        TX_LOCK_OUT_W { w: self }
    }
}
