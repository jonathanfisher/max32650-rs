#[doc = "Reader of register BBFCR0"]
pub type R = crate::R<u32, super::BBFCR0>;
#[doc = "Writer for register BBFCR0"]
pub type W = crate::W<u32, super::BBFCR0>;
#[doc = "Register BBFCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BBFCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CKPDRV`"]
pub type CKPDRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKPDRV`"]
pub struct CKPDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CKNPDRV`"]
pub type CKNPDRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKNPDRV`"]
pub struct CKNPDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKNPDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Hyperbus RDS DLL Power Up Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDSDLLEN_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<RDSDLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: RDSDLLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDSDLLEN`"]
pub type RDSDLLEN_R = crate::R<bool, RDSDLLEN_A>;
impl RDSDLLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDSDLLEN_A {
        match self.bits {
            false => RDSDLLEN_A::DIS,
            true => RDSDLLEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RDSDLLEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RDSDLLEN_A::EN
    }
}
#[doc = "Write proxy for field `RDSDLLEN`"]
pub struct RDSDLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSDLLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDSDLLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RDSDLLEN_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RDSDLLEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Hyperbus CK Pad Driver Control."]
    #[inline(always)]
    pub fn ckpdrv(&self) -> CKPDRV_R {
        CKPDRV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Hyperbus CKN Pad Driver Control."]
    #[inline(always)]
    pub fn cknpdrv(&self) -> CKNPDRV_R {
        CKNPDRV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Hyperbus RDS DLL Power Up Control."]
    #[inline(always)]
    pub fn rdsdllen(&self) -> RDSDLLEN_R {
        RDSDLLEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Hyperbus CK Pad Driver Control."]
    #[inline(always)]
    pub fn ckpdrv(&mut self) -> CKPDRV_W {
        CKPDRV_W { w: self }
    }
    #[doc = "Bits 4:7 - Hyperbus CKN Pad Driver Control."]
    #[inline(always)]
    pub fn cknpdrv(&mut self) -> CKNPDRV_W {
        CKNPDRV_W { w: self }
    }
    #[doc = "Bit 8 - Hyperbus RDS DLL Power Up Control."]
    #[inline(always)]
    pub fn rdsdllen(&mut self) -> RDSDLLEN_W {
        RDSDLLEN_W { w: self }
    }
}
