#[doc = "Reader of register PERCKCN0"]
pub type R = crate::R<u32, super::PERCKCN0>;
#[doc = "Writer for register PERCKCN0"]
pub type W = crate::W<u32, super::PERCKCN0>;
#[doc = "Register PERCKCN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PERCKCN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<GPIO0D_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO0D`"]
pub type GPIO0D_R = crate::R<bool, GPIO0D_A>;
impl GPIO0D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0D_A {
        match self.bits {
            false => GPIO0D_A::EN,
            true => GPIO0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO0D_A::DIS
    }
}
#[doc = "Write proxy for field `GPIO0D`"]
pub struct GPIO0D_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO0D_A::DIS)
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
#[doc = "GPIO1 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<GPIO1D_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO1D`"]
pub type GPIO1D_R = crate::R<bool, GPIO1D_A>;
impl GPIO1D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1D_A {
        match self.bits {
            false => GPIO1D_A::EN,
            true => GPIO1D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO1D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO1D_A::DIS
    }
}
#[doc = "Write proxy for field `GPIO1D`"]
pub struct GPIO1D_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO1D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO1D_A::DIS)
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
#[doc = "GPIO2 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<GPIO2D_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO2D`"]
pub type GPIO2D_R = crate::R<bool, GPIO2D_A>;
impl GPIO2D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2D_A {
        match self.bits {
            false => GPIO2D_A::EN,
            true => GPIO2D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO2D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO2D_A::DIS
    }
}
#[doc = "Write proxy for field `GPIO2D`"]
pub struct GPIO2D_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO2D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO2D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO2D_A::DIS)
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
#[doc = "USB Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBD_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<USBD_A> for bool {
    #[inline(always)]
    fn from(variant: USBD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBD`"]
pub type USBD_R = crate::R<bool, USBD_A>;
impl USBD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBD_A {
        match self.bits {
            false => USBD_A::EN,
            true => USBD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == USBD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == USBD_A::DIS
    }
}
#[doc = "Write proxy for field `USBD`"]
pub struct USBD_W<'a> {
    w: &'a mut W,
}
impl<'a> USBD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(USBD_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(USBD_A::DIS)
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
#[doc = "CLCD Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLCDD_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<CLCDD_A> for bool {
    #[inline(always)]
    fn from(variant: CLCDD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLCDD`"]
pub type CLCDD_R = crate::R<bool, CLCDD_A>;
impl CLCDD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLCDD_A {
        match self.bits {
            false => CLCDD_A::EN,
            true => CLCDD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CLCDD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CLCDD_A::DIS
    }
}
#[doc = "Write proxy for field `CLCDD`"]
pub struct CLCDD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLCDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLCDD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CLCDD_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CLCDD_A::DIS)
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
#[doc = "DMA Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAD_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<DMAD_A> for bool {
    #[inline(always)]
    fn from(variant: DMAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAD`"]
pub type DMAD_R = crate::R<bool, DMAD_A>;
impl DMAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAD_A {
        match self.bits {
            false => DMAD_A::EN,
            true => DMAD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMAD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMAD_A::DIS
    }
}
#[doc = "Write proxy for field `DMAD`"]
pub struct DMAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAD_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAD_A::DIS)
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
#[doc = "SPI 0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<SPI0D_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0D`"]
pub type SPI0D_R = crate::R<bool, SPI0D_A>;
impl SPI0D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0D_A {
        match self.bits {
            false => SPI0D_A::EN,
            true => SPI0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPI0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPI0D_A::DIS
    }
}
#[doc = "Write proxy for field `SPI0D`"]
pub struct SPI0D_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPI0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPI0D_A::DIS)
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
#[doc = "SPI 1 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<SPI1D_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1D`"]
pub type SPI1D_R = crate::R<bool, SPI1D_A>;
impl SPI1D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1D_A {
        match self.bits {
            false => SPI1D_A::EN,
            true => SPI1D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPI1D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPI1D_A::DIS
    }
}
#[doc = "Write proxy for field `SPI1D`"]
pub struct SPI1D_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPI1D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPI1D_A::DIS)
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
#[doc = "SPI 2 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI2D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<SPI2D_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI2D`"]
pub type SPI2D_R = crate::R<bool, SPI2D_A>;
impl SPI2D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2D_A {
        match self.bits {
            false => SPI2D_A::EN,
            true => SPI2D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPI2D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPI2D_A::DIS
    }
}
#[doc = "Write proxy for field `SPI2D`"]
pub struct SPI2D_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPI2D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPI2D_A::DIS)
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
#[doc = "UART 0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<UART0D_A> for bool {
    #[inline(always)]
    fn from(variant: UART0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0D`"]
pub type UART0D_R = crate::R<bool, UART0D_A>;
impl UART0D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0D_A {
        match self.bits {
            false => UART0D_A::EN,
            true => UART0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART0D_A::DIS
    }
}
#[doc = "Write proxy for field `UART0D`"]
pub struct UART0D_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART0D_A::DIS)
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
#[doc = "UART 1 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<UART1D_A> for bool {
    #[inline(always)]
    fn from(variant: UART1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART1D`"]
pub type UART1D_R = crate::R<bool, UART1D_A>;
impl UART1D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1D_A {
        match self.bits {
            false => UART1D_A::EN,
            true => UART1D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART1D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART1D_A::DIS
    }
}
#[doc = "Write proxy for field `UART1D`"]
pub struct UART1D_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART1D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART1D_A::DIS)
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
#[doc = "I2C 0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<I2C0D_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0D`"]
pub type I2C0D_R = crate::R<bool, I2C0D_A>;
impl I2C0D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0D_A {
        match self.bits {
            false => I2C0D_A::EN,
            true => I2C0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C0D_A::DIS
    }
}
#[doc = "Write proxy for field `I2C0D`"]
pub struct I2C0D_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C0D_A::DIS)
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
#[doc = "Crypto Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTOD_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<CRYPTOD_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPTOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYPTOD`"]
pub type CRYPTOD_R = crate::R<bool, CRYPTOD_A>;
impl CRYPTOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPTOD_A {
        match self.bits {
            false => CRYPTOD_A::EN,
            true => CRYPTOD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CRYPTOD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CRYPTOD_A::DIS
    }
}
#[doc = "Write proxy for field `CRYPTOD`"]
pub struct CRYPTOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CRYPTOD_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CRYPTOD_A::DIS)
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
#[doc = "Timer 0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<T0D_A> for bool {
    #[inline(always)]
    fn from(variant: T0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `T0D`"]
pub type T0D_R = crate::R<bool, T0D_A>;
impl T0D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T0D_A {
        match self.bits {
            false => T0D_A::EN,
            true => T0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == T0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == T0D_A::DIS
    }
}
#[doc = "Write proxy for field `T0D`"]
pub struct T0D_W<'a> {
    w: &'a mut W,
}
impl<'a> T0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T0D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(T0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(T0D_A::DIS)
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
#[doc = "Timer 1 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T1D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<T1D_A> for bool {
    #[inline(always)]
    fn from(variant: T1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `T1D`"]
pub type T1D_R = crate::R<bool, T1D_A>;
impl T1D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T1D_A {
        match self.bits {
            false => T1D_A::EN,
            true => T1D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == T1D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == T1D_A::DIS
    }
}
#[doc = "Write proxy for field `T1D`"]
pub struct T1D_W<'a> {
    w: &'a mut W,
}
impl<'a> T1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T1D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(T1D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(T1D_A::DIS)
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
#[doc = "Timer 2 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T2D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<T2D_A> for bool {
    #[inline(always)]
    fn from(variant: T2D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `T2D`"]
pub type T2D_R = crate::R<bool, T2D_A>;
impl T2D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T2D_A {
        match self.bits {
            false => T2D_A::EN,
            true => T2D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == T2D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == T2D_A::DIS
    }
}
#[doc = "Write proxy for field `T2D`"]
pub struct T2D_W<'a> {
    w: &'a mut W,
}
impl<'a> T2D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T2D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(T2D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(T2D_A::DIS)
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
#[doc = "Timer 3 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T3D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<T3D_A> for bool {
    #[inline(always)]
    fn from(variant: T3D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `T3D`"]
pub type T3D_R = crate::R<bool, T3D_A>;
impl T3D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T3D_A {
        match self.bits {
            false => T3D_A::EN,
            true => T3D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == T3D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == T3D_A::DIS
    }
}
#[doc = "Write proxy for field `T3D`"]
pub struct T3D_W<'a> {
    w: &'a mut W,
}
impl<'a> T3D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T3D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(T3D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(T3D_A::DIS)
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
#[doc = "Timer 4 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T4D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<T4D_A> for bool {
    #[inline(always)]
    fn from(variant: T4D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `T4D`"]
pub type T4D_R = crate::R<bool, T4D_A>;
impl T4D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T4D_A {
        match self.bits {
            false => T4D_A::EN,
            true => T4D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == T4D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == T4D_A::DIS
    }
}
#[doc = "Write proxy for field `T4D`"]
pub struct T4D_W<'a> {
    w: &'a mut W,
}
impl<'a> T4D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T4D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(T4D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(T4D_A::DIS)
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
#[doc = "Timer 5 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T5D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<T5D_A> for bool {
    #[inline(always)]
    fn from(variant: T5D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `T5D`"]
pub type T5D_R = crate::R<bool, T5D_A>;
impl T5D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T5D_A {
        match self.bits {
            false => T5D_A::EN,
            true => T5D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == T5D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == T5D_A::DIS
    }
}
#[doc = "Write proxy for field `T5D`"]
pub struct T5D_W<'a> {
    w: &'a mut W,
}
impl<'a> T5D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T5D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(T5D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(T5D_A::DIS)
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
#[doc = "ADC Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCD_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<ADCD_A> for bool {
    #[inline(always)]
    fn from(variant: ADCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCD`"]
pub type ADCD_R = crate::R<bool, ADCD_A>;
impl ADCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCD_A {
        match self.bits {
            false => ADCD_A::EN,
            true => ADCD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADCD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADCD_A::DIS
    }
}
#[doc = "Write proxy for field `ADCD`"]
pub struct ADCD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADCD_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCD_A::DIS)
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
#[doc = "Reader of field `RSV_0X19`"]
pub type RSV_0X19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSV_0X19`"]
pub struct RSV_0X19_W<'a> {
    w: &'a mut W,
}
impl<'a> RSV_0X19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "I2C 1 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<I2C1D_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C1D`"]
pub type I2C1D_R = crate::R<bool, I2C1D_A>;
impl I2C1D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1D_A {
        match self.bits {
            false => I2C1D_A::EN,
            true => I2C1D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C1D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C1D_A::DIS
    }
}
#[doc = "Write proxy for field `I2C1D`"]
pub struct I2C1D_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C1D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C1D_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "PT Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTD_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<PTD_A> for bool {
    #[inline(always)]
    fn from(variant: PTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTD`"]
pub type PTD_R = crate::R<bool, PTD_A>;
impl PTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTD_A {
        match self.bits {
            false => PTD_A::EN,
            true => PTD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PTD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PTD_A::DIS
    }
}
#[doc = "Write proxy for field `PTD`"]
pub struct PTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PTD_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PTD_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "SPI XiP Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIXIPD_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<SPIXIPD_A> for bool {
    #[inline(always)]
    fn from(variant: SPIXIPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIXIPD`"]
pub type SPIXIPD_R = crate::R<bool, SPIXIPD_A>;
impl SPIXIPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIXIPD_A {
        match self.bits {
            false => SPIXIPD_A::EN,
            true => SPIXIPD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPIXIPD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPIXIPD_A::DIS
    }
}
#[doc = "Write proxy for field `SPIXIPD`"]
pub struct SPIXIPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIXIPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIXIPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPIXIPD_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPIXIPD_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "SPI XiP Master Controller Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMD_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<SPIMD_A> for bool {
    #[inline(always)]
    fn from(variant: SPIMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIMD`"]
pub type SPIMD_R = crate::R<bool, SPIMD_A>;
impl SPIMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIMD_A {
        match self.bits {
            false => SPIMD_A::EN,
            true => SPIMD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SPIMD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SPIMD_A::DIS
    }
}
#[doc = "Write proxy for field `SPIMD`"]
pub struct SPIMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPIMD_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPIMD_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPIO0 Disable."]
    #[inline(always)]
    pub fn gpio0d(&self) -> GPIO0D_R {
        GPIO0D_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO1 Disable."]
    #[inline(always)]
    pub fn gpio1d(&self) -> GPIO1D_R {
        GPIO1D_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO2 Disable."]
    #[inline(always)]
    pub fn gpio2d(&self) -> GPIO2D_R {
        GPIO2D_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB Disable."]
    #[inline(always)]
    pub fn usbd(&self) -> USBD_R {
        USBD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CLCD Disable."]
    #[inline(always)]
    pub fn clcdd(&self) -> CLCDD_R {
        CLCDD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA Disable."]
    #[inline(always)]
    pub fn dmad(&self) -> DMAD_R {
        DMAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI 0 Disable."]
    #[inline(always)]
    pub fn spi0d(&self) -> SPI0D_R {
        SPI0D_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI 1 Disable."]
    #[inline(always)]
    pub fn spi1d(&self) -> SPI1D_R {
        SPI1D_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI 2 Disable."]
    #[inline(always)]
    pub fn spi2d(&self) -> SPI2D_R {
        SPI2D_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART 0 Disable."]
    #[inline(always)]
    pub fn uart0d(&self) -> UART0D_R {
        UART0D_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART 1 Disable."]
    #[inline(always)]
    pub fn uart1d(&self) -> UART1D_R {
        UART1D_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - I2C 0 Disable."]
    #[inline(always)]
    pub fn i2c0d(&self) -> I2C0D_R {
        I2C0D_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Crypto Disable."]
    #[inline(always)]
    pub fn cryptod(&self) -> CRYPTOD_R {
        CRYPTOD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer 0 Disable."]
    #[inline(always)]
    pub fn t0d(&self) -> T0D_R {
        T0D_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timer 1 Disable."]
    #[inline(always)]
    pub fn t1d(&self) -> T1D_R {
        T1D_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer 2 Disable."]
    #[inline(always)]
    pub fn t2d(&self) -> T2D_R {
        T2D_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer 3 Disable."]
    #[inline(always)]
    pub fn t3d(&self) -> T3D_R {
        T3D_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer 4 Disable."]
    #[inline(always)]
    pub fn t4d(&self) -> T4D_R {
        T4D_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Timer 5 Disable."]
    #[inline(always)]
    pub fn t5d(&self) -> T5D_R {
        T5D_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC Disable."]
    #[inline(always)]
    pub fn adcd(&self) -> ADCD_R {
        ADCD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rsv_0x19(&self) -> RSV_0X19_R {
        RSV_0X19_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 28 - I2C 1 Disable."]
    #[inline(always)]
    pub fn i2c1d(&self) -> I2C1D_R {
        I2C1D_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - PT Clock Disable."]
    #[inline(always)]
    pub fn ptd(&self) -> PTD_R {
        PTD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SPI XiP Disable."]
    #[inline(always)]
    pub fn spixipd(&self) -> SPIXIPD_R {
        SPIXIPD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPI XiP Master Controller Disable."]
    #[inline(always)]
    pub fn spimd(&self) -> SPIMD_R {
        SPIMD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO0 Disable."]
    #[inline(always)]
    pub fn gpio0d(&mut self) -> GPIO0D_W {
        GPIO0D_W { w: self }
    }
    #[doc = "Bit 1 - GPIO1 Disable."]
    #[inline(always)]
    pub fn gpio1d(&mut self) -> GPIO1D_W {
        GPIO1D_W { w: self }
    }
    #[doc = "Bit 2 - GPIO2 Disable."]
    #[inline(always)]
    pub fn gpio2d(&mut self) -> GPIO2D_W {
        GPIO2D_W { w: self }
    }
    #[doc = "Bit 3 - USB Disable."]
    #[inline(always)]
    pub fn usbd(&mut self) -> USBD_W {
        USBD_W { w: self }
    }
    #[doc = "Bit 4 - CLCD Disable."]
    #[inline(always)]
    pub fn clcdd(&mut self) -> CLCDD_W {
        CLCDD_W { w: self }
    }
    #[doc = "Bit 5 - DMA Disable."]
    #[inline(always)]
    pub fn dmad(&mut self) -> DMAD_W {
        DMAD_W { w: self }
    }
    #[doc = "Bit 6 - SPI 0 Disable."]
    #[inline(always)]
    pub fn spi0d(&mut self) -> SPI0D_W {
        SPI0D_W { w: self }
    }
    #[doc = "Bit 7 - SPI 1 Disable."]
    #[inline(always)]
    pub fn spi1d(&mut self) -> SPI1D_W {
        SPI1D_W { w: self }
    }
    #[doc = "Bit 8 - SPI 2 Disable."]
    #[inline(always)]
    pub fn spi2d(&mut self) -> SPI2D_W {
        SPI2D_W { w: self }
    }
    #[doc = "Bit 9 - UART 0 Disable."]
    #[inline(always)]
    pub fn uart0d(&mut self) -> UART0D_W {
        UART0D_W { w: self }
    }
    #[doc = "Bit 10 - UART 1 Disable."]
    #[inline(always)]
    pub fn uart1d(&mut self) -> UART1D_W {
        UART1D_W { w: self }
    }
    #[doc = "Bit 13 - I2C 0 Disable."]
    #[inline(always)]
    pub fn i2c0d(&mut self) -> I2C0D_W {
        I2C0D_W { w: self }
    }
    #[doc = "Bit 14 - Crypto Disable."]
    #[inline(always)]
    pub fn cryptod(&mut self) -> CRYPTOD_W {
        CRYPTOD_W { w: self }
    }
    #[doc = "Bit 15 - Timer 0 Disable."]
    #[inline(always)]
    pub fn t0d(&mut self) -> T0D_W {
        T0D_W { w: self }
    }
    #[doc = "Bit 16 - Timer 1 Disable."]
    #[inline(always)]
    pub fn t1d(&mut self) -> T1D_W {
        T1D_W { w: self }
    }
    #[doc = "Bit 17 - Timer 2 Disable."]
    #[inline(always)]
    pub fn t2d(&mut self) -> T2D_W {
        T2D_W { w: self }
    }
    #[doc = "Bit 18 - Timer 3 Disable."]
    #[inline(always)]
    pub fn t3d(&mut self) -> T3D_W {
        T3D_W { w: self }
    }
    #[doc = "Bit 19 - Timer 4 Disable."]
    #[inline(always)]
    pub fn t4d(&mut self) -> T4D_W {
        T4D_W { w: self }
    }
    #[doc = "Bit 20 - Timer 5 Disable."]
    #[inline(always)]
    pub fn t5d(&mut self) -> T5D_W {
        T5D_W { w: self }
    }
    #[doc = "Bit 23 - ADC Disable."]
    #[inline(always)]
    pub fn adcd(&mut self) -> ADCD_W {
        ADCD_W { w: self }
    }
    #[doc = "Bit 25 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rsv_0x19(&mut self) -> RSV_0X19_W {
        RSV_0X19_W { w: self }
    }
    #[doc = "Bit 28 - I2C 1 Disable."]
    #[inline(always)]
    pub fn i2c1d(&mut self) -> I2C1D_W {
        I2C1D_W { w: self }
    }
    #[doc = "Bit 29 - PT Clock Disable."]
    #[inline(always)]
    pub fn ptd(&mut self) -> PTD_W {
        PTD_W { w: self }
    }
    #[doc = "Bit 30 - SPI XiP Disable."]
    #[inline(always)]
    pub fn spixipd(&mut self) -> SPIXIPD_W {
        SPIXIPD_W { w: self }
    }
    #[doc = "Bit 31 - SPI XiP Master Controller Disable."]
    #[inline(always)]
    pub fn spimd(&mut self) -> SPIMD_W {
        SPIMD_W { w: self }
    }
}
