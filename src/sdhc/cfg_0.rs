#[doc = "Reader of register CFG_0"]
pub type R = crate::R<u32, super::CFG_0>;
#[doc = "Reader of field `TO_CLK_FREQ`"]
pub type TO_CLK_FREQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `TO_CLK_UNIT`"]
pub type TO_CLK_UNIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLK_FREQ`"]
pub type CLK_FREQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAX_BLK_LEN`"]
pub type MAX_BLK_LEN_R = crate::R<u8, u8>;
#[doc = "Reader of field `BIT_8`"]
pub type BIT_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADMA2`"]
pub type ADMA2_R = crate::R<bool, bool>;
#[doc = "Reader of field `HS`"]
pub type HS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDMA`"]
pub type SDMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `V3_3`"]
pub type V3_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `V3_0`"]
pub type V3_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `V1_8`"]
pub type V1_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `BIT_64_SYS_BUS`"]
pub type BIT_64_SYS_BUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASYNC_INT`"]
pub type ASYNC_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLOT_TYPE`"]
pub type SLOT_TYPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency."]
    #[inline(always)]
    pub fn to_clk_freq(&self) -> TO_CLK_FREQ_R {
        TO_CLK_FREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit."]
    #[inline(always)]
    pub fn to_clk_unit(&self) -> TO_CLK_UNIT_R {
        TO_CLK_UNIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency For SD Clock."]
    #[inline(always)]
    pub fn clk_freq(&self) -> CLK_FREQ_R {
        CLK_FREQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Max Block Length."]
    #[inline(always)]
    pub fn max_blk_len(&self) -> MAX_BLK_LEN_R {
        MAX_BLK_LEN_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - 8-bit Support for Embedded Device."]
    #[inline(always)]
    pub fn bit_8(&self) -> BIT_8_R {
        BIT_8_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support."]
    #[inline(always)]
    pub fn adma2(&self) -> ADMA2_R {
        ADMA2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - High Speed Support."]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDMA Support."]
    #[inline(always)]
    pub fn sdma(&self) -> SDMA_R {
        SDMA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Suspend/Resume Support."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V."]
    #[inline(always)]
    pub fn v3_3(&self) -> V3_3_R {
        V3_3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V."]
    #[inline(always)]
    pub fn v3_0(&self) -> V3_0_R {
        V3_0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V."]
    #[inline(always)]
    pub fn v1_8(&self) -> V1_8_R {
        V1_8_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 64-bit System Bus Support."]
    #[inline(always)]
    pub fn bit_64_sys_bus(&self) -> BIT_64_SYS_BUS_R {
        BIT_64_SYS_BUS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support."]
    #[inline(always)]
    pub fn async_int(&self) -> ASYNC_INT_R {
        ASYNC_INT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Slot Type."]
    #[inline(always)]
    pub fn slot_type(&self) -> SLOT_TYPE_R {
        SLOT_TYPE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
