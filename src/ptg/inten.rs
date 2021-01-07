#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pt0`"]
pub type PT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt0`"]
pub struct PT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PT0_W<'a> {
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
#[doc = "Reader of field `pt1`"]
pub type PT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt1`"]
pub struct PT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `pt2`"]
pub type PT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt2`"]
pub struct PT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `pt3`"]
pub type PT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt3`"]
pub struct PT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `pt4`"]
pub type PT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt4`"]
pub struct PT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `pt5`"]
pub type PT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt5`"]
pub struct PT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `pt6`"]
pub type PT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt6`"]
pub struct PT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `pt7`"]
pub type PT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt7`"]
pub struct PT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `pt8`"]
pub type PT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt8`"]
pub struct PT8_W<'a> {
    w: &'a mut W,
}
impl<'a> PT8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `pt9`"]
pub type PT9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt9`"]
pub struct PT9_W<'a> {
    w: &'a mut W,
}
impl<'a> PT9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `pt10`"]
pub type PT10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt10`"]
pub struct PT10_W<'a> {
    w: &'a mut W,
}
impl<'a> PT10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `pt11`"]
pub type PT11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt11`"]
pub struct PT11_W<'a> {
    w: &'a mut W,
}
impl<'a> PT11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `pt12`"]
pub type PT12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt12`"]
pub struct PT12_W<'a> {
    w: &'a mut W,
}
impl<'a> PT12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `pt13`"]
pub type PT13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt13`"]
pub struct PT13_W<'a> {
    w: &'a mut W,
}
impl<'a> PT13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `pt14`"]
pub type PT14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt14`"]
pub struct PT14_W<'a> {
    w: &'a mut W,
}
impl<'a> PT14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `pt15`"]
pub type PT15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pt15`"]
pub struct PT15_W<'a> {
    w: &'a mut W,
}
impl<'a> PT15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt0(&self) -> PT0_R {
        PT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt1(&self) -> PT1_R {
        PT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt2(&self) -> PT2_R {
        PT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt3(&self) -> PT3_R {
        PT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pulse Train 4 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt4(&self) -> PT4_R {
        PT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pulse Train 5 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt5(&self) -> PT5_R {
        PT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pulse Train 6 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt6(&self) -> PT6_R {
        PT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pulse Train 7 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt7(&self) -> PT7_R {
        PT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pulse Train 8 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt8(&self) -> PT8_R {
        PT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pulse Train 9 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt9(&self) -> PT9_R {
        PT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pulse Train 10 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt10(&self) -> PT10_R {
        PT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pulse Train 11 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt11(&self) -> PT11_R {
        PT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pulse Train 12 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt12(&self) -> PT12_R {
        PT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pulse Train 13 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt13(&self) -> PT13_R {
        PT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pulse Train 14 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt14(&self) -> PT14_R {
        PT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pulse Train 15 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt15(&self) -> PT15_R {
        PT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt0(&mut self) -> PT0_W {
        PT0_W { w: self }
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt1(&mut self) -> PT1_W {
        PT1_W { w: self }
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt2(&mut self) -> PT2_W {
        PT2_W { w: self }
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt3(&mut self) -> PT3_W {
        PT3_W { w: self }
    }
    #[doc = "Bit 4 - Pulse Train 4 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt4(&mut self) -> PT4_W {
        PT4_W { w: self }
    }
    #[doc = "Bit 5 - Pulse Train 5 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt5(&mut self) -> PT5_W {
        PT5_W { w: self }
    }
    #[doc = "Bit 6 - Pulse Train 6 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt6(&mut self) -> PT6_W {
        PT6_W { w: self }
    }
    #[doc = "Bit 7 - Pulse Train 7 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt7(&mut self) -> PT7_W {
        PT7_W { w: self }
    }
    #[doc = "Bit 8 - Pulse Train 8 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt8(&mut self) -> PT8_W {
        PT8_W { w: self }
    }
    #[doc = "Bit 9 - Pulse Train 9 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt9(&mut self) -> PT9_W {
        PT9_W { w: self }
    }
    #[doc = "Bit 10 - Pulse Train 10 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt10(&mut self) -> PT10_W {
        PT10_W { w: self }
    }
    #[doc = "Bit 11 - Pulse Train 11 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt11(&mut self) -> PT11_W {
        PT11_W { w: self }
    }
    #[doc = "Bit 12 - Pulse Train 12 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt12(&mut self) -> PT12_W {
        PT12_W { w: self }
    }
    #[doc = "Bit 13 - Pulse Train 13 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt13(&mut self) -> PT13_W {
        PT13_W { w: self }
    }
    #[doc = "Bit 14 - Pulse Train 14 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt14(&mut self) -> PT14_W {
        PT14_W { w: self }
    }
    #[doc = "Bit 15 - Pulse Train 15 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt15(&mut self) -> PT15_W {
        PT15_W { w: self }
    }
}
