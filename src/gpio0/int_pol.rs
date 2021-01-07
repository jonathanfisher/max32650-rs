#[doc = "Reader of register INT_POL"]
pub type R = crate::R<u32, super::INT_POL>;
#[doc = "Writer for register INT_POL"]
pub type W = crate::W<u32, super::INT_POL>;
#[doc = "Register INT_POL `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_POL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_POL_A {
    #[doc = "0: Interrupts are latched on a falling edge or low level condition for this pin."]
    FALLING = 0,
    #[doc = "1: Interrupts are latched on a rising edge or high condition for this pin."]
    RISING = 1,
}
impl From<GPIO_INT_POL_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_POL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_INT_POL`"]
pub type GPIO_INT_POL_R = crate::R<u32, GPIO_INT_POL_A>;
impl GPIO_INT_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_INT_POL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_INT_POL_A::FALLING),
            1 => Val(GPIO_INT_POL_A::RISING),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == GPIO_INT_POL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == GPIO_INT_POL_A::RISING
    }
}
#[doc = "Write proxy for field `GPIO_INT_POL`"]
pub struct GPIO_INT_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT_POL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupts are latched on a falling edge or low level condition for this pin."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(GPIO_INT_POL_A::FALLING)
    }
    #[doc = "Interrupts are latched on a rising edge or high condition for this pin."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(GPIO_INT_POL_A::RISING)
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
    pub fn gpio_int_pol(&self) -> GPIO_INT_POL_R {
        GPIO_INT_POL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_pol(&mut self) -> GPIO_INT_POL_W {
        GPIO_INT_POL_W { w: self }
    }
}
