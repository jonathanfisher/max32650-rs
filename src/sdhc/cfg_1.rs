#[doc = "Reader of register CFG_1"]
pub type R = crate::R<u32, super::CFG_1>;
#[doc = "Reader of field `SDR50`"]
pub type SDR50_R = crate::R<bool, bool>;
#[doc = "Reader of field `DDR50`"]
pub type DDR50_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRIVER_A`"]
pub type DRIVER_A_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRIVER_C`"]
pub type DRIVER_C_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRIVER_D`"]
pub type DRIVER_D_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER_CNT_TUNING`"]
pub type TIMER_CNT_TUNING_R = crate::R<u8, u8>;
#[doc = "Reader of field `TUNING_SDR50`"]
pub type TUNING_SDR50_R = crate::R<bool, bool>;
#[doc = "Reader of field `RETUNING`"]
pub type RETUNING_R = crate::R<u8, u8>;
#[doc = "Reader of field `CLK_MULTI`"]
pub type CLK_MULTI_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - SDR50 Support."]
    #[inline(always)]
    pub fn sdr50(&self) -> SDR50_R {
        SDR50_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support."]
    #[inline(always)]
    pub fn ddr50(&self) -> DDR50_R {
        DDR50_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support."]
    #[inline(always)]
    pub fn driver_a(&self) -> DRIVER_A_R {
        DRIVER_A_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support."]
    #[inline(always)]
    pub fn driver_c(&self) -> DRIVER_C_R {
        DRIVER_C_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support."]
    #[inline(always)]
    pub fn driver_d(&self) -> DRIVER_D_R {
        DRIVER_D_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning."]
    #[inline(always)]
    pub fn timer_cnt_tuning(&self) -> TIMER_CNT_TUNING_R {
        TIMER_CNT_TUNING_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50."]
    #[inline(always)]
    pub fn tuning_sdr50(&self) -> TUNING_SDR50_R {
        TUNING_SDR50_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Re-Tuning Modes."]
    #[inline(always)]
    pub fn retuning(&self) -> RETUNING_R {
        RETUNING_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier."]
    #[inline(always)]
    pub fn clk_multi(&self) -> CLK_MULTI_R {
        CLK_MULTI_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
