#[doc = "Reader of register LPPWST"]
pub type R = crate::R<u32, super::LPPWST>;
#[doc = "Writer for register LPPWST"]
pub type W = crate::W<u32, super::LPPWST>;
#[doc = "Register LPPWST `reset()`'s with value 0"]
impl crate::ResetValue for super::LPPWST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBLSWKST`"]
pub type USBLSWKST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBLSWKST`"]
pub struct USBLSWKST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLSWKST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `USBVBUSWKST`"]
pub type USBVBUSWKST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBVBUSWKST`"]
pub struct USBVBUSWKST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBVBUSWKST_W<'a> {
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
#[doc = "Reader of field `SDMAWKST`"]
pub type SDMAWKST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMAWKST`"]
pub struct SDMAWKST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMAWKST_W<'a> {
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
#[doc = "Reader of field `BBMODEST`"]
pub type BBMODEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBMODEST`"]
pub struct BBMODEST_W<'a> {
    w: &'a mut W,
}
impl<'a> BBMODEST_W<'a> {
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
    #[doc = "Bits 0:1 - USB UTMI Linestate Detect Wakeup Flag(write one to clear). One or both of these bits will be set when the USB bus activity causes the linestate to change and the device to wake while USB wakeup is enabled using PMLUSBWKEN."]
    #[inline(always)]
    pub fn usblswkst(&self) -> USBLSWKST_R {
        USBLSWKST_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - USB VBUS Detect Wakeup Flag (write one to clear). This bit will be set when the USB power supply is powered on or powered off."]
    #[inline(always)]
    pub fn usbvbuswkst(&self) -> USBVBUSWKST_R {
        USBVBUSWKST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Smart DMA Detect Wakeup Flag (write one to clear). This bit will be set when the SDMA IRQ transition from low to high or on high to low."]
    #[inline(always)]
    pub fn sdmawkst(&self) -> SDMAWKST_R {
        SDMAWKST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Battery Back Wakeup Flag (write one to clear). This bit will be set when exiting Battery Backup Mode."]
    #[inline(always)]
    pub fn bbmodest(&self) -> BBMODEST_R {
        BBMODEST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB UTMI Linestate Detect Wakeup Flag(write one to clear). One or both of these bits will be set when the USB bus activity causes the linestate to change and the device to wake while USB wakeup is enabled using PMLUSBWKEN."]
    #[inline(always)]
    pub fn usblswkst(&mut self) -> USBLSWKST_W {
        USBLSWKST_W { w: self }
    }
    #[doc = "Bit 2 - USB VBUS Detect Wakeup Flag (write one to clear). This bit will be set when the USB power supply is powered on or powered off."]
    #[inline(always)]
    pub fn usbvbuswkst(&mut self) -> USBVBUSWKST_W {
        USBVBUSWKST_W { w: self }
    }
    #[doc = "Bit 3 - Smart DMA Detect Wakeup Flag (write one to clear). This bit will be set when the SDMA IRQ transition from low to high or on high to low."]
    #[inline(always)]
    pub fn sdmawkst(&mut self) -> SDMAWKST_W {
        SDMAWKST_W { w: self }
    }
    #[doc = "Bit 16 - Battery Back Wakeup Flag (write one to clear). This bit will be set when exiting Battery Backup Mode."]
    #[inline(always)]
    pub fn bbmodest(&mut self) -> BBMODEST_W {
        BBMODEST_W { w: self }
    }
}
