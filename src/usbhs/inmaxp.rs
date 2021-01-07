#[doc = "Reader of register INMAXP"]
pub type R = crate::R<u16, super::INMAXP>;
#[doc = "Writer for register INMAXP"]
pub type W = crate::W<u16, super::INMAXP>;
#[doc = "Register INMAXP `reset()`'s with value 0"]
impl crate::ResetValue for super::INMAXP {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size in a Single Transaction. That is the maximum packet size in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations of the endpoint type set in USB 2.0 Specification, Chapter 9"]
    #[inline(always)]
    pub fn maxpacketsize(&self) -> MAXPACKETSIZE_R {
        MAXPACKETSIZE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - Number of Split Packets - 1. Defines the maximum number of packets minus 1 that a USB payload can be split into. THis must be an exact multiple of maxpacketsize. Only applicable for HS High-Bandwidth isochronous endpoints and Bulk endpoints. Ignored in all other cases."]
    #[inline(always)]
    pub fn numpackminus1(&self) -> NUMPACKMINUS1_R {
        NUMPACKMINUS1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size in a Single Transaction. That is the maximum packet size in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations of the endpoint type set in USB 2.0 Specification, Chapter 9"]
    #[inline(always)]
    pub fn maxpacketsize(&mut self) -> MAXPACKETSIZE_W {
        MAXPACKETSIZE_W { w: self }
    }
    #[doc = "Bits 11:15 - Number of Split Packets - 1. Defines the maximum number of packets minus 1 that a USB payload can be split into. THis must be an exact multiple of maxpacketsize. Only applicable for HS High-Bandwidth isochronous endpoints and Bulk endpoints. Ignored in all other cases."]
    #[inline(always)]
    pub fn numpackminus1(&mut self) -> NUMPACKMINUS1_W {
        NUMPACKMINUS1_W { w: self }
    }
}
