#[doc = "Reader of register CMD"]
pub type R = crate::R<u16, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u16, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESP_TYPE`"]
pub type RESP_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESP_TYPE`"]
pub struct RESP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CRC_CHK_EN`"]
pub type CRC_CHK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_CHK_EN`"]
pub struct CRC_CHK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_CHK_EN_W<'a> {
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
#[doc = "Reader of field `IDX_CHK_EN`"]
pub type IDX_CHK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDX_CHK_EN`"]
pub struct IDX_CHK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX_CHK_EN_W<'a> {
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
#[doc = "Reader of field `DATA_PRES_SEL`"]
pub type DATA_PRES_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_PRES_SEL`"]
pub struct DATA_PRES_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_PRES_SEL_W<'a> {
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
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TYPE`"]
pub struct TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `IDX`"]
pub type IDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX`"]
pub struct IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Response Type Select."]
    #[inline(always)]
    pub fn resp_type(&self) -> RESP_TYPE_R {
        RESP_TYPE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Command CRC Check Enable."]
    #[inline(always)]
    pub fn crc_chk_en(&self) -> CRC_CHK_EN_R {
        CRC_CHK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable."]
    #[inline(always)]
    pub fn idx_chk_en(&self) -> IDX_CHK_EN_R {
        IDX_CHK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Present Select."]
    #[inline(always)]
    pub fn data_pres_sel(&self) -> DATA_PRES_SEL_R {
        DATA_PRES_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Command Type."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - Command Index."]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type Select."]
    #[inline(always)]
    pub fn resp_type(&mut self) -> RESP_TYPE_W {
        RESP_TYPE_W { w: self }
    }
    #[doc = "Bit 3 - Command CRC Check Enable."]
    #[inline(always)]
    pub fn crc_chk_en(&mut self) -> CRC_CHK_EN_W {
        CRC_CHK_EN_W { w: self }
    }
    #[doc = "Bit 4 - Command Index Check Enable."]
    #[inline(always)]
    pub fn idx_chk_en(&mut self) -> IDX_CHK_EN_W {
        IDX_CHK_EN_W { w: self }
    }
    #[doc = "Bit 5 - Data Present Select."]
    #[inline(always)]
    pub fn data_pres_sel(&mut self) -> DATA_PRES_SEL_W {
        DATA_PRES_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Command Type."]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W { w: self }
    }
    #[doc = "Bits 8:13 - Command Index."]
    #[inline(always)]
    pub fn idx(&mut self) -> IDX_W {
        IDX_W { w: self }
    }
}
