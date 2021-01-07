#[doc = "Reader of register TESTMODE"]
pub type R = crate::R<u8, super::TESTMODE>;
#[doc = "Writer for register TESTMODE"]
pub type W = crate::W<u8, super::TESTMODE>;
#[doc = "Register TESTMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::TESTMODE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FORCE_FS`"]
pub type FORCE_FS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_FS`"]
pub struct FORCE_FS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_FS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FORCE_HS`"]
pub type FORCE_HS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_HS`"]
pub struct FORCE_HS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_HS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TEST_PKT`"]
pub type TEST_PKT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST_PKT`"]
pub struct TEST_PKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_PKT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TEST_K`"]
pub type TEST_K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST_K`"]
pub struct TEST_K_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TEST_J`"]
pub type TEST_J_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST_J`"]
pub struct TEST_J_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_J_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TEST_SE0_NAK`"]
pub type TEST_SE0_NAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST_SE0_NAK`"]
pub struct TEST_SE0_NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_SE0_NAK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Force USB to Full-speed after reset."]
    #[inline(always)]
    pub fn force_fs(&self) -> FORCE_FS_R {
        FORCE_FS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force USB to High-speed after reset."]
    #[inline(always)]
    pub fn force_hs(&self) -> FORCE_HS_R {
        FORCE_HS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit fixed test packet."]
    #[inline(always)]
    pub fn test_pkt(&self) -> TEST_PKT_R {
        TEST_PKT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force USB to continuous K state."]
    #[inline(always)]
    pub fn test_k(&self) -> TEST_K_R {
        TEST_K_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force USB to continuous J state."]
    #[inline(always)]
    pub fn test_j(&self) -> TEST_J_R {
        TEST_J_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Respond to any valid IN token with NAK."]
    #[inline(always)]
    pub fn test_se0_nak(&self) -> TEST_SE0_NAK_R {
        TEST_SE0_NAK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Force USB to Full-speed after reset."]
    #[inline(always)]
    pub fn force_fs(&mut self) -> FORCE_FS_W {
        FORCE_FS_W { w: self }
    }
    #[doc = "Bit 4 - Force USB to High-speed after reset."]
    #[inline(always)]
    pub fn force_hs(&mut self) -> FORCE_HS_W {
        FORCE_HS_W { w: self }
    }
    #[doc = "Bit 3 - Transmit fixed test packet."]
    #[inline(always)]
    pub fn test_pkt(&mut self) -> TEST_PKT_W {
        TEST_PKT_W { w: self }
    }
    #[doc = "Bit 2 - Force USB to continuous K state."]
    #[inline(always)]
    pub fn test_k(&mut self) -> TEST_K_W {
        TEST_K_W { w: self }
    }
    #[doc = "Bit 1 - Force USB to continuous J state."]
    #[inline(always)]
    pub fn test_j(&mut self) -> TEST_J_W {
        TEST_J_W { w: self }
    }
    #[doc = "Bit 0 - Respond to any valid IN token with NAK."]
    #[inline(always)]
    pub fn test_se0_nak(&mut self) -> TEST_SE0_NAK_W {
        TEST_SE0_NAK_W { w: self }
    }
}
