#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache ID Register."]
    pub cache_id: CACHE_ID,
    #[doc = "0x04 - Memory Configuration Register."]
    pub memcfg: MEMCFG,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - Cache Control and Status Register."]
    pub cache_ctrl: CACHE_CTRL,
    _reserved3: [u8; 1532usize],
    #[doc = "0x700 - Invalidate All Cache Contents. Any time this register location is written (regardless of the data value), the cache controller immediately begins invalidating the entire contents of the cache memory. The cache will be in bypass mode until the invalidate operation is complete. System software can examine the Cache Ready bit (CACHE_CTRL.CACHE_RDY) to determine when the invalidate operation is complete. Note that it is not necessary to disable the cache controller prior to beginning this operation. Reads from this register always return 0."]
    pub invalidate: INVALIDATE,
}
#[doc = "Cache ID Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_id](cache_id) module"]
pub type CACHE_ID = crate::Reg<u32, _CACHE_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHE_ID;
#[doc = "`read()` method returns [cache_id::R](cache_id::R) reader structure"]
impl crate::Readable for CACHE_ID {}
#[doc = "Cache ID Register."]
pub mod cache_id;
#[doc = "Memory Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memcfg](memcfg) module"]
pub type MEMCFG = crate::Reg<u32, _MEMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMCFG;
#[doc = "`read()` method returns [memcfg::R](memcfg::R) reader structure"]
impl crate::Readable for MEMCFG {}
#[doc = "Memory Configuration Register."]
pub mod memcfg;
#[doc = "Cache Control and Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ctrl](cache_ctrl) module"]
pub type CACHE_CTRL = crate::Reg<u32, _CACHE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHE_CTRL;
#[doc = "`read()` method returns [cache_ctrl::R](cache_ctrl::R) reader structure"]
impl crate::Readable for CACHE_CTRL {}
#[doc = "`write(|w| ..)` method takes [cache_ctrl::W](cache_ctrl::W) writer structure"]
impl crate::Writable for CACHE_CTRL {}
#[doc = "Cache Control and Status Register."]
pub mod cache_ctrl;
#[doc = "Invalidate All Cache Contents. Any time this register location is written (regardless of the data value), the cache controller immediately begins invalidating the entire contents of the cache memory. The cache will be in bypass mode until the invalidate operation is complete. System software can examine the Cache Ready bit (CACHE_CTRL.CACHE_RDY) to determine when the invalidate operation is complete. Note that it is not necessary to disable the cache controller prior to beginning this operation. Reads from this register always return 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [invalidate](invalidate) module"]
pub type INVALIDATE = crate::Reg<u32, _INVALIDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INVALIDATE;
#[doc = "`read()` method returns [invalidate::R](invalidate::R) reader structure"]
impl crate::Readable for INVALIDATE {}
#[doc = "`write(|w| ..)` method takes [invalidate::W](invalidate::W) writer structure"]
impl crate::Writable for INVALIDATE {}
#[doc = "Invalidate All Cache Contents. Any time this register location is written (regardless of the data value), the cache controller immediately begins invalidating the entire contents of the cache memory. The cache will be in bypass mode until the invalidate operation is complete. System software can examine the Cache Ready bit (CACHE_CTRL.CACHE_RDY) to determine when the invalidate operation is complete. Note that it is not necessary to disable the cache controller prior to beginning this operation. Reads from this register always return 0."]
pub mod invalidate;
