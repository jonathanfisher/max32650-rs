#[doc = "Reader of register INTRINEN"]
pub type R = crate::R<u16, super::INTRINEN>;
#[doc = "Writer for register INTRINEN"]
pub type W = crate::W<u16, super::INTRINEN>;
#[doc = "Register INTRINEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTRINEN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP15_IN_INT_EN`"]
pub type EP15_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP15_IN_INT_EN`"]
pub struct EP15_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP15_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP14_IN_INT_EN`"]
pub type EP14_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP14_IN_INT_EN`"]
pub struct EP14_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP14_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP13_IN_INT_EN`"]
pub type EP13_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP13_IN_INT_EN`"]
pub struct EP13_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP13_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP12_IN_INT_EN`"]
pub type EP12_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP12_IN_INT_EN`"]
pub struct EP12_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP12_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP11_IN_INT_EN`"]
pub type EP11_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP11_IN_INT_EN`"]
pub struct EP11_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP11_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP10_IN_INT_EN`"]
pub type EP10_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP10_IN_INT_EN`"]
pub struct EP10_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP10_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP9_IN_INT_EN`"]
pub type EP9_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP9_IN_INT_EN`"]
pub struct EP9_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP9_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP8_IN_INT_EN`"]
pub type EP8_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP8_IN_INT_EN`"]
pub struct EP8_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP7_IN_INT_EN`"]
pub type EP7_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7_IN_INT_EN`"]
pub struct EP7_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP6_IN_INT_EN`"]
pub type EP6_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6_IN_INT_EN`"]
pub struct EP6_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP5_IN_INT_EN`"]
pub type EP5_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5_IN_INT_EN`"]
pub struct EP5_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP4_IN_INT_EN`"]
pub type EP4_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4_IN_INT_EN`"]
pub struct EP4_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP3_IN_INT_EN`"]
pub type EP3_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3_IN_INT_EN`"]
pub struct EP3_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP2_IN_INT_EN`"]
pub type EP2_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2_IN_INT_EN`"]
pub struct EP2_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP1_IN_INT_EN`"]
pub type EP1_IN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_IN_INT_EN`"]
pub struct EP1_IN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_IN_INT_EN_W<'a> {
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
#[doc = "Reader of field `EP0_INT_EN`"]
pub type EP0_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INT_EN`"]
pub struct EP0_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Endpoint 15 interrupt enable."]
    #[inline(always)]
    pub fn ep15_in_int_en(&self) -> EP15_IN_INT_EN_R {
        EP15_IN_INT_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt enable."]
    #[inline(always)]
    pub fn ep14_in_int_en(&self) -> EP14_IN_INT_EN_R {
        EP14_IN_INT_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt enable."]
    #[inline(always)]
    pub fn ep13_in_int_en(&self) -> EP13_IN_INT_EN_R {
        EP13_IN_INT_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt enable."]
    #[inline(always)]
    pub fn ep12_in_int_en(&self) -> EP12_IN_INT_EN_R {
        EP12_IN_INT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt enable."]
    #[inline(always)]
    pub fn ep11_in_int_en(&self) -> EP11_IN_INT_EN_R {
        EP11_IN_INT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt enable."]
    #[inline(always)]
    pub fn ep10_in_int_en(&self) -> EP10_IN_INT_EN_R {
        EP10_IN_INT_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt enable."]
    #[inline(always)]
    pub fn ep9_in_int_en(&self) -> EP9_IN_INT_EN_R {
        EP9_IN_INT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt enable."]
    #[inline(always)]
    pub fn ep8_in_int_en(&self) -> EP8_IN_INT_EN_R {
        EP8_IN_INT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt enable."]
    #[inline(always)]
    pub fn ep7_in_int_en(&self) -> EP7_IN_INT_EN_R {
        EP7_IN_INT_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt enable."]
    #[inline(always)]
    pub fn ep6_in_int_en(&self) -> EP6_IN_INT_EN_R {
        EP6_IN_INT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt enable."]
    #[inline(always)]
    pub fn ep5_in_int_en(&self) -> EP5_IN_INT_EN_R {
        EP5_IN_INT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt enable."]
    #[inline(always)]
    pub fn ep4_in_int_en(&self) -> EP4_IN_INT_EN_R {
        EP4_IN_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt enable."]
    #[inline(always)]
    pub fn ep3_in_int_en(&self) -> EP3_IN_INT_EN_R {
        EP3_IN_INT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt enable."]
    #[inline(always)]
    pub fn ep2_in_int_en(&self) -> EP2_IN_INT_EN_R {
        EP2_IN_INT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 interrupt enable."]
    #[inline(always)]
    pub fn ep1_in_int_en(&self) -> EP1_IN_INT_EN_R {
        EP1_IN_INT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Endpoint 0 interrupt enable."]
    #[inline(always)]
    pub fn ep0_int_en(&self) -> EP0_INT_EN_R {
        EP0_INT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Endpoint 15 interrupt enable."]
    #[inline(always)]
    pub fn ep15_in_int_en(&mut self) -> EP15_IN_INT_EN_W {
        EP15_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt enable."]
    #[inline(always)]
    pub fn ep14_in_int_en(&mut self) -> EP14_IN_INT_EN_W {
        EP14_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt enable."]
    #[inline(always)]
    pub fn ep13_in_int_en(&mut self) -> EP13_IN_INT_EN_W {
        EP13_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt enable."]
    #[inline(always)]
    pub fn ep12_in_int_en(&mut self) -> EP12_IN_INT_EN_W {
        EP12_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt enable."]
    #[inline(always)]
    pub fn ep11_in_int_en(&mut self) -> EP11_IN_INT_EN_W {
        EP11_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt enable."]
    #[inline(always)]
    pub fn ep10_in_int_en(&mut self) -> EP10_IN_INT_EN_W {
        EP10_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt enable."]
    #[inline(always)]
    pub fn ep9_in_int_en(&mut self) -> EP9_IN_INT_EN_W {
        EP9_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt enable."]
    #[inline(always)]
    pub fn ep8_in_int_en(&mut self) -> EP8_IN_INT_EN_W {
        EP8_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt enable."]
    #[inline(always)]
    pub fn ep7_in_int_en(&mut self) -> EP7_IN_INT_EN_W {
        EP7_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt enable."]
    #[inline(always)]
    pub fn ep6_in_int_en(&mut self) -> EP6_IN_INT_EN_W {
        EP6_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt enable."]
    #[inline(always)]
    pub fn ep5_in_int_en(&mut self) -> EP5_IN_INT_EN_W {
        EP5_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt enable."]
    #[inline(always)]
    pub fn ep4_in_int_en(&mut self) -> EP4_IN_INT_EN_W {
        EP4_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt enable."]
    #[inline(always)]
    pub fn ep3_in_int_en(&mut self) -> EP3_IN_INT_EN_W {
        EP3_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt enable."]
    #[inline(always)]
    pub fn ep2_in_int_en(&mut self) -> EP2_IN_INT_EN_W {
        EP2_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 interrupt enable."]
    #[inline(always)]
    pub fn ep1_in_int_en(&mut self) -> EP1_IN_INT_EN_W {
        EP1_IN_INT_EN_W { w: self }
    }
    #[doc = "Bit 0 - Endpoint 0 interrupt enable."]
    #[inline(always)]
    pub fn ep0_int_en(&mut self) -> EP0_INT_EN_W {
        EP0_INT_EN_W { w: self }
    }
}
