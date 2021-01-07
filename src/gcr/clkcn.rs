#[doc = "Reader of register CLKCN"]
pub type R = crate::R<u32, super::CLKCN>;
#[doc = "Writer for register CLKCN"]
pub type W = crate::W<u32, super::CLKCN>;
#[doc = "Register CLKCN `reset()`'s with value 0x08"]
impl crate::ResetValue for super::CLKCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Divide by 1."]
    DIV1 = 0,
    #[doc = "1: Divide by 2."]
    DIV2 = 1,
    #[doc = "2: Divide by 4."]
    DIV4 = 2,
    #[doc = "3: Divide by 8."]
    DIV8 = 3,
    #[doc = "4: Divide by 16."]
    DIV16 = 4,
    #[doc = "5: Divide by 32."]
    DIV32 = 5,
    #[doc = "6: Divide by 64."]
    DIV64 = 6,
    #[doc = "7: Divide by 128."]
    DIV128 = 7,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSC`"]
pub type PSC_R = crate::R<u8, PSC_A>;
impl PSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSC_A {
        match self.bits {
            0 => PSC_A::DIV1,
            1 => PSC_A::DIV2,
            2 => PSC_A::DIV4,
            3 => PSC_A::DIV8,
            4 => PSC_A::DIV16,
            5 => PSC_A::DIV32,
            6 => PSC_A::DIV64,
            7 => PSC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PSC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PSC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PSC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PSC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PSC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PSC_A::DIV128
    }
}
#[doc = "Write proxy for field `PSC`"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PSC_A::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PSC_A::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PSC_A::DIV4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PSC_A::DIV8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PSC_A::DIV16)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PSC_A::DIV32)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PSC_A::DIV64)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PSC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Clock Source Select. This 3 bit field selects the source for the system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Crypto oscillator is used for the system clock."]
    CRYPTOOSC = 0,
    #[doc = "2: HFXIN is used for the system clock."]
    HFXIN = 2,
    #[doc = "3: The nano-ring output is used for the system clock."]
    NANORING = 3,
    #[doc = "4: The internal 96 MHz oscillator is used for the system clock."]
    HIRC96 = 4,
    #[doc = "5: The internal 8 MHz oscillator is used for the system clock."]
    HIRC8 = 5,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKSEL_A::CRYPTOOSC),
            2 => Val(CLKSEL_A::HFXIN),
            3 => Val(CLKSEL_A::NANORING),
            4 => Val(CLKSEL_A::HIRC96),
            5 => Val(CLKSEL_A::HIRC8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRYPTOOSC`"]
    #[inline(always)]
    pub fn is_crypto_osc(&self) -> bool {
        *self == CLKSEL_A::CRYPTOOSC
    }
    #[doc = "Checks if the value of the field is `HFXIN`"]
    #[inline(always)]
    pub fn is_hfx_in(&self) -> bool {
        *self == CLKSEL_A::HFXIN
    }
    #[doc = "Checks if the value of the field is `NANORING`"]
    #[inline(always)]
    pub fn is_nano_ring(&self) -> bool {
        *self == CLKSEL_A::NANORING
    }
    #[doc = "Checks if the value of the field is `HIRC96`"]
    #[inline(always)]
    pub fn is_hirc96(&self) -> bool {
        *self == CLKSEL_A::HIRC96
    }
    #[doc = "Checks if the value of the field is `HIRC8`"]
    #[inline(always)]
    pub fn is_hirc8(&self) -> bool {
        *self == CLKSEL_A::HIRC8
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Crypto oscillator is used for the system clock."]
    #[inline(always)]
    pub fn crypto_osc(self) -> &'a mut W {
        self.variant(CLKSEL_A::CRYPTOOSC)
    }
    #[doc = "HFXIN is used for the system clock."]
    #[inline(always)]
    pub fn hfx_in(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFXIN)
    }
    #[doc = "The nano-ring output is used for the system clock."]
    #[inline(always)]
    pub fn nano_ring(self) -> &'a mut W {
        self.variant(CLKSEL_A::NANORING)
    }
    #[doc = "The internal 96 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn hirc96(self) -> &'a mut W {
        self.variant(CLKSEL_A::HIRC96)
    }
    #[doc = "The internal 8 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn hirc8(self) -> &'a mut W {
        self.variant(CLKSEL_A::HIRC8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Clock Ready. This read only bit reflects whether the currently selected system clock source is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKRDY_A {
    #[doc = "0: Switchover to the new clock source (as selected by CLKSEL) has not yet occurred."]
    BUSY = 0,
    #[doc = "1: System clock running from CLKSEL clock source."]
    READY = 1,
}
impl From<CKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CKRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKRDY`"]
pub type CKRDY_R = crate::R<bool, CKRDY_A>;
impl CKRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKRDY_A {
        match self.bits {
            false => CKRDY_A::BUSY,
            true => CKRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == CKRDY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CKRDY_A::READY
    }
}
#[doc = "Cryptographic clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCD_A {
    #[doc = "0: The cryptographic accelerator clock is running in non-divided mode."]
    NON_DIV = 0,
    #[doc = "1: The cryptographic accelerator clock is running in divided mode."]
    DIV = 1,
}
impl From<CCD_A> for bool {
    #[inline(always)]
    fn from(variant: CCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCD`"]
pub type CCD_R = crate::R<bool, CCD_A>;
impl CCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCD_A {
        match self.bits {
            false => CCD_A::NON_DIV,
            true => CCD_A::DIV,
        }
    }
    #[doc = "Checks if the value of the field is `NON_DIV`"]
    #[inline(always)]
    pub fn is_non_div(&self) -> bool {
        *self == CCD_A::NON_DIV
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == CCD_A::DIV
    }
}
#[doc = "32kHz Crystal Oscillator Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X32K_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<X32K_EN_A> for bool {
    #[inline(always)]
    fn from(variant: X32K_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `X32K_EN`"]
pub type X32K_EN_R = crate::R<bool, X32K_EN_A>;
impl X32K_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X32K_EN_A {
        match self.bits {
            false => X32K_EN_A::DIS,
            true => X32K_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == X32K_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == X32K_EN_A::EN
    }
}
#[doc = "Write proxy for field `X32K_EN`"]
pub struct X32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X32K_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: X32K_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(X32K_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(X32K_EN_A::EN)
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
#[doc = "60MHz High Frequency Internal Reference Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<HIRC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRC_EN`"]
pub type HIRC_EN_R = crate::R<bool, HIRC_EN_A>;
impl HIRC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC_EN_A {
        match self.bits {
            false => HIRC_EN_A::DIS,
            true => HIRC_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HIRC_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == HIRC_EN_A::EN
    }
}
#[doc = "Write proxy for field `HIRC_EN`"]
pub struct HIRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HIRC_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HIRC_EN_A::EN)
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
#[doc = "96MHz High Frequency Internal Reference Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC96M_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<HIRC96M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC96M_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRC96M_EN`"]
pub type HIRC96M_EN_R = crate::R<bool, HIRC96M_EN_A>;
impl HIRC96M_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC96M_EN_A {
        match self.bits {
            false => HIRC96M_EN_A::DIS,
            true => HIRC96M_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HIRC96M_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == HIRC96M_EN_A::EN
    }
}
#[doc = "Write proxy for field `HIRC96M_EN`"]
pub struct HIRC96M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC96M_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC96M_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HIRC96M_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HIRC96M_EN_A::EN)
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
#[doc = "8MHz High Frequency Internal Reference Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC8M_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<HIRC8M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC8M_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRC8M_EN`"]
pub type HIRC8M_EN_R = crate::R<bool, HIRC8M_EN_A>;
impl HIRC8M_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC8M_EN_A {
        match self.bits {
            false => HIRC8M_EN_A::DIS,
            true => HIRC8M_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HIRC8M_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == HIRC8M_EN_A::EN
    }
}
#[doc = "Write proxy for field `HIRC8M_EN`"]
pub struct HIRC8M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC8M_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC8M_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HIRC8M_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HIRC8M_EN_A::EN)
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
#[doc = "32kHz Crystal Oscillator Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X32K_RDY_A {
    #[doc = "0: Not Ready"]
    NOT = 0,
    #[doc = "1: X32K Ready"]
    READY = 1,
}
impl From<X32K_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: X32K_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `X32K_RDY`"]
pub type X32K_RDY_R = crate::R<bool, X32K_RDY_A>;
impl X32K_RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X32K_RDY_A {
        match self.bits {
            false => X32K_RDY_A::NOT,
            true => X32K_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == X32K_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == X32K_RDY_A::READY
    }
}
#[doc = "60MHz HIRC Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC_RDY_A {
    #[doc = "0: Not Ready"]
    NOT = 0,
    #[doc = "1: HIRC Ready"]
    READY = 1,
}
impl From<HIRC_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRC_RDY`"]
pub type HIRC_RDY_R = crate::R<bool, HIRC_RDY_A>;
impl HIRC_RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC_RDY_A {
        match self.bits {
            false => HIRC_RDY_A::NOT,
            true => HIRC_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == HIRC_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HIRC_RDY_A::READY
    }
}
#[doc = "Write proxy for field `HIRC_RDY`"]
pub struct HIRC_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC_RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC_RDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(HIRC_RDY_A::NOT)
    }
    #[doc = "HIRC Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HIRC_RDY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "96MHz HIRC Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC96M_RDY_A {
    #[doc = "0: Not Ready"]
    NOT = 0,
    #[doc = "1: HIRC96M Ready"]
    READY = 1,
}
impl From<HIRC96M_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC96M_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRC96M_RDY`"]
pub type HIRC96M_RDY_R = crate::R<bool, HIRC96M_RDY_A>;
impl HIRC96M_RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC96M_RDY_A {
        match self.bits {
            false => HIRC96M_RDY_A::NOT,
            true => HIRC96M_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == HIRC96M_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HIRC96M_RDY_A::READY
    }
}
#[doc = "Write proxy for field `HIRC96M_RDY`"]
pub struct HIRC96M_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC96M_RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC96M_RDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(HIRC96M_RDY_A::NOT)
    }
    #[doc = "HIRC96M Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HIRC96M_RDY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "8MHz HIRC Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC8M_RDY_A {
    #[doc = "0: Not Ready"]
    NOT = 0,
    #[doc = "1: HIRC8M Ready"]
    READY = 1,
}
impl From<HIRC8M_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC8M_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRC8M_RDY`"]
pub type HIRC8M_RDY_R = crate::R<bool, HIRC8M_RDY_A>;
impl HIRC8M_RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC8M_RDY_A {
        match self.bits {
            false => HIRC8M_RDY_A::NOT,
            true => HIRC8M_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == HIRC8M_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HIRC8M_RDY_A::READY
    }
}
#[doc = "Write proxy for field `HIRC8M_RDY`"]
pub struct HIRC8M_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC8M_RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC8M_RDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(HIRC8M_RDY_A::NOT)
    }
    #[doc = "HIRC8M Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HIRC8M_RDY_A::READY)
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
#[doc = "8kHz Low Frequency Reference Clock Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIRC8K_RDY_A {
    #[doc = "0: Not Ready"]
    NOT = 0,
    #[doc = "1: Clock Ready"]
    READY = 1,
}
impl From<LIRC8K_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: LIRC8K_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LIRC8K_RDY`"]
pub type LIRC8K_RDY_R = crate::R<bool, LIRC8K_RDY_A>;
impl LIRC8K_RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIRC8K_RDY_A {
        match self.bits {
            false => LIRC8K_RDY_A::NOT,
            true => LIRC8K_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == LIRC8K_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LIRC8K_RDY_A::READY
    }
}
#[doc = "Write proxy for field `LIRC8K_RDY`"]
pub struct LIRC8K_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LIRC8K_RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIRC8K_RDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(LIRC8K_RDY_A::NOT)
    }
    #[doc = "Clock Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(LIRC8K_RDY_A::READY)
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
impl R {
    #[doc = "Bits 6:8 - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Clock Source Select. This 3 bit field selects the source for the system clock."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 13 - Clock Ready. This read only bit reflects whether the currently selected system clock source is running."]
    #[inline(always)]
    pub fn ckrdy(&self) -> CKRDY_R {
        CKRDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Cryptographic clock divider"]
    #[inline(always)]
    pub fn ccd(&self) -> CCD_R {
        CCD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 32kHz Crystal Oscillator Enable."]
    #[inline(always)]
    pub fn x32k_en(&self) -> X32K_EN_R {
        X32K_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 60MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn hirc_en(&self) -> HIRC_EN_R {
        HIRC_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 96MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn hirc96m_en(&self) -> HIRC96M_EN_R {
        HIRC96M_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 8MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn hirc8m_en(&self) -> HIRC8M_EN_R {
        HIRC8M_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 32kHz Crystal Oscillator Ready"]
    #[inline(always)]
    pub fn x32k_rdy(&self) -> X32K_RDY_R {
        X32K_RDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 60MHz HIRC Ready."]
    #[inline(always)]
    pub fn hirc_rdy(&self) -> HIRC_RDY_R {
        HIRC_RDY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 96MHz HIRC Ready."]
    #[inline(always)]
    pub fn hirc96m_rdy(&self) -> HIRC96M_RDY_R {
        HIRC96M_RDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 8MHz HIRC Ready."]
    #[inline(always)]
    pub fn hirc8m_rdy(&self) -> HIRC8M_RDY_R {
        HIRC8M_RDY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 8kHz Low Frequency Reference Clock Ready."]
    #[inline(always)]
    pub fn lirc8k_rdy(&self) -> LIRC8K_RDY_R {
        LIRC8K_RDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:8 - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
    #[doc = "Bits 9:11 - Clock Source Select. This 3 bit field selects the source for the system clock."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 17 - 32kHz Crystal Oscillator Enable."]
    #[inline(always)]
    pub fn x32k_en(&mut self) -> X32K_EN_W {
        X32K_EN_W { w: self }
    }
    #[doc = "Bit 18 - 60MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn hirc_en(&mut self) -> HIRC_EN_W {
        HIRC_EN_W { w: self }
    }
    #[doc = "Bit 19 - 96MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn hirc96m_en(&mut self) -> HIRC96M_EN_W {
        HIRC96M_EN_W { w: self }
    }
    #[doc = "Bit 20 - 8MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn hirc8m_en(&mut self) -> HIRC8M_EN_W {
        HIRC8M_EN_W { w: self }
    }
    #[doc = "Bit 26 - 60MHz HIRC Ready."]
    #[inline(always)]
    pub fn hirc_rdy(&mut self) -> HIRC_RDY_W {
        HIRC_RDY_W { w: self }
    }
    #[doc = "Bit 27 - 96MHz HIRC Ready."]
    #[inline(always)]
    pub fn hirc96m_rdy(&mut self) -> HIRC96M_RDY_W {
        HIRC96M_RDY_W { w: self }
    }
    #[doc = "Bit 28 - 8MHz HIRC Ready."]
    #[inline(always)]
    pub fn hirc8m_rdy(&mut self) -> HIRC8M_RDY_W {
        HIRC8M_RDY_W { w: self }
    }
    #[doc = "Bit 29 - 8kHz Low Frequency Reference Clock Ready."]
    #[inline(always)]
    pub fn lirc8k_rdy(&mut self) -> LIRC8K_RDY_W {
        LIRC8K_RDY_W { w: self }
    }
}
