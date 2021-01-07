#[doc = "Reader of register OUT_EN"]
pub type R = crate::R<u32, super::OUT_EN>;
#[doc = "Writer for register OUT_EN"]
pub type W = crate::W<u32, super::OUT_EN>;
#[doc = "Register OUT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_OUT_EN_A {
    #[doc = "0: GPIO Output Disable"]
    DIS = 0,
    #[doc = "1: GPIO Output Enable"]
    EN = 1,
}
impl From<GPIO_OUT_EN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_OUT_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_OUT_EN`"]
pub type GPIO_OUT_EN_R = crate::R<u32, GPIO_OUT_EN_A>;
impl GPIO_OUT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_OUT_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_OUT_EN_A::DIS),
            1 => Val(GPIO_OUT_EN_A::EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO_OUT_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_OUT_EN_A::EN
    }
}
#[doc = "Write proxy for field `GPIO_OUT_EN`"]
pub struct GPIO_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_OUT_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO Output Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO_OUT_EN_A::DIS)
    }
    #[doc = "GPIO Output Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO_OUT_EN_A::EN)
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
    pub fn gpio_out_en(&self) -> GPIO_OUT_EN_R {
        GPIO_OUT_EN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out_en(&mut self) -> GPIO_OUT_EN_W {
        GPIO_OUT_EN_W { w: self }
    }
}
