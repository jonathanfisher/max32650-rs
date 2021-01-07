#[doc = "Reader of register LPPWEN"]
pub type R = crate::R<u32, super::LPPWEN>;
#[doc = "Writer for register LPPWEN"]
pub type W = crate::W<u32, super::LPPWEN>;
#[doc = "Register LPPWEN `reset()`'s with value 0"]
impl crate::ResetValue for super::LPPWEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBLSWKEN`"]
pub type USBLSWKEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBLSWKEN`"]
pub struct USBLSWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLSWKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `USBVBUSWKEN`"]
pub type USBVBUSWKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBVBUSWKEN`"]
pub struct USBVBUSWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBVBUSWKEN_W<'a> {
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
#[doc = "Reader of field `SDMAWKEN`"]
pub type SDMAWKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMAWKEN`"]
pub struct SDMAWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMAWKEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - USB UTMI Linestate Detect Wakeup Enable. These bits allow wakeup from the corresponding USB linestate signal(s) on transition(s) from low to high or high to low when PM.USBWKEN is set."]
    #[inline(always)]
    pub fn usblswken(&self) -> USBLSWKEN_R {
        USBLSWKEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - USB VBUS Detect Wakeup Enable. This bit allows wakeup from the USB power supply on or off status."]
    #[inline(always)]
    pub fn usbvbuswken(&self) -> USBVBUSWKEN_R {
        USBVBUSWKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Smart DMA Wakeup Enable. This bit allows wakeup from the Smart DMA IRQ."]
    #[inline(always)]
    pub fn sdmawken(&self) -> SDMAWKEN_R {
        SDMAWKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB UTMI Linestate Detect Wakeup Enable. These bits allow wakeup from the corresponding USB linestate signal(s) on transition(s) from low to high or high to low when PM.USBWKEN is set."]
    #[inline(always)]
    pub fn usblswken(&mut self) -> USBLSWKEN_W {
        USBLSWKEN_W { w: self }
    }
    #[doc = "Bit 2 - USB VBUS Detect Wakeup Enable. This bit allows wakeup from the USB power supply on or off status."]
    #[inline(always)]
    pub fn usbvbuswken(&mut self) -> USBVBUSWKEN_W {
        USBVBUSWKEN_W { w: self }
    }
    #[doc = "Bit 3 - Smart DMA Wakeup Enable. This bit allows wakeup from the Smart DMA IRQ."]
    #[inline(always)]
    pub fn sdmawken(&mut self) -> SDMAWKEN_W {
        SDMAWKEN_W { w: self }
    }
}
