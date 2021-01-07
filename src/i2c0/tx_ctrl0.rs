#[doc = "Reader of register TX_CTRL0"]
pub type R = crate::R<u32, super::TX_CTRL0>;
#[doc = "Writer for register TX_CTRL0"]
pub type W = crate::W<u32, super::TX_CTRL0>;
#[doc = "Register TX_CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_PRELOAD`"]
pub type TX_PRELOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PRELOAD`"]
pub struct TX_PRELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PRELOAD_W<'a> {
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
#[doc = "Transmit FIFO Ready Manual Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_READY_MODE_A {
    #[doc = "0: HW control of I2CTXRDY enabled."]
    EN = 0,
    #[doc = "1: HW control of I2CTXRDY disabled."]
    DIS = 1,
}
impl From<TX_READY_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_READY_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_READY_MODE`"]
pub type TX_READY_MODE_R = crate::R<bool, TX_READY_MODE_A>;
impl TX_READY_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_READY_MODE_A {
        match self.bits {
            false => TX_READY_MODE_A::EN,
            true => TX_READY_MODE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_READY_MODE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_READY_MODE_A::DIS
    }
}
#[doc = "Write proxy for field `TX_READY_MODE`"]
pub struct TX_READY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_READY_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_READY_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HW control of I2CTXRDY enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_READY_MODE_A::EN)
    }
    #[doc = "HW control of I2CTXRDY disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_READY_MODE_A::DIS)
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
#[doc = "Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FLUSH_A {
    #[doc = "0: FIFO not flushed."]
    NOT_FLUSHED = 0,
    #[doc = "1: Flush TX_FIFO."]
    FLUSH = 1,
}
impl From<TX_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_FLUSH`"]
pub type TX_FLUSH_R = crate::R<bool, TX_FLUSH_A>;
impl TX_FLUSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FLUSH_A {
        match self.bits {
            false => TX_FLUSH_A::NOT_FLUSHED,
            true => TX_FLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FLUSHED`"]
    #[inline(always)]
    pub fn is_not_flushed(&self) -> bool {
        *self == TX_FLUSH_A::NOT_FLUSHED
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == TX_FLUSH_A::FLUSH
    }
}
#[doc = "Write proxy for field `TX_FLUSH`"]
pub struct TX_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FLUSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn not_flushed(self) -> &'a mut W {
        self.variant(TX_FLUSH_A::NOT_FLUSHED)
    }
    #[doc = "Flush TX_FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(TX_FLUSH_A::FLUSH)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TX_THRESH`"]
pub type TX_THRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_THRESH`"]
pub struct TX_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
    #[inline(always)]
    pub fn tx_preload(&self) -> TX_PRELOAD_R {
        TX_PRELOAD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    pub fn tx_ready_mode(&self) -> TX_READY_MODE_R {
        TX_READY_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
    #[inline(always)]
    pub fn tx_flush(&self) -> TX_FLUSH_R {
        TX_FLUSH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn tx_thresh(&self) -> TX_THRESH_R {
        TX_THRESH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
    #[inline(always)]
    pub fn tx_preload(&mut self) -> TX_PRELOAD_W {
        TX_PRELOAD_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    pub fn tx_ready_mode(&mut self) -> TX_READY_MODE_W {
        TX_READY_MODE_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
    #[inline(always)]
    pub fn tx_flush(&mut self) -> TX_FLUSH_W {
        TX_FLUSH_W { w: self }
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn tx_thresh(&mut self) -> TX_THRESH_W {
        TX_THRESH_W { w: self }
    }
}
