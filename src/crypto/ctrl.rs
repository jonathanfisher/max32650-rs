#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reset. This bit is used to reset the crypto accelerator. All crypto internal states and related registers are reset to their default reset values. Control register such as CRYPTO_CTRL, CIPHER_CTRL, HASH_CTRL, CRC_CTRL, MAA_CTRL (with the exception of the STC bit), HASH_MSG_SZ_\\[3:0\\]
and MAA_MAWS will retain their values. This bit will automatically clear itself after one cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Reset in progress."]
    BUSY = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, RST_A>;
impl RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::RESET_DONE,
            true => RST_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RST_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RST_A::BUSY
    }
}
#[doc = "Reset. This bit is used to reset the crypto accelerator. All crypto internal states and related registers are reset to their default reset values. Control register such as CRYPTO_CTRL, CIPHER_CTRL, HASH_CTRL, CRC_CTRL, MAA_CTRL (with the exception of the STC bit), HASH_MSG_SZ_\\[3:0\\]
and MAA_MAWS will retain their values. This bit will automatically clear itself after one cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_AW {
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<RST_AW> for bool {
    #[inline(always)]
    fn from(variant: RST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RST_AW::RESET)
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
#[doc = "Interrupt Enable. Generates an interrupt when done or error set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTR_A {
    #[doc = "0: Disable"]
    DIS = 0,
    #[doc = "1: Enable"]
    EN = 1,
}
impl From<INTR_A> for bool {
    #[inline(always)]
    fn from(variant: INTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTR`"]
pub type INTR_R = crate::R<bool, INTR_A>;
impl INTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTR_A {
        match self.bits {
            false => INTR_A::DIS,
            true => INTR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == INTR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == INTR_A::EN
    }
}
#[doc = "Write proxy for field `INTR`"]
pub struct INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INTR_A::DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INTR_A::EN)
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
#[doc = "Source Select. This bit selects the hash function and CRC generator input source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: Input FIFO"]
    INPUTFIFO = 0,
    #[doc = "1: Output FIFO"]
    OUTPUTFIFO = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::INPUTFIFO,
            true => SRC_A::OUTPUTFIFO,
        }
    }
    #[doc = "Checks if the value of the field is `INPUTFIFO`"]
    #[inline(always)]
    pub fn is_input_fifo(&self) -> bool {
        *self == SRC_A::INPUTFIFO
    }
    #[doc = "Checks if the value of the field is `OUTPUTFIFO`"]
    #[inline(always)]
    pub fn is_output_fifo(&self) -> bool {
        *self == SRC_A::OUTPUTFIFO
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input FIFO"]
    #[inline(always)]
    pub fn input_fifo(self) -> &'a mut W {
        self.variant(SRC_A::INPUTFIFO)
    }
    #[doc = "Output FIFO"]
    #[inline(always)]
    pub fn output_fifo(self) -> &'a mut W {
        self.variant(SRC_A::OUTPUTFIFO)
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
#[doc = "Reader of field `BSO`"]
pub type BSO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSO`"]
pub struct BSO_W<'a> {
    w: &'a mut W,
}
impl<'a> BSO_W<'a> {
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
#[doc = "Reader of field `BSI`"]
pub type BSI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSI`"]
pub struct BSI_W<'a> {
    w: &'a mut W,
}
impl<'a> BSI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `WAIT_EN`"]
pub type WAIT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAIT_EN`"]
pub struct WAIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Wait Pin Polarity. When the wait pin is enabled, this bit selects its active state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_POL_A {
    #[doc = "0: Active Low."]
    ACTIVELO = 0,
    #[doc = "1: Active High."]
    ACTIVEHI = 1,
}
impl From<WAIT_POL_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAIT_POL`"]
pub type WAIT_POL_R = crate::R<bool, WAIT_POL_A>;
impl WAIT_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_POL_A {
        match self.bits {
            false => WAIT_POL_A::ACTIVELO,
            true => WAIT_POL_A::ACTIVEHI,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELO`"]
    #[inline(always)]
    pub fn is_active_lo(&self) -> bool {
        *self == WAIT_POL_A::ACTIVELO
    }
    #[doc = "Checks if the value of the field is `ACTIVEHI`"]
    #[inline(always)]
    pub fn is_active_hi(&self) -> bool {
        *self == WAIT_POL_A::ACTIVEHI
    }
}
#[doc = "Write proxy for field `WAIT_POL`"]
pub struct WAIT_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAIT_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active Low."]
    #[inline(always)]
    pub fn active_lo(self) -> &'a mut W {
        self.variant(WAIT_POL_A::ACTIVELO)
    }
    #[doc = "Active High."]
    #[inline(always)]
    pub fn active_hi(self) -> &'a mut W {
        self.variant(WAIT_POL_A::ACTIVEHI)
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
#[doc = "Write FIFO Source Select. This field determines where data written to the write FIFO comes from. When data is written to the write FIFO, it is always written out the DMA. To decrypt or encrypt data, the write FIFO source should be set to the cipher output. To implement memcpy() or memset() functions, or to fill memory with random data, the write FIFO source should be set to the read FIFO. When calculating a HASH or CMAC, the write FIFO should be disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WRSRC_A {
    #[doc = "0: None."]
    NONE = 0,
    #[doc = "1: Cipher Output."]
    CIPHEROUTPUT = 1,
    #[doc = "2: Read FIFO."]
    READFIFO = 2,
    #[doc = "3: Reserved. Do not use."]
    RFU = 3,
}
impl From<WRSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: WRSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WRSRC`"]
pub type WRSRC_R = crate::R<u8, WRSRC_A>;
impl WRSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRSRC_A {
        match self.bits {
            0 => WRSRC_A::NONE,
            1 => WRSRC_A::CIPHEROUTPUT,
            2 => WRSRC_A::READFIFO,
            3 => WRSRC_A::RFU,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WRSRC_A::NONE
    }
    #[doc = "Checks if the value of the field is `CIPHEROUTPUT`"]
    #[inline(always)]
    pub fn is_cipher_output(&self) -> bool {
        *self == WRSRC_A::CIPHEROUTPUT
    }
    #[doc = "Checks if the value of the field is `READFIFO`"]
    #[inline(always)]
    pub fn is_read_fifo(&self) -> bool {
        *self == WRSRC_A::READFIFO
    }
    #[doc = "Checks if the value of the field is `RFU`"]
    #[inline(always)]
    pub fn is_rfu(&self) -> bool {
        *self == WRSRC_A::RFU
    }
}
#[doc = "Write proxy for field `WRSRC`"]
pub struct WRSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WRSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "None."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WRSRC_A::NONE)
    }
    #[doc = "Cipher Output."]
    #[inline(always)]
    pub fn cipher_output(self) -> &'a mut W {
        self.variant(WRSRC_A::CIPHEROUTPUT)
    }
    #[doc = "Read FIFO."]
    #[inline(always)]
    pub fn read_fifo(self) -> &'a mut W {
        self.variant(WRSRC_A::READFIFO)
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(WRSRC_A::RFU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Read FIFO Source Select. This field selects the source of the read FIFO. Typically, it is set to use the DMA. To implement a memset() function, the read FIFO DMA should be disabled. To fill memory with random data or to hash random numbers, the read FIFO source should be set to the random number generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDSRC_A {
    #[doc = "0: DMA Disable."]
    DMADISABLED = 0,
    #[doc = "1: DMA Or APB."]
    DMAORAPB = 1,
    #[doc = "2: RNG."]
    RNG = 2,
}
impl From<RDSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RDSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RDSRC`"]
pub type RDSRC_R = crate::R<u8, RDSRC_A>;
impl RDSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RDSRC_A::DMADISABLED),
            1 => Val(RDSRC_A::DMAORAPB),
            2 => Val(RDSRC_A::RNG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMADISABLED`"]
    #[inline(always)]
    pub fn is_dma_disabled(&self) -> bool {
        *self == RDSRC_A::DMADISABLED
    }
    #[doc = "Checks if the value of the field is `DMAORAPB`"]
    #[inline(always)]
    pub fn is_dma_or_apb(&self) -> bool {
        *self == RDSRC_A::DMAORAPB
    }
    #[doc = "Checks if the value of the field is `RNG`"]
    #[inline(always)]
    pub fn is_rng(&self) -> bool {
        *self == RDSRC_A::RNG
    }
}
#[doc = "Write proxy for field `RDSRC`"]
pub struct RDSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DMA Disable."]
    #[inline(always)]
    pub fn dma_disabled(self) -> &'a mut W {
        self.variant(RDSRC_A::DMADISABLED)
    }
    #[doc = "DMA Or APB."]
    #[inline(always)]
    pub fn dma_or_apb(self) -> &'a mut W {
        self.variant(RDSRC_A::DMAORAPB)
    }
    #[doc = "RNG."]
    #[inline(always)]
    pub fn rng(self) -> &'a mut W {
        self.variant(RDSRC_A::RNG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Done Flag Mode. This bit configures the access behavior of the individual CRYPTO_CTRL Done flags (CRYPTO_CTRL\\[27:24\\]). This bit is cleared only on reset to limit upkeep, i.e. once set, it will remain set until a reset occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLAG_MODE_A {
    #[doc = "0: Unrestricted write (0 or 1) of CRYPTO_CTRL\\[27:24\\]
flags."]
    UNRES_WR = 0,
    #[doc = "1: Access to CRYPTO_CTRL\\[27:24\\]
are write 1 to clear/write 0 no effect."]
    RES_WR = 1,
}
impl From<FLAG_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLAG_MODE`"]
pub type FLAG_MODE_R = crate::R<bool, FLAG_MODE_A>;
impl FLAG_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG_MODE_A {
        match self.bits {
            false => FLAG_MODE_A::UNRES_WR,
            true => FLAG_MODE_A::RES_WR,
        }
    }
    #[doc = "Checks if the value of the field is `UNRES_WR`"]
    #[inline(always)]
    pub fn is_unres_wr(&self) -> bool {
        *self == FLAG_MODE_A::UNRES_WR
    }
    #[doc = "Checks if the value of the field is `RES_WR`"]
    #[inline(always)]
    pub fn is_res_wr(&self) -> bool {
        *self == FLAG_MODE_A::RES_WR
    }
}
#[doc = "Write proxy for field `FLAG_MODE`"]
pub struct FLAG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLAG_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLAG_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Unrestricted write (0 or 1) of CRYPTO_CTRL\\[27:24\\]
flags."]
    #[inline(always)]
    pub fn unres_wr(self) -> &'a mut W {
        self.variant(FLAG_MODE_A::UNRES_WR)
    }
    #[doc = "Access to CRYPTO_CTRL\\[27:24\\]
