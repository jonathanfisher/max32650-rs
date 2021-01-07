#[doc = "Reader of register INT_STAT"]
pub type R = crate::R<u32, super::INT_STAT>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_STAT_A {
    #[doc = "0: No Interrupt is pending on this GPIO pin."]
    NO = 0,
    #[doc = "1: An Interrupt is pending on this GPIO pin."]
    PENDING = 1,
}
impl From<GPIO_INT_STAT_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_INT_STAT`"]
pub type GPIO_INT_STAT_R = crate::R<u32, GPIO_INT_STAT_A>;
impl GPIO_INT_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_INT_STAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_INT_STAT_A::NO),
            1 => Val(GPIO_INT_STAT_A::PENDING),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_INT_STAT_A::NO
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == GPIO_INT_STAT_A::PENDING
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_int_stat(&self) -> GPIO_INT_STAT_R {
        GPIO_INT_STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
