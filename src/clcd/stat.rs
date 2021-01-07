#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Under FLow Interupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UFLO_A {
    #[doc = "0: No interrupt pending"]
    INACTIVE = 0,
    #[doc = "1: Interrupt pending"]
    PENDING = 1,
}
impl From<UFLO_A> for bool {
    #[inline(always)]
    fn from(variant: UFLO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UFLO`"]
pub type UFLO_R = crate::R<bool, UFLO_A>;
impl UFLO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UFLO_A {
        match self.bits {
            false => UFLO_A::INACTIVE,
            true => UFLO_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == UFLO_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == UFLO_A::PENDING
    }
}
#[doc = "Under FLow Interupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UFLO_AW {
    #[doc = "1: Clears the interrupt flag"]
    CLEAR = 1,
}
impl From<UFLO_AW> for bool {
    #[inline(always)]
    fn from(variant: UFLO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `UFLO`"]
pub struct UFLO_W<'a> {
    w: &'a mut W,
}
impl<'a> UFLO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UFLO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UFLO_AW::CLEAR)
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
#[doc = "Address Ready Interupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRRDY_A {
    #[doc = "0: No interrupt pending"]
    INACTIVE = 0,
    #[doc = "1: Interrupt pending"]
    PENDING = 1,
}
impl From<ADRRDY_A> for bool {
    #[inline(always)]
    fn from(variant: ADRRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADRRDY`"]
pub type ADRRDY_R = crate::R<bool, ADRRDY_A>;
impl ADRRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRRDY_A {
        match self.bits {
            false => ADRRDY_A::INACTIVE,
            true => ADRRDY_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ADRRDY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADRRDY_A::PENDING
    }
}
#[doc = "Address Ready Interupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRRDY_AW {
    #[doc = "1: Clears the interrupt flag"]
    CLEAR = 1,
}
impl From<ADRRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: ADRRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ADRRDY`"]
pub struct ADRRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADRRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADRRDY_AW::CLEAR)
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
#[doc = "VCI Interupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCI_A {
    #[doc = "0: No interrupt pending"]
    INACTIVE = 0,
    #[doc = "1: Interrupt pending"]
    PENDING = 1,
}
impl From<VCI_A> for bool {
    #[inline(always)]
    fn from(variant: VCI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCI`"]
pub type VCI_R = crate::R<bool, VCI_A>;
impl VCI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCI_A {
        match self.bits {
            false => VCI_A::INACTIVE,
            true => VCI_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == VCI_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == VCI_A::PENDING
    }
}
#[doc = "VCI Interupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCI_AW {
    #[doc = "1: Clears the interrupt flag"]
    CLEAR = 1,
}
impl From<VCI_AW> for bool {
    #[inline(always)]
    fn from(variant: VCI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VCI`"]
pub struct VCI_W<'a> {
    w: &'a mut W,
}
impl<'a> VCI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(VCI_AW::CLEAR)
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
#[doc = "BERR Interupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERR_A {
    #[doc = "0: No interrupt pending"]
    INACTIVE = 0,
    #[doc = "1: Interrupt pending"]
    PENDING = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BERR`"]
pub type BERR_R = crate::R<bool, BERR_A>;
impl BERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BERR_A {
        match self.bits {
            false => BERR_A::INACTIVE,
            true => BERR_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BERR_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BERR_A::PENDING
    }
}
#[doc = "BERR Interupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERR_AW {
    #[doc = "1: Clears the interrupt flag"]
    CLEAR = 1,
}
impl From<BERR_AW> for bool {
    #[inline(always)]
    fn from(variant: BERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BERR`"]
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BERR_AW::CLEAR)
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
#[doc = "LCD IDLE Staus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDIDLE_A {
    #[doc = "0: BUSY"]
    BUSY = 0,
    #[doc = "1: READY"]
    READY = 1,
}
impl From<LCDIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDIDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCDIDLE`"]
pub type LCDIDLE_R = crate::R<bool, LCDIDLE_A>;
impl LCDIDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDIDLE_A {
        match self.bits {
            false => LCDIDLE_A::BUSY,
            true => LCDIDLE_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == LCDIDLE_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LCDIDLE_A::READY
    }
}
#[doc = "Write proxy for field `LCDIDLE`"]
pub struct LCDIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDIDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDIDLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BUSY"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(LCDIDLE_A::BUSY)
    }
    #[doc = "READY"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(LCDIDLE_A::READY)
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
impl R {
    #[doc = "Bit 0 - Under FLow Interupt Status"]
    #[inline(always)]
    pub fn uflo(&self) -> UFLO_R {
        UFLO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Address Ready Interupt Status"]
    #[inline(always)]
    pub fn adrrdy(&self) -> ADRRDY_R {
        ADRRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VCI Interupt Status"]
    #[inline(always)]
    pub fn vci(&self) -> VCI_R {
        VCI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BERR Interupt Status"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD IDLE Staus"]
    #[inline(always)]
    pub fn lcdidle(&self) -> LCDIDLE_R {
        LCDIDLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Under FLow Interupt Status"]
    #[inline(always)]
    pub fn uflo(&mut self) -> UFLO_W {
        UFLO_W { w: self }
    }
    #[doc = "Bit 1 - Address Ready Interupt Status"]
    #[inline(always)]
    pub fn adrrdy(&mut self) -> ADRRDY_W {
        ADRRDY_W { w: self }
    }
    #[doc = "Bit 2 - VCI Interupt Status"]
    #[inline(always)]
    pub fn vci(&mut self) -> VCI_W {
        VCI_W { w: self }
    }
    #[doc = "Bit 3 - BERR Interupt Status"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
    #[doc = "Bit 8 - LCD IDLE Staus"]
    #[inline(always)]
    pub fn lcdidle(&mut self) -> LCDIDLE_W {
        LCDIDLE_W { w: self }
    }
}
