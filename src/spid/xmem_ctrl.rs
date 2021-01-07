#[doc = "Reader of register XMEM_CTRL"]
pub type R = crate::R<u32, super::XMEM_CTRL>;
#[doc = "Writer for register XMEM_CTRL"]
pub type W = crate::W<u32, super::XMEM_CTRL>;
#[doc = "Register XMEM_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::XMEM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RD_CMD`"]
pub type RD_CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_CMD`"]
pub struct RD_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WR_CMD`"]
pub type WR_CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WR_CMD`"]
pub struct WR_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DUMMY_CLK`"]
pub type DUMMY_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DUMMY_CLK`"]
pub struct DUMMY_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMMY_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `XMEM_EN`"]
pub type XMEM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XMEM_EN`"]
pub struct XMEM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XMEM_EN_W<'a> {
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
    #[doc = "Bits 0:7 - Read command."]
    #[inline(always)]
    pub fn rd_cmd(&self) -> RD_CMD_R {
        RD_CMD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Write command."]
    #[inline(always)]
    pub fn wr_cmd(&self) -> WR_CMD_R {
        WR_CMD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Dummy clocks."]
    #[inline(always)]
    pub fn dummy_clk(&self) -> DUMMY_CLK_R {
        DUMMY_CLK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - XMEM enable."]
    #[inline(always)]
    pub fn xmem_en(&self) -> XMEM_EN_R {
        XMEM_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read command."]
    #[inline(always)]
    pub fn rd_cmd(&mut self) -> RD_CMD_W {
        RD_CMD_W { w: self }
    }
    #[doc = "Bits 8:15 - Write command."]
    #[inline(always)]
    pub fn wr_cmd(&mut self) -> WR_CMD_W {
        WR_CMD_W { w: self }
    }
    #[doc = "Bits 16:23 - Dummy clocks."]
    #[inline(always)]
    pub fn dummy_clk(&mut self) -> DUMMY_CLK_W {
        DUMMY_CLK_W { w: self }
    }
    #[doc = "Bit 31 - XMEM enable."]
    #[inline(always)]
    pub fn xmem_en(&mut self) -> XMEM_EN_W {
        XMEM_EN_W { w: self }
    }
}
