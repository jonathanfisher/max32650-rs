#[doc = "Reader of register PERCKCN1"]
pub type R = crate::R<u32, super::PERCKCN1>;
#[doc = "Writer for register PERCKCN1"]
pub type W = crate::W<u32, super::PERCKCN1>;
#[doc = "Register PERCKCN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERCKCN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BTLE Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2D_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<UART2D_A> for bool {
    #[inline(always)]
    fn from(variant: UART2D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART2D`"]
pub type UART2D_R = crate::R<bool, UART2D_A>;
impl UART2D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2D_A {
        match self.bits {
            false => UART2D_A::EN,
            true => UART2D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART2D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART2D_A::DIS
    }
}
#[doc = "Write proxy for field `UART2D`"]
pub struct UART2D_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART2D_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART2D_A::DIS)
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
#[doc = "TRNG Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNGD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<TRNGD_A> for bool {
    #[inline(always)]
    fn from(variant: TRNGD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRNGD`"]
pub type TRNGD_R = crate::R<bool, TRNGD_A>;
impl TRNGD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNGD_A {
        match self.bits {
            false => TRNGD_A::EN,
            true => TRNGD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRNGD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRNGD_A::DIS
    }
}
#[doc = "Write proxy for field `TRNGD`"]
pub struct TRNGD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNGD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRNGD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRNGD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRNGD_A::DIS)
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
#[doc = "Secure Flash Controller Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLCD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<FLCD_A> for bool {
    #[inline(always)]
    fn from(variant: FLCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLCD`"]
pub type FLCD_R = crate::R<bool, FLCD_A>;
impl FLCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLCD_A {
        match self.bits {
            false => FLCD_A::EN,
            true => FLCD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLCD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLCD_A::DIS
    }
}
#[doc = "Write proxy for field `FLCD`"]
pub struct FLCD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLCD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLCD_A::DIS)
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
#[doc = "Hyper Bus Controller Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HBCD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<HBCD_A> for bool {
    #[inline(always)]
    fn from(variant: HBCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HBCD`"]
pub type HBCD_R = crate::R<bool, HBCD_A>;
impl HBCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HBCD_A {
        match self.bits {
            false => HBCD_A::EN,
            true => HBCD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == HBCD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HBCD_A::DIS
    }
}
#[doc = "Write proxy for field `HBCD`"]
pub struct HBCD_W<'a> {
    w: &'a mut W,
}
impl<'a> HBCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HBCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HBCD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HBCD_A::DIS)
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
#[doc = "Reader of field `RSV_0X05`"]
pub type RSV_0X05_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSV_0X05`"]
pub struct RSV_0X05_W<'a> {
    w: &'a mut W,
}
impl<'a> RSV_0X05_W<'a> {
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
#[doc = "GPIO3 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3D_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<GPIO3D_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO3D`"]
pub type GPIO3D_R = crate::R<bool, GPIO3D_A>;
impl GPIO3D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3D_A {
        match self.bits {
            false => GPIO3D_A::EN,
            true => GPIO3D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO3D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO3D_A::DIS
    }
}
#[doc = "Write proxy for field `GPIO3D`"]
pub struct GPIO3D_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO3D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO3D_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO3D_A::DIS)
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
#[doc = "System Cache Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCACHED_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<SCACHED_A> for bool {
    #[inline(always)]
    fn from(variant: SCACHED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCACHED`"]
pub type SCACHED_R = crate::R<bool, SCACHED_A>;
impl SCACHED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCACHED_A {
        match self.bits {
            false => SCACHED_A::EN,
            true => SCACHED_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SCACHED_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SCACHED_A::DIS
    }
}
#[doc = "Write proxy for field `SCACHED`"]
pub struct SCACHED_W<'a> {
    w: &'a mut W,
}
impl<'a> SCACHED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCACHED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SCACHED_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SCACHED_A::DIS)
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
#[doc = "SDMA Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMAD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<SDMAD_A> for bool {
    #[inline(always)]
    fn from(variant: SDMAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMAD`"]
pub type SDMAD_R = crate::R<bool, SDMAD_A>;
impl SDMAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMAD_A {
        match self.bits {
            false => SDMAD_A::EN,
            true => SDMAD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SDMAD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SDMAD_A::DIS
    }
}
#[doc = "Write proxy for field `SDMAD`"]
pub struct SDMAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SDMAD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SDMAD_A::DIS)
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
#[doc = "Semaphore Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPHRD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<SMPHRD_A> for bool {
    #[inline(always)]
    fn from(variant: SMPHRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPHRD`"]
pub type SMPHRD_R = crate::R<bool, SMPHRD_A>;
impl SMPHRD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPHRD_A {
        match self.bits {
            false => SMPHRD_A::EN,
            true => SMPHRD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SMPHRD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SMPHRD_A::DIS
    }
}
#[doc = "Write proxy for field `SMPHRD`"]
pub struct SMPHRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPHRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPHRD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SMPHRD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SMPHRD_A::DIS)
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
#[doc = "SDHC/SDIO Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHCD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<SDHCD_A> for bool {
    #[inline(always)]
    fn from(variant: SDHCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDHCD`"]
pub type SDHCD_R = crate::R<bool, SDHCD_A>;
impl SDHCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHCD_A {
        match self.bits {
            false => SDHCD_A::EN,
            true => SDHCD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SDHCD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SDHCD_A::DIS
    }
}
#[doc = "Write proxy for field `SDHCD`"]
pub struct SDHCD_W<'a> {
    w: &'a mut W,
}
impl<'a> SDHCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDHCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SDHCD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SDHCD_A::DIS)
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
#[doc = "ICache Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHED_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<ICACHED_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICACHED`"]
pub type ICACHED_R = crate::R<bool, ICACHED_A>;
impl ICACHED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHED_A {
        match self.bits {
            false => ICACHED_A::EN,
            true => ICACHED_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ICACHED_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ICACHED_A::DIS
    }
}
#[doc = "Write proxy for field `ICACHED`"]
pub struct ICACHED_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ICACHED_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ICACHED_A::DIS)
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
#[doc = "ICache XIP Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHEXIPD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<ICACHEXIPD_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHEXIPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICACHEXIPD`"]
pub type ICACHEXIPD_R = crate::R<bool, ICACHEXIPD_A>;
impl ICACHEXIPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHEXIPD_A {
        match self.bits {
            false => ICACHEXIPD_A::EN,
            true => ICACHEXIPD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ICACHEXIPD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ICACHEXIPD_A::DIS
    }
}
#[doc = "Write proxy for field `ICACHEXIPD`"]
pub struct ICACHEXIPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEXIPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHEXIPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ICACHEXIPD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ICACHEXIPD_A::DIS)
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
#[doc = "One-Wire Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWIRED_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<OWIRED_A> for bool {
    #[inline(always)]
    fn from(variant: OWIRED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OWIRED`"]
pub type OWIRED_R = crate::R<bool, OWIRED_A>;
impl OWIRED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWIRED_A {
        match self.bits {
            false => OWIRED_A::EN,
            true => OWIRED_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == OWIRED_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == OWIRED_A::DIS
    }
}
#[doc = "Write proxy for field `OWIRED`"]
pub struct OWIRED_W<'a> {
    w: &'a mut W,
}
impl<'a> OWIRED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OWIRED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(OWIRED_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(OWIRED_A::DIS)
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
#[doc = "SPI3 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI3D_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<SPI3D_A> for bool {
    #[inline(always)]
    fn from(variant: SPI3D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI3D`"]
pub type SPI3D_R = crate::R<bool, SPI3D_A>;
impl SPI3D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI3D_A {
        match self.bits {
            false => SPI3D_A::EN,
            true => SPI3D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPI3D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPI3D_A::DIS
    }
}
#[doc = "Write proxy for field `SPI3D`"]
pub struct SPI3D_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI3D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPI3D_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPI3D_A::DIS)
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
#[doc = "I2S(SPI_MSS) Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<I2SD_A> for bool {
    #[inline(always)]
    fn from(variant: I2SD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2SD`"]
pub type I2SD_R = crate::R<bool, I2SD_A>;
impl I2SD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SD_A {
        match self.bits {
            false => I2SD_A::EN,
            true => I2SD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2SD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2SD_A::DIS
    }
}
#[doc = "Write proxy for field `I2SD`"]
pub struct I2SD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2SD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2SD_A::DIS)
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
#[doc = "SPI Medical Master 0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMM0_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
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
            false => SPIMM0_A::EN,
            true => SPIMM0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPIMM0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPIMM0_A::DIS
    }
}
#[doc = "Write proxy for field `SPIMM0`"]
pub struct SPIMM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPIMM0_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPIMM0_A::DIS)
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
#[doc = "SPI Medical Master 1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMM1_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
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
            false => SPIMM1_A::EN,
            true => SPIMM1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPIMM1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPIMM1_A::DIS
    }
}
#[doc = "Write proxy for field `SPIMM1`"]
pub struct SPIMM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPIMM1_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPIMM1_A::DIS)
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
#[doc = "SPI Medical Master 2 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMM2_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
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
            false => SPIMM2_A::EN,
            true => SPIMM2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPIMM2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPIMM2_A::DIS
    }
}
#[doc = "Write proxy for field `SPIMM2`"]
pub struct SPIMM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPIMM2_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPIMM2_A::DIS)
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
#[doc = "SPI Medical Slave 0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMS0_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
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
            false => SPIMS0_A::EN,
            true => SPIMS0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPIMS0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPIMS0_A::DIS
    }
}
#[doc = "Write proxy for field `SPIMS0`"]
pub struct SPIMS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPIMS0_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPIMS0_A::DIS)
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
#[doc = "SPI-XIP Data Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIXIPDD_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<SPIXIPDD_A> for bool {
    #[inline(always)]
    fn from(variant: SPIXIPDD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIXIPDD`"]
pub type SPIXIPDD_R = crate::R<bool, SPIXIPDD_A>;
impl SPIXIPDD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIXIPDD_A {
        match self.bits {
            false => SPIXIPDD_A::EN,
            true => SPIXIPDD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPIXIPDD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPIXIPDD_A::DIS
    }
}
#[doc = "Write proxy for field `SPIXIPDD`"]
pub struct SPIXIPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIXIPDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIXIPDD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPIXIPDD_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPIXIPDD_A::DIS)
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
impl R {
    #[doc = "Bit 1 - BTLE Disable."]
    #[inline(always)]
    pub fn uart2d(&self) -> UART2D_R {
        UART2D_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TRNG Disable."]
    #[inline(always)]
    pub fn trngd(&self) -> TRNGD_R {
        TRNGD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Secure Flash Controller Disable."]
    #[inline(always)]
    pub fn flcd(&self) -> FLCD_R {
        FLCD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hyper Bus Controller Clock Disable."]
    #[inline(always)]
    pub fn hbcd(&self) -> HBCD_R {
        HBCD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rsv_0x05(&self) -> RSV_0X05_R {
        RSV_0X05_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO3 Clock Disable."]
    #[inline(always)]
    pub fn gpio3d(&self) -> GPIO3D_R {
        GPIO3D_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - System Cache Clock Disable."]
    #[inline(always)]
    pub fn scached(&self) -> SCACHED_R {
        SCACHED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SDMA Clock Disable."]
    #[inline(always)]
    pub fn sdmad(&self) -> SDMAD_R {
        SDMAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Semaphore Clock Disable."]
    #[inline(always)]
    pub fn smphrd(&self) -> SMPHRD_R {
        SMPHRD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SDHC/SDIO Clock Disable."]
    #[inline(always)]
    pub fn sdhcd(&self) -> SDHCD_R {
        SDHCD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ICache Clock Disable."]
    #[inline(always)]
    pub fn icached(&self) -> ICACHED_R {
        ICACHED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ICache XIP Clock Disable."]
    #[inline(always)]
    pub fn icachexipd(&self) -> ICACHEXIPD_R {
        ICACHEXIPD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - One-Wire Clock Disable."]
    #[inline(always)]
    pub fn owired(&self) -> OWIRED_R {
        OWIRED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI3 Clock Disable."]
    #[inline(always)]
    pub fn spi3d(&self) -> SPI3D_R {
        SPI3D_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2S(SPI_MSS) Clock Disable"]
    #[inline(always)]
    pub fn i2sd(&self) -> I2SD_R {
        I2SD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI Medical Master 0 Clock Disable."]
    #[inline(always)]
    pub fn spimm0(&self) -> SPIMM0_R {
        SPIMM0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SPI Medical Master 1 Clock Disable."]
    #[inline(always)]
    pub fn spimm1(&self) -> SPIMM1_R {
        SPIMM1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SPI Medical Master 2 Clock Disable."]
    #[inline(always)]
    pub fn spimm2(&self) -> SPIMM2_R {
        SPIMM2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SPI Medical Slave 0 Clock Disable."]
    #[inline(always)]
    pub fn spims0(&self) -> SPIMS0_R {
        SPIMS0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SPI-XIP Data Clock Disable"]
    #[inline(always)]
    pub fn spixipdd(&self) -> SPIXIPDD_R {
        SPIXIPDD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BTLE Disable."]
    #[inline(always)]
    pub fn uart2d(&mut self) -> UART2D_W {
        UART2D_W { w: self }
    }
    #[doc = "Bit 2 - TRNG Disable."]
    #[inline(always)]
    pub fn trngd(&mut self) -> TRNGD_W {
        TRNGD_W { w: self }
    }
    #[doc = "Bit 3 - Secure Flash Controller Disable."]
    #[inline(always)]
    pub fn flcd(&mut self) -> FLCD_W {
        FLCD_W { w: self }
    }
    #[doc = "Bit 4 - Hyper Bus Controller Clock Disable."]
    #[inline(always)]
    pub fn hbcd(&mut self) -> HBCD_W {
        HBCD_W { w: self }
    }
    #[doc = "Bit 5 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rsv_0x05(&mut self) -> RSV_0X05_W {
        RSV_0X05_W { w: self }
    }
    #[doc = "Bit 6 - GPIO3 Clock Disable."]
    #[inline(always)]
    pub fn gpio3d(&mut self) -> GPIO3D_W {
        GPIO3D_W { w: self }
    }
    #[doc = "Bit 7 - System Cache Clock Disable."]
    #[inline(always)]
    pub fn scached(&mut self) -> SCACHED_W {
        SCACHED_W { w: self }
    }
    #[doc = "Bit 8 - SDMA Clock Disable."]
    #[inline(always)]
    pub fn sdmad(&mut self) -> SDMAD_W {
        SDMAD_W { w: self }
    }
    #[doc = "Bit 9 - Semaphore Clock Disable."]
    #[inline(always)]
    pub fn smphrd(&mut self) -> SMPHRD_W {
        SMPHRD_W { w: self }
    }
    #[doc = "Bit 10 - SDHC/SDIO Clock Disable."]
    #[inline(always)]
    pub fn sdhcd(&mut self) -> SDHCD_W {
        SDHCD_W { w: self }
    }
    #[doc = "Bit 11 - ICache Clock Disable."]
    #[inline(always)]
    pub fn icached(&mut self) -> ICACHED_W {
        ICACHED_W { w: self }
    }
    #[doc = "Bit 12 - ICache XIP Clock Disable."]
    #[inline(always)]
    pub fn icachexipd(&mut self) -> ICACHEXIPD_W {
        ICACHEXIPD_W { w: self }
    }
    #[doc = "Bit 13 - One-Wire Clock Disable."]
    #[inline(always)]
    pub fn owired(&mut self) -> OWIRED_W {
        OWIRED_W { w: self }
    }
    #[doc = "Bit 14 - SPI3 Clock Disable."]
    #[inline(always)]
    pub fn spi3d(&mut self) -> SPI3D_W {
        SPI3D_W { w: self }
    }
    #[doc = "Bit 15 - I2S(SPI_MSS) Clock Disable"]
    #[inline(always)]
    pub fn i2sd(&mut self) -> I2SD_W {
        I2SD_W { w: self }
    }
    #[doc = "Bit 16 - SPI Medical Master 0 Clock Disable."]
    #[inline(always)]
    pub fn spimm0(&mut self) -> SPIMM0_W {
        SPIMM0_W { w: self }
    }
    #[doc = "Bit 17 - SPI Medical Master 1 Clock Disable."]
    #[inline(always)]
    pub fn spimm1(&mut self) -> SPIMM1_W {
        SPIMM1_W { w: self }
    }
    #[doc = "Bit 18 - SPI Medical Master 2 Clock Disable."]
    #[inline(always)]
    pub fn spimm2(&mut self) -> SPIMM2_W {
        SPIMM2_W { w: self }
    }
    #[doc = "Bit 19 - SPI Medical Slave 0 Clock Disable."]
    #[inline(always)]
    pub fn spims0(&mut self) -> SPIMS0_W {
        SPIMS0_W { w: self }
    }
    #[doc = "Bit 20 - SPI-XIP Data Clock Disable"]
    #[inline(always)]
    pub fn spixipdd(&mut self) -> SPIXIPDD_W {
        SPIXIPDD_W { w: self }
    }
}
