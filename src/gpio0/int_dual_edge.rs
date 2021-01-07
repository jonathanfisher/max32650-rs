#[doc = "Reader of register INT_DUAL_EDGE"]
pub type R = crate::R<u32, super::INT_DUAL_EDGE>;
#[doc = "Writer for register INT_DUAL_EDGE"]
pub type W = crate::W<u32, super::INT_DUAL_EDGE>;
#[doc = "Register INT_DUAL_EDGE `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_DUAL_EDGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_DUAL_EDGE_A {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    EN = 1,
}
impl From<GPIO_INT_DUAL_EDGE_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_DUAL_EDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_INT_DUAL_EDGE`"]
pub type GPIO_INT_DUAL_EDGE_R = crate::R<u32, GPIO_INT_DUAL_EDGE_A>;
impl GPIO_INT_DUAL_EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_INT_DUAL_EDGE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_INT_DUAL_EDGE_A::NO),
            1 => Val(GPIO_INT_DUAL_EDGE_A::EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_INT_DUAL_EDGE_A::NO
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_INT_DUAL_EDGE_A::EN
    }
}
#[doc = "Write proxy for field `GPIO_INT_DUAL_EDGE`"]
pub struct GPIO_INT_DUAL_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_DUAL_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT_DUAL_EDGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_INT_DUAL_EDGE_A::NO)
    }
    #[doc = "Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO_INT_DUAL_EDGE_A::EN)
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
    pub fn gpio_int_dual_edge(&self) -> GPIO_INT_DUAL_EDGE_R {
        GPIO_INT_DUAL_EDGE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_dual_edge(&mut self) -> GPIO_INT_DUAL_EDGE_W {
        GPIO_INT_DUAL_EDGE_W { w: self }
    }
}
