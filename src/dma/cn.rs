#[doc = "Reader of register CN"]
pub type R = crate::R<u32, super::CN>;
#[doc = "Writer for register CN"]
pub type W = crate::W<u32, super::CN>;
#[doc = "Register CN `reset()`'s with value 0"]
impl crate::ResetValue for super::CN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_IEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CH0_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0_IEN`"]
pub type CH0_IEN_R = crate::R<bool, CH0_IEN_A>;
impl CH0_IEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_IEN_A {
        match self.bits {
            false => CH0_IEN_A::DIS,
            true => CH0_IEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH0_IEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH0_IEN_A::EN
    }
}
#[doc = "Write proxy for field `CH0_IEN`"]
pub struct CH0_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_IEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_IEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CH0_IEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CH0_IEN_A::EN)
    }
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
#[doc = "Reader of field `CH1_IEN`"]
pub type CH1_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_IEN`"]
pub struct CH1_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_IEN_W<'a> {
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
#[doc = "Reader of field `CH2_IEN`"]
pub type CH2_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_IEN`"]
pub struct CH2_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_IEN_W<'a> {
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
#[doc = "Reader of field `CH3_IEN`"]
pub type CH3_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3_IEN`"]
pub struct CH3_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_IEN_W<'a> {
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
#[doc = "Reader of field `CH4_IEN`"]
pub type CH4_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4_IEN`"]
pub struct CH4_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_IEN_W<'a> {
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
#[doc = "Reader of field `CH5_IEN`"]
pub type CH5_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5_IEN`"]
pub struct CH5_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_IEN_W<'a> {
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
#[doc = "Reader of field `CH6_IEN`"]
pub type CH6_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6_IEN`"]
pub struct CH6_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_IEN_W<'a> {
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
#[doc = "Reader of field `CH7_IEN`"]
pub type CH7_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7_IEN`"]
pub struct CH7_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_IEN_W<'a> {
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
#[doc = "Reader of field `CH8_IEN`"]
pub type CH8_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH8_IEN`"]
pub struct CH8_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8_IEN_W<'a> {
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
#[doc = "Reader of field `CH9_IEN`"]
pub type CH9_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH9_IEN`"]
pub struct CH9_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9_IEN_W<'a> {
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
#[doc = "Reader of field `CH10_IEN`"]
pub type CH10_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH10_IEN`"]
pub struct CH10_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10_IEN_W<'a> {
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
#[doc = "Reader of field `CH11_IEN`"]
pub type CH11_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH11_IEN`"]
pub struct CH11_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11_IEN_W<'a> {
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
#[doc = "Reader of field `CH12_IEN`"]
pub type CH12_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH12_IEN`"]
pub struct CH12_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12_IEN_W<'a> {
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
#[doc = "Reader of field `CH13_IEN`"]
pub type CH13_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH13_IEN`"]
pub struct CH13_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13_IEN_W<'a> {
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
#[doc = "Reader of field `CH14_IEN`"]
pub type CH14_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH14_IEN`"]
pub struct CH14_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14_IEN_W<'a> {
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
#[doc = "Reader of field `CH15_IEN`"]
pub type CH15_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH15_IEN`"]
pub struct CH15_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15_IEN_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    pub fn ch0_ien(&self) -> CH0_IEN_R {
        CH0_IEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    pub fn ch1_ien(&self) -> CH1_IEN_R {
        CH1_IEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    pub fn ch2_ien(&self) -> CH2_IEN_R {
        CH2_IEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    pub fn ch3_ien(&self) -> CH3_IEN_R {
        CH3_IEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Interrupt Enable."]
    #[inline(always)]
    pub fn ch4_ien(&self) -> CH4_IEN_R {
        CH4_IEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Interrupt Enable."]
    #[inline(always)]
    pub fn ch5_ien(&self) -> CH5_IEN_R {
        CH5_IEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Interrupt Enable."]
    #[inline(always)]
    pub fn ch6_ien(&self) -> CH6_IEN_R {
        CH6_IEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Interrupt Enable."]
    #[inline(always)]
    pub fn ch7_ien(&self) -> CH7_IEN_R {
        CH7_IEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Interrupt Enable."]
    #[inline(always)]
    pub fn ch8_ien(&self) -> CH8_IEN_R {
        CH8_IEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Interrupt Enable."]
    #[inline(always)]
    pub fn ch9_ien(&self) -> CH9_IEN_R {
        CH9_IEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Interrupt Enable."]
    #[inline(always)]
    pub fn ch10_ien(&self) -> CH10_IEN_R {
        CH10_IEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Interrupt Enable."]
    #[inline(always)]
    pub fn ch11_ien(&self) -> CH11_IEN_R {
        CH11_IEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Interrupt Enable."]
    #[inline(always)]
    pub fn ch12_ien(&self) -> CH12_IEN_R {
        CH12_IEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Interrupt Enable."]
    #[inline(always)]
    pub fn ch13_ien(&self) -> CH13_IEN_R {
        CH13_IEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Interrupt Enable."]
    #[inline(always)]
    pub fn ch14_ien(&self) -> CH14_IEN_R {
        CH14_IEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Interrupt Enable."]
    #[inline(always)]
    pub fn ch15_ien(&self) -> CH15_IEN_R {
        CH15_IEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    pub fn ch0_ien(&mut self) -> CH0_IEN_W {
        CH0_IEN_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    pub fn ch1_ien(&mut self) -> CH1_IEN_W {
        CH1_IEN_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    pub fn ch2_ien(&mut self) -> CH2_IEN_W {
        CH2_IEN_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    pub fn ch3_ien(&mut self) -> CH3_IEN_W {
        CH3_IEN_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Interrupt Enable."]
    #[inline(always)]
    pub fn ch4_ien(&mut self) -> CH4_IEN_W {
        CH4_IEN_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Interrupt Enable."]
    #[inline(always)]
    pub fn ch5_ien(&mut self) -> CH5_IEN_W {
        CH5_IEN_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Interrupt Enable."]
    #[inline(always)]
    pub fn ch6_ien(&mut self) -> CH6_IEN_W {
        CH6_IEN_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Interrupt Enable."]
    #[inline(always)]
    pub fn ch7_ien(&mut self) -> CH7_IEN_W {
        CH7_IEN_W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Interrupt Enable."]
    #[inline(always)]
    pub fn ch8_ien(&mut self) -> CH8_IEN_W {
        CH8_IEN_W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Interrupt Enable."]
    #[inline(always)]
    pub fn ch9_ien(&mut self) -> CH9_IEN_W {
        CH9_IEN_W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Interrupt Enable."]
    #[inline(always)]
    pub fn ch10_ien(&mut self) -> CH10_IEN_W {
        CH10_IEN_W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Interrupt Enable."]
    #[inline(always)]
    pub fn ch11_ien(&mut self) -> CH11_IEN_W {
        CH11_IEN_W { w: self }
    }
    #[doc = "Bit 12 - Channel 12 Interrupt Enable."]
    #[inline(always)]
    pub fn ch12_ien(&mut self) -> CH12_IEN_W {
        CH12_IEN_W { w: self }
    }
    #[doc = "Bit 13 - Channel 13 Interrupt Enable."]
    #[inline(always)]
    pub fn ch13_ien(&mut self) -> CH13_IEN_W {
        CH13_IEN_W { w: self }
    }
    #[doc = "Bit 14 - Channel 14 Interrupt Enable."]
    #[inline(always)]
    pub fn ch14_ien(&mut self) -> CH14_IEN_W {
        CH14_IEN_W { w: self }
    }
    #[doc = "Bit 15 - Channel 15 Interrupt Enable."]
    #[inline(always)]
    pub fn ch15_ien(&mut self) -> CH15_IEN_W {
        CH15_IEN_W { w: self }
    }
}
