#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `active`"]
pub type ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `afe_pwr_up_active`"]
pub type AFE_PWR_UP_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `overflow`"]
pub type OVERFLOW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ADC Conversion In Progress"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - AFE Power Up Delay Active"]
    #[inline(always)]
    pub fn afe_pwr_up_active(&self) -> AFE_PWR_UP_ACTIVE_R {
        AFE_PWR_UP_ACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC Overflow"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {}
