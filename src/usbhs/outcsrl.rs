#[doc = "Reader of register OUTCSRL"]
pub type R = crate::R<u8, super::OUTCSRL>;
#[doc = "Writer for register OUTCSRL"]
pub type W = crate::W<u8, super::OUTCSRL>;
#[doc = "Register OUTCSRL `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTCSRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRDATATOG`"]
pub type CLRDATATOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRDATATOG`"]
pub struct CLRDATATOG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRDATATOG_W<'a> {
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
#[doc = "Reader of field `SENTSTALL`"]
pub type SENTSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENTSTALL`"]
pub struct SENTSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENTSTALL_W<'a> {
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
#[doc = "Reader of field `SENDSTALL`"]
pub type SENDSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENDSTALL`"]
pub struct SENDSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDSTALL_W<'a> {
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
#[doc = "Reader of field `FLUSHFIFO`"]
pub type FLUSHFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLUSHFIFO`"]
pub struct FLUSHFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHFIFO_W<'a> {
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
#[doc = "Reader of field `DATAERROR`"]
pub type DATAERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERRUN`"]
pub struct OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FIFOFULL`"]
pub type FIFOFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTPKTRDY`"]
pub type OUTPKTRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTPKTRDY`"]
pub struct OUTPKTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPKTRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clrdatatog(&self) -> CLRDATATOG_R {
        CLRDATATOG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sentstall(&self) -> SENTSTALL_R {
        SENTSTALL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sendstall(&self) -> SENDSTALL_R {
        SENDSTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flushfifo(&self) -> FLUSHFIFO_R {
        FLUSHFIFO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dataerror(&self) -> DATAERROR_R {
        DATAERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn outpktrdy(&self) -> OUTPKTRDY_R {
        OUTPKTRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> CLRDATATOG_W {
        CLRDATATOG_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SENTSTALL_W {
        SENTSTALL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SENDSTALL_W {
        SENDSTALL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FLUSHFIFO_W {
        FLUSHFIFO_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn outpktrdy(&mut self) -> OUTPKTRDY_W {
        OUTPKTRDY_W { w: self }
    }
}
