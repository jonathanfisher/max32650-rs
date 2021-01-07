#[doc = "Reader of register PRESET_5"]
pub type R = crate::R<u16, super::PRESET_5>;
#[doc = "Reader of field `SDCLK_FREQ`"]
pub type SDCLK_FREQ_R = crate::R<u16, u16>;
#[doc = "Reader of field `CLK_GEN`"]
pub type CLK_GEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRIVER_STRENGTH`"]
pub type DRIVER_STRENGTH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value."]
    #[inline(always)]
    pub fn sdclk_freq(&self) -> SDCLK_FREQ_R {
        SDCLK_FREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value."]
    #[inline(always)]
    pub fn clk_gen(&self) -> CLK_GEN_R {
        CLK_GEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value."]
    #[inline(always)]
    pub fn driver_strength(&self) -> DRIVER_STRENGTH_R {
        DRIVER_STRENGTH_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
