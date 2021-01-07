#[doc = "Reader of register SECST"]
pub type R = crate::R<u32, super::SECST>;
#[doc = "External Sensor Control Register Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSRS_A {
    #[doc = "0: Access authorized."]
    ALLOWED = 0,
    #[doc = "1: Access not authorized."]
    NOTALLOWED = 1,
}
impl From<EXTSRS_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTSRS`"]
pub type EXTSRS_R = crate::R<bool, EXTSRS_A>;
impl EXTSRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSRS_A {
        match self.bits {
            false => EXTSRS_A::ALLOWED,
            true => EXTSRS_A::NOTALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == EXTSRS_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == EXTSRS_A::NOTALLOWED
    }
}
#[doc = "Internal Sensor Control Register Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRS_A {
    #[doc = "0: Access authorized."]
    ALLOWED = 0,
    #[doc = "1: Access not authorized."]
    NOTALLOWED = 1,
}
impl From<INTSRS_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTSRS`"]
pub type INTSRS_R = crate::R<bool, INTSRS_A>;
impl INTSRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRS_A {
        match self.bits {
            false => INTSRS_A::ALLOWED,
            true => INTSRS_A::NOTALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == INTSRS_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == INTSRS_A::NOTALLOWED
    }
}
#[doc = "Security Alarm Register Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECALRS_A {
    #[doc = "0: Access authorized."]
    ALLOWED = 0,
    #[doc = "1: Access not authorized."]
    NOTALLOWED = 1,
}
impl From<SECALRS_A> for bool {
    #[inline(always)]
    fn from(variant: SECALRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SECALRS`"]
pub type SECALRS_R = crate::R<bool, SECALRS_A>;
impl SECALRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECALRS_A {
        match self.bits {
            false => SECALRS_A::ALLOWED,
            true => SECALRS_A::NOTALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SECALRS_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == SECALRS_A::NOTALLOWED
    }
}
impl R {
    #[doc = "Bit 0 - External Sensor Control Register Status."]
    #[inline(always)]
    pub fn extsrs(&self) -> EXTSRS_R {
        EXTSRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Sensor Control Register Status."]
    #[inline(always)]
    pub fn intsrs(&self) -> INTSRS_R {
        INTSRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security Alarm Register Status."]
    #[inline(always)]
    pub fn secalrs(&self) -> SECALRS_R {
        SECALRS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
