#[doc = "Reader of register PRESENT"]
pub type R = crate::R<u32, super::PRESENT>;
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAT`"]
pub type DAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAT_LINE_ACTIVE`"]
pub type DAT_LINE_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RETUNING`"]
pub type RETUNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRITE_TRANSFER`"]
pub type WRITE_TRANSFER_R = crate::R<bool, bool>;
#[doc = "Reader of field `READ_TRANSFER`"]
pub type READ_TRANSFER_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFFER_WRITE`"]
pub type BUFFER_WRITE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFFER_READ`"]
pub type BUFFER_READ_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARD_INSERTED`"]
pub type CARD_INSERTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARD_STATE`"]
pub type CARD_STATE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARD_DETECT`"]
pub type CARD_DETECT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WP`"]
pub type WP_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAT_SIGNAL_LEVEL`"]
pub type DAT_SIGNAL_LEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `CMD_SIGNAL_LEVEL`"]
pub type CMD_SIGNAL_LEVEL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)."]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active."]
    #[inline(always)]
    pub fn dat_line_active(&self) -> DAT_LINE_ACTIVE_R {
        DAT_LINE_ACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request."]
    #[inline(always)]
    pub fn retuning(&self) -> RETUNING_R {
        RETUNING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active."]
    #[inline(always)]
    pub fn write_transfer(&self) -> WRITE_TRANSFER_R {
        WRITE_TRANSFER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active."]
    #[inline(always)]
    pub fn read_transfer(&self) -> READ_TRANSFER_R {
        READ_TRANSFER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable."]
    #[inline(always)]
    pub fn buffer_write(&self) -> BUFFER_WRITE_R {
        BUFFER_WRITE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable."]
    #[inline(always)]
    pub fn buffer_read(&self) -> BUFFER_READ_R {
        BUFFER_READ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Card Inserted."]
    #[inline(always)]
    pub fn card_inserted(&self) -> CARD_INSERTED_R {
        CARD_INSERTED_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Card State Stable."]
    #[inline(always)]
    pub fn card_state(&self) -> CARD_STATE_R {
        CARD_STATE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level."]
    #[inline(always)]
    pub fn card_detect(&self) -> CARD_DETECT_R {
        CARD_DETECT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level."]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - DAT\\[3:0\\]
Line Signal Level."]
    #[inline(always)]
    pub fn dat_signal_level(&self) -> DAT_SIGNAL_LEVEL_R {
        DAT_SIGNAL_LEVEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - CMD Line Signal Level."]
    #[inline(always)]
    pub fn cmd_signal_level(&self) -> CMD_SIGNAL_LEVEL_R {
        CMD_SIGNAL_LEVEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
