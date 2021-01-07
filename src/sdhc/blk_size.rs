#[doc = "Reader of register BLK_SIZE"]
pub type R = crate::R<u16, super::BLK_SIZE>;
#[doc = "Writer for register BLK_SIZE"]
pub type W = crate::W<u16, super::BLK_SIZE>;
#[doc = "Register BLK_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK_SIZE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRANS`"]
pub type TRANS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TRANS`"]
pub struct TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u16) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `HOST_BUFF`"]
pub type HOST_BUFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_BUFF`"]
pub struct HOST_BUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_BUFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size."]
    #[inline(always)]
    pub fn trans(&self) -> TRANS_R {
        TRANS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Boundary."]
    #[inline(always)]
    pub fn host_buff(&self) -> HOST_BUFF_R {
        HOST_BUFF_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size."]
    #[inline(always)]
    pub fn trans(&mut self) -> TRANS_W {
        TRANS_W { w: self }
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Boundary."]
    #[inline(always)]
    pub fn host_buff(&mut self) -> HOST_BUFF_W {
        HOST_BUFF_W { w: self }
    }
}
