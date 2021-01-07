#[doc = "Reader of register OUTMAXP"]
pub type R = crate::R<u16, super::OUTMAXP>;
#[doc = "Writer for register OUTMAXP"]
pub type W = crate::W<u16, super::OUTMAXP>;
#[doc = "Register OUTMAXP `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTMAXP {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NUMPACKMINUS1`"]
pub type NUMPACKMINUS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMPACKMINUS1`"]
pub struct NUMPACKMINUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMPACKMINUS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u16) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `MAXPACKETSIZE`"]
pub type MAXPACKETSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAXPACKETSIZE`"]
pub struct MAXPACKETSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXPACKETSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u16) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:15 - Number of Split Packets -1. Defines the maximum number of packets - 1 that a usb payload is combined into. The value must be exact multiple of maxpacketsize."]
    #[inline(always)]
    pub fn numpackminus1(&self) -> NUMPACKMINUS1_R {
        NUMPACKMINUS1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 0:10 - Maximum Packet in a Single Transaction. This is the maximum packet size, in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations for the endpoint type set in USB2.0 spesification, chapter 9."]
    #[inline(always)]
    pub fn maxpacketsize(&self) -> MAXPACKETSIZE_R {
        MAXPACKETSIZE_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 11:15 - Number of Split Packets -1. Defines the maximum number of packets - 1 that a usb payload is combined into. The value must be exact multiple of maxpacketsize."]
    #[inline(always)]
    pub fn numpackminus1(&mut self) -> NUMPACKMINUS1_W {
        NUMPACKMINUS1_W { w: self }
    }
    #[doc = "Bits 0:10 - Maximum Packet in a Single Transaction. This is the maximum packet size, in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations for the endpoint type set in USB2.0 spesification, chapter 9."]
    #[inline(always)]
    pub fn maxpacketsize(&mut self) -> MAXPACKETSIZE_W {
        MAXPACKETSIZE_W { w: self }
    }
}
