#[doc = "Reader of register PCKDIV"]
pub type R = crate::R<u32, super::PCKDIV>;
#[doc = "Writer for register PCKDIV"]
pub type W = crate::W<u32, super::PCKDIV>;
#[doc = "Register PCKDIV `reset()`'s with value 0x01"]
impl crate::ResetValue for super::PCKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "These bits determine the clock frequency for the UART, I2C and Key Pad peripherals. These peripherals have an adaptive clock generator that dynamically adjusts the peripheral frequency based on the main system bus frequency. These bits are dynamically updated when the PLL0 is selected as the system clock source and are set by hardware. These bits determine the clock frequency for the UART, I2C and Key Pad peripherals. These peripherals have an adaptive clock generator that dynamically adjusts the peripheral frequency based on the main system bus frequency. These bits are dynamically updated when the PLL0 is selected as the system clock source and are set by hardware.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCF_A {
    #[doc = "2: `10`"]
    _96MHZ = 2,
    #[doc = "3: `11`"]
    _48MHZ = 3,
    #[doc = "4: `100`"]
    _24MHZ = 4,
    #[doc = "5: `101`"]
    _12MHZ = 5,
    #[doc = "6: `110`"]
    _6MHZ = 6,
    #[doc = "7: `111`"]
    _3MHZ = 7,
}
impl From<PCF_A> for u8 {
    #[inline(always)]
    fn from(variant: PCF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCF`"]
pub type PCF_R = crate::R<u8, PCF_A>;
impl PCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PCF_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(PCF_A::_96MHZ),
            3 => Val(PCF_A::_48MHZ),
            4 => Val(PCF_A::_24MHZ),
            5 => Val(PCF_A::_12MHZ),
            6 => Val(PCF_A::_6MHZ),
            7 => Val(PCF_A::_3MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_96MHZ`"]
    #[inline(always)]
    pub fn is_96mhz(&self) -> bool {
        *self == PCF_A::_96MHZ
    }
    #[doc = "Checks if the value of the field is `_48MHZ`"]
    #[inline(always)]
    pub fn is_48mhz(&self) -> bool {
        *self == PCF_A::_48MHZ
    }
    #[doc = "Checks if the value of the field is `_24MHZ`"]
    #[inline(always)]
    pub fn is_24mhz(&self) -> bool {
        *self == PCF_A::_24MHZ
    }
    #[doc = "Checks if the value of the field is `_12MHZ`"]
    #[inline(always)]
    pub fn is_12mhz(&self) -> bool {
        *self == PCF_A::_12MHZ
    }
    #[doc = "Checks if the value of the field is `_6MHZ`"]
    #[inline(always)]
    pub fn is_6mhz(&self) -> bool {
        *self == PCF_A::_6MHZ
    }
    #[doc = "Checks if the value of the field is `_3MHZ`"]
    #[inline(always)]
    pub fn is_3mhz(&self) -> bool {
        *self == PCF_A::_3MHZ
    }
}
#[doc = "Write proxy for field `PCF`"]
pub struct PCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _96mhz(self) -> &'a mut W {
        self.variant(PCF_A::_96MHZ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _48mhz(self) -> &'a mut W {
        self.variant(PCF_A::_48MHZ)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _24mhz(self) -> &'a mut W {
        self.variant(PCF_A::_24MHZ)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _12mhz(self) -> &'a mut W {
        self.variant(PCF_A::_12MHZ)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _6mhz(self) -> &'a mut W {
        self.variant(PCF_A::_6MHZ)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn _3mhz(self) -> &'a mut W {
        self.variant(PCF_A::_3MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "PCF Write Enable. This bit allows the PCF Register bits to be updated by Software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCFWEN_A {
    #[doc = "0: Writes to PCF are blocked."]
    BLOCKED = 0,
    #[doc = "1: Writes to PCF are allowed"]
    ALLOWED = 1,
}
impl From<PCFWEN_A> for bool {
    #[inline(always)]
    fn from(variant: PCFWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PCFWEN`"]
pub type PCFWEN_R = crate::R<bool, PCFWEN_A>;
impl PCFWEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCFWEN_A {
        match self.bits {
            false => PCFWEN_A::BLOCKED,
            true => PCFWEN_A::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PCFWEN_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == PCFWEN_A::ALLOWED
    }
}
#[doc = "Write proxy for field `PCFWEN`"]
pub struct PCFWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCFWEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCFWEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to PCF are blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PCFWEN_A::BLOCKED)
    }
    #[doc = "Writes to PCF are allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(PCFWEN_A::ALLOWED)
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
#[doc = "SDHC Clock Frequency. This bits defines the clock frequency of SDHC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHCFRQ_A {
    #[doc = "0: `0`"]
    _48MHZ = 0,
    #[doc = "1: `1`"]
    _24MHZ = 1,
}
impl From<SDHCFRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SDHCFRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDHCFRQ`"]
pub type SDHCFRQ_R = crate::R<bool, SDHCFRQ_A>;
impl SDHCFRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHCFRQ_A {
        match self.bits {
            false => SDHCFRQ_A::_48MHZ,
            true => SDHCFRQ_A::_24MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `_48MHZ`"]
    #[inline(always)]
    pub fn is_48mhz(&self) -> bool {
        *self == SDHCFRQ_A::_48MHZ
    }
    #[doc = "Checks if the value of the field is `_24MHZ`"]
    #[inline(always)]
    pub fn is_24mhz(&self) -> bool {
        *self == SDHCFRQ_A::_24MHZ
    }
}
#[doc = "Write proxy for field `SDHCFRQ`"]
pub struct SDHCFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SDHCFRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDHCFRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _48mhz(self) -> &'a mut W {
        self.variant(SDHCFRQ_A::_48MHZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _24mhz(self) -> &'a mut W {
        self.variant(SDHCFRQ_A::_24MHZ)
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
#[doc = "Reader of field `ADCFRQ`"]
pub type ADCFRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCFRQ`"]
pub struct ADCFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCFRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Always-ON(AON) domain CLock Divider. These bits define the AON domain clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AONCD_A {
    #[doc = "0: PCLK divide by 4."]
    DIV_4 = 0,
    #[doc = "1: PCLK divide by 8."]
    DIV_8 = 1,
    #[doc = "2: PCLK divide by 16."]
    DIV_16 = 2,
    #[doc = "3: PCLK divide by 32."]
    DIV_32 = 3,
}
impl From<AONCD_A> for u8 {
    #[inline(always)]
    fn from(variant: AONCD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AONCD`"]
pub type AONCD_R = crate::R<u8, AONCD_A>;
impl AONCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AONCD_A {
        match self.bits {
            0 => AONCD_A::DIV_4,
            1 => AONCD_A::DIV_8,
            2 => AONCD_A::DIV_16,
            3 => AONCD_A::DIV_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == AONCD_A::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == AONCD_A::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == AONCD_A::DIV_16
    }
    #[doc = "Checks if the value of the field is `DIV_32`"]
    #[inline(always)]
    pub fn is_div_32(&self) -> bool {
        *self == AONCD_A::DIV_32
    }
}
#[doc = "Write proxy for field `AONCD`"]
pub struct AONCD_W<'a> {
    w: &'a mut W,
}
impl<'a> AONCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AONCD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PCLK divide by 4."]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut W {
        self.variant(AONCD_A::DIV_4)
    }
    #[doc = "PCLK divide by 8."]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut W {
        self.variant(AONCD_A::DIV_8)
    }
    #[doc = "PCLK divide by 16."]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut W {
        self.variant(AONCD_A::DIV_16)
    }
    #[doc = "PCLK divide by 32."]
    #[inline(always)]
    pub fn div_32(self) -> &'a mut W {
        self.variant(AONCD_A::DIV_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - These bits determine the clock frequency for the UART, I2C and Key Pad peripherals. These peripherals have an adaptive clock generator that dynamically adjusts the peripheral frequency based on the main system bus frequency. These bits are dynamically updated when the PLL0 is selected as the system clock source and are set by hardware. These bits determine the clock frequency for the UART, I2C and Key Pad peripherals. These peripherals have an adaptive clock generator that dynamically adjusts the peripheral frequency based on the main system bus frequency. These bits are dynamically updated when the PLL0 is selected as the system clock source and are set by hardware."]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - PCF Write Enable. This bit allows the PCF Register bits to be updated by Software."]
    #[inline(always)]
    pub fn pcfwen(&self) -> PCFWEN_R {
        PCFWEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SDHC Clock Frequency. This bits defines the clock frequency of SDHC."]
    #[inline(always)]
    pub fn sdhcfrq(&self) -> SDHCFRQ_R {
        SDHCFRQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - ADC clock Frequency. These bits define the ADC clock frequency. FADC = FPCLK/(ADCFRQ)."]
    #[inline(always)]
    pub fn adcfrq(&self) -> ADCFRQ_R {
        ADCFRQ_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Always-ON(AON) domain CLock Divider. These bits define the AON domain clock divider."]
    #[inline(always)]
    pub fn aoncd(&self) -> AONCD_R {
        AONCD_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - These bits determine the clock frequency for the UART, I2C and Key Pad peripherals. These peripherals have an adaptive clock generator that dynamically adjusts the peripheral frequency based on the main system bus frequency. These bits are dynamically updated when the PLL0 is selected as the system clock source and are set by hardware. These bits determine the clock frequency for the UART, I2C and Key Pad peripherals. These peripherals have an adaptive clock generator that dynamically adjusts the peripheral frequency based on the main system bus frequency. These bits are dynamically updated when the PLL0 is selected as the system clock source and are set by hardware."]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W {
        PCF_W { w: self }
    }
    #[doc = "Bit 3 - PCF Write Enable. This bit allows the PCF Register bits to be updated by Software."]
    #[inline(always)]
    pub fn pcfwen(&mut self) -> PCFWEN_W {
        PCFWEN_W { w: self }
    }
    #[doc = "Bit 7 - SDHC Clock Frequency. This bits defines the clock frequency of SDHC."]
    #[inline(always)]
    pub fn sdhcfrq(&mut self) -> SDHCFRQ_W {
        SDHCFRQ_W { w: self }
    }
    #[doc = "Bits 10:13 - ADC clock Frequency. These bits define the ADC clock frequency. FADC = FPCLK/(ADCFRQ)."]
    #[inline(always)]
    pub fn adcfrq(&mut self) -> ADCFRQ_W {
        ADCFRQ_W { w: self }
    }
    #[doc = "Bits 14:15 - Always-ON(AON) domain CLock Divider. These bits define the AON domain clock divider."]
    #[inline(always)]
    pub fn aoncd(&mut self) -> AONCD_W {
        AONCD_W { w: self }
    }
}
