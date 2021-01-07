#[doc = "Reader of register CN"]
pub type R = crate::R<u32, super::CN>;
#[doc = "Writer for register CN"]
pub type W = crate::W<u32, super::CN>;
#[doc = "Register CN `reset()`'s with value 0"]
impl crate::ResetValue for super::CN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write. This bit is automatically cleared after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WR_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<WR_A> for bool {
    #[inline(always)]
    fn from(variant: WR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WR`"]
pub type WR_R = crate::R<bool, WR_A>;
impl WR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WR_A {
        match self.bits {
            false => WR_A::COMPLETE,
            true => WR_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == WR_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WR_A::START
    }
}
#[doc = "Write proxy for field `WR`"]
pub struct WR_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(WR_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(WR_A::START)
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
#[doc = "Reader of field `ME`"]
pub type ME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ME`"]
pub struct ME_W<'a> {
    w: &'a mut W,
}
impl<'a> ME_W<'a> {
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
#[doc = "Reader of field `PGE`"]
pub type PGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE`"]
pub struct PGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE_W<'a> {
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
#[doc = "Data Width. This bits selects write data width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTH_A {
    #[doc = "0: 128-bit."]
    SIZE128 = 0,
    #[doc = "1: 32-bit."]
    SIZE32 = 1,
}
impl From<WDTH_A> for bool {
    #[inline(always)]
    fn from(variant: WDTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDTH`"]
pub type WDTH_R = crate::R<bool, WDTH_A>;
impl WDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTH_A {
        match self.bits {
            false => WDTH_A::SIZE128,
            true => WDTH_A::SIZE32,
        }
    }
    #[doc = "Checks if the value of the field is `SIZE128`"]
    #[inline(always)]
    pub fn is_size128(&self) -> bool {
        *self == WDTH_A::SIZE128
    }
    #[doc = "Checks if the value of the field is `SIZE32`"]
    #[inline(always)]
    pub fn is_size32(&self) -> bool {
        *self == WDTH_A::SIZE32
    }
}
#[doc = "Write proxy for field `WDTH`"]
pub struct WDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "128-bit."]
    #[inline(always)]
    pub fn size128(self) -> &'a mut W {
        self.variant(WDTH_A::SIZE128)
    }
    #[doc = "32-bit."]
    #[inline(always)]
    pub fn size32(self) -> &'a mut W {
        self.variant(WDTH_A::SIZE32)
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
#[doc = "Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERASE_CODE_A {
    #[doc = "0: No operation."]
    NOP = 0,
    #[doc = "85: Enable Page Erase."]
    ERASEPAGE = 85,
    #[doc = "170: Enable Mass Erase. The debug port must be enabled."]
    ERASEALL = 170,
}
impl From<ERASE_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERASE_CODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ERASE_CODE`"]
pub type ERASE_CODE_R = crate::R<u8, ERASE_CODE_A>;
impl ERASE_CODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ERASE_CODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ERASE_CODE_A::NOP),
            85 => Val(ERASE_CODE_A::ERASEPAGE),
            170 => Val(ERASE_CODE_A::ERASEALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ERASE_CODE_A::NOP
    }
    #[doc = "Checks if the value of the field is `ERASEPAGE`"]
    #[inline(always)]
    pub fn is_erase_page(&self) -> bool {
        *self == ERASE_CODE_A::ERASEPAGE
    }
    #[doc = "Checks if the value of the field is `ERASEALL`"]
    #[inline(always)]
    pub fn is_erase_all(&self) -> bool {
        *self == ERASE_CODE_A::ERASEALL
    }
}
#[doc = "Write proxy for field `ERASE_CODE`"]
pub struct ERASE_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_CODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASE_CODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No operation."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(ERASE_CODE_A::NOP)
    }
    #[doc = "Enable Page Erase."]
    #[inline(always)]
    pub fn erase_page(self) -> &'a mut W {
        self.variant(ERASE_CODE_A::ERASEPAGE)
    }
    #[doc = "Enable Mass Erase. The debug port must be enabled."]
    #[inline(always)]
    pub fn erase_all(self) -> &'a mut W {
        self.variant(ERASE_CODE_A::ERASEALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEND_A {
    #[doc = "0: Idle."]
    IDLE = 0,
    #[doc = "1: Busy."]
    BUSY = 1,
}
impl From<PEND_A> for bool {
    #[inline(always)]
    fn from(variant: PEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEND`"]
