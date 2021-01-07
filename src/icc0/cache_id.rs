#[doc = "Reader of register CACHE_ID"]
pub type R = crate::R<u32, super::CACHE_ID>;
#[doc = "Reader of field `RELNUM`"]
pub type RELNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `PARTNUM`"]
pub type PARTNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `CCHID`"]
pub type CCHID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Release Number. Identifies the RTL release version."]
    #[inline(always)]
    pub fn relnum(&self) -> RELNUM_R {
        RELNUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - Part Number. This field reflects the value of C_ID_PART_NUMBER configuration parameter."]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:15 - Cache ID. This field reflects the value of the C_ID_CACHEID configuration parameter."]
    #[inline(always)]
    pub fn cchid(&self) -> CCHID_R {
        CCHID_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
