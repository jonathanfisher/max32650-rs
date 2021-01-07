#[doc = "Reader of register MEMCFG"]
pub type R = crate::R<u32, super::MEMCFG>;
#[doc = "Reader of field `CCHSZ`"]
pub type CCHSZ_R = crate::R<u16, u16>;
#[doc = "Reader of field `MEMSZ`"]
pub type MEMSZ_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Cache Size. Indicates total size in Kbytes of cache."]
    #[inline(always)]
    pub fn cchsz(&self) -> CCHSZ_R {
        CCHSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Main Memory Size. Indicates the total size, in units of 128 Kbytes, of code memory accessible to the cache controller."]
    #[inline(always)]
    pub fn memsz(&self) -> MEMSZ_R {
        MEMSZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
