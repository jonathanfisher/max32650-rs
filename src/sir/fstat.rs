#[doc = "Reader of register FSTAT"]
pub type R = crate::R<u32, super::FSTAT>;
#[doc = "FPU Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<FPU_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FPU`"]
pub type FPU_R = crate::R<bool, FPU_A>;
impl FPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_A {
        match self.bits {
            false => FPU_A::NO,
            true => FPU_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == FPU_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == FPU_A::YES
    }
}
#[doc = "USB Device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<USB_A> for bool {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB`"]
pub type USB_R = crate::R<bool, USB_A>;
impl USB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_A {
        match self.bits {
            false => USB_A::NO,
            true => USB_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == USB_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == USB_A::YES
    }
}
#[doc = "10-bit Sigma Delta ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, ADC_A>;
impl ADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::NO,
            true => ADC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ADC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ADC_A::YES
    }
}
#[doc = "XiP function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIP_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<XIP_A> for bool {
    #[inline(always)]
    fn from(variant: XIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XIP`"]
pub type XIP_R = crate::R<bool, XIP_A>;
impl XIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIP_A {
        match self.bits {
            false => XIP_A::NO,
            true => XIP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == XIP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == XIP_A::YES
    }
}
#[doc = "PBM function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBM_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<PBM_A> for bool {
    #[inline(always)]
    fn from(variant: PBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PBM`"]
pub type PBM_R = crate::R<bool, PBM_A>;
impl PBM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBM_A {
        match self.bits {
            false => PBM_A::NO,
            true => PBM_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PBM_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == PBM_A::YES
    }
}
#[doc = "HBC function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HBC_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<HBC_A> for bool {
    #[inline(always)]
    fn from(variant: HBC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HBC`"]
pub type HBC_R = crate::R<bool, HBC_A>;
impl HBC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HBC_A {
        match self.bits {
            false => HBC_A::NO,
            true => HBC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == HBC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == HBC_A::YES
    }
}
#[doc = "SDHC function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHC_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<SDHC_A> for bool {
    #[inline(always)]
    fn from(variant: SDHC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDHC`"]
pub type SDHC_R = crate::R<bool, SDHC_A>;
impl SDHC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHC_A {
        match self.bits {
            false => SDHC_A::NO,
            true => SDHC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SDHC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SDHC_A::YES
    }
}
#[doc = "SMPHR function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPHR_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<SMPHR_A> for bool {
    #[inline(always)]
    fn from(variant: SMPHR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPHR`"]
pub type SMPHR_R = crate::R<bool, SMPHR_A>;
impl SMPHR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPHR_A {
        match self.bits {
            false => SMPHR_A::NO,
            true => SMPHR_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMPHR_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SMPHR_A::YES
    }
}
#[doc = "System Cache function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCACHE_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<SCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: SCACHE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCACHE`"]
pub type SCACHE_R = crate::R<bool, SCACHE_A>;
impl SCACHE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCACHE_A {
        match self.bits {
            false => SCACHE_A::NO,
            true => SCACHE_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SCACHE_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SCACHE_A::YES
    }
}
impl R {
    #[doc = "Bit 0 - FPU Function."]
    #[inline(always)]
    pub fn fpu(&self) -> FPU_R {
        FPU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB Device."]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 10-bit Sigma Delta ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XiP function."]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PBM function."]
    #[inline(always)]
    pub fn pbm(&self) -> PBM_R {
        PBM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HBC function."]
    #[inline(always)]
    pub fn hbc(&self) -> HBC_R {
        HBC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SDHC function."]
    #[inline(always)]
    pub fn sdhc(&self) -> SDHC_R {
        SDHC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SMPHR function."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - System Cache function."]
    #[inline(always)]
    pub fn scache(&self) -> SCACHE_R {
        SCACHE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
