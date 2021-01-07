#[doc = "Reader of register RSTR1"]
pub type R = crate::R<u32, super::RSTR1>;
#[doc = "Writer for register RSTR1"]
pub type W = crate::W<u32, super::RSTR1>;
#[doc = "Register RSTR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<I2C1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C1`"]
pub type I2C1_R = crate::R<bool, I2C1_A>;
impl I2C1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_A {
        match self.bits {
            false => I2C1_A::RESET_DONE,
            true => I2C1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == I2C1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == I2C1_A::BUSY
    }
}
#[doc = "I2C1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<I2C1_AW> for bool {
    #[inline(always)]
    fn from(variant: I2C1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `I2C1`"]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(I2C1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C1_AW::RESET)
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
#[doc = "PT Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<PT_A> for bool {
    #[inline(always)]
    fn from(variant: PT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<bool, PT_A>;
impl PT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT_A {
        match self.bits {
            false => PT_A::RESET_DONE,
            true => PT_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == PT_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PT_A::BUSY
    }
}
#[doc = "PT Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<PT_AW> for bool {
    #[inline(always)]
    fn from(variant: PT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(PT_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PT_AW::RESET)
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
#[doc = "Reader of field `RSV_0X02`"]
pub type RSV_0X02_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSV_0X02`"]
pub struct RSV_0X02_W<'a> {
    w: &'a mut W,
}
impl<'a> RSV_0X02_W<'a> {
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
#[doc = "SPI XiP Master Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIXIP_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<SPIXIP_A> for bool {
    #[inline(always)]
    fn from(variant: SPIXIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIXIP`"]
pub type SPIXIP_R = crate::R<bool, SPIXIP_A>;
impl SPIXIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIXIP_A {
        match self.bits {
            false => SPIXIP_A::RESET_DONE,
            true => SPIXIP_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPIXIP_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPIXIP_A::BUSY
    }
}
#[doc = "SPI XiP Master Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIXIP_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPIXIP_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIXIP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPIXIP`"]
pub struct SPIXIP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIXIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIXIP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPIXIP_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPIXIP_AW::RESET)
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
#[doc = "GSPI XiP Master Controller Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XSPIM_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<XSPIM_A> for bool {
    #[inline(always)]
    fn from(variant: XSPIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XSPIM`"]
pub type XSPIM_R = crate::R<bool, XSPIM_A>;
impl XSPIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XSPIM_A {
        match self.bits {
            false => XSPIM_A::RESET_DONE,
            true => XSPIM_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == XSPIM_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == XSPIM_A::BUSY
    }
}
#[doc = "GSPI XiP Master Controller Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XSPIM_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<XSPIM_AW> for bool {
    #[inline(always)]
    fn from(variant: XSPIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `XSPIM`"]
pub struct XSPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XSPIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XSPIM_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(XSPIM_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(XSPIM_AW::RESET)
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
#[doc = "GPIO3 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<GPIO3_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO3`"]
pub type GPIO3_R = crate::R<bool, GPIO3_A>;
impl GPIO3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3_A {
        match self.bits {
            false => GPIO3_A::RESET_DONE,
            true => GPIO3_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == GPIO3_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == GPIO3_A::BUSY
    }
}
#[doc = "GPIO3 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<GPIO3_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIO3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `GPIO3`"]
pub struct GPIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(GPIO3_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIO3_AW::RESET)
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
#[doc = "SDHC/SDIO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHC_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<SDHC_A> for bool {
    #[inline(always)]
    fn from(variant: SDHC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDHC`"]
pub type SDHC_R = crate::R<bool, SDHC_A>;
impl SDHC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHC_A {
        match self.bits {
            false => SDHC_A::RESET_DONE,
            true => SDHC_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SDHC_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SDHC_A::BUSY
    }
}
#[doc = "SDHC/SDIO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHC_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SDHC_AW> for bool {
    #[inline(always)]
    fn from(variant: SDHC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SDHC`"]
pub struct SDHC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDHC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDHC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SDHC_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SDHC_AW::RESET)
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
#[doc = "OWIRE Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWIRE_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<OWIRE_A> for bool {
    #[inline(always)]
    fn from(variant: OWIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OWIRE`"]
pub type OWIRE_R = crate::R<bool, OWIRE_A>;
impl OWIRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWIRE_A {
        match self.bits {
            false => OWIRE_A::RESET_DONE,
            true => OWIRE_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == OWIRE_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == OWIRE_A::BUSY
    }
}
#[doc = "OWIRE Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWIRE_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<OWIRE_AW> for bool {
    #[inline(always)]
    fn from(variant: OWIRE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OWIRE`"]
pub struct OWIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> OWIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OWIRE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(OWIRE_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OWIRE_AW::RESET)
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
#[doc = "WDT1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT1_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<WDT1_A> for bool {
    #[inline(always)]
    fn from(variant: WDT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDT1`"]
pub type WDT1_R = crate::R<bool, WDT1_A>;
impl WDT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT1_A {
        match self.bits {
            false => WDT1_A::RESET_DONE,
            true => WDT1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == WDT1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == WDT1_A::BUSY
    }
}
#[doc = "WDT1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<WDT1_AW> for bool {
    #[inline(always)]
    fn from(variant: WDT1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `WDT1`"]
pub struct WDT1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(WDT1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDT1_AW::RESET)
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
#[doc = "SPI3 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI3_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<SPI3_A> for bool {
    #[inline(always)]
    fn from(variant: SPI3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI3`"]
pub type SPI3_R = crate::R<bool, SPI3_A>;
impl SPI3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI3_A {
        match self.bits {
            false => SPI3_A::RESET_DONE,
            true => SPI3_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPI3_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPI3_A::BUSY
    }
}
#[doc = "SPI3 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI3_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPI3_AW> for bool {
    #[inline(always)]
    fn from(variant: SPI3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPI3`"]
pub struct SPI3_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPI3_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI3_AW::RESET)
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
#[doc = "I2S Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<I2S_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2S`"]
pub type I2S_R = crate::R<bool, I2S_A>;
impl I2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_A {
        match self.bits {
            false => I2S_A::RESET_DONE,
            true => I2S_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == I2S_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == I2S_A::BUSY
    }
}
#[doc = "I2S Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<I2S_AW> for bool {
    #[inline(always)]
    fn from(variant: I2S_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `I2S`"]
pub struct I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(I2S_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2S_AW::RESET)
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
#[doc = "SPIMM0 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMM0_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<SPIMM0_A> for bool {
    #[inline(always)]
    fn from(variant: SPIMM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIMM0`"]
pub type SPIMM0_R = crate::R<bool, SPIMM0_A>;
impl SPIMM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIMM0_A {
        match self.bits {
            false => SPIMM0_A::RESET_DONE,
            true => SPIMM0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPIMM0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPIMM0_A::BUSY
    }
}
#[doc = "SPIMM0 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMM0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPIMM0_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIMM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPIMM0`"]
pub struct SPIMM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMM0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPIMM0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPIMM0_AW::RESET)
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
#[doc = "SPIMM1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMM1_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<SPIMM1_A> for bool {
    #[inline(always)]
    fn from(variant: SPIMM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIMM1`"]
pub type SPIMM1_R = crate::R<bool, SPIMM1_A>;
impl SPIMM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIMM1_A {
        match self.bits {
            false => SPIMM1_A::RESET_DONE,
            true => SPIMM1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPIMM1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPIMM1_A::BUSY
    }
}
#[doc = "SPIMM1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMM1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPIMM1_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIMM1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPIMM1`"]
pub struct SPIMM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMM1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPIMM1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPIMM1_AW::RESET)
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
#[doc = "SPIMM2 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMM2_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<SPIMM2_A> for bool {
    #[inline(always)]
    fn from(variant: SPIMM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIMM2`"]
pub type SPIMM2_R = crate::R<bool, SPIMM2_A>;
impl SPIMM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIMM2_A {
        match self.bits {
            false => SPIMM2_A::RESET_DONE,
            true => SPIMM2_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPIMM2_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPIMM2_A::BUSY
    }
}
#[doc = "SPIMM2 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMM2_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPIMM2_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIMM2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPIMM2`"]
pub struct SPIMM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMM2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPIMM2_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPIMM2_AW::RESET)
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
#[doc = "SPIMS0 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMS0_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<SPIMS0_A> for bool {
    #[inline(always)]
    fn from(variant: SPIMS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIMS0`"]
pub type SPIMS0_R = crate::R<bool, SPIMS0_A>;
impl SPIMS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIMS0_A {
        match self.bits {
            false => SPIMS0_A::RESET_DONE,
            true => SPIMS0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPIMS0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPIMS0_A::BUSY
    }
}
#[doc = "SPIMS0 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMS0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPIMS0_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIMS0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPIMS0`"]
pub struct SPIMS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMS0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPIMS0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPIMS0_AW::RESET)
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
#[doc = "SPIXMEM Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIXMEM_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<SPIXMEM_A> for bool {
    #[inline(always)]
    fn from(variant: SPIXMEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIXMEM`"]
pub type SPIXMEM_R = crate::R<bool, SPIXMEM_A>;
impl SPIXMEM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIXMEM_A {
        match self.bits {
            false => SPIXMEM_A::RESET_DONE,
            true => SPIXMEM_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPIXMEM_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPIXMEM_A::BUSY
    }
}
#[doc = "SPIXMEM Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIXMEM_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPIXMEM_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIXMEM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPIXMEM`"]
pub struct SPIXMEM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIXMEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIXMEM_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPIXMEM_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPIXMEM_AW::RESET)
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
#[doc = "SMPHR Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPHR_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<SMPHR_A> for bool {
    #[inline(always)]
    fn from(variant: SMPHR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPHR`"]
pub type SMPHR_R = crate::R<bool, SMPHR_A>;
impl SMPHR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPHR_A {
        match self.bits {
            false => SMPHR_A::RESET_DONE,
            true => SMPHR_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SMPHR_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SMPHR_A::BUSY
    }
}
#[doc = "SMPHR Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPHR_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SMPHR_AW> for bool {
    #[inline(always)]
    fn from(variant: SMPHR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SMPHR`"]
pub struct SMPHR_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPHR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPHR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SMPHR_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SMPHR_AW::RESET)
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
impl R {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PT Reset."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rsv_0x02(&self) -> RSV_0X02_R {
        RSV_0X02_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SPI XiP Master Reset."]
    #[inline(always)]
    pub fn spixip(&self) -> SPIXIP_R {
        SPIXIP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GSPI XiP Master Controller Reset."]
    #[inline(always)]
    pub fn xspim(&self) -> XSPIM_R {
        XSPIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO3 Reset."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SDHC/SDIO Reset."]
    #[inline(always)]
    pub fn sdhc(&self) -> SDHC_R {
        SDHC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OWIRE Reset."]
    #[inline(always)]
    pub fn owire(&self) -> OWIRE_R {
        OWIRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WDT1 Reset."]
    #[inline(always)]
    pub fn wdt1(&self) -> WDT1_R {
        WDT1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI3 Reset."]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - I2S Reset."]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SPIMM0 Reset."]
    #[inline(always)]
    pub fn spimm0(&self) -> SPIMM0_R {
        SPIMM0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPIMM1 Reset."]
    #[inline(always)]
    pub fn spimm1(&self) -> SPIMM1_R {
        SPIMM1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPIMM2 Reset."]
    #[inline(always)]
    pub fn spimm2(&self) -> SPIMM2_R {
        SPIMM2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPIMS0 Reset."]
    #[inline(always)]
    pub fn spims0(&self) -> SPIMS0_R {
        SPIMS0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPIXMEM Reset."]
    #[inline(always)]
    pub fn spixmem(&self) -> SPIXMEM_R {
        SPIXMEM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SMPHR Reset."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 1 - PT Reset."]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
    #[doc = "Bit 2 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rsv_0x02(&mut self) -> RSV_0X02_W {
        RSV_0X02_W { w: self }
    }
    #[doc = "Bit 3 - SPI XiP Master Reset."]
    #[inline(always)]
    pub fn spixip(&mut self) -> SPIXIP_W {
        SPIXIP_W { w: self }
    }
    #[doc = "Bit 4 - GSPI XiP Master Controller Reset."]
    #[inline(always)]
    pub fn xspim(&mut self) -> XSPIM_W {
        XSPIM_W { w: self }
    }
    #[doc = "Bit 5 - GPIO3 Reset."]
    #[inline(always)]
    pub fn gpio3(&mut self) -> GPIO3_W {
        GPIO3_W { w: self }
    }
    #[doc = "Bit 6 - SDHC/SDIO Reset."]
    #[inline(always)]
    pub fn sdhc(&mut self) -> SDHC_W {
        SDHC_W { w: self }
    }
    #[doc = "Bit 7 - OWIRE Reset."]
    #[inline(always)]
    pub fn owire(&mut self) -> OWIRE_W {
        OWIRE_W { w: self }
    }
    #[doc = "Bit 8 - WDT1 Reset."]
    #[inline(always)]
    pub fn wdt1(&mut self) -> WDT1_W {
        WDT1_W { w: self }
    }
    #[doc = "Bit 9 - SPI3 Reset."]
    #[inline(always)]
    pub fn spi3(&mut self) -> SPI3_W {
        SPI3_W { w: self }
    }
    #[doc = "Bit 10 - I2S Reset."]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2S_W {
        I2S_W { w: self }
    }
    #[doc = "Bit 11 - SPIMM0 Reset."]
    #[inline(always)]
    pub fn spimm0(&mut self) -> SPIMM0_W {
        SPIMM0_W { w: self }
    }
    #[doc = "Bit 12 - SPIMM1 Reset."]
    #[inline(always)]
    pub fn spimm1(&mut self) -> SPIMM1_W {
        SPIMM1_W { w: self }
    }
    #[doc = "Bit 13 - SPIMM2 Reset."]
    #[inline(always)]
    pub fn spimm2(&mut self) -> SPIMM2_W {
        SPIMM2_W { w: self }
    }
    #[doc = "Bit 14 - SPIMS0 Reset."]
    #[inline(always)]
    pub fn spims0(&mut self) -> SPIMS0_W {
        SPIMS0_W { w: self }
    }
    #[doc = "Bit 15 - SPIXMEM Reset."]
    #[inline(always)]
    pub fn spixmem(&mut self) -> SPIXMEM_W {
        SPIXMEM_W { w: self }
    }
    #[doc = "Bit 16 - SMPHR Reset."]
    #[inline(always)]
    pub fn smphr(&mut self) -> SMPHR_W {
        SMPHR_W { w: self }
    }
}
