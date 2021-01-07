#[doc = "Reader of register I2S_CTRL"]
pub type R = crate::R<u32, super::I2S_CTRL>;
#[doc = "Writer for register I2S_CTRL"]
pub type W = crate::W<u32, super::I2S_CTRL>;
#[doc = "Register I2S_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Low duty cycle control. In timer mode, reload\\[7:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_EN_A {
    #[doc = "0: I2C mode is disabled."]
    DIS = 0,
    #[doc = "1: I2C mode is enabled."]
    EN = 1,
}
impl From<I2S_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2S_EN`"]
pub type I2S_EN_R = crate::R<bool, I2S_EN_A>;
impl I2S_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_EN_A {
        match self.bits {
            false => I2S_EN_A::DIS,
            true => I2S_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2S_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2S_EN_A::EN
    }
}
#[doc = "Write proxy for field `I2S_EN`"]
pub struct I2S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2S_EN_A::DIS)
    }
    #[doc = "I2C mode is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2S_EN_A::EN)
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
#[doc = "I2S Mute.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_MUTE_A {
    #[doc = "0: Normal Transmit."]
    DIS = 0,
    #[doc = "1: Transmit data is replaced with 0."]
    EN = 1,
}
impl From<I2S_MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_MUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2S_MUTE`"]
pub type I2S_MUTE_R = crate::R<bool, I2S_MUTE_A>;
impl I2S_MUTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_MUTE_A {
        match self.bits {
            false => I2S_MUTE_A::DIS,
            true => I2S_MUTE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2S_MUTE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2S_MUTE_A::EN
    }
}
#[doc = "Write proxy for field `I2S_MUTE`"]
pub struct I2S_MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_MUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_MUTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Transmit."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2S_MUTE_A::DIS)
    }
    #[doc = "Transmit data is replaced with 0."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2S_MUTE_A::EN)
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
#[doc = "I2S Pause.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_PAUSE_A {
    #[doc = "0: Normal Transmit/Receive."]
    DIS = 0,
    #[doc = "1: Halt Transmit and Receive FIFO and DMA accesses, Transmit 0s."]
    EN = 1,
}
impl From<I2S_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2S_PAUSE`"]
pub type I2S_PAUSE_R = crate::R<bool, I2S_PAUSE_A>;
impl I2S_PAUSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_PAUSE_A {
        match self.bits {
            false => I2S_PAUSE_A::DIS,
            true => I2S_PAUSE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2S_PAUSE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2S_PAUSE_A::EN
    }
}
#[doc = "Write proxy for field `I2S_PAUSE`"]
pub struct I2S_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_PAUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_PAUSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Transmit/Receive."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2S_PAUSE_A::DIS)
    }
    #[doc = "Halt Transmit and Receive FIFO and DMA accesses, Transmit 0s."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2S_PAUSE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "I2S Monotone.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_MONO_A {
    #[doc = "0: Stereophonic audio format."]
    DIS = 0,
    #[doc = "1: Monophonic audio format. Each transmit data word is replicated on both left/right channels. Receive data is taken from left channel, right channel receive data is ignored."]
    EN = 1,
}
impl From<I2S_MONO_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_MONO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2S_MONO`"]
pub type I2S_MONO_R = crate::R<bool, I2S_MONO_A>;
impl I2S_MONO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_MONO_A {
        match self.bits {
            false => I2S_MONO_A::DIS,
            true => I2S_MONO_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2S_MONO_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2S_MONO_A::EN
    }
}
#[doc = "Write proxy for field `I2S_MONO`"]
pub struct I2S_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_MONO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_MONO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stereophonic audio format."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2S_MONO_A::DIS)
    }
    #[doc = "Monophonic audio format. Each transmit data word is replicated on both left/right channels. Receive data is taken from left channel, right channel receive data is ignored."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2S_MONO_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "I2S Left Justify.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_LJ_A {
    #[doc = "0: Normal I 2 S audio protocol, audio data lags left/right channel signal by one SCLK period."]
    DIS = 0,
    #[doc = "1: Audio data is synchronized with SSEL (left/right channel signal)."]
    EN = 1,
}
impl From<I2S_LJ_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_LJ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2S_LJ`"]
pub type I2S_LJ_R = crate::R<bool, I2S_LJ_A>;
impl I2S_LJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_LJ_A {
        match self.bits {
            false => I2S_LJ_A::DIS,
            true => I2S_LJ_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2S_LJ_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2S_LJ_A::EN
    }
}
#[doc = "Write proxy for field `I2S_LJ`"]
pub struct I2S_LJ_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_LJ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal I 2 S audio protocol, audio data lags left/right channel signal by one SCLK period."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2S_LJ_A::DIS)
    }
    #[doc = "Audio data is synchronized with SSEL (left/right channel signal)."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2S_LJ_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn i2s_en(&self) -> I2S_EN_R {
        I2S_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2S Mute."]
    #[inline(always)]
    pub fn i2s_mute(&self) -> I2S_MUTE_R {
        I2S_MUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2S Pause."]
    #[inline(always)]
    pub fn i2s_pause(&self) -> I2S_PAUSE_R {
        I2S_PAUSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2S Monotone."]
    #[inline(always)]
    pub fn i2s_mono(&self) -> I2S_MONO_R {
        I2S_MONO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2S Left Justify."]
    #[inline(always)]
    pub fn i2s_lj(&self) -> I2S_LJ_R {
        I2S_LJ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn i2s_en(&mut self) -> I2S_EN_W {
        I2S_EN_W { w: self }
    }
    #[doc = "Bit 1 - I2S Mute."]
    #[inline(always)]
    pub fn i2s_mute(&mut self) -> I2S_MUTE_W {
        I2S_MUTE_W { w: self }
    }
    #[doc = "Bit 2 - I2S Pause."]
    #[inline(always)]
    pub fn i2s_pause(&mut self) -> I2S_PAUSE_W {
        I2S_PAUSE_W { w: self }
    }
    #[doc = "Bit 3 - I2S Monotone."]
    #[inline(always)]
    pub fn i2s_mono(&mut self) -> I2S_MONO_W {
        I2S_MONO_W { w: self }
    }
    #[doc = "Bit 4 - I2S Left Justify."]
    #[inline(always)]
    pub fn i2s_lj(&mut self) -> I2S_LJ_W {
        I2S_LJ_W { w: self }
    }
}
