#[doc = "Reader of register REG1"]
pub type R = crate::R<u32, super::REG1>;
#[doc = "Writer for register REG1"]
pub type W = crate::W<u32, super::REG1>;
#[doc = "Register REG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::REG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Auto-calibration Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACEN_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<ACEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACEN`"]
pub type ACEN_R = crate::R<bool, ACEN_A>;
impl ACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACEN_A {
        match self.bits {
            false => ACEN_A::DIS,
            true => ACEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ACEN_A::EN
    }
}
#[doc = "Write proxy for field `ACEN`"]
pub struct ACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACEN_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ACEN_A::EN)
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
#[doc = "Autocalibration Run.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACRUN_A {
    #[doc = "0: Not Running."]
    NOT = 0,
    #[doc = "1: Running."]
    RUN = 1,
}
impl From<ACRUN_A> for bool {
    #[inline(always)]
    fn from(variant: ACRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACRUN`"]
pub type ACRUN_R = crate::R<bool, ACRUN_A>;
impl ACRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACRUN_A {
        match self.bits {
            false => ACRUN_A::NOT,
            true => ACRUN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == ACRUN_A::NOT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == ACRUN_A::RUN
    }
}
#[doc = "Write proxy for field `ACRUN`"]
pub struct ACRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACRUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(ACRUN_A::NOT)
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(ACRUN_A::RUN)
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
#[doc = "Reader of field `LDTRM`"]
pub type LDTRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDTRM`"]
pub struct LDTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> LDTRM_W<'a> {
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
#[doc = "Invert Gain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAININV_A {
    #[doc = "0: Not Running."]
    NOT = 0,
    #[doc = "1: Running."]
    RUN = 1,
}
impl From<GAININV_A> for bool {
    #[inline(always)]
    fn from(variant: GAININV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GAININV`"]
pub type GAININV_R = crate::R<bool, GAININV_A>;
impl GAININV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAININV_A {
        match self.bits {
            false => GAININV_A::NOT,
            true => GAININV_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == GAININV_A::NOT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == GAININV_A::RUN
    }
}
#[doc = "Write proxy for field `GAININV`"]
pub struct GAININV_W<'a> {
    w: &'a mut W,
}
impl<'a> GAININV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAININV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(GAININV_A::NOT)
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(GAININV_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Atomic mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATOMIC_A {
    #[doc = "0: Not Running."]
    NOT = 0,
    #[doc = "1: Running."]
    RUN = 1,
}
impl From<ATOMIC_A> for bool {
    #[inline(always)]
    fn from(variant: ATOMIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ATOMIC`"]
pub type ATOMIC_R = crate::R<bool, ATOMIC_A>;
impl ATOMIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATOMIC_A {
        match self.bits {
            false => ATOMIC_A::NOT,
            true => ATOMIC_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == ATOMIC_A::NOT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == ATOMIC_A::RUN
    }
}
#[doc = "Write proxy for field `ATOMIC`"]
pub struct ATOMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOMIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATOMIC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(ATOMIC_A::NOT)
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(ATOMIC_A::RUN)
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
#[doc = "Reader of field `MU`"]
pub type MU_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MU`"]
pub struct MU_W<'a> {
    w: &'a mut W,
}
impl<'a> MU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | (((value as u32) & 0x0fff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Auto-calibration Enable."]
    #[inline(always)]
    pub fn acen(&self) -> ACEN_R {
        ACEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Autocalibration Run."]
    #[inline(always)]
    pub fn acrun(&self) -> ACRUN_R {
        ACRUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Load Trim."]
    #[inline(always)]
    pub fn ldtrm(&self) -> LDTRM_R {
        LDTRM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Invert Gain."]
    #[inline(always)]
    pub fn gaininv(&self) -> GAININV_R {
        GAININV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Atomic mode."]
    #[inline(always)]
    pub fn atomic(&self) -> ATOMIC_R {
        ATOMIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:19 - MU value."]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Auto-calibration Enable."]
    #[inline(always)]
    pub fn acen(&mut self) -> ACEN_W {
        ACEN_W { w: self }
    }
    #[doc = "Bit 1 - Autocalibration Run."]
    #[inline(always)]
    pub fn acrun(&mut self) -> ACRUN_W {
        ACRUN_W { w: self }
    }
    #[doc = "Bit 2 - Load Trim."]
    #[inline(always)]
    pub fn ldtrm(&mut self) -> LDTRM_W {
        LDTRM_W { w: self }
    }
    #[doc = "Bit 3 - Invert Gain."]
    #[inline(always)]
    pub fn gaininv(&mut self) -> GAININV_W {
        GAININV_W { w: self }
    }
    #[doc = "Bit 4 - Atomic mode."]
    #[inline(always)]
    pub fn atomic(&mut self) -> ATOMIC_W {
        ATOMIC_W { w: self }
    }
    #[doc = "Bits 8:19 - MU value."]
    #[inline(always)]
    pub fn mu(&mut self) -> MU_W {
        MU_W { w: self }
    }
}
