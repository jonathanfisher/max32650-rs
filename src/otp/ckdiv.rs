#[doc = "Reader of register CKDIV"]
pub type R = crate::R<u32, super::CKDIV>;
#[doc = "Writer for register CKDIV"]
pub type W = crate::W<u32, super::CKDIV>;
#[doc = "Register CKDIV `reset()`'s with value 0x64"]
impl crate::ResetValue for super::CKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x64
    }
}
#[doc = "The input clock(APB) is divided by this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKDIV_A {
    #[doc = "0: Divide by 2."]
    DIV_BY_2 = 0,
    #[doc = "1: Divide by 4."]
    DIV_BY_4 = 1,
    #[doc = "2: Divide by 8."]
    DIV_BY_8 = 2,
    #[doc = "3: Divide by 16."]
    DIV_BY_16 = 3,
}
impl From<CKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CKDIV`"]
pub type CKDIV_R = crate::R<u8, CKDIV_A>;
impl CKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKDIV_A {
        match self.bits {
            0 => CKDIV_A::DIV_BY_2,
            1 => CKDIV_A::DIV_BY_4,
            2 => CKDIV_A::DIV_BY_8,
            3 => CKDIV_A::DIV_BY_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == CKDIV_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == CKDIV_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == CKDIV_A::DIV_BY_8
    }
    #[doc = "Checks if the value of the field is `DIV_BY_16`"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == CKDIV_A::DIV_BY_16
    }
}
#[doc = "Write proxy for field `CKDIV`"]
pub struct CKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(CKDIV_A::DIV_BY_2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(CKDIV_A::DIV_BY_4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(CKDIV_A::DIV_BY_8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut W {
        self.variant(CKDIV_A::DIV_BY_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - The input clock(APB) is divided by this value."]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - The input clock(APB) is divided by this value."]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W {
        CKDIV_W { w: self }
    }
}
