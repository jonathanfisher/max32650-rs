#[doc = "Reader of register INCSRL"]
pub type R = crate::R<u8, super::INCSRL>;
#[doc = "Writer for register INCSRL"]
pub type W = crate::W<u8, super::INCSRL>;
#[doc = "Register INCSRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INCSRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INCOMPTX`"]
pub type INCOMPTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCOMPTX`"]
pub struct INCOMPTX_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CLRDATATOG`"]
pub type CLRDATATOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRDATATOG`"]
pub struct CLRDATATOG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRDATATOG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENTSTALL`"]
pub type SENTSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENTSTALL`"]
pub struct SENTSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENTSTALL_W<'a> {
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
#[doc = "Send STALL Handshake.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENDSTALL_A {
    #[doc = "0: Terminate STALL handhsake"]
    TERMINATE = 0,
    #[doc = "1: Respond to an IN token with a STALL handshake"]
    RESPOND = 1,
}
impl From<SENDSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: SENDSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SENDSTALL`"]
pub type SENDSTALL_R = crate::R<bool, SENDSTALL_A>;
impl SENDSTALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SENDSTALL_A {
        match self.bits {
            false => SENDSTALL_A::TERMINATE,
            true => SENDSTALL_A::RESPOND,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATE`"]
    #[inline(always)]
    pub fn is_terminate(&self) -> bool {
        *self == SENDSTALL_A::TERMINATE
    }
    #[doc = "Checks if the value of the field is `RESPOND`"]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        *self == SENDSTALL_A::RESPOND
    }
}
#[doc = "Reader of field `FLUSHFIFO`"]
pub type FLUSHFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLUSHFIFO`"]
pub struct FLUSHFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHFIFO_W<'a> {
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
#[doc = "Reader of field `UNDERRUN`"]
pub type UNDERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDERRUN`"]
pub struct UNDERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERRUN_W<'a> {
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
#[doc = "Reader of field `FIFONOTEMPTY`"]
pub type FIFONOTEMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFONOTEMPTY`"]
pub struct FIFONOTEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFONOTEMPTY_W<'a> {
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
#[doc = "Reader of field `INPKTRDY`"]
pub type INPKTRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - Read Incomplete Split Transfer Error Status High-bandwidth isochronous transfers: Automatically set when a payload is split into multiple packets but insufficient IN tokens were received to send all packets. The current packets is flushed from the IN FIFO. Write 0 to clear."]
    #[inline(always)]
    pub fn incomptx(&self) -> INCOMPTX_R {
        INCOMPTX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write 1 to clear IN endpoint data-toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&self) -> CLRDATATOG_R {
        CLRDATATOG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted, at which time the IN FIFO is flushed, and inpktrdy is cleared. Write 0 to clear."]
    #[inline(always)]
    pub fn sentstall(&self) -> SENTSTALL_R {
        SENTSTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Send STALL Handshake."]
    #[inline(always)]
    pub fn sendstall(&self) -> SENDSTALL_R {
        SENDSTALL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flush Next Packet from IN FIFO. Write 1 to clear"]
    #[inline(always)]
    pub fn flushfifo(&self) -> FLUSHFIFO_R {
        FLUSHFIFO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read IN FIFO Underrun Error Status Isochronous Mode: Automatically set if the IN FIFO is empty. Write 0 to clear"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read FIFO Not Empty Status. Automatically set when there is at least one packet in the IN FIFO. Write a 0 to clear."]
    #[inline(always)]
    pub fn fifonotempty(&self) -> FIFONOTEMPTY_R {
        FIFONOTEMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IN Packet Ready. Write a 1 to clear"]
    #[inline(always)]
    pub fn inpktrdy(&self) -> INPKTRDY_R {
        INPKTRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Read Incomplete Split Transfer Error Status High-bandwidth isochronous transfers: Automatically set when a payload is split into multiple packets but insufficient IN tokens were received to send all packets. The current packets is flushed from the IN FIFO. Write 0 to clear."]
    #[inline(always)]
    pub fn incomptx(&mut self) -> INCOMPTX_W {
        INCOMPTX_W { w: self }
    }
    #[doc = "Bit 6 - Write 1 to clear IN endpoint data-toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> CLRDATATOG_W {
        CLRDATATOG_W { w: self }
    }
    #[doc = "Bit 5 - Read STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted, at which time the IN FIFO is flushed, and inpktrdy is cleared. Write 0 to clear."]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SENTSTALL_W {
        SENTSTALL_W { w: self }
    }
    #[doc = "Bit 3 - Flush Next Packet from IN FIFO. Write 1 to clear"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FLUSHFIFO_W {
        FLUSHFIFO_W { w: self }
    }
    #[doc = "Bit 2 - Read IN FIFO Underrun Error Status Isochronous Mode: Automatically set if the IN FIFO is empty. Write 0 to clear"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UNDERRUN_W {
        UNDERRUN_W { w: self }
    }
    #[doc = "Bit 1 - Read FIFO Not Empty Status. Automatically set when there is at least one packet in the IN FIFO. Write a 0 to clear."]
    #[inline(always)]
    pub fn fifonotempty(&mut self) -> FIFONOTEMPTY_W {
        FIFONOTEMPTY_W { w: self }
    }
}
