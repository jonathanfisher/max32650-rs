#[doc = "Reader of register SCON"]
pub type R = crate::R<u32, super::SCON>;
#[doc = "Writer for register SCON"]
pub type W = crate::W<u32, super::SCON>;
#[doc = "Register SCON `reset()`'s with value 0"]
impl crate::ResetValue for super::SCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Boundary Scan TAP enable. When enabled, the JTAG port is connected to the Boundary Scan TAP. Otherwise, the port is connected to the ARM ICE function. This bit is reset by the POR. Reset value and access depend on the part number.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSTAPEN_A {
    #[doc = "0: Boundary Scan TAP port disabled."]
    DIS = 0,
    #[doc = "1: Boundary Scan TAP port enabled."]
    EN = 1,
}
impl From<BSTAPEN_A> for bool {
    #[inline(always)]
    fn from(variant: BSTAPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BSTAPEN`"]
pub type BSTAPEN_R = crate::R<bool, BSTAPEN_A>;
impl BSTAPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSTAPEN_A {
        match self.bits {
            false => BSTAPEN_A::DIS,
            true => BSTAPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BSTAPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BSTAPEN_A::EN
    }
}
#[doc = "Write proxy for field `BSTAPEN`"]
pub struct BSTAPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BSTAPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSTAPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Boundary Scan TAP port disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BSTAPEN_A::DIS)
    }
    #[doc = "Boundary Scan TAP port enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BSTAPEN_A::EN)
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
#[doc = "System bus abritration scheme. These bits are used to select between Fixed-burst abritration and Round-Robin scheme. The Round-Robin scheme is selected by default. These bits are reset by the system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SBUSARB_A {
    #[doc = "0: Fixed Burst abritration."]
    FIX = 0,
    #[doc = "1: Round-robin scheme."]
    ROUND = 1,
}
impl From<SBUSARB_A> for u8 {
    #[inline(always)]
    fn from(variant: SBUSARB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SBUSARB`"]
pub type SBUSARB_R = crate::R<u8, SBUSARB_A>;
impl SBUSARB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SBUSARB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SBUSARB_A::FIX),
            1 => Val(SBUSARB_A::ROUND),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIX`"]
    #[inline(always)]
    pub fn is_fix(&self) -> bool {
        *self == SBUSARB_A::FIX
    }
    #[doc = "Checks if the value of the field is `ROUND`"]
    #[inline(always)]
    pub fn is_round(&self) -> bool {
        *self == SBUSARB_A::ROUND
    }
}
#[doc = "Write proxy for field `SBUSARB`"]
pub struct SBUSARB_W<'a> {
    w: &'a mut W,
}
impl<'a> SBUSARB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBUSARB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Fixed Burst abritration."]
    #[inline(always)]
    pub fn fix(self) -> &'a mut W {
        self.variant(SBUSARB_A::FIX)
    }
    #[doc = "Round-robin scheme."]
    #[inline(always)]
    pub fn round(self) -> &'a mut W {
        self.variant(SBUSARB_A::ROUND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PAGE_FLIP_A {
    #[doc = "0: Physical layout matches logical layout."]
    NORMAL = 0,
    #[doc = "1: Bottom half mapped to logical top half and vice versa."]
    SWAPPED = 1,
}
impl From<FLASH_PAGE_FLIP_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PAGE_FLIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH_PAGE_FLIP`"]
pub type FLASH_PAGE_FLIP_R = crate::R<bool, FLASH_PAGE_FLIP_A>;
impl FLASH_PAGE_FLIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_PAGE_FLIP_A {
        match self.bits {
            false => FLASH_PAGE_FLIP_A::NORMAL,
            true => FLASH_PAGE_FLIP_A::SWAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FLASH_PAGE_FLIP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SWAPPED`"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == FLASH_PAGE_FLIP_A::SWAPPED
    }
}
#[doc = "Write proxy for field `FLASH_PAGE_FLIP`"]
pub struct FLASH_PAGE_FLIP_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PAGE_FLIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_PAGE_FLIP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Physical layout matches logical layout."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FLASH_PAGE_FLIP_A::NORMAL)
    }
    #[doc = "Bottom half mapped to logical top half and vice versa."]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut W {
        self.variant(FLASH_PAGE_FLIP_A::SWAPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCACHE_FLUSH_A {
    #[doc = "0: Normal Code Cache Operation"]
    NORMAL = 0,
    #[doc = "1: Code Caches and CPU instruction buffer are flushed"]
    FLUSH = 1,
}
impl From<CCACHE_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: CCACHE_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCACHE_FLUSH`"]
pub type CCACHE_FLUSH_R = crate::R<bool, CCACHE_FLUSH_A>;
impl CCACHE_FLUSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCACHE_FLUSH_A {
        match self.bits {
            false => CCACHE_FLUSH_A::NORMAL,
            true => CCACHE_FLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CCACHE_FLUSH_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == CCACHE_FLUSH_A::FLUSH
    }
}
#[doc = "Write proxy for field `CCACHE_FLUSH`"]
pub struct CCACHE_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> CCACHE_FLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCACHE_FLUSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Code Cache Operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CCACHE_FLUSH_A::NORMAL)
    }
    #[doc = "Code Caches and CPU instruction buffer are flushed"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(CCACHE_FLUSH_A::FLUSH)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Data Cache Flush. The system cache(s) will be flushed when this bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCACHE_FLUSH_A {
    #[doc = "0: Normal System Cache Operation"]
    NORMAL = 0,
    #[doc = "1: System Cache is flushed"]
    FLUSH = 1,
}
impl From<DCACHE_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: DCACHE_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCACHE_FLUSH`"]
pub type DCACHE_FLUSH_R = crate::R<bool, DCACHE_FLUSH_A>;
impl DCACHE_FLUSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCACHE_FLUSH_A {
        match self.bits {
            false => DCACHE_FLUSH_A::NORMAL,
            true => DCACHE_FLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DCACHE_FLUSH_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == DCACHE_FLUSH_A::FLUSH
    }
}
#[doc = "Write proxy for field `DCACHE_FLUSH`"]
pub struct DCACHE_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_FLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCACHE_FLUSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal System Cache Operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DCACHE_FLUSH_A::NORMAL)
    }
    #[doc = "System Cache is flushed"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(DCACHE_FLUSH_A::FLUSH)
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
#[doc = "Data Cache Disable. The system cache(s) will be completely disabled when this bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCACHE_DIS_A {
    #[doc = "0: Is enabled."]
    EN = 0,
    #[doc = "1: Is Disabled."]
    DIS = 1,
}
impl From<DCACHE_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: DCACHE_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCACHE_DIS`"]
pub type DCACHE_DIS_R = crate::R<bool, DCACHE_DIS_A>;
impl DCACHE_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCACHE_DIS_A {
        match self.bits {
            false => DCACHE_DIS_A::EN,
            true => DCACHE_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DCACHE_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DCACHE_DIS_A::DIS
    }
}
#[doc = "Write proxy for field `DCACHE_DIS`"]
pub struct DCACHE_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCACHE_DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DCACHE_DIS_A::EN)
    }
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DCACHE_DIS_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCHK_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<CCHK_A> for bool {
    #[inline(always)]
    fn from(variant: CCHK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCHK`"]
pub type CCHK_R = crate::R<bool, CCHK_A>;
impl CCHK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCHK_A {
        match self.bits {
            false => CCHK_A::COMPLETE,
            true => CCHK_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCHK_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CCHK_A::START
    }
}
#[doc = "Write proxy for field `CCHK`"]
pub struct CCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> CCHK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCHK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(CCHK_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CCHK_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "ROM Checksum Result. This bit is only valid when CHKRD=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHKRES_A {
    #[doc = "0: ROM Checksum Correct."]
    PASS = 0,
    #[doc = "1: ROM Checksum Fail."]
    FAIL = 1,
}
impl From<CHKRES_A> for bool {
    #[inline(always)]
    fn from(variant: CHKRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHKRES`"]
pub type CHKRES_R = crate::R<bool, CHKRES_A>;
impl CHKRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHKRES_A {
        match self.bits {
            false => CHKRES_A::PASS,
            true => CHKRES_A::FAIL,
        }
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == CHKRES_A::PASS
    }
    #[doc = "Checks if the value of the field is `FAIL`"]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == CHKRES_A::FAIL
    }
}
#[doc = "Write proxy for field `CHKRES`"]
pub struct CHKRES_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHKRES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ROM Checksum Correct."]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(CHKRES_A::PASS)
    }
    #[doc = "ROM Checksum Fail."]
    #[inline(always)]
    pub fn fail(self) -> &'a mut W {
        self.variant(CHKRES_A::FAIL)
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
#[doc = "Operating Voltage Range. Setting these bits according to the VCore voltage allows the on-chip Random-Access memories to operate in their optimal timing range.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVR_A {
    #[doc = "0: 0.9V +/- 10 Percent"]
    _0_9V = 0,
    #[doc = "1: 1.0V +/- 10 Percent"]
    _1_0V = 1,
    #[doc = "2: 1.1V +/- 10 Percent"]
    _1_1V = 2,
}
impl From<OVR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<u8, OVR_A>;
impl OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OVR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OVR_A::_0_9V),
            1 => Val(OVR_A::_1_0V),
            2 => Val(OVR_A::_1_1V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_9V`"]
    #[inline(always)]
    pub fn is_0_9v(&self) -> bool {
        *self == OVR_A::_0_9V
    }
    #[doc = "Checks if the value of the field is `_1_0V`"]
    #[inline(always)]
    pub fn is_1_0v(&self) -> bool {
        *self == OVR_A::_1_0V
    }
    #[doc = "Checks if the value of the field is `_1_1V`"]
    #[inline(always)]
    pub fn is_1_1v(&self) -> bool {
        *self == OVR_A::_1_1V
    }
}
#[doc = "Write proxy for field `OVR`"]
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.9V +/- 10 Percent"]
    #[inline(always)]
    pub fn _0_9v(self) -> &'a mut W {
        self.variant(OVR_A::_0_9V)
    }
    #[doc = "1.0V +/- 10 Percent"]
    #[inline(always)]
    pub fn _1_0v(self) -> &'a mut W {
        self.variant(OVR_A::_1_0V)
    }
    #[doc = "1.1V +/- 10 Percent"]
    #[inline(always)]
    pub fn _1_1v(self) -> &'a mut W {
        self.variant(OVR_A::_1_1V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Boundary Scan TAP enable. When enabled, the JTAG port is connected to the Boundary Scan TAP. Otherwise, the port is connected to the ARM ICE function. This bit is reset by the POR. Reset value and access depend on the part number."]
    #[inline(always)]
    pub fn bstapen(&self) -> BSTAPEN_R {
        BSTAPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - System bus abritration scheme. These bits are used to select between Fixed-burst abritration and Round-Robin scheme. The Round-Robin scheme is selected by default. These bits are reset by the system reset."]
    #[inline(always)]
    pub fn sbusarb(&self) -> SBUSARB_R {
        SBUSARB_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    pub fn flash_page_flip(&self) -> FLASH_PAGE_FLIP_R {
        FLASH_PAGE_FLIP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    pub fn ccache_flush(&self) -> CCACHE_FLUSH_R {
        CCACHE_FLUSH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Cache Flush. The system cache(s) will be flushed when this bit is set."]
    #[inline(always)]
    pub fn dcache_flush(&self) -> DCACHE_FLUSH_R {
        DCACHE_FLUSH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data Cache Disable. The system cache(s) will be completely disabled when this bit is set."]
    #[inline(always)]
    pub fn dcache_dis(&self) -> DCACHE_DIS_R {
        DCACHE_DIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
    #[inline(always)]
    pub fn cchk(&self) -> CCHK_R {
        CCHK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ROM Checksum Result. This bit is only valid when CHKRD=1."]
    #[inline(always)]
    pub fn chkres(&self) -> CHKRES_R {
        CHKRES_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Operating Voltage Range. Setting these bits according to the VCore voltage allows the on-chip Random-Access memories to operate in their optimal timing range."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Boundary Scan TAP enable. When enabled, the JTAG port is connected to the Boundary Scan TAP. Otherwise, the port is connected to the ARM ICE function. This bit is reset by the POR. Reset value and access depend on the part number."]
    #[inline(always)]
    pub fn bstapen(&mut self) -> BSTAPEN_W {
        BSTAPEN_W { w: self }
    }
    #[doc = "Bits 1:2 - System bus abritration scheme. These bits are used to select between Fixed-burst abritration and Round-Robin scheme. The Round-Robin scheme is selected by default. These bits are reset by the system reset."]
    #[inline(always)]
    pub fn sbusarb(&mut self) -> SBUSARB_W {
        SBUSARB_W { w: self }
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    pub fn flash_page_flip(&mut self) -> FLASH_PAGE_FLIP_W {
        FLASH_PAGE_FLIP_W { w: self }
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    pub fn ccache_flush(&mut self) -> CCACHE_FLUSH_W {
        CCACHE_FLUSH_W { w: self }
    }
    #[doc = "Bit 7 - Data Cache Flush. The system cache(s) will be flushed when this bit is set."]
    #[inline(always)]
    pub fn dcache_flush(&mut self) -> DCACHE_FLUSH_W {
        DCACHE_FLUSH_W { w: self }
    }
    #[doc = "Bit 9 - Data Cache Disable. The system cache(s) will be completely disabled when this bit is set."]
    #[inline(always)]
    pub fn dcache_dis(&mut self) -> DCACHE_DIS_W {
        DCACHE_DIS_W { w: self }
    }
    #[doc = "Bit 13 - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
    #[inline(always)]
    pub fn cchk(&mut self) -> CCHK_W {
        CCHK_W { w: self }
    }
    #[doc = "Bit 15 - ROM Checksum Result. This bit is only valid when CHKRD=1."]
    #[inline(always)]
    pub fn chkres(&mut self) -> CHKRES_W {
        CHKRES_W { w: self }
    }
    #[doc = "Bits 16:17 - Operating Voltage Range. Setting these bits according to the VCore voltage allows the on-chip Random-Access memories to operate in their optimal timing range."]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
}
