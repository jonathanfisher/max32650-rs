#[doc = "Reader of register SFSTAT"]
pub type R = crate::R<u32, super::SFSTAT>;
#[doc = "TRNG function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<TRNG_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRNG`"]
pub type TRNG_R = crate::R<bool, TRNG_A>;
impl TRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNG_A {
        match self.bits {
            false => TRNG_A::NO,
            true => TRNG_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == TRNG_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == TRNG_A::YES
    }
}
#[doc = "AES function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<AES_A> for bool {
    #[inline(always)]
    fn from(variant: AES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, AES_A>;
impl AES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_A {
        match self.bits {
            false => AES_A::NO,
            true => AES_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == AES_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == AES_A::YES
    }
}
#[doc = "SHA function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHA_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<SHA_A> for bool {
    #[inline(always)]
    fn from(variant: SHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SHA`"]
pub type SHA_R = crate::R<bool, SHA_A>;
impl SHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHA_A {
        match self.bits {
            false => SHA_A::NO,
            true => SHA_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SHA_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SHA_A::YES
    }
}
#[doc = "MAA function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAA_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<MAA_A> for bool {
    #[inline(always)]
    fn from(variant: MAA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAA`"]
pub type MAA_R = crate::R<bool, MAA_A>;
impl MAA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAA_A {
        match self.bits {
            false => MAA_A::NO,
            true => MAA_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == MAA_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == MAA_A::YES
    }
}
impl R {
    #[doc = "Bit 2 - TRNG function."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AES function."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SHA function."]
    #[inline(always)]
    pub fn sha(&self) -> SHA_R {
        SHA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MAA function."]
    #[inline(always)]
    pub fn maa(&self) -> MAA_R {
        MAA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
