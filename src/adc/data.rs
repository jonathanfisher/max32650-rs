#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u32, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `adc_data`"]
pub type ADC_DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC Converted Sample Data Output"]
    #[inline(always)]
    pub fn adc_data(&self) -> ADC_DATA_R {
        ADC_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
