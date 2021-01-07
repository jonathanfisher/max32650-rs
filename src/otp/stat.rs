#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "This flag indicates whether the OTP controller is currently performing a read or write operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: OTP controller ready for new operation."]
    NOT_BUSY = 0,
    #[doc = "1: OTP controller busy either programming or reading."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NOT_BUSY,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Description not available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAIL_A {
    #[doc = "0: No failure."]
    NFAIL = 0,
    #[doc = "1: Failure occurs."]
    FAIL = 1,
}
impl From<FAIL_A> for bool {
    #[inline(always)]
    fn from(variant: FAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAIL`"]
pub type FAIL_R = crate::R<bool, FAIL_A>;
impl FAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAIL_A {
        match self.bits {
            false => FAIL_A::NFAIL,
            true => FAIL_A::FAIL,
        }
    }
    #[doc = "Checks if the value of the field is `NFAIL`"]
    #[inline(always)]
    pub fn is_nfail(&self) -> bool {
        *self == FAIL_A::NFAIL
    }
    #[doc = "Checks if the value of the field is `FAIL`"]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == FAIL_A::FAIL
    }
}
impl R {
    #[doc = "Bit 0 - This flag indicates whether the OTP controller is currently performing a read or write operation."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Description not available."]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
