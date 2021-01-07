#[doc = "Reader of register INTRUSB"]
pub type R = crate::R<u8, super::INTRUSB>;
#[doc = "Reader of field `SOF_INT`"]
pub type SOF_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESET_INT`"]
pub type RESET_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESUME_INT`"]
pub type RESUME_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSPEND_INT`"]
pub type SUSPEND_INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - Start of Frame."]
    #[inline(always)]
    pub fn sof_int(&self) -> SOF_INT_R {
        SOF_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus reset detected."]
    #[inline(always)]
    pub fn reset_int(&self) -> RESET_INT_R {
        RESET_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Resume detected."]
    #[inline(always)]
    pub fn resume_int(&self) -> RESUME_INT_R {
        RESUME_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Suspend detected."]
    #[inline(always)]
    pub fn suspend_int(&self) -> SUSPEND_INT_R {
        SUSPEND_INT_R::new((self.bits & 0x01) != 0)
    }
}
