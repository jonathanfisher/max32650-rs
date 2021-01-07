#[doc = "Reader of register MCR[%s]"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR[%s]"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR[%s]
`reset()`'s with value 0x03"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `MAXEN`"]
pub type MAXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAXEN`"]
pub struct MAXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXEN_W<'a> {
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
#[doc = "Reader of field `MAXLEN`"]
pub type MAXLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAXLEN`"]
pub struct MAXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 18)) | (((value as u32) & 0x01ff) << 18);
        self.w
    }
}
#[doc = "Reader of field `HSE`"]
pub type HSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSE`"]
pub struct HSE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSE_W<'a> {
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
#[doc = "Reader of field `FRL`"]
pub type FRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRL`"]
pub struct FRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRL_W<'a> {
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
#[doc = "Reader of field `CRT`"]
pub type CRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRT`"]
pub struct CRT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRT_W<'a> {
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
#[doc = "Device Type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEVTYPE_A {
    #[doc = "0: Hyper flash device."]
    HYPER_FLASH = 0,
    #[doc = "1: Hyper PSRAM device."]
    XCCELA_PSRAM = 1,
    #[doc = "2: Hyper RAM device."]
    HYPER_RAM = 2,
}
impl From<DEVTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEVTYPE`"]
pub type DEVTYPE_R = crate::R<u8, DEVTYPE_A>;
impl DEVTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEVTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEVTYPE_A::HYPER_FLASH),
            1 => Val(DEVTYPE_A::XCCELA_PSRAM),
            2 => Val(DEVTYPE_A::HYPER_RAM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HYPER_FLASH`"]
    #[inline(always)]
    pub fn is_hyper_flash(&self) -> bool {
        *self == DEVTYPE_A::HYPER_FLASH
    }
    #[doc = "Checks if the value of the field is `XCCELA_PSRAM`"]
    #[inline(always)]
    pub fn is_xccela_psram(&self) -> bool {
        *self == DEVTYPE_A::XCCELA_PSRAM
    }
    #[doc = "Checks if the value of the field is `HYPER_RAM`"]
    #[inline(always)]
    pub fn is_hyper_ram(&self) -> bool {
        *self == DEVTYPE_A::HYPER_RAM
    }
}
#[doc = "Write proxy for field `DEVTYPE`"]
pub struct DEVTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVTYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Hyper flash device."]
    #[inline(always)]
    pub fn hyper_flash(self) -> &'a mut W {
        self.variant(DEVTYPE_A::HYPER_FLASH)
    }
    #[doc = "Hyper PSRAM device."]
    #[inline(always)]
    pub fn xccela_psram(self) -> &'a mut W {
        self.variant(DEVTYPE_A::XCCELA_PSRAM)
    }
    #[doc = "Hyper RAM device."]
    #[inline(always)]
    pub fn hyper_ram(self) -> &'a mut W {
        self.variant(DEVTYPE_A::HYPER_RAM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Maximum length Enable."]
    #[inline(always)]
    pub fn maxen(&self) -> MAXEN_R {
        MAXEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 18:26 - Maximum Length."]
    #[inline(always)]
    pub fn maxlen(&self) -> MAXLEN_R {
        MAXLEN_R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 7 - Halt Sleep Exit."]
    #[inline(always)]
    pub fn hse(&self) -> HSE_R {
        HSE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fixed Read latency enable."]
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Configuration Register Target."]
    #[inline(always)]
    pub fn crt(&self) -> CRT_R {
        CRT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Device Type."]
    #[inline(always)]
    pub fn devtype(&self) -> DEVTYPE_R {
        DEVTYPE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Maximum length Enable."]
    #[inline(always)]
    pub fn maxen(&mut self) -> MAXEN_W {
        MAXEN_W { w: self }
    }
    #[doc = "Bits 18:26 - Maximum Length."]
    #[inline(always)]
    pub fn maxlen(&mut self) -> MAXLEN_W {
        MAXLEN_W { w: self }
    }
    #[doc = "Bit 7 - Halt Sleep Exit."]
    #[inline(always)]
    pub fn hse(&mut self) -> HSE_W {
        HSE_W { w: self }
    }
    #[doc = "Bit 6 - Fixed Read latency enable."]
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W {
        FRL_W { w: self }
    }
    #[doc = "Bit 5 - Configuration Register Target."]
    #[inline(always)]
    pub fn crt(&mut self) -> CRT_W {
        CRT_W { w: self }
    }
    #[doc = "Bits 3:4 - Device Type."]
    #[inline(always)]
    pub fn devtype(&mut self) -> DEVTYPE_W {
        DEVTYPE_W { w: self }
    }
}
