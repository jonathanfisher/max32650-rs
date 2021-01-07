#[doc = "Reader of register INDEX"]
pub type R = crate::R<u8, super::INDEX>;
#[doc = "Writer for register INDEX"]
pub type W = crate::W<u8, super::INDEX>;
#[doc = "Register INDEX `reset()`'s with value 0"]
impl crate::ResetValue for super::INDEX {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INDEX`"]
pub type INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INDEX`"]
pub struct INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Index Register Access Selector."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Index Register Access Selector."]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W {
        INDEX_W { w: self }
    }
}
