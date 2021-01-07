#[doc = "Reader of register CSR0"]
pub type R = crate::R<u8, super::CSR0>;
#[doc = "Writer for register CSR0"]
pub type W = crate::W<u8, super::CSR0>;
#[doc = "Register CSR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SERV_SETUP_END`"]
pub type SERV_SETUP_END_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERV_SETUP_END`"]
pub struct SERV_SETUP_END_W<'a> {
    w: &'a mut W,
}
impl<'a> SERV_SETUP_END_W<'a> {
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
#[doc = "Reader of field `SERV_OUTPKTRDY`"]
pub type SERV_OUTPKTRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERV_OUTPKTRDY`"]
pub struct SERV_OUTPKTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SERV_OUTPKTRDY_W<'a> {
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
#[doc = "Reader of field `SEND_STALL`"]
pub type SEND_STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_STALL`"]
pub struct SEND_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_STALL_W<'a> {
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
#[doc = "Reader of field `SETUP_END`"]
pub type SETUP_END_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATA_END`"]
pub type DATA_END_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_END`"]
pub struct DATA_END_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_END_W<'a> {
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
#[doc = "Reader of field `SENT_STALL`"]
pub type SENT_STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENT_STALL`"]
pub struct SENT_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENT_STALL_W<'a> {
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
#[doc = "Reader of field `INPKTRDY`"]
pub type INPKTRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INPKTRDY`"]
pub struct INPKTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> INPKTRDY_W<'a> {
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
#[doc = "Reader of field `OUTPKTRDY`"]
pub type OUTPKTRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - Clear EP0 Setup End Bit. Write a 1 to clear the setupend bit. Automatically cleared after being set"]
    #[inline(always)]
    pub fn serv_setup_end(&self) -> SERV_SETUP_END_R {
        SERV_SETUP_END_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clear EP0 Out Packet Ready Bit. Write a 1 to clear the outpktrdy bit. Automatically cleared after being set."]
    #[inline(always)]
    pub fn serv_outpktrdy(&self) -> SERV_OUTPKTRDY_R {
        SERV_OUTPKTRDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Send EP0 STALL Handshake. Write a 1 to this bit to terminate the current control transaction by sneding a STALL handshake. Automatically cleared after being set."]
    #[inline(always)]
    pub fn send_stall(&self) -> SEND_STALL_R {
        SEND_STALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read Setup End Status. Automatically set when a contorl transaction ends before the dataend bit has been set by fimrware. An interrupt is generated when this bit is set. Write 1 to servicedsetupend to clear."]
    #[inline(always)]
    pub fn setup_end(&self) -> SETUP_END_R {
        SETUP_END_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control Transaction Data End. Write a 1 to this bit after firmware completes any of the following three transactions: 1. set inpktrdy = 1 for the last data packet. 2. Set inpktrdy =1 for a zero-length data packet. 3. Clear outpktrdy = 0 after unloading the last data packet."]
    #[inline(always)]
    pub fn data_end(&self) -> DATA_END_R {
        DATA_END_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read EP0 STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted. Write a 0 to clear."]
    #[inline(always)]
    pub fn sent_stall(&self) -> SENT_STALL_R {
        SENT_STALL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - EP0 IN Packet Ready 1: Write a 1 after writing a data packet to the IN FIFO. Automatically cleared when the data packet is transmitted. An interrupt is generated when this bit is cleared."]
    #[inline(always)]
    pub fn inpktrdy(&self) -> INPKTRDY_R {
        INPKTRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - EP0 OUT Packet Ready Status Automatically set when a data packet is received in the OUT FIFO. An interrupt is generated when this bit is set. Write a 1 to the servicedoutpktrdy bit (above) to clear after the packet is unloaded from the OUT FIFO."]
    #[inline(always)]
    pub fn outpktrdy(&self) -> OUTPKTRDY_R {
        OUTPKTRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Clear EP0 Setup End Bit. Write a 1 to clear the setupend bit. Automatically cleared after being set"]
    #[inline(always)]
    pub fn serv_setup_end(&mut self) -> SERV_SETUP_END_W {
        SERV_SETUP_END_W { w: self }
    }
    #[doc = "Bit 6 - Clear EP0 Out Packet Ready Bit. Write a 1 to clear the outpktrdy bit. Automatically cleared after being set."]
    #[inline(always)]
    pub fn serv_outpktrdy(&mut self) -> SERV_OUTPKTRDY_W {
        SERV_OUTPKTRDY_W { w: self }
    }
    #[doc = "Bit 5 - Send EP0 STALL Handshake. Write a 1 to this bit to terminate the current control transaction by sneding a STALL handshake. Automatically cleared after being set."]
    #[inline(always)]
    pub fn send_stall(&mut self) -> SEND_STALL_W {
        SEND_STALL_W { w: self }
    }
    #[doc = "Bit 3 - Control Transaction Data End. Write a 1 to this bit after firmware completes any of the following three transactions: 1. set inpktrdy = 1 for the last data packet. 2. Set inpktrdy =1 for a zero-length data packet. 3. Clear outpktrdy = 0 after unloading the last data packet."]
    #[inline(always)]
    pub fn data_end(&mut self) -> DATA_END_W {
        DATA_END_W { w: self }
    }
    #[doc = "Bit 2 - Read EP0 STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted. Write a 0 to clear."]
    #[inline(always)]
    pub fn sent_stall(&mut self) -> SENT_STALL_W {
        SENT_STALL_W { w: self }
    }
    #[doc = "Bit 1 - EP0 IN Packet Ready 1: Write a 1 after writing a data packet to the IN FIFO. Automatically cleared when the data packet is transmitted. An interrupt is generated when this bit is cleared."]
    #[inline(always)]
    pub fn inpktrdy(&mut self) -> INPKTRDY_W {
        INPKTRDY_W { w: self }
    }
}
