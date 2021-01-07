#[doc = "Reader of register INT_EN"]
pub type R = crate::R<u32, super::INT_EN>;
#[doc = "Writer for register INT_EN"]
pub type W = crate::W<u32, super::INT_EN>;
#[doc = "Register INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TX FIFO Threshold interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_THRESH_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
            false => TX_THRESH_A::DIS,
            true => TX_THRESH_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_THRESH_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_THRESH_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_THRESH_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_THRESH_A::EN)
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
#[doc = "TX FIFO Empty interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_EMPTY_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
            false => TX_EMPTY_A::DIS,
            true => TX_EMPTY_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_EMPTY_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_EMPTY_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_EMPTY_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_EMPTY_A::EN)
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
#[doc = "RX FIFO Threshold Crossed interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_THRESH_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
            false => RX_THRESH_A::DIS,
            true => RX_THRESH_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_THRESH_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_THRESH_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_THRESH_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_THRESH_A::EN)
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
#[doc = "RX FIFO FULL interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FULL_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
            false => RX_FULL_A::DIS,
            true => RX_FULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_FULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_FULL_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_FULL_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_FULL_A::EN)
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
#[doc = "Slave Select Asserted interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSA_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
    pub fn variant(&self) -> SSA_A {
        match self.bits {
            false => SSA_A::DIS,
            true => SSA_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SSA_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SSA_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SSA_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SSA_A::EN)
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
#[doc = "Slave Select Deasserted interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSD_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
    pub fn variant(&self) -> SSD_A {
        match self.bits {
            false => SSD_A::DIS,
            true => SSD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SSD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SSD_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SSD_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SSD_A::EN)
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
#[doc = "Multi-Master Mode Fault interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
    pub fn variant(&self) -> FAULT_A {
        match self.bits {
            false => FAULT_A::DIS,
            true => FAULT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FAULT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FAULT_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FAULT_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FAULT_A::EN)
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
#[doc = "Slave Abort Detected interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
    pub fn variant(&self) -> ABORT_A {
        match self.bits {
            false => ABORT_A::DIS,
            true => ABORT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ABORT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ABORT_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ABORT_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ABORT_A::EN)
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
#[doc = "Master Done interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M_DONE_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
    pub fn variant(&self) -> M_DONE_A {
        match self.bits {
            false => M_DONE_A::DIS,
            true => M_DONE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == M_DONE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == M_DONE_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(M_DONE_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(M_DONE_A::EN)
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
#[doc = "Transmit FIFO Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_OVR_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
    pub fn variant(&self) -> TX_OVR_A {
        match self.bits {
            false => TX_OVR_A::DIS,
            true => TX_OVR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_OVR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_OVR_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_OVR_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_OVR_A::EN)
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
#[doc = "Transmit FIFO Underrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_UND_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
    pub fn variant(&self) -> TX_UND_A {
        match self.bits {
            false => TX_UND_A::DIS,
            true => TX_UND_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_UND_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_UND_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_UND_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_UND_A::EN)
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
#[doc = "Receive FIFO Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_OVR_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
    pub fn variant(&self) -> RX_OVR_A {
        match self.bits {
            false => RX_OVR_A::DIS,
            true => RX_OVR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_OVR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_OVR_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_OVR_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_OVR_A::EN)
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
#[doc = "Receive FIFO Underrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_UND_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
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
    pub fn variant(&self) -> RX_UND_A {
        match self.bits {
            false => RX_UND_A::DIS,
            true => RX_UND_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_UND_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_UND_A::EN
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
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_UND_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_UND_A::EN)
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
impl R {
    #[doc = "Bit 0 - TX FIFO Threshold interrupt enable."]
    #[inline(always)]
    pub fn tx_thresh(&self) -> TX_THRESH_R {
        TX_THRESH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty interrupt enable."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed interrupt enable."]
    #[inline(always)]
    pub fn rx_thresh(&self) -> RX_THRESH_R {
        RX_THRESH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX FIFO FULL interrupt enable."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Select Asserted interrupt enable."]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deasserted interrupt enable."]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault interrupt enable."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Slave Abort Detected interrupt enable."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Master Done interrupt enable."]
    #[inline(always)]
    pub fn m_done(&self) -> M_DONE_R {
        M_DONE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn tx_ovr(&self) -> TX_OVR_R {
        TX_OVR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn tx_und(&self) -> TX_UND_R {
        TX_UND_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn rx_ovr(&self) -> RX_OVR_R {
        RX_OVR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn rx_und(&self) -> RX_UND_R {
        RX_UND_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Threshold interrupt enable."]
    #[inline(always)]
    pub fn tx_thresh(&mut self) -> TX_THRESH_W {
        TX_THRESH_W { w: self }
    }
    #[doc = "Bit 1 - TX FIFO Empty interrupt enable."]
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W {
        TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed interrupt enable."]
    #[inline(always)]
    pub fn rx_thresh(&mut self) -> RX_THRESH_W {
        RX_THRESH_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFO FULL interrupt enable."]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RX_FULL_W {
        RX_FULL_W { w: self }
    }
    #[doc = "Bit 4 - Slave Select Asserted interrupt enable."]
    #[inline(always)]
    pub fn ssa(&mut self) -> SSA_W {
        SSA_W { w: self }
    }
    #[doc = "Bit 5 - Slave Select Deasserted interrupt enable."]
    #[inline(always)]
    pub fn ssd(&mut self) -> SSD_W {
        SSD_W { w: self }
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault interrupt enable."]
    #[inline(always)]
    pub fn fault(&mut self) -> FAULT_W {
        FAULT_W { w: self }
    }
    #[doc = "Bit 9 - Slave Abort Detected interrupt enable."]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bit 11 - Master Done interrupt enable."]
    #[inline(always)]
    pub fn m_done(&mut self) -> M_DONE_W {
        M_DONE_W { w: self }
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn tx_ovr(&mut self) -> TX_OVR_W {
        TX_OVR_W { w: self }
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn tx_und(&mut self) -> TX_UND_W {
        TX_UND_W { w: self }
    }
    #[doc = "Bit 14 - Receive FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn rx_ovr(&mut self) -> RX_OVR_W {
        RX_OVR_W { w: self }
    }
    #[doc = "Bit 15 - Receive FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn rx_und(&mut self) -> RX_UND_W {
        RX_UND_W { w: self }
    }
}
