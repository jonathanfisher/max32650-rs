#[doc = "Reader of register INCSRU"]
pub type R = crate::R<u8, super::INCSRU>;
#[doc = "Writer for register INCSRU"]
pub type W = crate::W<u8, super::INCSRU>;
#[doc = "Register INCSRU `reset()`'s with value 0"]
impl crate::ResetValue for super::INCSRU {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Auto Set inpktrdy.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOSET_A {
    #[doc = "0: USBHS_INCSRL_inpktrdy must be set by firmware."]
    SET = 0,
    #[doc = "1: USBHS_INCSRL_inpktrdy is automatically set."]
    AUTO = 1,
}
impl From<AUTOSET_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTOSET`"]
pub type AUTOSET_R = crate::R<bool, AUTOSET_A>;
impl AUTOSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOSET_A {
        match self.bits {
            false => AUTOSET_A::SET,
            true => AUTOSET_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == AUTOSET_A::SET
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == AUTOSET_A::AUTO
    }
}
#[doc = "Write proxy for field `AUTOSET`"]
pub struct AUTOSET_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USBHS_INCSRL_inpktrdy must be set by firmware."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(AUTOSET_A::SET)
    }
    #[doc = "USBHS_INCSRL_inpktrdy is automatically set."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(AUTOSET_A::AUTO)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
#[doc = "Isochronous Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISO_A {
    #[doc = "0: Enable IN Bulk and IN interrupt transfers."]
    INTERRUPT = 0,
    #[doc = "1: Enable IN Isochronous transfers."]
    ISOCHRONOUS = 1,
}
impl From<ISO_A> for bool {
    #[inline(always)]
    fn from(variant: ISO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISO`"]
pub type ISO_R = crate::R<bool, ISO_A>;
impl ISO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO_A {
        match self.bits {
            false => ISO_A::INTERRUPT,
            true => ISO_A::ISOCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == ISO_A::INTERRUPT
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == ISO_A::ISOCHRONOUS
    }
}
#[doc = "Write proxy for field `ISO`"]
pub struct ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable IN Bulk and IN interrupt transfers."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(ISO_A::INTERRUPT)
    }
    #[doc = "Enable IN Isochronous transfers."]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(ISO_A::ISOCHRONOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Endpoint Direction Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Endpoint direction is OUT."]
    OUT = 0,
    #[doc = "1: Endpoint direction is IN."]
    IN = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::OUT,
            true => MODE_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == MODE_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == MODE_A::IN
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
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint direction is OUT."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(MODE_A::OUT)
    }
    #[doc = "Endpoint direction is IN."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(MODE_A::IN)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAREQEN_A {
    #[doc = "0: Disable DMA for this IN endpoint."]
    DIS = 0,
    #[doc = "1: Enable DMA for this IN endpoint."]
    EN = 1,
}
impl From<DMAREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAREQEN`"]
pub type DMAREQEN_R = crate::R<bool, DMAREQEN_A>;
impl DMAREQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAREQEN_A {
        match self.bits {
            false => DMAREQEN_A::DIS,
            true => DMAREQEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMAREQEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMAREQEN_A::EN
    }
}
#[doc = "Write proxy for field `DMAREQEN`"]
pub struct DMAREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAREQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable DMA for this IN endpoint."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAREQEN_A::DIS)
    }
    #[doc = "Enable DMA for this IN endpoint."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAREQEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Force In Data - Toggle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRCDATATOG_A {
    #[doc = "0: Toggle data-toglge only when an ACK is received."]
    RECEIVED = 0,
    #[doc = "1: Toggle data-toggle regardless of ACK."]
    DONTCARE = 1,
}
impl From<FRCDATATOG_A> for bool {
    #[inline(always)]
    fn from(variant: FRCDATATOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRCDATATOG`"]
