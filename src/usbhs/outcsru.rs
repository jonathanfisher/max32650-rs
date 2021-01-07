#[doc = "Reader of register OUTCSRU"]
pub type R = crate::R<u8, super::OUTCSRU>;
#[doc = "Writer for register OUTCSRU"]
pub type W = crate::W<u8, super::OUTCSRU>;
#[doc = "Register OUTCSRU `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTCSRU {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTOCLEAR`"]
pub type AUTOCLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOCLEAR`"]
pub struct AUTOCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ISO`"]
pub type ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISO`"]
pub struct ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMAREQEN`"]
pub type DMAREQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAREQEN`"]
pub struct DMAREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DISNYET`"]
pub type DISNYET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISNYET`"]
pub struct DISNYET_W<'a> {
    w: &'a mut W,
}
impl<'a> DISNYET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DMAREQMODE`"]
pub type DMAREQMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAREQMODE`"]
pub struct DMAREQMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DPKTBUFDIS`"]
pub type DPKTBUFDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPKTBUFDIS`"]
pub struct DPKTBUFDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPKTBUFDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `INCOMPRX`"]
pub type INCOMPRX_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn autoclear(&self) -> AUTOCLEAR_R {
        AUTOCLEAR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmareqen(&self) -> DMAREQEN_R {
        DMAREQEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disnyet(&self) -> DISNYET_R {
        DISNYET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dmareqmode(&self) -> DMAREQMODE_R {
        DMAREQMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dpktbufdis(&self) -> DPKTBUFDIS_R {
        DPKTBUFDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn incomprx(&self) -> INCOMPRX_R {
        INCOMPRX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn autoclear(&mut self) -> AUTOCLEAR_W {
        AUTOCLEAR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W {
        ISO_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmareqen(&mut self) -> DMAREQEN_W {
        DMAREQEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disnyet(&mut self) -> DISNYET_W {
        DISNYET_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dmareqmode(&mut self) -> DMAREQMODE_W {
        DMAREQMODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dpktbufdis(&mut self) -> DPKTBUFDIS_W {
        DPKTBUFDIS_W { w: self }
    }
}