pub type PEND_R = crate::R<bool, PEND_A>;
impl PEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEND_A {
        match self.bits {
            false => PEND_A::IDLE,
            true => PEND_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PEND_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PEND_A::BUSY
    }
}
#[doc = "Low Voltage Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVE_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    EN = 1,
}
impl From<LVE_A> for bool {
    #[inline(always)]
    fn from(variant: LVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LVE`"]
pub type LVE_R = crate::R<bool, LVE_A>;
impl LVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVE_A {
        match self.bits {
            false => LVE_A::DIS,
            true => LVE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LVE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == LVE_A::EN
    }
}
#[doc = "Burst Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BRST_A> for bool {
    #[inline(always)]
    fn from(variant: BRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRST`"]
pub type BRST_R = crate::R<bool, BRST_A>;
impl BRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRST_A {
        match self.bits {
            false => BRST_A::DISABLE,
            true => BRST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BRST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BRST_A::ENABLE
    }
}
#[doc = "Write proxy for field `BRST`"]
pub struct BRST_W<'a> {
    w: &'a mut W,
}
impl<'a> BRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BRST_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BRST_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UNLOCK_A {
    #[doc = "2: Flash Unlocked"]
    UNLOCKED = 2,
    #[doc = "3: Flash Locked"]
    LOCKED = 3,
}
impl From<UNLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: UNLOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UNLOCK`"]
pub type UNLOCK_R = crate::R<u8, UNLOCK_A>;
impl UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UNLOCK_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(UNLOCK_A::UNLOCKED),
            3 => Val(UNLOCK_A::LOCKED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == UNLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == UNLOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `UNLOCK`"]
pub struct UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNLOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash Unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(UNLOCK_A::UNLOCKED)
    }
    #[doc = "Flash Locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(UNLOCK_A::LOCKED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mass Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn me(&self) -> ME_R {
        ME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Page Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Width. This bits selects write data width."]
    #[inline(always)]
    pub fn wdth(&self) -> WDTH_R {
        WDTH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
    #[inline(always)]
    pub fn erase_code(&self) -> ERASE_CODE_R {
        ERASE_CODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored."]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Low Voltage Read Enable"]
    #[inline(always)]
    pub fn lve(&self) -> LVE_R {
        LVE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Burst Mode Enable."]
    #[inline(always)]
    pub fn brst(&self) -> BRST_R {
        BRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W {
        WR_W { w: self }
    }
    #[doc = "Bit 1 - Mass Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn me(&mut self) -> ME_W {
        ME_W { w: self }
    }
    #[doc = "Bit 2 - Page Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn pge(&mut self) -> PGE_W {
        PGE_W { w: self }
    }
    #[doc = "Bit 4 - Data Width. This bits selects write data width."]
    #[inline(always)]
    pub fn wdth(&mut self) -> WDTH_W {
        WDTH_W { w: self }
    }
    #[doc = "Bits 8:15 - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
    #[inline(always)]
    pub fn erase_code(&mut self) -> ERASE_CODE_W {
        ERASE_CODE_W { w: self }
    }
    #[doc = "Bit 27 - Burst Mode Enable."]
    #[inline(always)]
    pub fn brst(&mut self) -> BRST_W {
        BRST_W { w: self }
    }
    #[doc = "Bits 28:31 - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W {
        UNLOCK_W { w: self }
    }
}
