#[doc = "Reader of register SISTAT"]
pub type R = crate::R<u32, super::SISTAT>;
#[doc = "Magic Word Validation. This bit is set by the system initialization block following power-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAGIC_A {
    #[doc = "0: Magic word was not set (OTP has not been initialized properly)."]
    MAGICNOTSET = 0,
    #[doc = "1: Magic word was set (OTP contains valid settings)."]
    MAGICSET = 1,
}
impl From<MAGIC_A> for bool {
    #[inline(always)]
    fn from(variant: MAGIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAGIC`"]
pub type MAGIC_R = crate::R<bool, MAGIC_A>;
impl MAGIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAGIC_A {
        match self.bits {
            false => MAGIC_A::MAGICNOTSET,
            true => MAGIC_A::MAGICSET,
        }
    }
    #[doc = "Checks if the value of the field is `MAGICNOTSET`"]
    #[inline(always)]
    pub fn is_magic_not_set(&self) -> bool {
        *self == MAGIC_A::MAGICNOTSET
    }
    #[doc = "Checks if the value of the field is `MAGICSET`"]
    #[inline(always)]
    pub fn is_magic_set(&self) -> bool {
        *self == MAGIC_A::MAGICSET
    }
}
#[doc = "CRC Error Status. This bit is set by the system initialization block following power-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_A {
    #[doc = "0: No CRC errors occurred during the read of the OTP memory block."]
    NOERROR = 0,
    #[doc = "1: A CRC error occurred while reading the OTP. The address of the failure location in the OTP memory is stored in the ERRADDR register."]
    ERROR = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, CRCERR_A>;
impl CRCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::NOERROR,
            true => CRCERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == CRCERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == CRCERR_A::ERROR
    }
}
impl R {
    #[doc = "Bit 0 - Magic Word Validation. This bit is set by the system initialization block following power-up."]
    #[inline(always)]
    pub fn magic(&self) -> MAGIC_R {
        MAGIC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CRC Error Status. This bit is set by the system initialization block following power-up."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
