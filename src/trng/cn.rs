#[doc = "Reader of register CN"]
pub type R = crate::R<u32, super::CN>;
#[doc = "Writer for register CN"]
pub type W = crate::W<u32, super::CN>;
#[doc = "Register CN `reset()`'s with value 0x03"]
impl crate::ResetValue for super::CN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Random Number Interrupt Enable. This bit enables an interrupt to be generated when a new, 128 bit, random number is ready to be read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_IE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RNG_IE_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNG_IE`"]
pub type RNG_IE_R = crate::R<bool, RNG_IE_A>;
impl RNG_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_IE_A {
        match self.bits {
            false => RNG_IE_A::DIS,
            true => RNG_IE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RNG_IE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RNG_IE_A::EN
    }
}
#[doc = "Write proxy for field `RNG_IE`"]
pub struct RNG_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RNG_IE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RNG_IE_A::EN)
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
#[doc = "Random Number Interrupt Status Clear. Setting this bit to 1 clears the RNG_I4S bit and acknowledges the interrupt, if enabled. This bit is a write only bit and always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_ISC_AW {
    #[doc = "0: No Effect."]
    NO_EFFECT = 0,
    #[doc = "1: Clear the Status bit."]
    CLEAR = 1,
}
impl From<RNG_ISC_AW> for bool {
    #[inline(always)]
    fn from(variant: RNG_ISC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RNG_ISC`"]
pub struct RNG_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_ISC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG_ISC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RNG_ISC_AW::NO_EFFECT)
    }
    #[doc = "Clear the Status bit."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RNG_ISC_AW::CLEAR)
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
#[doc = "Random Number 4 Word Status. This bit is set when a new 128 bit random number is ready to be read (using 4 consecutive reads of the TRNG Data Register). When set, an interrupt will be generated if the RNG_IE bit is also set. This bit is cleared by setting the RNG_ISC bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_I4S_A {
    #[doc = "0: Result not ready."]
    BUSY = 0,
    #[doc = "1: Operation complete and result ready."]
    READY = 1,
}
impl From<RNG_I4S_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_I4S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNG_I4S`"]
pub type RNG_I4S_R = crate::R<bool, RNG_I4S_A>;
impl RNG_I4S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_I4S_A {
        match self.bits {
            false => RNG_I4S_A::BUSY,
            true => RNG_I4S_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RNG_I4S_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RNG_I4S_A::READY
    }
}
#[doc = "Random Number Word Status. This bit is set when at least one 32 bit random number is ready to be read. This bit is cleared by hardware if all the random words have been read. It is needed to poll this bit before reading the TRNG Data Register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_IS_A {
    #[doc = "0: Result not ready."]
    BUSY = 0,
    #[doc = "1: Operation complete and result ready."]
    READY = 1,
}
impl From<RNG_IS_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_IS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNG_IS`"]
pub type RNG_IS_R = crate::R<bool, RNG_IS_A>;
impl RNG_IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_IS_A {
        match self.bits {
            false => RNG_IS_A::BUSY,
            true => RNG_IS_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RNG_IS_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RNG_IS_A::READY
    }
}
#[doc = "AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESKG_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<AESKG_A> for bool {
    #[inline(always)]
    fn from(variant: AESKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESKG`"]
pub type AESKG_R = crate::R<bool, AESKG_A>;
impl AESKG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESKG_A {
        match self.bits {
            false => AESKG_A::COMPLETE,
            true => AESKG_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == AESKG_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == AESKG_A::START
    }
}
#[doc = "Write proxy for field `AESKG`"]
pub struct AESKG_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(AESKG_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(AESKG_A::START)
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
impl R {
    #[doc = "Bit 2 - Random Number Interrupt Enable. This bit enables an interrupt to be generated when a new, 128 bit, random number is ready to be read."]
    #[inline(always)]
    pub fn rng_ie(&self) -> RNG_IE_R {
        RNG_IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Random Number 4 Word Status. This bit is set when a new 128 bit random number is ready to be read (using 4 consecutive reads of the TRNG Data Register). When set, an interrupt will be generated if the RNG_IE bit is also set. This bit is cleared by setting the RNG_ISC bit."]
    #[inline(always)]
    pub fn rng_i4s(&self) -> RNG_I4S_R {
        RNG_I4S_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Random Number Word Status. This bit is set when at least one 32 bit random number is ready to be read. This bit is cleared by hardware if all the random words have been read. It is needed to poll this bit before reading the TRNG Data Register."]
    #[inline(always)]
    pub fn rng_is(&self) -> RNG_IS_R {
        RNG_IS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
    #[inline(always)]
    pub fn aeskg(&self) -> AESKG_R {
        AESKG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Random Number Interrupt Enable. This bit enables an interrupt to be generated when a new, 128 bit, random number is ready to be read."]
    #[inline(always)]
    pub fn rng_ie(&mut self) -> RNG_IE_W {
        RNG_IE_W { w: self }
    }
    #[doc = "Bit 3 - Random Number Interrupt Status Clear. Setting this bit to 1 clears the RNG_I4S bit and acknowledges the interrupt, if enabled. This bit is a write only bit and always reads as zero."]
    #[inline(always)]
    pub fn rng_isc(&mut self) -> RNG_ISC_W {
        RNG_ISC_W { w: self }
    }
    #[doc = "Bit 6 - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
    #[inline(always)]
    pub fn aeskg(&mut self) -> AESKG_W {
        AESKG_W { w: self }
    }
}