are write 1 to clear/write 0 no effect."]
    #[inline(always)]
    pub fn res_wr(self) -> &'a mut W {
        self.variant(FLAG_MODE_A::RES_WR)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "DMA Done Flag Mask. This bit masks the DMA_DONE flag from being used to generate the CRYPTO_CTRL.DONE flag, and this disables a DMA_DONE condition from generating and interrupt. The DMA_DONE flag itself is unaffected and still may be monitored. This allows more optimal interrupt-driven crypto operations using DMA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMADNEMSK_A {
    #[doc = "0: DMA_DONE not used in setting CRYPTO_CTRL.DONE bit."]
    NOT_USED = 0,
    #[doc = "1: DMA_DONE used in setting CRYPTO_CTRL.DONE bit."]
    USED = 1,
}
impl From<DMADNEMSK_A> for bool {
    #[inline(always)]
    fn from(variant: DMADNEMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMADNEMSK`"]
pub type DMADNEMSK_R = crate::R<bool, DMADNEMSK_A>;
impl DMADNEMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADNEMSK_A {
        match self.bits {
            false => DMADNEMSK_A::NOT_USED,
            true => DMADNEMSK_A::USED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == DMADNEMSK_A::NOT_USED
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == DMADNEMSK_A::USED
    }
}
#[doc = "Write proxy for field `DMADNEMSK`"]
pub struct DMADNEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADNEMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADNEMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA_DONE not used in setting CRYPTO_CTRL.DONE bit."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut W {
        self.variant(DMADNEMSK_A::NOT_USED)
    }
    #[doc = "DMA_DONE used in setting CRYPTO_CTRL.DONE bit."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(DMADNEMSK_A::USED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "DMA Done. DMA write/read operation is complete. This bit must be cleared before starting a DMA operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_DONE_A {
    #[doc = "0: Not Done."]
    NOTDONE = 0,
    #[doc = "1: Done."]
    DONE = 1,
}
impl From<DMA_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_DONE`"]
pub type DMA_DONE_R = crate::R<bool, DMA_DONE_A>;
impl DMA_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_DONE_A {
        match self.bits {
            false => DMA_DONE_A::NOTDONE,
            true => DMA_DONE_A::DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == DMA_DONE_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == DMA_DONE_A::DONE
    }
}
#[doc = "Write proxy for field `DMA_DONE`"]
pub struct DMA_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Done."]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(DMA_DONE_A::NOTDONE)
    }
    #[doc = "Done."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(DMA_DONE_A::DONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `GLS_DONE`"]
pub type GLS_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLS_DONE`"]
pub struct GLS_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> GLS_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `HSH_DONE`"]
pub type HSH_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSH_DONE`"]
pub struct HSH_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSH_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CPH_DONE`"]
pub type CPH_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPH_DONE`"]
pub struct CPH_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPH_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `MAA_DONE`"]
pub type MAA_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAA_DONE`"]
pub struct MAA_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAA_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "AHB Bus Error. This bit is set when the DMA encounters a bus error during a read or write operation. Once this bit is set, the DMA will stop. This bit can only be cleared by resetting the crypto block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_A {
    #[doc = "0: No Error."]
    NOERROR = 0,
    #[doc = "1: Error."]
    ERROR = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, ERR_A>;
impl ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::NOERROR,
            true => ERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERR_A::ERROR
    }
}
#[doc = "Ready. Crypto block ready for more data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY_A {
    #[doc = "0: Busy."]
    BUSY = 0,
    #[doc = "1: Ready."]
    READY = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, RDY_A>;
impl RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::BUSY,
            true => RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RDY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RDY_A::READY
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Reset. This bit is used to reset the crypto accelerator. All crypto internal states and related registers are reset to their default reset values. Control register such as CRYPTO_CTRL, CIPHER_CTRL, HASH_CTRL, CRC_CTRL, MAA_CTRL (with the exception of the STC bit), HASH_MSG_SZ_\\[3:0\\]
and MAA_MAWS will retain their values. This bit will automatically clear itself after one cycle."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable. Generates an interrupt when done or error set."]
    #[inline(always)]
    pub fn intr(&self) -> INTR_R {
        INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Source Select. This bit selects the hash function and CRC generator input source."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Byte Swap Output. Note. No byte swap will occur if there is not a full word."]
    #[inline(always)]
    pub fn bso(&self) -> BSO_R {
        BSO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Byte Swap Input. Note. No byte swap will occur if there is not a full word."]
    #[inline(always)]
    pub fn bsi(&self) -> BSI_R {
        BSI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wait Pin Enable. This can be used to hold off the crypto DMA until an external memory is ready. This is useful for transferring pages from NAND flash which may take several microseconds to become ready."]
    #[inline(always)]
    pub fn wait_en(&self) -> WAIT_EN_R {
        WAIT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wait Pin Polarity. When the wait pin is enabled, this bit selects its active state."]
    #[inline(always)]
    pub fn wait_pol(&self) -> WAIT_POL_R {
        WAIT_POL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Write FIFO Source Select. This field determines where data written to the write FIFO comes from. When data is written to the write FIFO, it is always written out the DMA. To decrypt or encrypt data, the write FIFO source should be set to the cipher output. To implement memcpy() or memset() functions, or to fill memory with random data, the write FIFO source should be set to the read FIFO. When calculating a HASH or CMAC, the write FIFO should be disabled."]
    #[inline(always)]
    pub fn wrsrc(&self) -> WRSRC_R {
        WRSRC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Read FIFO Source Select. This field selects the source of the read FIFO. Typically, it is set to use the DMA. To implement a memset() function, the read FIFO DMA should be disabled. To fill memory with random data or to hash random numbers, the read FIFO source should be set to the random number generator."]
    #[inline(always)]
    pub fn rdsrc(&self) -> RDSRC_R {
        RDSRC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Done Flag Mode. This bit configures the access behavior of the individual CRYPTO_CTRL Done flags (CRYPTO_CTRL\\[27:24\\]). This bit is cleared only on reset to limit upkeep, i.e. once set, it will remain set until a reset occurs."]
    #[inline(always)]
    pub fn flag_mode(&self) -> FLAG_MODE_R {
        FLAG_MODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DMA Done Flag Mask. This bit masks the DMA_DONE flag from being used to generate the CRYPTO_CTRL.DONE flag, and this disables a DMA_DONE condition from generating and interrupt. The DMA_DONE flag itself is unaffected and still may be monitored. This allows more optimal interrupt-driven crypto operations using DMA."]
    #[inline(always)]
    pub fn dmadnemsk(&self) -> DMADNEMSK_R {
        DMADNEMSK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA Done. DMA write/read operation is complete. This bit must be cleared before starting a DMA operation."]
    #[inline(always)]
    pub fn dma_done(&self) -> DMA_DONE_R {
        DMA_DONE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Galois Done. FIFO is full and CRC or Hamming Code Generator is enabled. This bit must be cleared before starting a CRC operation Note that DMA_DONE must be polled instead of this bit to determine the end of DMA operation during the utilization of Hamming Code Generator."]
    #[inline(always)]
    pub fn gls_done(&self) -> GLS_DONE_R {
        GLS_DONE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Hash Done. SHA operation is complete. This bit must be cleared before starting a HASH operation."]
    #[inline(always)]
    pub fn hsh_done(&self) -> HSH_DONE_R {
        HSH_DONE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Cipher Done. Either AES or DES encryption/decryption operation is complete. This bit must be cleared before starting a cipher operation."]
    #[inline(always)]
    pub fn cph_done(&self) -> CPH_DONE_R {
        CPH_DONE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - MAA Done. MAA operation is complete. This bit must be cleared before starting a new MAA operation. This bit is read only while the MAA is in progress. This bit is negate of MAA_CTRL.STC."]
    #[inline(always)]
    pub fn maa_done(&self) -> MAA_DONE_R {
        MAA_DONE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - AHB Bus Error. This bit is set when the DMA encounters a bus error during a read or write operation. Once this bit is set, the DMA will stop. This bit can only be cleared by resetting the crypto block."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ready. Crypto block ready for more data."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Done. One or more cryptographic calculations complete (logical OR of done flags)."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset. This bit is used to reset the crypto accelerator. All crypto internal states and related registers are reset to their default reset values. Control register such as CRYPTO_CTRL, CIPHER_CTRL, HASH_CTRL, CRC_CTRL, MAA_CTRL (with the exception of the STC bit), HASH_MSG_SZ_\\[3:0\\]
and MAA_MAWS will retain their values. This bit will automatically clear itself after one cycle."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Enable. Generates an interrupt when done or error set."]
    #[inline(always)]
    pub fn intr(&mut self) -> INTR_W {
        INTR_W { w: self }
    }
    #[doc = "Bit 2 - Source Select. This bit selects the hash function and CRC generator input source."]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bit 4 - Byte Swap Output. Note. No byte swap will occur if there is not a full word."]
    #[inline(always)]
    pub fn bso(&mut self) -> BSO_W {
        BSO_W { w: self }
    }
    #[doc = "Bit 5 - Byte Swap Input. Note. No byte swap will occur if there is not a full word."]
    #[inline(always)]
    pub fn bsi(&mut self) -> BSI_W {
        BSI_W { w: self }
    }
    #[doc = "Bit 6 - Wait Pin Enable. This can be used to hold off the crypto DMA until an external memory is ready. This is useful for transferring pages from NAND flash which may take several microseconds to become ready."]
    #[inline(always)]
    pub fn wait_en(&mut self) -> WAIT_EN_W {
        WAIT_EN_W { w: self }
    }
    #[doc = "Bit 7 - Wait Pin Polarity. When the wait pin is enabled, this bit selects its active state."]
    #[inline(always)]
    pub fn wait_pol(&mut self) -> WAIT_POL_W {
        WAIT_POL_W { w: self }
    }
    #[doc = "Bits 8:9 - Write FIFO Source Select. This field determines where data written to the write FIFO comes from. When data is written to the write FIFO, it is always written out the DMA. To decrypt or encrypt data, the write FIFO source should be set to the cipher output. To implement memcpy() or memset() functions, or to fill memory with random data, the write FIFO source should be set to the read FIFO. When calculating a HASH or CMAC, the write FIFO should be disabled."]
    #[inline(always)]
    pub fn wrsrc(&mut self) -> WRSRC_W {
        WRSRC_W { w: self }
    }
    #[doc = "Bits 10:11 - Read FIFO Source Select. This field selects the source of the read FIFO. Typically, it is set to use the DMA. To implement a memset() function, the read FIFO DMA should be disabled. To fill memory with random data or to hash random numbers, the read FIFO source should be set to the random number generator."]
    #[inline(always)]
    pub fn rdsrc(&mut self) -> RDSRC_W {
        RDSRC_W { w: self }
    }
    #[doc = "Bit 14 - Done Flag Mode. This bit configures the access behavior of the individual CRYPTO_CTRL Done flags (CRYPTO_CTRL\\[27:24\\]). This bit is cleared only on reset to limit upkeep, i.e. once set, it will remain set until a reset occurs."]
    #[inline(always)]
    pub fn flag_mode(&mut self) -> FLAG_MODE_W {
        FLAG_MODE_W { w: self }
    }
    #[doc = "Bit 15 - DMA Done Flag Mask. This bit masks the DMA_DONE flag from being used to generate the CRYPTO_CTRL.DONE flag, and this disables a DMA_DONE condition from generating and interrupt. The DMA_DONE flag itself is unaffected and still may be monitored. This allows more optimal interrupt-driven crypto operations using DMA."]
    #[inline(always)]
    pub fn dmadnemsk(&mut self) -> DMADNEMSK_W {
        DMADNEMSK_W { w: self }
    }
    #[doc = "Bit 24 - DMA Done. DMA write/read operation is complete. This bit must be cleared before starting a DMA operation."]
    #[inline(always)]
    pub fn dma_done(&mut self) -> DMA_DONE_W {
        DMA_DONE_W { w: self }
    }
    #[doc = "Bit 25 - Galois Done. FIFO is full and CRC or Hamming Code Generator is enabled. This bit must be cleared before starting a CRC operation Note that DMA_DONE must be polled instead of this bit to determine the end of DMA operation during the utilization of Hamming Code Generator."]
    #[inline(always)]
    pub fn gls_done(&mut self) -> GLS_DONE_W {
        GLS_DONE_W { w: self }
    }
    #[doc = "Bit 26 - Hash Done. SHA operation is complete. This bit must be cleared before starting a HASH operation."]
    #[inline(always)]
    pub fn hsh_done(&mut self) -> HSH_DONE_W {
        HSH_DONE_W { w: self }
    }
    #[doc = "Bit 27 - Cipher Done. Either AES or DES encryption/decryption operation is complete. This bit must be cleared before starting a cipher operation."]
    #[inline(always)]
    pub fn cph_done(&mut self) -> CPH_DONE_W {
        CPH_DONE_W { w: self }
    }
    #[doc = "Bit 28 - MAA Done. MAA operation is complete. This bit must be cleared before starting a new MAA operation. This bit is read only while the MAA is in progress. This bit is negate of MAA_CTRL.STC."]
    #[inline(always)]
    pub fn maa_done(&mut self) -> MAA_DONE_W {
        MAA_DONE_W { w: self }
    }
}
