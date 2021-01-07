#[doc = "Reader of register SLAVE_ADDR"]
pub type R = crate::R<u32, super::SLAVE_ADDR>;
#[doc = "Writer for register SLAVE_ADDR"]
pub type W = crate::W<u32, super::SLAVE_ADDR>;
#[doc = "Register SLAVE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SLAVE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLAVE_ADDR`"]
pub type SLAVE_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLAVE_ADDR`"]
pub struct SLAVE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_ADDR_DIS`"]
pub type SLAVE_ADDR_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_ADDR_DIS`"]
pub struct SLAVE_ADDR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_DIS_W<'a> {
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
#[doc = "Reader of field `SLAVE_ADDR_IDX`"]
pub type SLAVE_ADDR_IDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLAVE_ADDR_IDX`"]
pub struct SLAVE_ADDR_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Extended Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EX_ADDR_A {
    #[doc = "0: 7-bit address."]
    _7_BITS_ADDRESS = 0,
    #[doc = "1: 10-bit address."]
    _10_BITS_ADDRESS = 1,
}
impl From<EX_ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: EX_ADDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EX_ADDR`"]
pub type EX_ADDR_R = crate::R<bool, EX_ADDR_A>;
impl EX_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EX_ADDR_A {
        match self.bits {
            false => EX_ADDR_A::_7_BITS_ADDRESS,
            true => EX_ADDR_A::_10_BITS_ADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `_7_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        *self == EX_ADDR_A::_7_BITS_ADDRESS
    }
    #[doc = "Checks if the value of the field is `_10_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        *self == EX_ADDR_A::_10_BITS_ADDRESS
    }
}
#[doc = "Write proxy for field `EX_ADDR`"]
pub struct EX_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EX_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EX_ADDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut W {
        self.variant(EX_ADDR_A::_7_BITS_ADDRESS)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut W {
        self.variant(EX_ADDR_A::_10_BITS_ADDRESS)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    pub fn slave_addr(&self) -> SLAVE_ADDR_R {
        SLAVE_ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Slave Address DIS."]
    #[inline(always)]
    pub fn slave_addr_dis(&self) -> SLAVE_ADDR_DIS_R {
        SLAVE_ADDR_DIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - Slave Address Index."]
    #[inline(always)]
    pub fn slave_addr_idx(&self) -> SLAVE_ADDR_IDX_R {
        SLAVE_ADDR_IDX_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    pub fn ex_addr(&self) -> EX_ADDR_R {
        EX_ADDR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    pub fn slave_addr(&mut self) -> SLAVE_ADDR_W {
        SLAVE_ADDR_W { w: self }
    }
    #[doc = "Bit 10 - Slave Address DIS."]
    #[inline(always)]
    pub fn slave_addr_dis(&mut self) -> SLAVE_ADDR_DIS_W {
        SLAVE_ADDR_DIS_W { w: self }
    }
    #[doc = "Bits 11:14 - Slave Address Index."]
    #[inline(always)]
    pub fn slave_addr_idx(&mut self) -> SLAVE_ADDR_IDX_W {
        SLAVE_ADDR_IDX_W { w: self }
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    pub fn ex_addr(&mut self) -> EX_ADDR_W {
        EX_ADDR_W { w: self }
    }
}
