#[doc = "Reader of register CACHE_CTRL"]
pub type R = crate::R<u32, super::CACHE_CTRL>;
#[doc = "Writer for register CACHE_CTRL"]
pub type W = crate::W<u32, super::CACHE_CTRL>;
#[doc = "Register CACHE_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CACHE_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_EN_A {
    #[doc = "0: Cache Bypassed. Instruction data is stored in the line fill buffer but is not written to main cache memory array."]
    DIS = 0,
    #[doc = "1: Cache Enabled."]
    EN = 1,
}
impl From<CACHE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHE_EN`"]
pub type CACHE_EN_R = crate::R<bool, CACHE_EN_A>;
impl CACHE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_EN_A {
        match self.bits {
            false => CACHE_EN_A::DIS,
            true => CACHE_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CACHE_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CACHE_EN_A::EN
    }
}
#[doc = "Write proxy for field `CACHE_EN`"]
pub struct CACHE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cache Bypassed. Instruction data is stored in the line fill buffer but is not written to main cache memory array."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHE_EN_A::DIS)
    }
    #[doc = "Cache Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHE_EN_A::EN)
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
#[doc = "Write Allocate Enable. This bit only writable while the cache is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_ALLOC_EN_A {
    #[doc = "0: Write-no-allocate."]
    DIS = 0,
    #[doc = "1: Write-allocate enabled."]
    EN = 1,
}
impl From<WRITE_ALLOC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_ALLOC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRITE_ALLOC_EN`"]
pub type WRITE_ALLOC_EN_R = crate::R<bool, WRITE_ALLOC_EN_A>;
impl WRITE_ALLOC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_ALLOC_EN_A {
        match self.bits {
            false => WRITE_ALLOC_EN_A::DIS,
            true => WRITE_ALLOC_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WRITE_ALLOC_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WRITE_ALLOC_EN_A::EN
    }
}
#[doc = "Write proxy for field `WRITE_ALLOC_EN`"]
pub struct WRITE_ALLOC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ALLOC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_ALLOC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write-no-allocate."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WRITE_ALLOC_EN_A::DIS)
    }
    #[doc = "Write-allocate enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(WRITE_ALLOC_EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Critical word first and streaming disable. This bit only writeable while the cache is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWFST_DIS_A {
    #[doc = "1: Critical word first and streaming disabled."]
    DIS = 1,
    #[doc = "0: Critical word first and streaming enabled."]
    EN = 0,
}
impl From<CWFST_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: CWFST_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CWFST_DIS`"]
pub type CWFST_DIS_R = crate::R<bool, CWFST_DIS_A>;
impl CWFST_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CWFST_DIS_A {
        match self.bits {
            true => CWFST_DIS_A::DIS,
            false => CWFST_DIS_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CWFST_DIS_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CWFST_DIS_A::EN
    }
}
#[doc = "Write proxy for field `CWFST_DIS`"]
pub struct CWFST_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CWFST_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWFST_DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Critical word first and streaming disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CWFST_DIS_A::DIS)
    }
    #[doc = "Critical word first and streaming enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CWFST_DIS_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_RDY_A {
    #[doc = "0: Not Ready."]
    NOTREADY = 0,
    #[doc = "1: Ready."]
    READY = 1,
}
impl From<CACHE_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: CACHE_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHE_RDY`"]
pub type CACHE_RDY_R = crate::R<bool, CACHE_RDY_A>;
impl CACHE_RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_RDY_A {
        match self.bits {
            false => CACHE_RDY_A::NOTREADY,
            true => CACHE_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == CACHE_RDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CACHE_RDY_A::READY
    }
}
#[doc = "Write proxy for field `CACHE_RDY`"]
pub struct CACHE_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHE_RDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Ready."]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(CACHE_RDY_A::NOTREADY)
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(CACHE_RDY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
    #[inline(always)]
    pub fn cache_en(&self) -> CACHE_EN_R {
        CACHE_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Allocate Enable. This bit only writable while the cache is disabled."]
    #[inline(always)]
    pub fn write_alloc_en(&self) -> WRITE_ALLOC_EN_R {
        WRITE_ALLOC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Critical word first and streaming disable. This bit only writeable while the cache is disabled."]
    #[inline(always)]
    pub fn cwfst_dis(&self) -> CWFST_DIS_R {
        CWFST_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready."]
    #[inline(always)]
    pub fn cache_rdy(&self) -> CACHE_RDY_R {
        CACHE_RDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
    #[inline(always)]
    pub fn cache_en(&mut self) -> CACHE_EN_W {
        CACHE_EN_W { w: self }
    }
    #[doc = "Bit 1 - Write Allocate Enable. This bit only writable while the cache is disabled."]
    #[inline(always)]
    pub fn write_alloc_en(&mut self) -> WRITE_ALLOC_EN_W {
        WRITE_ALLOC_EN_W { w: self }
    }
    #[doc = "Bit 2 - Critical word first and streaming disable. This bit only writeable while the cache is disabled."]
    #[inline(always)]
    pub fn cwfst_dis(&mut self) -> CWFST_DIS_W {
        CWFST_DIS_W { w: self }
    }
    #[doc = "Bit 16 - Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready."]
    #[inline(always)]
    pub fn cache_rdy(&mut self) -> CACHE_RDY_W {
        CACHE_RDY_W { w: self }
    }
}
