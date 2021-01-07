#[doc = "Reader of register INT_EN_SET"]
pub type R = crate::R<u32, super::INT_EN_SET>;
#[doc = "Writer for register INT_EN_SET"]
pub type W = crate::W<u32, super::INT_EN_SET>;
#[doc = "Register INT_EN_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_EN_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_EN_SET_A {
    #[doc = "0: No effect."]
    NO = 0,
    #[doc = "1: Set GPIO_INT_EN bit in this position to '1'"]
    SET = 1,
}
impl From<GPIO_INT_EN_SET_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_EN_SET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_INT_EN_SET`"]
pub type GPIO_INT_EN_SET_R = crate::R<u32, GPIO_INT_EN_SET_A>;
impl GPIO_INT_EN_SET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_INT_EN_SET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_INT_EN_SET_A::NO),
            1 => Val(GPIO_INT_EN_SET_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_INT_EN_SET_A::NO
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == GPIO_INT_EN_SET_A::SET
    }
}
#[doc = "Write proxy for field `GPIO_INT_EN_SET`"]
pub struct GPIO_INT_EN_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_EN_SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT_EN_SET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_INT_EN_SET_A::NO)
    }
    #[doc = "Set GPIO_INT_EN bit in this position to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(GPIO_INT_EN_SET_A::SET)
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
    pub fn gpio_int_en_set(&self) -> GPIO_INT_EN_SET_R {
        GPIO_INT_EN_SET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_en_set(&mut self) -> GPIO_INT_EN_SET_W {
        GPIO_INT_EN_SET_W { w: self }
    }
}
