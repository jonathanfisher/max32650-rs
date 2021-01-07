#[doc = "Reader of register PWR"]
pub type R = crate::R<u8, super::PWR>;
#[doc = "Writer for register PWR"]
pub type W = crate::W<u8, super::PWR>;
#[doc = "Register PWR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUS_POWER`"]
pub type BUS_POWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_POWER`"]
pub struct BUS_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_POWER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `BUS_VOLT_SEL`"]
pub type BUS_VOLT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUS_VOLT_SEL`"]
pub struct BUS_VOLT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_VOLT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u8) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SD Bus Power."]
    #[inline(always)]
    pub fn bus_power(&self) -> BUS_POWER_R {
        BUS_POWER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select."]
    #[inline(always)]
    pub fn bus_volt_sel(&self) -> BUS_VOLT_SEL_R {
        BUS_VOLT_SEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power."]
    #[inline(always)]
    pub fn bus_power(&mut self) -> BUS_POWER_W {
        BUS_POWER_W { w: self }
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select."]
    #[inline(always)]
    pub fn bus_volt_sel(&mut self) -> BUS_VOLT_SEL_W {
        BUS_VOLT_SEL_W { w: self }
    }
}
