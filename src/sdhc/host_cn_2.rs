#[doc = "Reader of register HOST_CN_2"]
pub type R = crate::R<u16, super::HOST_CN_2>;
#[doc = "Writer for register HOST_CN_2"]
pub type W = crate::W<u16, super::HOST_CN_2>;
#[doc = "Register HOST_CN_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_CN_2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHS`"]
pub type UHS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHS`"]
pub struct UHS_W<'a> {
    w: &'a mut W,
}
impl<'a> UHS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SIGNAL_V1_8`"]
pub type SIGNAL_V1_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIGNAL_V1_8`"]
pub struct SIGNAL_V1_8_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNAL_V1_8_W<'a> {
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
#[doc = "Reader of field `DRIVER_STRENGTH`"]
pub type DRIVER_STRENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVER_STRENGTH`"]
pub struct DRIVER_STRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVER_STRENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXCUTE`"]
pub type EXCUTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXCUTE`"]
pub struct EXCUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCUTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SAMPLING_CLK`"]
pub type SAMPLING_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAMPLING_CLK`"]
pub struct SAMPLING_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING_CLK_W<'a> {
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
#[doc = "Reader of field `ASYNCH_INT`"]
pub type ASYNCH_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNCH_INT`"]
pub struct ASYNCH_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCH_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PRESET_VAL_EN`"]
pub type PRESET_VAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRESET_VAL_EN`"]
pub struct PRESET_VAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESET_VAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - UHS Mode Select."]
    #[inline(always)]
    pub fn uhs(&self) -> UHS_R {
        UHS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable."]
    #[inline(always)]
    pub fn signal_v1_8(&self) -> SIGNAL_V1_8_R {
        SIGNAL_V1_8_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Driver Strength Select."]
    #[inline(always)]
    pub fn driver_strength(&self) -> DRIVER_STRENGTH_R {
        DRIVER_STRENGTH_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Execute Tuning."]
    #[inline(always)]
    pub fn excute(&self) -> EXCUTE_R {
        EXCUTE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sampling Clock Select."]
    #[inline(always)]
    pub fn sampling_clk(&self) -> SAMPLING_CLK_R {
        SAMPLING_CLK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable."]
    #[inline(always)]
    pub fn asynch_int(&self) -> ASYNCH_INT_R {
        ASYNCH_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Preset Value Enable."]
    #[inline(always)]
    pub fn preset_val_en(&self) -> PRESET_VAL_EN_R {
        PRESET_VAL_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - UHS Mode Select."]
    #[inline(always)]
    pub fn uhs(&mut self) -> UHS_W {
        UHS_W { w: self }
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable."]
    #[inline(always)]
    pub fn signal_v1_8(&mut self) -> SIGNAL_V1_8_W {
        SIGNAL_V1_8_W { w: self }
    }
    #[doc = "Bits 4:5 - Driver Strength Select."]
    #[inline(always)]
    pub fn driver_strength(&mut self) -> DRIVER_STRENGTH_W {
        DRIVER_STRENGTH_W { w: self }
    }
    #[doc = "Bit 6 - Execute Tuning."]
    #[inline(always)]
    pub fn excute(&mut self) -> EXCUTE_W {
        EXCUTE_W { w: self }
    }
    #[doc = "Bit 7 - Sampling Clock Select."]
    #[inline(always)]
    pub fn sampling_clk(&mut self) -> SAMPLING_CLK_W {
        SAMPLING_CLK_W { w: self }
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable."]
    #[inline(always)]
    pub fn asynch_int(&mut self) -> ASYNCH_INT_W {
        ASYNCH_INT_W { w: self }
    }
    #[doc = "Bit 15 - Preset Value Enable."]
    #[inline(always)]
    pub fn preset_val_en(&mut self) -> PRESET_VAL_EN_W {
        PRESET_VAL_EN_W { w: self }
    }
}
