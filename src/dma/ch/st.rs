#[doc = "Reader of register ST"]
pub type R = crate::R<u32, super::ST>;
#[doc = "Writer for register ST"]
pub type W = crate::W<u32, super::ST>;
#[doc = "Register ST `reset()`'s with value 0"]
impl crate::ResetValue for super::ST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel Status. This bit is used to indicate to the programmer when it is safe to change the configuration, address, and count registers for the channel. Whenever this bit is cleared by hardware, the DMA_CFG.CHEN bit is also cleared (if not cleared already).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_ST_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CH_ST_A> for bool {
    #[inline(always)]
    fn from(variant: CH_ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH_ST`"]
pub type CH_ST_R = crate::R<bool, CH_ST_A>;
impl CH_ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_ST_A {
        match self.bits {
            false => CH_ST_A::DIS,
            true => CH_ST_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH_ST_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH_ST_A::EN
    }
}
#[doc = "Channel Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPEND_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IPEND_A> for bool {
    #[inline(always)]
    fn from(variant: IPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPEND`"]
pub type IPEND_R = crate::R<bool, IPEND_A>;
impl IPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEND_A {
        match self.bits {
            false => IPEND_A::INACTIVE,
            true => IPEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IPEND_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IPEND_A::PENDING
    }
}
#[doc = "Count-to-Zero (CTZ) Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTZ_ST_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<CTZ_ST_A> for bool {
    #[inline(always)]
    fn from(variant: CTZ_ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTZ_ST`"]
pub type CTZ_ST_R = crate::R<bool, CTZ_ST_A>;
impl CTZ_ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTZ_ST_A {
        match self.bits {
            false => CTZ_ST_A::NOEVENT,
            true => CTZ_ST_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CTZ_ST_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == CTZ_ST_A::OCCURRED
    }
}
#[doc = "Count-to-Zero (CTZ) Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTZ_ST_AW {
    #[doc = "1: Clears the interrupt flag"]
    CLEAR = 1,
}
impl From<CTZ_ST_AW> for bool {
    #[inline(always)]
    fn from(variant: CTZ_ST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTZ_ST`"]
pub struct CTZ_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> CTZ_ST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTZ_ST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTZ_ST_AW::CLEAR)
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
#[doc = "Reload Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLD_ST_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<RLD_ST_A> for bool {
    #[inline(always)]
    fn from(variant: RLD_ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RLD_ST`"]
pub type RLD_ST_R = crate::R<bool, RLD_ST_A>;
impl RLD_ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLD_ST_A {
        match self.bits {
            false => RLD_ST_A::NOEVENT,
            true => RLD_ST_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RLD_ST_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == RLD_ST_A::OCCURRED
    }
}
#[doc = "Reload Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLD_ST_AW {
    #[doc = "1: Clears the interrupt flag"]
    CLEAR = 1,
}
impl From<RLD_ST_AW> for bool {
    #[inline(always)]
    fn from(variant: RLD_ST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RLD_ST`"]
pub struct RLD_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RLD_ST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLD_ST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RLD_ST_AW::CLEAR)
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
#[doc = "Bus Error. Indicates that an AHB abort was received and the channel has been disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_ERR_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<BUS_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUS_ERR`"]
pub type BUS_ERR_R = crate::R<bool, BUS_ERR_A>;
impl BUS_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_ERR_A {
        match self.bits {
            false => BUS_ERR_A::NOEVENT,
            true => BUS_ERR_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == BUS_ERR_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == BUS_ERR_A::OCCURRED
    }
}
#[doc = "Bus Error. Indicates that an AHB abort was received and the channel has been disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_ERR_AW {
    #[doc = "1: Clears the interrupt flag"]
    CLEAR = 1,
}
impl From<BUS_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: BUS_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BUS_ERR`"]
pub struct BUS_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUS_ERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BUS_ERR_AW::CLEAR)
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
#[doc = "Time-Out Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TO_ST_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<TO_ST_A> for bool {
    #[inline(always)]
    fn from(variant: TO_ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TO_ST`"]
pub type TO_ST_R = crate::R<bool, TO_ST_A>;
impl TO_ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_ST_A {
        match self.bits {
            false => TO_ST_A::NOEVENT,
            true => TO_ST_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TO_ST_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == TO_ST_A::OCCURRED
    }
}
#[doc = "Time-Out Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TO_ST_AW {
    #[doc = "1: Clears the interrupt flag"]
    CLEAR = 1,
}
impl From<TO_ST_AW> for bool {
    #[inline(always)]
    fn from(variant: TO_ST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TO_ST`"]
pub struct TO_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_ST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TO_ST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TO_ST_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Channel Status. This bit is used to indicate to the programmer when it is safe to change the configuration, address, and count registers for the channel. Whenever this bit is cleared by hardware, the DMA_CFG.CHEN bit is also cleared (if not cleared already)."]
    #[inline(always)]
    pub fn ch_st(&self) -> CH_ST_R {
        CH_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Interrupt."]
    #[inline(always)]
    pub fn ipend(&self) -> IPEND_R {
        IPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Count-to-Zero (CTZ) Status"]
    #[inline(always)]
    pub fn ctz_st(&self) -> CTZ_ST_R {
        CTZ_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reload Status."]
    #[inline(always)]
    pub fn rld_st(&self) -> RLD_ST_R {
        RLD_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
    #[inline(always)]
    pub fn bus_err(&self) -> BUS_ERR_R {
        BUS_ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Time-Out Status."]
    #[inline(always)]
    pub fn to_st(&self) -> TO_ST_R {
        TO_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Count-to-Zero (CTZ) Status"]
    #[inline(always)]
    pub fn ctz_st(&mut self) -> CTZ_ST_W {
        CTZ_ST_W { w: self }
    }
    #[doc = "Bit 3 - Reload Status."]
    #[inline(always)]
    pub fn rld_st(&mut self) -> RLD_ST_W {
        RLD_ST_W { w: self }
    }
    #[doc = "Bit 4 - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
    #[inline(always)]
    pub fn bus_err(&mut self) -> BUS_ERR_W {
        BUS_ERR_W { w: self }
    }
    #[doc = "Bit 6 - Time-Out Status."]
    #[inline(always)]
    pub fn to_st(&mut self) -> TO_ST_W {
        TO_ST_W { w: self }
    }
}
