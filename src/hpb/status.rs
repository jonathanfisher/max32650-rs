#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `WRSTOERR`"]
pub type WRSTOERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDECERR`"]
pub type WDECERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WACT`"]
pub type WACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDSSTALL`"]
pub type RDSSTALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RRSTOERR`"]
pub type RRSTOERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDECERR`"]
pub type RDECERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RACT`"]
pub type RACT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 26 - RSTO Error in Write Transaction."]
    #[inline(always)]
    pub fn wrstoerr(&self) -> WRSTOERR_R {
        WRSTOERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Decode Error in Write Transaction."]
    #[inline(always)]
    pub fn wdecerr(&self) -> WDECERR_R {
        WDECERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Write Transaction Active."]
    #[inline(always)]
    pub fn wact(&self) -> WACT_R {
        WACT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RDS Stall Error in Read Transaction."]
    #[inline(always)]
    pub fn rdsstall(&self) -> RDSSTALL_R {
        RDSSTALL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RSTO Error in Read Transaction."]
    #[inline(always)]
    pub fn rrstoerr(&self) -> RRSTOERR_R {
        RRSTOERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Decode Error in Read Transaction."]
    #[inline(always)]
    pub fn rdecerr(&self) -> RDECERR_R {
        RDECERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Read Transaction Active."]
    #[inline(always)]
    pub fn ract(&self) -> RACT_R {
        RACT_R::new((self.bits & 0x01) != 0)
    }
}
