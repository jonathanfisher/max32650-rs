#[doc = "Reader of register MODE_CTRL"]
pub type R = crate::R<u32, super::MODE_CTRL>;
#[doc = "Writer for register MODE_CTRL"]
pub type W = crate::W<u32, super::MODE_CTRL>;
#[doc = "Register MODE_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MODE_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDCLK`"]
pub type MDCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MDCLK`"]
pub struct MDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MDCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "No Command Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NO_CMD_A {
    #[doc = "0: Send read command every time SPI transaction is initiated."]
    ALWAYS = 0,
    #[doc = "1: Send read command only once. NO read command in subsequent SPI transactions."]
    ONCE = 1,
}
impl From<NO_CMD_A> for bool {
    #[inline(always)]
    fn from(variant: NO_CMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NO_CMD`"]
pub type NO_CMD_R = crate::R<bool, NO_CMD_A>;
impl NO_CMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NO_CMD_A {
        match self.bits {
            false => NO_CMD_A::ALWAYS,
            true => NO_CMD_A::ONCE,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == NO_CMD_A::ALWAYS
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == NO_CMD_A::ONCE
    }
}
#[doc = "Write proxy for field `NO_CMD`"]
pub struct NO_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NO_CMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Send read command every time SPI transaction is initiated."]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(NO_CMD_A::ALWAYS)
    }
    #[doc = "Send read command only once. NO read command in subsequent SPI transactions."]
    #[inline(always)]
    pub fn once(self) -> &'a mut W {
        self.variant(NO_CMD_A::ONCE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Mode Clocks. Number of SPI clocks needed during mode/dummy phase of fetch."]
    #[inline(always)]
    pub fn mdclk(&self) -> MDCLK_R {
        MDCLK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - No Command Mode."]
    #[inline(always)]
    pub fn no_cmd(&self) -> NO_CMD_R {
        NO_CMD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode Clocks. Number of SPI clocks needed during mode/dummy phase of fetch."]
    #[inline(always)]
    pub fn mdclk(&mut self) -> MDCLK_W {
        MDCLK_W { w: self }
    }
    #[doc = "Bit 8 - No Command Mode."]
    #[inline(always)]
    pub fn no_cmd(&mut self) -> NO_CMD_W {
        NO_CMD_W { w: self }
    }
}
