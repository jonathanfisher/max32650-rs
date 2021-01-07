#[doc = "Reader of register CNT_RLD"]
pub type R = crate::R<u32, super::CNT_RLD>;
#[doc = "Writer for register CNT_RLD"]
pub type W = crate::W<u32, super::CNT_RLD>;
#[doc = "Register CNT_RLD `reset()`'s with value 0"]
impl crate::ResetValue for super::CNT_RLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT_RLD`"]
pub type CNT_RLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CNT_RLD`"]
pub struct CNT_RLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_RLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLDEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RLDEN`"]
pub type RLDEN_R = crate::R<bool, RLDEN_A>;
impl RLDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLDEN_A {
        match self.bits {
            false => RLDEN_A::DIS,
            true => RLDEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RLDEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RLDEN_A::EN
    }
}
#[doc = "Write proxy for field `RLDEN`"]
pub struct RLDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RLDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RLDEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RLDEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt_rld(&self) -> CNT_RLD_R {
        CNT_RLD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 31 - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt_rld(&mut self) -> CNT_RLD_W {
        CNT_RLD_W { w: self }
    }
    #[doc = "Bit 31 - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
    #[inline(always)]
    pub fn rlden(&mut self) -> RLDEN_W {
        RLDEN_W { w: self }
    }
}
