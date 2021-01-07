#[doc = "Reader of register EN2"]
pub type R = crate::R<u32, super::EN2>;
#[doc = "Writer for register EN2"]
pub type W = crate::W<u32, super::EN2>;
#[doc = "Register EN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_EN2_A {
    #[doc = "0: Primary function selected."]
    PRIMARY = 0,
    #[doc = "1: Secondary function selected."]
    SECONDARY = 1,
}
impl From<GPIO_EN2_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_EN2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_EN2`"]
pub type GPIO_EN2_R = crate::R<u32, GPIO_EN2_A>;
impl GPIO_EN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_EN2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_EN2_A::PRIMARY),
            1 => Val(GPIO_EN2_A::SECONDARY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRIMARY`"]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        *self == GPIO_EN2_A::PRIMARY
    }
    #[doc = "Checks if the value of the field is `SECONDARY`"]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == GPIO_EN2_A::SECONDARY
    }
}
#[doc = "Write proxy for field `GPIO_EN2`"]
pub struct GPIO_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_EN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_EN2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Primary function selected."]
    #[inline(always)]
    pub fn primary(self) -> &'a mut W {
        self.variant(GPIO_EN2_A::PRIMARY)
    }
    #[doc = "Secondary function selected."]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut W {
        self.variant(GPIO_EN2_A::SECONDARY)
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
    pub fn gpio_en2(&self) -> GPIO_EN2_R {
        GPIO_EN2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en2(&mut self) -> GPIO_EN2_W {
        GPIO_EN2_W { w: self }
    }
}
