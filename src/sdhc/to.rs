#[doc = "Reader of register TO"]
pub type R = crate::R<u8, super::TO>;
#[doc = "Writer for register TO"]
pub type W = crate::W<u8, super::TO>;
#[doc = "Register TO `reset()`'s with value 0"]
impl crate::ResetValue for super::TO {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_COUNT_VALUE`"]
pub type DATA_COUNT_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_COUNT_VALUE`"]
pub struct DATA_COUNT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_COUNT_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Data Timeout Counter Value."]
    #[inline(always)]
    pub fn data_count_value(&self) -> DATA_COUNT_VALUE_R {
        DATA_COUNT_VALUE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Timeout Counter Value."]
    #[inline(always)]
    pub fn data_count_value(&mut self) -> DATA_COUNT_VALUE_W {
        DATA_COUNT_VALUE_W { w: self }
    }
}
