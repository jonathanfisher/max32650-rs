#[doc = "Reader of register AUTO_CMD_ER"]
pub type R = crate::R<u16, super::AUTO_CMD_ER>;
#[doc = "Writer for register AUTO_CMD_ER"]
pub type W = crate::W<u16, super::AUTO_CMD_ER>;
#[doc = "Register AUTO_CMD_ER `reset()`'s with value 0"]
impl crate::ResetValue for super::AUTO_CMD_ER {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NOT_EXCUTED`"]
pub type NOT_EXCUTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOT_EXCUTED`"]
pub struct NOT_EXCUTED_W<'a> {
    w: &'a mut W,
}
impl<'a> NOT_EXCUTED_W<'a> {
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
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TO`"]
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
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
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC`"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
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
#[doc = "Reader of field `END_BIT`"]
pub type END_BIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_BIT`"]
pub struct END_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> END_BIT_W<'a> {
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
#[doc = "Reader of field `INDEX`"]
pub type INDEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDEX`"]
pub struct INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> INDEX_W<'a> {
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
#[doc = "Reader of field `NOT_ISSUED`"]
pub type NOT_ISSUED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOT_ISSUED`"]
pub struct NOT_ISSUED_W<'a> {
    w: &'a mut W,
}
impl<'a> NOT_ISSUED_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed."]
    #[inline(always)]
    pub fn not_excuted(&self) -> NOT_EXCUTED_R {
        NOT_EXCUTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error."]
    #[inline(always)]
    pub fn end_bit(&self) -> END_BIT_R {
        END_BIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error."]
    #[inline(always)]
    pub fn not_issued(&self) -> NOT_ISSUED_R {
        NOT_ISSUED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto CMD12 Not Executed."]
    #[inline(always)]
    pub fn not_excuted(&mut self) -> NOT_EXCUTED_W {
        NOT_EXCUTED_W { w: self }
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error."]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
    #[doc = "Bit 2 - Auto CMD CRC Error."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error."]
    #[inline(always)]
    pub fn end_bit(&mut self) -> END_BIT_W {
        END_BIT_W { w: self }
    }
    #[doc = "Bit 4 - Auto CMD Index Error."]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W {
        INDEX_W { w: self }
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error."]
    #[inline(always)]
    pub fn not_issued(&mut self) -> NOT_ISSUED_W {
        NOT_ISSUED_W { w: self }
    }
}
