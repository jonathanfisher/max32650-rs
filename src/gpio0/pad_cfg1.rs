#[doc = "Reader of register PAD_CFG1"]
pub type R = crate::R<u32, super::PAD_CFG1>;
#[doc = "Writer for register PAD_CFG1"]
pub type W = crate::W<u32, super::PAD_CFG1>;
#[doc = "Register PAD_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PAD_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_PAD_CFG1_A {
    #[doc = "0: High Impedance."]
    IMPEDANCE = 0,
    #[doc = "1: Weak pull-up mode."]
    PU = 1,
    #[doc = "2: weak pull-down mode."]
    PD = 2,
}
impl From<GPIO_PAD_CFG1_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_PAD_CFG1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_PAD_CFG1`"]
pub type GPIO_PAD_CFG1_R = crate::R<u32, GPIO_PAD_CFG1_A>;
impl GPIO_PAD_CFG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_PAD_CFG1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_PAD_CFG1_A::IMPEDANCE),
            1 => Val(GPIO_PAD_CFG1_A::PU),
            2 => Val(GPIO_PAD_CFG1_A::PD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMPEDANCE`"]
    #[inline(always)]
    pub fn is_impedance(&self) -> bool {
        *self == GPIO_PAD_CFG1_A::IMPEDANCE
    }
    #[doc = "Checks if the value of the field is `PU`"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == GPIO_PAD_CFG1_A::PU
    }
    #[doc = "Checks if the value of the field is `PD`"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == GPIO_PAD_CFG1_A::PD
    }
}
#[doc = "Write proxy for field `GPIO_PAD_CFG1`"]
pub struct GPIO_PAD_CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PAD_CFG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_PAD_CFG1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "High Impedance."]
    #[inline(always)]
    pub fn impedance(self) -> &'a mut W {
        self.variant(GPIO_PAD_CFG1_A::IMPEDANCE)
    }
    #[doc = "Weak pull-up mode."]
    #[inline(always)]
    pub fn pu(self) -> &'a mut W {
        self.variant(GPIO_PAD_CFG1_A::PU)
    }
    #[doc = "weak pull-down mode."]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(GPIO_PAD_CFG1_A::PD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_pad_cfg1(&self) -> GPIO_PAD_CFG1_R {
        GPIO_PAD_CFG1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_pad_cfg1(&mut self) -> GPIO_PAD_CFG1_W {
        GPIO_PAD_CFG1_W { w: self }
    }
}