pub type FRCDATATOG_R = crate::R<bool, FRCDATATOG_A>;
impl FRCDATATOG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRCDATATOG_A {
        match self.bits {
            false => FRCDATATOG_A::RECEIVED,
            true => FRCDATATOG_A::DONTCARE,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == FRCDATATOG_A::RECEIVED
    }
    #[doc = "Checks if the value of the field is `DONTCARE`"]
    #[inline(always)]
    pub fn is_dontcare(&self) -> bool {
        *self == FRCDATATOG_A::DONTCARE
    }
}
#[doc = "Write proxy for field `FRCDATATOG`"]
pub struct FRCDATATOG_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCDATATOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRCDATATOG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Toggle data-toglge only when an ACK is received."]
    #[inline(always)]
    pub fn received(self) -> &'a mut W {
        self.variant(FRCDATATOG_A::RECEIVED)
    }
    #[doc = "Toggle data-toggle regardless of ACK."]
    #[inline(always)]
    pub fn dontcare(self) -> &'a mut W {
        self.variant(FRCDATATOG_A::DONTCARE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "DMA Request Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAREQMODE_A {
    #[doc = "0: Enable DMA Request Mode 0."]
    _0 = 0,
    #[doc = "1: Enable DMA Request Mode 1."]
    _1 = 1,
}
impl From<DMAREQMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMAREQMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAREQMODE`"]
pub type DMAREQMODE_R = crate::R<bool, DMAREQMODE_A>;
impl DMAREQMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAREQMODE_A {
        match self.bits {
            false => DMAREQMODE_A::_0,
            true => DMAREQMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAREQMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAREQMODE_A::_1
    }
}
#[doc = "Write proxy for field `DMAREQMODE`"]
pub struct DMAREQMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAREQMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable DMA Request Mode 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAREQMODE_A::_0)
    }
    #[doc = "Enable DMA Request Mode 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAREQMODE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Double Packet Buffering Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPKTBUFDIS_A {
    #[doc = "0: Enable Double packet buffering."]
    EN = 0,
    #[doc = "1: Disable Double Packet Buffering."]
    DIS = 1,
}
impl From<DPKTBUFDIS_A> for bool {
    #[inline(always)]
    fn from(variant: DPKTBUFDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPKTBUFDIS`"]
pub type DPKTBUFDIS_R = crate::R<bool, DPKTBUFDIS_A>;
impl DPKTBUFDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPKTBUFDIS_A {
        match self.bits {
            false => DPKTBUFDIS_A::EN,
            true => DPKTBUFDIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DPKTBUFDIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DPKTBUFDIS_A::DIS
    }
}
#[doc = "Write proxy for field `DPKTBUFDIS`"]
pub struct DPKTBUFDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPKTBUFDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPKTBUFDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Double packet buffering."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DPKTBUFDIS_A::EN)
    }
    #[doc = "Disable Double Packet Buffering."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DPKTBUFDIS_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Auto Set inpktrdy."]
    #[inline(always)]
    pub fn autoset(&self) -> AUTOSET_R {
        AUTOSET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Isochronous Transfer Enable"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint Direction Mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA Request Enable"]
    #[inline(always)]
    pub fn dmareqen(&self) -> DMAREQEN_R {
        DMAREQEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force In Data - Toggle"]
    #[inline(always)]
    pub fn frcdatatog(&self) -> FRCDATATOG_R {
        FRCDATATOG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Request Mode Enable"]
    #[inline(always)]
    pub fn dmareqmode(&self) -> DMAREQMODE_R {
        DMAREQMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Double Packet Buffering Disable"]
    #[inline(always)]
    pub fn dpktbufdis(&self) -> DPKTBUFDIS_R {
        DPKTBUFDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Auto Set inpktrdy."]
    #[inline(always)]
    pub fn autoset(&mut self) -> AUTOSET_W {
        AUTOSET_W { w: self }
    }
    #[doc = "Bit 6 - Isochronous Transfer Enable"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W {
        ISO_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint Direction Mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - DMA Request Enable"]
    #[inline(always)]
    pub fn dmareqen(&mut self) -> DMAREQEN_W {
        DMAREQEN_W { w: self }
    }
    #[doc = "Bit 3 - Force In Data - Toggle"]
    #[inline(always)]
    pub fn frcdatatog(&mut self) -> FRCDATATOG_W {
        FRCDATATOG_W { w: self }
    }
    #[doc = "Bit 2 - DMA Request Mode Enable"]
    #[inline(always)]
    pub fn dmareqmode(&mut self) -> DMAREQMODE_W {
        DMAREQMODE_W { w: self }
    }
    #[doc = "Bit 1 - Double Packet Buffering Disable"]
    #[inline(always)]
    pub fn dpktbufdis(&mut self) -> DPKTBUFDIS_W {
        DPKTBUFDIS_W { w: self }
    }
}
