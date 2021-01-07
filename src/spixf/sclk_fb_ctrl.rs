#[doc = "Reader of register SCLK_FB_CTRL"]
pub type R = crate::R<u32, super::SCLK_FB_CTRL>;
#[doc = "Writer for register SCLK_FB_CTRL"]
pub type W = crate::W<u32, super::SCLK_FB_CTRL>;
#[doc = "Register SCLK_FB_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SCLK_FB_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable SCLK feedback mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FB_EN_A {
    #[doc = "0: Disable SCLK feedback mode."]
    DIS = 0,
    #[doc = "1: Enable SCLK feedback mode."]
    EN = 1,
}
impl From<FB_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FB_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FB_EN`"]
pub type FB_EN_R = crate::R<bool, FB_EN_A>;
impl FB_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FB_EN_A {
        match self.bits {
            false => FB_EN_A::DIS,
            true => FB_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FB_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FB_EN_A::EN
    }
}
#[doc = "Write proxy for field `FB_EN`"]
pub struct FB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable SCLK feedback mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FB_EN_A::DIS)
    }
    #[doc = "Enable SCLK feedback mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FB_EN_A::EN)
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
#[doc = "Invert SCLK in feedback mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVERT_EN_A {
    #[doc = "0: Disable Invert SCLK feedback mode."]
    DIS = 0,
    #[doc = "1: Enable Invert SCLK feedback mode."]
    EN = 1,
}
impl From<INVERT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INVERT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVERT_EN`"]
pub type INVERT_EN_R = crate::R<bool, INVERT_EN_A>;
impl INVERT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVERT_EN_A {
        match self.bits {
            false => INVERT_EN_A::DIS,
            true => INVERT_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == INVERT_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == INVERT_EN_A::EN
    }
}
#[doc = "Write proxy for field `INVERT_EN`"]
pub struct INVERT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INVERT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVERT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Invert SCLK feedback mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INVERT_EN_A::DIS)
    }
    #[doc = "Enable Invert SCLK feedback mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INVERT_EN_A::EN)
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
#[doc = "Reader of field `IGNORE_CLKS`"]
pub type IGNORE_CLKS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IGNORE_CLKS`"]
pub struct IGNORE_CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNORE_CLKS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Reader of field `IGNORE_CLKS_NO_CMD`"]
pub type IGNORE_CLKS_NO_CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IGNORE_CLKS_NO_CMD`"]
pub struct IGNORE_CLKS_NO_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNORE_CLKS_NO_CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable SCLK feedback mode."]
    #[inline(always)]
    pub fn fb_en(&self) -> FB_EN_R {
        FB_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Invert SCLK in feedback mode."]
    #[inline(always)]
    pub fn invert_en(&self) -> INVERT_EN_R {
        INVERT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:9 - Number of clocks to ignore after SS asertion prior to reading data."]
    #[inline(always)]
    pub fn ignore_clks(&self) -> IGNORE_CLKS_R {
        IGNORE_CLKS_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Number of clocks to ignore after SS asertion prior to reading data when a read command is not explicitly sent."]
    #[inline(always)]
    pub fn ignore_clks_no_cmd(&self) -> IGNORE_CLKS_NO_CMD_R {
        IGNORE_CLKS_NO_CMD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SCLK feedback mode."]
    #[inline(always)]
    pub fn fb_en(&mut self) -> FB_EN_W {
        FB_EN_W { w: self }
    }
    #[doc = "Bit 1 - Invert SCLK in feedback mode."]
    #[inline(always)]
    pub fn invert_en(&mut self) -> INVERT_EN_W {
        INVERT_EN_W { w: self }
    }
    #[doc = "Bits 4:9 - Number of clocks to ignore after SS asertion prior to reading data."]
    #[inline(always)]
    pub fn ignore_clks(&mut self) -> IGNORE_CLKS_W {
        IGNORE_CLKS_W { w: self }
    }
    #[doc = "Bits 12:17 - Number of clocks to ignore after SS asertion prior to reading data when a read command is not explicitly sent."]
    #[inline(always)]
    pub fn ignore_clks_no_cmd(&mut self) -> IGNORE_CLKS_NO_CMD_W {
        IGNORE_CLKS_NO_CMD_W { w: self }
    }
}
