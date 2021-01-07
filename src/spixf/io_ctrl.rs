#[doc = "Reader of register IO_CTRL"]
pub type R = crate::R<u32, super::IO_CTRL>;
#[doc = "Writer for register IO_CTRL"]
pub type W = crate::W<u32, super::IO_CTRL>;
#[doc = "Register IO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::IO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SCLK drive Strength. This bit controls the drive strength on the SCLK pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_DS_A {
    #[doc = "0: Low drive strength."]
    LOW = 0,
    #[doc = "1: High drive strength."]
    HIGH = 1,
}
impl From<SCLK_DS_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_DS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLK_DS`"]
pub type SCLK_DS_R = crate::R<bool, SCLK_DS_A>;
impl SCLK_DS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_DS_A {
        match self.bits {
            false => SCLK_DS_A::LOW,
            true => SCLK_DS_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SCLK_DS_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SCLK_DS_A::HIGH
    }
}
#[doc = "Write proxy for field `SCLK_DS`"]
pub struct SCLK_DS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_DS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SCLK_DS_A::LOW)
    }
    #[doc = "High drive strength."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SCLK_DS_A::HIGH)
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
#[doc = "Slave Select Drive Strength. This bit controls the drive strength on the dedicated slave select pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_DS_A {
    #[doc = "0: Low drive strength."]
    LOW = 0,
    #[doc = "1: High drive strength."]
    HIGH = 1,
}
impl From<SS_DS_A> for bool {
    #[inline(always)]
    fn from(variant: SS_DS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SS_DS`"]
pub type SS_DS_R = crate::R<bool, SS_DS_A>;
impl SS_DS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_DS_A {
        match self.bits {
            false => SS_DS_A::LOW,
            true => SS_DS_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SS_DS_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SS_DS_A::HIGH
    }
}
#[doc = "Write proxy for field `SS_DS`"]
pub struct SS_DS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_DS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SS_DS_A::LOW)
    }
    #[doc = "High drive strength."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SS_DS_A::HIGH)
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
#[doc = "SDIO Drive Strength. This bit controls the drive strength of all SDIO pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_DS_A {
    #[doc = "0: Low drive strength."]
    LOW = 0,
    #[doc = "1: High drive strength."]
    HIGH = 1,
}
impl From<SDIO_DS_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO_DS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDIO_DS`"]
pub type SDIO_DS_R = crate::R<bool, SDIO_DS_A>;
impl SDIO_DS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_DS_A {
        match self.bits {
            false => SDIO_DS_A::LOW,
            true => SDIO_DS_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SDIO_DS_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SDIO_DS_A::HIGH
    }
}
#[doc = "Write proxy for field `SDIO_DS`"]
pub struct SDIO_DS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_DS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SDIO_DS_A::LOW)
    }
    #[doc = "High drive strength."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SDIO_DS_A::HIGH)
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
#[doc = "IO Pullup/Pulldown Control. These bits control the pullups and pulldowns associated with all SPI XiP SDIO pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PU_PD_CTRL_A {
    #[doc = "0: Tristate."]
    TRI_STATE = 0,
    #[doc = "1: Pull-Up."]
    PULL_UP = 1,
    #[doc = "2: Pull-Down."]
    PULL_DOWN = 2,
}
impl From<PU_PD_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: PU_PD_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PU_PD_CTRL`"]
pub type PU_PD_CTRL_R = crate::R<u8, PU_PD_CTRL_A>;
impl PU_PD_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PU_PD_CTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PU_PD_CTRL_A::TRI_STATE),
            1 => Val(PU_PD_CTRL_A::PULL_UP),
            2 => Val(PU_PD_CTRL_A::PULL_DOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TRI_STATE`"]
    #[inline(always)]
    pub fn is_tri_state(&self) -> bool {
        *self == PU_PD_CTRL_A::TRI_STATE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PU_PD_CTRL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PU_PD_CTRL_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `PU_PD_CTRL`"]
pub struct PU_PD_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PD_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU_PD_CTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Tristate."]
    #[inline(always)]
    pub fn tri_state(self) -> &'a mut W {
        self.variant(PU_PD_CTRL_A::TRI_STATE)
    }
    #[doc = "Pull-Up."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PU_PD_CTRL_A::PULL_UP)
    }
    #[doc = "Pull-Down."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PU_PD_CTRL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SCLK drive Strength. This bit controls the drive strength on the SCLK pin."]
    #[inline(always)]
    pub fn sclk_ds(&self) -> SCLK_DS_R {
        SCLK_DS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave Select Drive Strength. This bit controls the drive strength on the dedicated slave select pin."]
    #[inline(always)]
    pub fn ss_ds(&self) -> SS_DS_R {
        SS_DS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDIO Drive Strength. This bit controls the drive strength of all SDIO pins."]
    #[inline(always)]
    pub fn sdio_ds(&self) -> SDIO_DS_R {
        SDIO_DS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - IO Pullup/Pulldown Control. These bits control the pullups and pulldowns associated with all SPI XiP SDIO pins."]
    #[inline(always)]
    pub fn pu_pd_ctrl(&self) -> PU_PD_CTRL_R {
        PU_PD_CTRL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SCLK drive Strength. This bit controls the drive strength on the SCLK pin."]
    #[inline(always)]
    pub fn sclk_ds(&mut self) -> SCLK_DS_W {
        SCLK_DS_W { w: self }
    }
    #[doc = "Bit 1 - Slave Select Drive Strength. This bit controls the drive strength on the dedicated slave select pin."]
    #[inline(always)]
    pub fn ss_ds(&mut self) -> SS_DS_W {
        SS_DS_W { w: self }
    }
    #[doc = "Bit 2 - SDIO Drive Strength. This bit controls the drive strength of all SDIO pins."]
    #[inline(always)]
    pub fn sdio_ds(&mut self) -> SDIO_DS_W {
        SDIO_DS_W { w: self }
    }
    #[doc = "Bits 3:4 - IO Pullup/Pulldown Control. These bits control the pullups and pulldowns associated with all SPI XiP SDIO pins."]
    #[inline(always)]
    pub fn pu_pd_ctrl(&mut self) -> PU_PD_CTRL_W {
        PU_PD_CTRL_W { w: self }
    }
}
