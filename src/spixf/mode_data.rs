#[doc = "Reader of register MODE_DATA"]
pub type R = crate::R<u32, super::MODE_DATA>;
#[doc = "Writer for register MODE_DATA"]
pub type W = crate::W<u32, super::MODE_DATA>;
#[doc = "Register MODE_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::MODE_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `OUT_EN`"]
pub type OUT_EN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OUT_EN`"]
pub struct OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Mode Data. Specifies the data to send with the Dummy/Mode clocks."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Mode Output Enable. Output enable state for each corresponding data bit in MD_DATA. 0: output enable off, I/O is tristate and 1: Output enable on, I/O is driving MD_DATA."]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mode Data. Specifies the data to send with the Dummy/Mode clocks."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 16:31 - Mode Output Enable. Output enable state for each corresponding data bit in MD_DATA. 0: output enable off, I/O is tristate and 1: Output enable on, I/O is driving MD_DATA."]
    #[inline(always)]
    pub fn out_en(&mut self) -> OUT_EN_W {
        OUT_EN_W { w: self }
    }
}
