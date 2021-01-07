#[doc = "Reader of register INT_MOD"]
pub type R = crate::R<u32, super::INT_MOD>;
#[doc = "Writer for register INT_MOD"]
pub type W = crate::W<u32, super::INT_MOD>;
#[doc = "Register INT_MOD `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_MOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_MOD_A {
    #[doc = "0: Interrupts for this pin are level triggered."]
    LEVEL = 0,
    #[doc = "1: Interrupts for this pin are edge triggered."]
    EDGE = 1,
}
impl From<GPIO_INT_MOD_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_MOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_INT_MOD`"]
pub type GPIO_INT_MOD_R = crate::R<u32, GPIO_INT_MOD_A>;
impl GPIO_INT_MOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_INT_MOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_INT_MOD_A::LEVEL),
            1 => Val(GPIO_INT_MOD_A::EDGE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_INT_MOD_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_INT_MOD_A::EDGE
    }
}
#[doc = "Write proxy for field `GPIO_INT_MOD`"]
pub struct GPIO_INT_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_MOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT_MOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupts for this pin are level triggered."]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_INT_MOD_A::LEVEL)
    }
    #[doc = "Interrupts for this pin are edge triggered."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_INT_MOD_A::EDGE)
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
    pub fn gpio_int_mod(&self) -> GPIO_INT_MOD_R {
        GPIO_INT_MOD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_mod(&mut self) -> GPIO_INT_MOD_W {
        GPIO_INT_MOD_W { w: self }
    }
}
