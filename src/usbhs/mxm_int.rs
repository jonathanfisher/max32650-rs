#[doc = "Reader of register MXM_INT"]
pub type R = crate::R<u32, super::MXM_INT>;
#[doc = "Writer for register MXM_INT"]
pub type W = crate::W<u32, super::MXM_INT>;
#[doc = "Register MXM_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::MXM_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBUS`"]
pub type VBUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUS`"]
pub struct VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_W<'a> {
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
#[doc = "Reader of field `NOVBUS`"]
pub type NOVBUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOVBUS`"]
pub struct NOVBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> NOVBUS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - VBUS"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NOVBUS"]
    #[inline(always)]
    pub fn novbus(&self) -> NOVBUS_R {
        NOVBUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VBUS_W {
        VBUS_W { w: self }
    }
    #[doc = "Bit 1 - NOVBUS"]
    #[inline(always)]
    pub fn novbus(&mut self) -> NOVBUS_W {
        NOVBUS_W { w: self }
    }
}
