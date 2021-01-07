#[doc = "Reader of register INT_EN_CLR"]
pub type R = crate::R<u32, super::INT_EN_CLR>;
#[doc = "Writer for register INT_EN_CLR"]
pub type W = crate::W<u32, super::INT_EN_CLR>;
#[doc = "Register INT_EN_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_EN_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_EN_CLR_A {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Clear GPIO_INT_EN bit in this position to '0'"]
    CLEAR = 1,
}
impl From<GPIO_INT_EN_CLR_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_EN_CLR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_INT_EN_CLR`"]
pub type GPIO_INT_EN_CLR_R = crate::R<u32, GPIO_INT_EN_CLR_A>;
impl GPIO_INT_EN_CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_INT_EN_CLR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_INT_EN_CLR_A::NO),
            1 => Val(GPIO_INT_EN_CLR_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_INT_EN_CLR_A::NO
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO_INT_EN_CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `GPIO_INT_EN_CLR`"]
pub struct GPIO_INT_EN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_EN_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT_EN_CLR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_INT_EN_CLR_A::NO)
    }
    #[doc = "Clear GPIO_INT_EN bit in this position to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO_INT_EN_CLR_A::CLEAR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_en_clr(&self) -> GPIO_INT_EN_CLR_R {
        GPIO_INT_EN_CLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_en_clr(&mut self) -> GPIO_INT_EN_CLR_W {
        GPIO_INT_EN_CLR_W { w: self }
    }
}
