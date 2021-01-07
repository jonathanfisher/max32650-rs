#[doc = "Reader of register INTROUTEN"]
pub type R = crate::R<u16, super::INTROUTEN>;
#[doc = "Writer for register INTROUTEN"]
pub type W = crate::W<u16, super::INTROUTEN>;
#[doc = "Register INTROUTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTROUTEN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP15_OUT_INT_EN`"]
pub type EP15_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP15_OUT_INT_EN`"]
pub struct EP15_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP15_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `EP14_OUT_INT_EN`"]
pub type EP14_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP14_OUT_INT_EN`"]
pub struct EP14_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP14_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `EP13_OUT_INT_EN`"]
pub type EP13_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP13_OUT_INT_EN`"]
pub struct EP13_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP13_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `EP12_OUT_INT_EN`"]
pub type EP12_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP12_OUT_INT_EN`"]
pub struct EP12_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP12_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `EP11_OUT_INT_EN`"]
pub type EP11_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP11_OUT_INT_EN`"]
pub struct EP11_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP11_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `EP10_OUT_INT_EN`"]
pub type EP10_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP10_OUT_INT_EN`"]
pub struct EP10_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP10_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `EP9_OUT_INT_EN`"]
pub type EP9_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP9_OUT_INT_EN`"]
pub struct EP9_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP9_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `EP8_OUT_INT_EN`"]
pub type EP8_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP8_OUT_INT_EN`"]
pub struct EP8_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EP7_OUT_INT_EN`"]
pub type EP7_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7_OUT_INT_EN`"]
pub struct EP7_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `EP6_OUT_INT_EN`"]
pub type EP6_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6_OUT_INT_EN`"]
pub struct EP6_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EP5_OUT_INT_EN`"]
pub type EP5_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5_OUT_INT_EN`"]
pub struct EP5_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EP4_OUT_INT_EN`"]
pub type EP4_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4_OUT_INT_EN`"]
pub struct EP4_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `EP3_OUT_INT_EN`"]
pub type EP3_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3_OUT_INT_EN`"]
pub struct EP3_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EP2_OUT_INT_EN`"]
pub type EP2_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2_OUT_INT_EN`"]
pub struct EP2_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EP1_OUT_INT_EN`"]
pub type EP1_OUT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_OUT_INT_EN`"]
pub struct EP1_OUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_OUT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Endpoint 15 interrupt."]
    #[inline(always)]
    pub fn ep15_out_int_en(&self) -> EP15_OUT_INT_EN_R {
        EP15_OUT_INT_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt."]
    #[inline(always)]
    pub fn ep14_out_int_en(&self) -> EP14_OUT_INT_EN_R {
        EP14_OUT_INT_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt."]
    #[inline(always)]
    pub fn ep13_out_int_en(&self) -> EP13_OUT_INT_EN_R {
        EP13_OUT_INT_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt."]
    #[inline(always)]
    pub fn ep12_out_int_en(&self) -> EP12_OUT_INT_EN_R {
        EP12_OUT_INT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt."]
    #[inline(always)]
    pub fn ep11_out_int_en(&self) -> EP11_OUT_INT_EN_R {
        EP11_OUT_INT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt."]
    #[inline(always)]
    pub fn ep10_out_int_en(&self) -> EP10_OUT_INT_EN_R {
        EP10_OUT_INT_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt."]
    #[inline(always)]
    pub fn ep9_out_int_en(&self) -> EP9_OUT_INT_EN_R {
        EP9_OUT_INT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt."]
    #[inline(always)]
    pub fn ep8_out_int_en(&self) -> EP8_OUT_INT_EN_R {
        EP8_OUT_INT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt."]
    #[inline(always)]
    pub fn ep7_out_int_en(&self) -> EP7_OUT_INT_EN_R {
        EP7_OUT_INT_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt."]
    #[inline(always)]
    pub fn ep6_out_int_en(&self) -> EP6_OUT_INT_EN_R {
        EP6_OUT_INT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt."]
    #[inline(always)]
    pub fn ep5_out_int_en(&self) -> EP5_OUT_INT_EN_R {
        EP5_OUT_INT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt."]
    #[inline(always)]
    pub fn ep4_out_int_en(&self) -> EP4_OUT_INT_EN_R {
        EP4_OUT_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt."]
    #[inline(always)]
    pub fn ep3_out_int_en(&self) -> EP3_OUT_INT_EN_R {
        EP3_OUT_INT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt."]
    #[inline(always)]
    pub fn ep2_out_int_en(&self) -> EP2_OUT_INT_EN_R {
        EP2_OUT_INT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 interrupt."]
    #[inline(always)]
    pub fn ep1_out_int_en(&self) -> EP1_OUT_INT_EN_R {
        EP1_OUT_INT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Endpoint 15 interrupt."]
    #[inline(always)]
    pub fn ep15_out_int_en(&mut self) -> EP15_OUT_INT_EN_W {
        EP15_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt."]
    #[inline(always)]
    pub fn ep14_out_int_en(&mut self) -> EP14_OUT_INT_EN_W {
        EP14_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt."]
    #[inline(always)]
    pub fn ep13_out_int_en(&mut self) -> EP13_OUT_INT_EN_W {
        EP13_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt."]
    #[inline(always)]
    pub fn ep12_out_int_en(&mut self) -> EP12_OUT_INT_EN_W {
        EP12_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt."]
    #[inline(always)]
    pub fn ep11_out_int_en(&mut self) -> EP11_OUT_INT_EN_W {
        EP11_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt."]
    #[inline(always)]
    pub fn ep10_out_int_en(&mut self) -> EP10_OUT_INT_EN_W {
        EP10_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt."]
    #[inline(always)]
    pub fn ep9_out_int_en(&mut self) -> EP9_OUT_INT_EN_W {
        EP9_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt."]
    #[inline(always)]
    pub fn ep8_out_int_en(&mut self) -> EP8_OUT_INT_EN_W {
        EP8_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt."]
    #[inline(always)]
    pub fn ep7_out_int_en(&mut self) -> EP7_OUT_INT_EN_W {
        EP7_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt."]
    #[inline(always)]
    pub fn ep6_out_int_en(&mut self) -> EP6_OUT_INT_EN_W {
        EP6_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt."]
    #[inline(always)]
    pub fn ep5_out_int_en(&mut self) -> EP5_OUT_INT_EN_W {
        EP5_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt."]
    #[inline(always)]
    pub fn ep4_out_int_en(&mut self) -> EP4_OUT_INT_EN_W {
        EP4_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt."]
    #[inline(always)]
    pub fn ep3_out_int_en(&mut self) -> EP3_OUT_INT_EN_W {
        EP3_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt."]
    #[inline(always)]
    pub fn ep2_out_int_en(&mut self) -> EP2_OUT_INT_EN_W {
        EP2_OUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 interrupt."]
    #[inline(always)]
    pub fn ep1_out_int_en(&mut self) -> EP1_OUT_INT_EN_W {
        EP1_OUT_INT_EN_W { w: self }
    }
}
