#[doc = "Reader of register IRQ_FLAG"]
pub type R = crate::R<u32, super::IRQ_FLAG>;
#[doc = "Writer for register IRQ_FLAG"]
pub type W = crate::W<u32, super::IRQ_FLAG>;
#[doc = "Register IRQ_FLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_FLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRQ_FLAG`"]
pub type IRQ_FLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ_FLAG`"]
pub struct IRQ_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_FLAG_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Flag."]
    #[inline(always)]
    pub fn irq_flag(&self) -> IRQ_FLAG_R {
        IRQ_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Flag."]
    #[inline(always)]
    pub fn irq_flag(&mut self) -> IRQ_FLAG_W {
        IRQ_FLAG_W { w: self }
    }
}
