#[doc = "Reader of register CRC_CTRL"]
pub type R = crate::R<u32, super::CRC_CTRL>;
#[doc = "Writer for register CRC_CTRL"]
pub type W = crate::W<u32, super::CRC_CTRL>;
#[doc = "Register CRC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cyclic Redundancy Check Enable. The CRC cannot be enabled if the PRNG is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<bool, CRC_A>;
impl CRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::DIS,
            true => CRC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CRC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CRC_A::EN
    }
}
#[doc = "Write proxy for field `CRC`"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CRC_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CRC_A::EN)
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
#[doc = "MSB select. This bit selects the order of calculating CRC on data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSB_A {
    #[doc = "0: LSB First."]
    LSBFIRST = 0,
    #[doc = "1: MSB First."]
    MSBFIRST = 1,
}
impl From<MSB_A> for bool {
    #[inline(always)]
    fn from(variant: MSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSB`"]
pub type MSB_R = crate::R<bool, MSB_A>;
impl MSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSB_A {
        match self.bits {
            false => MSB_A::LSBFIRST,
            true => MSB_A::MSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == MSB_A::LSBFIRST
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == MSB_A::MSBFIRST
    }
}
#[doc = "Write proxy for field `MSB`"]
pub struct MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSB First."]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(MSB_A::LSBFIRST)
    }
    #[doc = "MSB First."]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(MSB_A::MSBFIRST)
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
#[doc = "Reader of field `PRNG`"]
pub type PRNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRNG`"]
pub struct PRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRNG_W<'a> {
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
#[doc = "Reader of field `ENT`"]
pub type ENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENT`"]
pub struct ENT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENT_W<'a> {
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
#[doc = "Reader of field `HAM`"]
pub type HAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HAM`"]
pub struct HAM_W<'a> {
    w: &'a mut W,
}
impl<'a> HAM_W<'a> {
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
#[doc = "Hamming Reset. Reset Hamming code ECC generator for next block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRST_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<HRST_AW> for bool {
    #[inline(always)]
    fn from(variant: HRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `HRST`"]
pub struct HRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(HRST_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HRST_AW::RESET)
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
    #[doc = "Bit 0 - Cyclic Redundancy Check Enable. The CRC cannot be enabled if the PRNG is enabled."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MSB select. This bit selects the order of calculating CRC on data."]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pseudo Random Number Generator Enable. If entropy is disabled, this outputs one byte of pseudo random data per clock cycle. If entropy is enabled, data is output at a rate of one bit per clock cycle."]
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Entropy Enable. If the PRNG is enabled, this mixes the high frequency ring oscillator with the LFSR. If the PRNG is disabled, the raw entropy data is output at a rate of 1 bit per clock. This makes it possible to characterize the quality of the entropy source."]
    #[inline(always)]
    pub fn ent(&self) -> ENT_R {
        ENT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hamming Code Enable. Enable hamming code calculation."]
    #[inline(always)]
    pub fn ham(&self) -> HAM_R {
        HAM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cyclic Redundancy Check Enable. The CRC cannot be enabled if the PRNG is enabled."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 1 - MSB select. This bit selects the order of calculating CRC on data."]
    #[inline(always)]
    pub fn msb(&mut self) -> MSB_W {
        MSB_W { w: self }
    }
    #[doc = "Bit 2 - Pseudo Random Number Generator Enable. If entropy is disabled, this outputs one byte of pseudo random data per clock cycle. If entropy is enabled, data is output at a rate of one bit per clock cycle."]
    #[inline(always)]
    pub fn prng(&mut self) -> PRNG_W {
        PRNG_W { w: self }
    }
    #[doc = "Bit 3 - Entropy Enable. If the PRNG is enabled, this mixes the high frequency ring oscillator with the LFSR. If the PRNG is disabled, the raw entropy data is output at a rate of 1 bit per clock. This makes it possible to characterize the quality of the entropy source."]
    #[inline(always)]
    pub fn ent(&mut self) -> ENT_W {
        ENT_W { w: self }
    }
    #[doc = "Bit 4 - Hamming Code Enable. Enable hamming code calculation."]
    #[inline(always)]
    pub fn ham(&mut self) -> HAM_W {
        HAM_W { w: self }
    }
    #[doc = "Bit 5 - Hamming Reset. Reset Hamming code ECC generator for next block."]
    #[inline(always)]
    pub fn hrst(&mut self) -> HRST_W {
        HRST_W { w: self }
    }
}
