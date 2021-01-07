#[doc = "Reader of register MAX_CURR_CFG"]
pub type R = crate::R<u32, super::MAX_CURR_CFG>;
#[doc = "Reader of field `V3_3`"]
pub type V3_3_R = crate::R<u8, u8>;
#[doc = "Reader of field `V3_0`"]
pub type V3_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `V1_8`"]
pub type V1_8_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V."]
    #[inline(always)]
    pub fn v3_3(&self) -> V3_3_R {
        V3_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V."]
    #[inline(always)]
    pub fn v3_0(&self) -> V3_0_R {
        V3_0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V."]
    #[inline(always)]
    pub fn v1_8(&self) -> V1_8_R {
        V1_8_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
