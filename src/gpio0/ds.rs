#[doc = "Reader of register DS"]
pub type R = crate::R<u32, super::DS>;
#[doc = "Writer for register DS"]
pub type W = crate::W<u32, super::DS>;
#[doc = "Register DS `reset()`'s with value 0"]
impl crate::ResetValue for super::DS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum DS_A {
    #[doc = "0: GPIO port pin is in low-drive mode."]
    LD = 0,
    #[doc = "1: GPIO port pin is in high-drive mode."]
    HD = 1,
}
impl From<DS_A> for u32 {
    #[inline(always)]
    fn from(variant: DS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<u32, DS_A>;
impl DS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, DS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DS_A::LD),
            1 => Val(DS_A::HD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LD`"]
    #[inline(always)]
    pub fn is_ld(&self) -> bool {
        *self == DS_A::LD
    }
    #[doc = "Checks if the value of the field is `HD`"]
    #[inline(always)]
    pub fn is_hd(&self) -> bool {
        *self == DS_A::HD
    }
}
#[doc = "Write proxy for field `DS`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port pin is in low-drive mode."]
    #[inline(always)]
    pub fn ld(self) -> &'a mut W {
        self.variant(DS_A::LD)
    }
    #[doc = "GPIO port pin is in high-drive mode."]
    #[inline(always)]
    pub fn hd(self) -> &'a mut W {
        self.variant(DS_A::HD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
}
