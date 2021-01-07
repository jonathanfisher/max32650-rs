#[doc = "Reader of register EN"]
pub type R = crate::R<u32, super::EN>;
#[doc = "Writer for register EN"]
pub type W = crate::W<u32, super::EN>;
#[doc = "Register EN `reset()`'s with value 0"]
impl crate::ResetValue for super::EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_EN_A {
    #[doc = "0: Alternate function enabled."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function is enabled."]
    GPIO = 1,
}
impl From<GPIO_EN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_EN`"]
pub type GPIO_EN_R = crate::R<u32, GPIO_EN_A>;
impl GPIO_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_EN_A::ALTERNATE),
            1 => Val(GPIO_EN_A::GPIO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == GPIO_EN_A::ALTERNATE
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == GPIO_EN_A::GPIO
    }
}
#[doc = "Write proxy for field `GPIO_EN`"]
pub struct GPIO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Alternate function enabled."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(GPIO_EN_A::ALTERNATE)
    }
    #[doc = "GPIO function is enabled."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(GPIO_EN_A::GPIO)
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
    pub fn gpio_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en(&mut self) -> GPIO_EN_W {
        GPIO_EN_W { w: self }
    }
}
