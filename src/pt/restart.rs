#[doc = "Reader of register RESTART"]
pub type R = crate::R<u32, super::RESTART>;
#[doc = "Writer for register RESTART"]
pub type W = crate::W<u32, super::RESTART>;
#[doc = "Register RESTART `reset()`'s with value 0"]
impl crate::ResetValue for super::RESTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pt_x_select`"]
pub type PT_X_SELECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pt_x_select`"]
pub struct PT_X_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_X_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `on_pt_x_loop_exit`"]
pub type ON_PT_X_LOOP_EXIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `on_pt_x_loop_exit`"]
pub struct ON_PT_X_LOOP_EXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_PT_X_LOOP_EXIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `pt_y_select`"]
pub type PT_Y_SELECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pt_y_select`"]
pub struct PT_Y_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_Y_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `on_pt_y_loop_exit`"]
pub type ON_PT_Y_LOOP_EXIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `on_pt_y_loop_exit`"]
pub struct ON_PT_Y_LOOP_EXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_PT_Y_LOOP_EXIT_W<'a> {
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
    #[doc = "Bits 0:4 - Auto-Restart PT X Select"]
    #[inline(always)]
    pub fn pt_x_select(&self) -> PT_X_SELECT_R {
        PT_X_SELECT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Enable Auto-Restart on PT X Loop Exit"]
    #[inline(always)]
    pub fn on_pt_x_loop_exit(&self) -> ON_PT_X_LOOP_EXIT_R {
        ON_PT_X_LOOP_EXIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Auto-Restart PT Y Select"]
    #[inline(always)]
    pub fn pt_y_select(&self) -> PT_Y_SELECT_R {
        PT_Y_SELECT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Enable Auto-Restart on PT Y Loop Exit"]
    #[inline(always)]
    pub fn on_pt_y_loop_exit(&self) -> ON_PT_Y_LOOP_EXIT_R {
        ON_PT_Y_LOOP_EXIT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Auto-Restart PT X Select"]
    #[inline(always)]
    pub fn pt_x_select(&mut self) -> PT_X_SELECT_W {
        PT_X_SELECT_W { w: self }
    }
    #[doc = "Bit 7 - Enable Auto-Restart on PT X Loop Exit"]
    #[inline(always)]
    pub fn on_pt_x_loop_exit(&mut self) -> ON_PT_X_LOOP_EXIT_W {
        ON_PT_X_LOOP_EXIT_W { w: self }
    }
    #[doc = "Bits 8:12 - Auto-Restart PT Y Select"]
    #[inline(always)]
    pub fn pt_y_select(&mut self) -> PT_Y_SELECT_W {
        PT_Y_SELECT_W { w: self }
    }
    #[doc = "Bit 15 - Enable Auto-Restart on PT Y Loop Exit"]
    #[inline(always)]
    pub fn on_pt_y_loop_exit(&mut self) -> ON_PT_Y_LOOP_EXIT_W {
        ON_PT_Y_LOOP_EXIT_W { w: self }
    }
}
