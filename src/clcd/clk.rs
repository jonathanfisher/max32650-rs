#[doc = "Reader of register CLK"]
pub type R = crate::R<u32, super::CLK>;
#[doc = "Writer for register CLK"]
pub type W = crate::W<u32, super::CLK>;
#[doc = "Register CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ACB`"]
pub type ACB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACB`"]
pub struct ACB_W<'a> {
    w: &'a mut W,
}
impl<'a> ACB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "D Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPOL_A {
    #[doc = "0: Active Hi"]
    ACTIVEHI = 0,
    #[doc = "1: Active Low"]
    ACTIVELO = 1,
}
impl From<DPOL_A> for bool {
    #[inline(always)]
    fn from(variant: DPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPOL`"]
pub type DPOL_R = crate::R<bool, DPOL_A>;
impl DPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPOL_A {
        match self.bits {
            false => DPOL_A::ACTIVEHI,
            true => DPOL_A::ACTIVELO,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHI`"]
    #[inline(always)]
    pub fn is_activehi(&self) -> bool {
        *self == DPOL_A::ACTIVEHI
    }
    #[doc = "Checks if the value of the field is `ACTIVELO`"]
    #[inline(always)]
    pub fn is_activelo(&self) -> bool {
        *self == DPOL_A::ACTIVELO
    }
}
#[doc = "Write proxy for field `DPOL`"]
pub struct DPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active Hi"]
    #[inline(always)]
    pub fn activehi(self) -> &'a mut W {
        self.variant(DPOL_A::ACTIVEHI)
    }
    #[doc = "Active Low"]
    #[inline(always)]
    pub fn activelo(self) -> &'a mut W {
        self.variant(DPOL_A::ACTIVELO)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `VPOL`"]
pub type VPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VPOL`"]
pub struct VPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "H Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPOL_A {
    #[doc = "1: Active Hi"]
    ACTIVEHI = 1,
    #[doc = "0: Active Low"]
    ACTIVELO = 0,
}
impl From<HPOL_A> for bool {
    #[inline(always)]
    fn from(variant: HPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HPOL`"]
pub type HPOL_R = crate::R<bool, HPOL_A>;
impl HPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPOL_A {
        match self.bits {
            true => HPOL_A::ACTIVEHI,
            false => HPOL_A::ACTIVELO,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHI`"]
    #[inline(always)]
    pub fn is_activehi(&self) -> bool {
        *self == HPOL_A::ACTIVEHI
    }
    #[doc = "Checks if the value of the field is `ACTIVELO`"]
    #[inline(always)]
    pub fn is_activelo(&self) -> bool {
        *self == HPOL_A::ACTIVELO
    }
}
#[doc = "Write proxy for field `HPOL`"]
pub struct HPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active Hi"]
    #[inline(always)]
    pub fn activehi(self) -> &'a mut W {
        self.variant(HPOL_A::ACTIVEHI)
    }
    #[doc = "Active Low"]
    #[inline(always)]
    pub fn activelo(self) -> &'a mut W {
        self.variant(HPOL_A::ACTIVELO)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE_A {
    #[doc = "0: Positve edge"]
    POSEDGE = 0,
    #[doc = "1: Negative Edge"]
    NEGEDGE = 1,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDGE`"]
pub type EDGE_R = crate::R<bool, EDGE_A>;
impl EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            false => EDGE_A::POSEDGE,
            true => EDGE_A::NEGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDGE_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDGE_A::NEGEDGE
    }
}
#[doc = "Write proxy for field `EDGE`"]
pub struct EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Positve edge"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EDGE_A::POSEDGE)
    }
    #[doc = "Negative Edge"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(EDGE_A::NEGEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Clock Active on Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PASCLK_A {
    #[doc = "0: Always Active"]
    ALWAYSACTIVE = 0,
    #[doc = "1: ACTIVE ON DATA"]
    ACTIVEONDATA = 1,
}
impl From<PASCLK_A> for bool {
    #[inline(always)]
    fn from(variant: PASCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PASCLK`"]
pub type PASCLK_R = crate::R<bool, PASCLK_A>;
impl PASCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PASCLK_A {
        match self.bits {
            false => PASCLK_A::ALWAYSACTIVE,
            true => PASCLK_A::ACTIVEONDATA,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYSACTIVE`"]
    #[inline(always)]
    pub fn is_alwaysactive(&self) -> bool {
        *self == PASCLK_A::ALWAYSACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVEONDATA`"]
    #[inline(always)]
    pub fn is_activeondata(&self) -> bool {
        *self == PASCLK_A::ACTIVEONDATA
    }
}
#[doc = "Write proxy for field `PASCLK`"]
pub struct PASCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PASCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PASCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Always Active"]
    #[inline(always)]
    pub fn alwaysactive(self) -> &'a mut W {
        self.variant(PASCLK_A::ALWAYSACTIVE)
    }
    #[doc = "ACTIVE ON DATA"]
    #[inline(always)]
    pub fn activeondata(self) -> &'a mut W {
        self.variant(PASCLK_A::ACTIVEONDATA)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock divsor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ACB"]
    #[inline(always)]
    pub fn acb(&self) -> ACB_R {
        ACB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - D Polarity"]
    #[inline(always)]
    pub fn dpol(&self) -> DPOL_R {
        DPOL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - V Polarity"]
    #[inline(always)]
    pub fn vpol(&self) -> VPOL_R {
        VPOL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - H Polarity"]
    #[inline(always)]
    pub fn hpol(&self) -> HPOL_R {
        HPOL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Edge Selection"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Clock Active on Data"]
    #[inline(always)]
    pub fn pasclk(&self) -> PASCLK_R {
        PASCLK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divsor"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bits 8:15 - ACB"]
    #[inline(always)]
    pub fn acb(&mut self) -> ACB_W {
        ACB_W { w: self }
    }
    #[doc = "Bit 16 - D Polarity"]
    #[inline(always)]
    pub fn dpol(&mut self) -> DPOL_W {
        DPOL_W { w: self }
    }
    #[doc = "Bit 17 - V Polarity"]
    #[inline(always)]
    pub fn vpol(&mut self) -> VPOL_W {
        VPOL_W { w: self }
    }
    #[doc = "Bit 18 - H Polarity"]
    #[inline(always)]
    pub fn hpol(&mut self) -> HPOL_W {
        HPOL_W { w: self }
    }
    #[doc = "Bit 19 - Edge Selection"]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W {
        EDGE_W { w: self }
    }
    #[doc = "Bit 20 - Clock Active on Data"]
    #[inline(always)]
    pub fn pasclk(&mut self) -> PASCLK_W {
        PASCLK_W { w: self }
    }
}
