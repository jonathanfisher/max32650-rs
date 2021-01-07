#[doc = "Reader of register SYSSIE"]
pub type R = crate::R<u32, super::SYSSIE>;
#[doc = "Writer for register SYSSIE"]
pub type W = crate::W<u32, super::SYSSIE>;
#[doc = "Register SYSSIE `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSSIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ARM ICE Unlock Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEULIE_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<ICEULIE_A> for bool {
    #[inline(always)]
    fn from(variant: ICEULIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICEULIE`"]
pub type ICEULIE_R = crate::R<bool, ICEULIE_A>;
impl ICEULIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEULIE_A {
        match self.bits {
            false => ICEULIE_A::DIS,
            true => ICEULIE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ICEULIE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ICEULIE_A::EN
    }
}
#[doc = "Write proxy for field `ICEULIE`"]
pub struct ICEULIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEULIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICEULIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ICEULIE_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ICEULIE_A::EN)
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
#[doc = "Code Integrity Error Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIEIE_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<CIEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CIEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CIEIE`"]
pub type CIEIE_R = crate::R<bool, CIEIE_A>;
impl CIEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIEIE_A {
        match self.bits {
            false => CIEIE_A::DIS,
            true => CIEIE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CIEIE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CIEIE_A::EN
    }
}
#[doc = "Write proxy for field `CIEIE`"]
pub struct CIEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CIEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CIEIE_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CIEIE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "System Cache Memory Fault Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCMFIE_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<SCMFIE_A> for bool {
    #[inline(always)]
    fn from(variant: SCMFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCMFIE`"]
pub type SCMFIE_R = crate::R<bool, SCMFIE_A>;
impl SCMFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCMFIE_A {
        match self.bits {
            false => SCMFIE_A::DIS,
            true => SCMFIE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SCMFIE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SCMFIE_A::EN
    }
}
#[doc = "Write proxy for field `SCMFIE`"]
pub struct SCMFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCMFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SCMFIE_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SCMFIE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    pub fn iceulie(&self) -> ICEULIE_R {
        ICEULIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Code Integrity Error Interrupt Enable."]
    #[inline(always)]
    pub fn cieie(&self) -> CIEIE_R {
        CIEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System Cache Memory Fault Interrupt Enable."]
    #[inline(always)]
    pub fn scmfie(&self) -> SCMFIE_R {
        SCMFIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    pub fn iceulie(&mut self) -> ICEULIE_W {
        ICEULIE_W { w: self }
    }
    #[doc = "Bit 1 - Code Integrity Error Interrupt Enable."]
    #[inline(always)]
    pub fn cieie(&mut self) -> CIEIE_W {
        CIEIE_W { w: self }
    }
    #[doc = "Bit 5 - System Cache Memory Fault Interrupt Enable."]
    #[inline(always)]
    pub fn scmfie(&mut self) -> SCMFIE_W {
        SCMFIE_W { w: self }
    }
}
