#[doc = "Reader of register MEMSECCN"]
pub type R = crate::R<u32, super::MEMSECCN>;
#[doc = "Writer for register MEMSECCN"]
pub type W = crate::W<u32, super::MEMSECCN>;
#[doc = "Register MEMSECCN `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMSECCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Decryption Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECEN_A {
    #[doc = "0: Disable decryption of SPIX data."]
    DIS = 0,
    #[doc = "1: Enable decryption of SPIX data."]
    EN = 1,
}
impl From<DECEN_A> for bool {
    #[inline(always)]
    fn from(variant: DECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECEN`"]
pub type DECEN_R = crate::R<bool, DECEN_A>;
impl DECEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECEN_A {
        match self.bits {
            false => DECEN_A::DIS,
            true => DECEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DECEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DECEN_A::EN
    }
}
#[doc = "Write proxy for field `DECEN`"]
pub struct DECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable decryption of SPIX data."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DECEN_A::DIS)
    }
    #[doc = "Enable decryption of SPIX data."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DECEN_A::EN)
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
impl R {
    #[doc = "Bit 0 - Decryption Enable."]
    #[inline(always)]
    pub fn decen(&self) -> DECEN_R {
        DECEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Decryption Enable."]
    #[inline(always)]
    pub fn decen(&mut self) -> DECEN_W {
        DECEN_W { w: self }
    }
}
