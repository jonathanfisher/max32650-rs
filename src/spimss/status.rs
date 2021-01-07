#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0x01"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Slave Select. If the SPI is in slave mode, this bit indicates if the SPI is selected. If the SPI is in master mode this bit has no meaning.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAS_A {
    #[doc = "0: `0`"]
    SELECTED = 0,
    #[doc = "1: `1`"]
    NOTSELECTED = 1,
}
impl From<SLAS_A> for bool {
    #[inline(always)]
    fn from(variant: SLAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLAS`"]
pub type SLAS_R = crate::R<bool, SLAS_A>;
impl SLAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAS_A {
        match self.bits {
            false => SLAS_A::SELECTED,
            true => SLAS_A::NOTSELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `SELECTED`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == SLAS_A::SELECTED
    }
    #[doc = "Checks if the value of the field is `NOTSELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == SLAS_A::NOTSELECTED
    }
}
#[doc = "Transmit Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXST_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<TXST_A> for bool {
    #[inline(always)]
    fn from(variant: TXST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXST`"]
pub type TXST_R = crate::R<bool, TXST_A>;
impl TXST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXST_A {
        match self.bits {
            false => TXST_A::IDLE,
            true => TXST_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TXST_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TXST_A::BUSY
    }
}
#[doc = "Transmit Underrun.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TUND_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<TUND_A> for bool {
    #[inline(always)]
    fn from(variant: TUND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TUND`"]
pub type TUND_R = crate::R<bool, TUND_A>;
impl TUND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TUND_A {
        match self.bits {
            false => TUND_A::NOEVENT,
            true => TUND_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TUND_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == TUND_A::OCCURRED
    }
}
#[doc = "Write proxy for field `TUND`"]
pub struct TUND_W<'a> {
    w: &'a mut W,
}
impl<'a> TUND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TUND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(TUND_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(TUND_A::OCCURRED)
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
#[doc = "Receive Overrun.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVR_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<ROVR_A> for bool {
    #[inline(always)]
    fn from(variant: ROVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROVR`"]
pub type ROVR_R = crate::R<bool, ROVR_A>;
impl ROVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVR_A {
        match self.bits {
            false => ROVR_A::NOEVENT,
            true => ROVR_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ROVR_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == ROVR_A::OCCURRED
    }
}
#[doc = "Write proxy for field `ROVR`"]
pub struct ROVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(ROVR_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(ROVR_A::OCCURRED)
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
#[doc = "Slave Mode Transaction Abort.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABT_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<ABT_A> for bool {
    #[inline(always)]
    fn from(variant: ABT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABT`"]
pub type ABT_R = crate::R<bool, ABT_A>;
impl ABT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABT_A {
        match self.bits {
            false => ABT_A::NOEVENT,
            true => ABT_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ABT_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == ABT_A::OCCURRED
    }
}
#[doc = "Write proxy for field `ABT`"]
pub struct ABT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(ABT_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(ABT_A::OCCURRED)
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
#[doc = "Collision.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COL_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<COL_A> for bool {
    #[inline(always)]
    fn from(variant: COL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COL`"]
pub type COL_R = crate::R<bool, COL_A>;
impl COL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COL_A {
        match self.bits {
            false => COL_A::NOEVENT,
            true => COL_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == COL_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == COL_A::OCCURRED
    }
}
#[doc = "Write proxy for field `COL`"]
pub struct COL_W<'a> {
    w: &'a mut W,
}
impl<'a> COL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(COL_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(COL_A::OCCURRED)
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
#[doc = "Transmit Overrun.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOVR_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<TOVR_A> for bool {
    #[inline(always)]
    fn from(variant: TOVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TOVR`"]
pub type TOVR_R = crate::R<bool, TOVR_A>;
impl TOVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOVR_A {
        match self.bits {
            false => TOVR_A::NOEVENT,
            true => TOVR_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TOVR_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == TOVR_A::OCCURRED
    }
}
#[doc = "Write proxy for field `TOVR`"]
pub struct TOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(TOVR_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(TOVR_A::OCCURRED)
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
#[doc = "SPI Interrupt Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRQ`"]
pub type IRQ_R = crate::R<bool, IRQ_A>;
impl IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ_A {
        match self.bits {
            false => IRQ_A::INACTIVE,
            true => IRQ_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IRQ_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IRQ_A::PENDING
    }
}
#[doc = "Write proxy for field `IRQ`"]
pub struct IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(IRQ_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(IRQ_A::PENDING)
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
impl R {
    #[doc = "Bit 0 - Slave Select. If the SPI is in slave mode, this bit indicates if the SPI is selected. If the SPI is in master mode this bit has no meaning."]
    #[inline(always)]
    pub fn slas(&self) -> SLAS_R {
        SLAS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Status."]
    #[inline(always)]
    pub fn txst(&self) -> TXST_R {
        TXST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Underrun."]
    #[inline(always)]
    pub fn tund(&self) -> TUND_R {
        TUND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Overrun."]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Mode Transaction Abort."]
    #[inline(always)]
    pub fn abt(&self) -> ABT_R {
        ABT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Collision."]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Overrun."]
    #[inline(always)]
    pub fn tovr(&self) -> TOVR_R {
        TOVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Request."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Transmit Underrun."]
    #[inline(always)]
    pub fn tund(&mut self) -> TUND_W {
        TUND_W { w: self }
    }
    #[doc = "Bit 3 - Receive Overrun."]
    #[inline(always)]
    pub fn rovr(&mut self) -> ROVR_W {
        ROVR_W { w: self }
    }
    #[doc = "Bit 4 - Slave Mode Transaction Abort."]
    #[inline(always)]
    pub fn abt(&mut self) -> ABT_W {
        ABT_W { w: self }
    }
    #[doc = "Bit 5 - Collision."]
    #[inline(always)]
    pub fn col(&mut self) -> COL_W {
        COL_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Overrun."]
    #[inline(always)]
    pub fn tovr(&mut self) -> TOVR_W {
        TOVR_W { w: self }
    }
    #[doc = "Bit 7 - SPI Interrupt Request."]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W {
        IRQ_W { w: self }
    }
}
