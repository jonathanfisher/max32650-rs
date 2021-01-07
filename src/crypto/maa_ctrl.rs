#[doc = "Reader of register MAA_CTRL"]
pub type R = crate::R<u32, super::MAA_CTRL>;
#[doc = "Writer for register MAA_CTRL"]
pub type W = crate::W<u32, super::MAA_CTRL>;
#[doc = "Register MAA_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MAA_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start Calculation. This bit functions as both the control and the status of the MAA. If the size value in the MAWS register is invalid, the STC bit will be cleared by hardware immediately. Otherwise, the STC bit is automatically cleared following the completion of each calculation or detecting an error. Clearing the STC bit resets the controller to its default state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STC_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<STC_A> for bool {
    #[inline(always)]
    fn from(variant: STC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STC`"]
pub type STC_R = crate::R<bool, STC_A>;
impl STC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STC_A {
        match self.bits {
            false => STC_A::NOP,
            true => STC_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == STC_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STC_A::START
    }
}
#[doc = "Write proxy for field `STC`"]
pub struct STC_W<'a> {
    w: &'a mut W,
}
impl<'a> STC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(STC_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STC_A::START)
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
#[doc = "Calculation Configuration. These bits select desired calculation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLC_A {
    #[doc = "0: Exponentiation."]
    EXP = 0,
    #[doc = "1: Square operation."]
    SQ = 1,
    #[doc = "2: Multiplication."]
    MUL = 2,
    #[doc = "3: Square followed by a multiplication."]
    SQMUL = 3,
    #[doc = "4: Addition."]
    ADD = 4,
    #[doc = "5: Subtraction."]
    SUB = 5,
}
impl From<CLC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLC`"]
pub type CLC_R = crate::R<u8, CLC_A>;
impl CLC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLC_A::EXP),
            1 => Val(CLC_A::SQ),
            2 => Val(CLC_A::MUL),
            3 => Val(CLC_A::SQMUL),
            4 => Val(CLC_A::ADD),
            5 => Val(CLC_A::SUB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXP`"]
    #[inline(always)]
    pub fn is_exp(&self) -> bool {
        *self == CLC_A::EXP
    }
    #[doc = "Checks if the value of the field is `SQ`"]
    #[inline(always)]
    pub fn is_sq(&self) -> bool {
        *self == CLC_A::SQ
    }
    #[doc = "Checks if the value of the field is `MUL`"]
    #[inline(always)]
    pub fn is_mul(&self) -> bool {
        *self == CLC_A::MUL
    }
    #[doc = "Checks if the value of the field is `SQMUL`"]
    #[inline(always)]
    pub fn is_sq_mul(&self) -> bool {
        *self == CLC_A::SQMUL
    }
    #[doc = "Checks if the value of the field is `ADD`"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == CLC_A::ADD
    }
    #[doc = "Checks if the value of the field is `SUB`"]
    #[inline(always)]
    pub fn is_sub(&self) -> bool {
        *self == CLC_A::SUB
    }
}
#[doc = "Write proxy for field `CLC`"]
pub struct CLC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Exponentiation."]
    #[inline(always)]
    pub fn exp(self) -> &'a mut W {
        self.variant(CLC_A::EXP)
    }
    #[doc = "Square operation."]
    #[inline(always)]
    pub fn sq(self) -> &'a mut W {
        self.variant(CLC_A::SQ)
    }
    #[doc = "Multiplication."]
    #[inline(always)]
    pub fn mul(self) -> &'a mut W {
        self.variant(CLC_A::MUL)
    }
    #[doc = "Square followed by a multiplication."]
    #[inline(always)]
    pub fn sq_mul(self) -> &'a mut W {
        self.variant(CLC_A::SQMUL)
    }
    #[doc = "Addition."]
    #[inline(always)]
    pub fn add(self) -> &'a mut W {
        self.variant(CLC_A::ADD)
    }
    #[doc = "Subtraction."]
    #[inline(always)]
    pub fn sub(self) -> &'a mut W {
        self.variant(CLC_A::SUB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Optimized Calculation Control. For optimized calculation, unnecessary multiply operations after normalizing the exponent are skipped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCALC_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<OCALC_A> for bool {
    #[inline(always)]
    fn from(variant: OCALC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCALC`"]
pub type OCALC_R = crate::R<bool, OCALC_A>;
impl OCALC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCALC_A {
        match self.bits {
            false => OCALC_A::DIS,
            true => OCALC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == OCALC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == OCALC_A::EN
    }
}
#[doc = "Write proxy for field `OCALC`"]
pub struct OCALC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCALC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCALC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(OCALC_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(OCALC_A::EN)
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
#[doc = "MAA Error. The MAAER bit defaults to 0 and can only be set by hardware. Once set, it must be cleared by software otherwise no new operation can be initiated. Software writes 1 to this bit has no effect and MAAER will maintain its original state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAAER_A {
    #[doc = "0: No Error."]
    NOERROR = 0,
    #[doc = "1: Error."]
    ERROR = 1,
}
impl From<MAAER_A> for bool {
    #[inline(always)]
    fn from(variant: MAAER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAAER`"]
pub type MAAER_R = crate::R<bool, MAAER_A>;
impl MAAER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAAER_A {
        match self.bits {
            false => MAAER_A::NOERROR,
            true => MAAER_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MAAER_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MAAER_A::ERROR
    }
}
#[doc = "Write proxy for field `MAAER`"]
pub struct MAAER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAAER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAAER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Error."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(MAAER_A::NOERROR)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(MAAER_A::ERROR)
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
#[doc = "Reader of field `AMS`"]
pub type AMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMS`"]
pub struct AMS_W<'a> {
    w: &'a mut W,
}
impl<'a> AMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `BMS`"]
pub type BMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BMS`"]
pub struct BMS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `EMS`"]
pub type EMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EMS`"]
pub struct EMS_W<'a> {
    w: &'a mut W,
}
impl<'a> EMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `MMS`"]
pub type MMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MMS`"]
pub struct MMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `AMA`"]
pub type AMA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMA`"]
pub struct AMA_W<'a> {
    w: &'a mut W,
}
impl<'a> AMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `BMA`"]
pub type BMA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BMA`"]
pub struct BMA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RMA`"]
pub type RMA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMA`"]
pub struct RMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `TMA`"]
pub type TMA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMA`"]
pub struct TMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start Calculation. This bit functions as both the control and the status of the MAA. If the size value in the MAWS register is invalid, the STC bit will be cleared by hardware immediately. Otherwise, the STC bit is automatically cleared following the completion of each calculation or detecting an error. Clearing the STC bit resets the controller to its default state."]
    #[inline(always)]
    pub fn stc(&self) -> STC_R {
        STC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Calculation Configuration. These bits select desired calculation."]
    #[inline(always)]
    pub fn clc(&self) -> CLC_R {
        CLC_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Optimized Calculation Control. For optimized calculation, unnecessary multiply operations after normalizing the exponent are skipped."]
    #[inline(always)]
    pub fn ocalc(&self) -> OCALC_R {
        OCALC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MAA Error. The MAAER bit defaults to 0 and can only be set by hardware. Once set, it must be cleared by software otherwise no new operation can be initiated. Software writes 1 to this bit has no effect and MAAER will maintain its original state."]
    #[inline(always)]
    pub fn maaer(&self) -> MAAER_R {
        MAAER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Multiplier A Memory Select. These bits select the starting position of the parameter 'a' within the logical segment specified by AMA."]
    #[inline(always)]
    pub fn ams(&self) -> AMS_R {
        AMS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Multiplicand B Memory Select. These bits select the starting position of the parameter 'b' within the logical segment specified by BMA."]
    #[inline(always)]
    pub fn bms(&self) -> BMS_R {
        BMS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Exponent Memory Select. These bits select the starting position of the parameter 'e' within the logical segment specified by EMA."]
    #[inline(always)]
    pub fn ems(&self) -> EMS_R {
        EMS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Modulus Memory Select. These bits select the starting position of the parameter 'm' within the logical segment 5."]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Multiplier / Operand A Memory Assignment. These bits select the logical cryptographic RAM segment for the parameter 'a'."]
    #[inline(always)]
    pub fn ama(&self) -> AMA_R {
        AMA_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Multiplicand / Operand B Memory Assignment. These bits select the logical cryptographic RAM segment for the parameter 'b'."]
    #[inline(always)]
    pub fn bma(&self) -> BMA_R {
        BMA_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Result Memory Assignment. These bits select the logical cryptographic RAM segment for the parameter 'r'."]
    #[inline(always)]
    pub fn rma(&self) -> RMA_R {
        RMA_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Temporary Memory Assignment. These bits select the logical cryptographic RAM segment for the parameter 't'."]
    #[inline(always)]
    pub fn tma(&self) -> TMA_R {
        TMA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start Calculation. This bit functions as both the control and the status of the MAA. If the size value in the MAWS register is invalid, the STC bit will be cleared by hardware immediately. Otherwise, the STC bit is automatically cleared following the completion of each calculation or detecting an error. Clearing the STC bit resets the controller to its default state."]
    #[inline(always)]
    pub fn stc(&mut self) -> STC_W {
        STC_W { w: self }
    }
    #[doc = "Bits 1:3 - Calculation Configuration. These bits select desired calculation."]
    #[inline(always)]
    pub fn clc(&mut self) -> CLC_W {
        CLC_W { w: self }
    }
    #[doc = "Bit 4 - Optimized Calculation Control. For optimized calculation, unnecessary multiply operations after normalizing the exponent are skipped."]
    #[inline(always)]
    pub fn ocalc(&mut self) -> OCALC_W {
        OCALC_W { w: self }
    }
    #[doc = "Bit 7 - MAA Error. The MAAER bit defaults to 0 and can only be set by hardware. Once set, it must be cleared by software otherwise no new operation can be initiated. Software writes 1 to this bit has no effect and MAAER will maintain its original state."]
    #[inline(always)]
    pub fn maaer(&mut self) -> MAAER_W {
        MAAER_W { w: self }
    }
    #[doc = "Bits 8:9 - Multiplier A Memory Select. These bits select the starting position of the parameter 'a' within the logical segment specified by AMA."]
    #[inline(always)]
    pub fn ams(&mut self) -> AMS_W {
        AMS_W { w: self }
    }
    #[doc = "Bits 10:11 - Multiplicand B Memory Select. These bits select the starting position of the parameter 'b' within the logical segment specified by BMA."]
    #[inline(always)]
    pub fn bms(&mut self) -> BMS_W {
        BMS_W { w: self }
    }
    #[doc = "Bits 12:13 - Exponent Memory Select. These bits select the starting position of the parameter 'e' within the logical segment specified by EMA."]
    #[inline(always)]
    pub fn ems(&mut self) -> EMS_W {
        EMS_W { w: self }
    }
    #[doc = "Bits 14:15 - Modulus Memory Select. These bits select the starting position of the parameter 'm' within the logical segment 5."]
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W {
        MMS_W { w: self }
    }
    #[doc = "Bits 16:19 - Multiplier / Operand A Memory Assignment. These bits select the logical cryptographic RAM segment for the parameter 'a'."]
    #[inline(always)]
    pub fn ama(&mut self) -> AMA_W {
        AMA_W { w: self }
    }
    #[doc = "Bits 20:23 - Multiplicand / Operand B Memory Assignment. These bits select the logical cryptographic RAM segment for the parameter 'b'."]
    #[inline(always)]
    pub fn bma(&mut self) -> BMA_W {
        BMA_W { w: self }
    }
    #[doc = "Bits 24:27 - Result Memory Assignment. These bits select the logical cryptographic RAM segment for the parameter 'r'."]
    #[inline(always)]
    pub fn rma(&mut self) -> RMA_W {
        RMA_W { w: self }
    }
    #[doc = "Bits 28:31 - Temporary Memory Assignment. These bits select the logical cryptographic RAM segment for the parameter 't'."]
    #[inline(always)]
    pub fn tma(&mut self) -> TMA_W {
        TMA_W { w: self }
    }
}
