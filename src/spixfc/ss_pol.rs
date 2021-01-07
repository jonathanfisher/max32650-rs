#[doc = "Reader of register SS_POL"]
pub type R = crate::R<u32, super::SS_POL>;
#[doc = "Writer for register SS_POL"]
pub type W = crate::W<u32, super::SS_POL>;
#[doc = "Register SS_POL `reset()`'s with value 0"]
impl crate::ResetValue for super::SS_POL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slave Select Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_POLARITY_A {
    #[doc = "0: Active Low."]
    LO = 0,
    #[doc = "1: Active High."]
    HI = 1,
}
impl From<SS_POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: SS_POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SS_POLARITY`"]
pub type SS_POLARITY_R = crate::R<bool, SS_POLARITY_A>;
impl SS_POLARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_POLARITY_A {
        match self.bits {
            false => SS_POLARITY_A::LO,
            true => SS_POLARITY_A::HI,
        }
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == SS_POLARITY_A::LO
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == SS_POLARITY_A::HI
    }
}
#[doc = "Write proxy for field `SS_POLARITY`"]
pub struct SS_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_POLARITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active Low."]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(SS_POLARITY_A::LO)
    }
    #[doc = "Active High."]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(SS_POLARITY_A::HI)
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
impl R {
    #[doc = "Bit 0 - Slave Select Polarity."]
    #[inline(always)]
    pub fn ss_polarity(&self) -> SS_POLARITY_R {
        SS_POLARITY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Select Polarity."]
    #[inline(always)]
    pub fn ss_polarity(&mut self) -> SS_POLARITY_W {
        SS_POLARITY_W { w: self }
    }
}
