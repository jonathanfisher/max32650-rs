#[doc = "Reader of register REG2"]
pub type R = crate::R<u32, super::REG2>;
#[doc = "Writer for register REG2"]
pub type W = crate::W<u32, super::REG2>;
#[doc = "Register REG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::REG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INITTRM`"]
pub type INITTRM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INITTRM`"]
pub struct INITTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> INITTRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `MINTRM`"]
pub type MINTRM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MINTRM`"]
pub struct MINTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> MINTRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 10)) | (((value as u32) & 0x01ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `MAXTRM`"]
pub type MAXTRM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAXTRM`"]
pub struct MAXTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXTRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 20)) | (((value as u32) & 0x01ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Initial Trim Setting."]
    #[inline(always)]
    pub fn inittrm(&self) -> INITTRM_R {
        INITTRM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 10:18 - Minimum Trim Setting."]
    #[inline(always)]
    pub fn mintrm(&self) -> MINTRM_R {
        MINTRM_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:28 - Maximum Trim Setting."]
    #[inline(always)]
    pub fn maxtrm(&self) -> MAXTRM_R {
        MAXTRM_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Initial Trim Setting."]
    #[inline(always)]
    pub fn inittrm(&mut self) -> INITTRM_W {
        INITTRM_W { w: self }
    }
    #[doc = "Bits 10:18 - Minimum Trim Setting."]
    #[inline(always)]
    pub fn mintrm(&mut self) -> MINTRM_W {
        MINTRM_W { w: self }
    }
    #[doc = "Bits 20:28 - Maximum Trim Setting."]
    #[inline(always)]
    pub fn maxtrm(&mut self) -> MAXTRM_W {
        MAXTRM_W { w: self }
    }
}
