#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Key 0"]
    pub aes_key0: AES_KEY0,
    _reserved1: [u8; 124usize],
    #[doc = "0x80 - AES Key 1"]
    pub aes_key1: AES_KEY1,
    _reserved2: [u8; 124usize],
    #[doc = "0x100 - AES Key 2"]
    pub aes_key2: AES_KEY2,
    _reserved3: [u8; 124usize],
    #[doc = "0x180 - AES Key 3"]
    pub aes_key3: AES_KEY3,
}
#[doc = "AES Key 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key0](aes_key0) module"]
pub type AES_KEY0 = crate::Reg<u32, _AES_KEY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY0;
#[doc = "`read()` method returns [aes_key0::R](aes_key0::R) reader structure"]
impl crate::Readable for AES_KEY0 {}
#[doc = "`write(|w| ..)` method takes [aes_key0::W](aes_key0::W) writer structure"]
impl crate::Writable for AES_KEY0 {}
#[doc = "AES Key 0"]
pub mod aes_key0;
#[doc = "AES Key 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key1](aes_key1) module"]
pub type AES_KEY1 = crate::Reg<u32, _AES_KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY1;
#[doc = "`read()` method returns [aes_key1::R](aes_key1::R) reader structure"]
impl crate::Readable for AES_KEY1 {}
#[doc = "`write(|w| ..)` method takes [aes_key1::W](aes_key1::W) writer structure"]
impl crate::Writable for AES_KEY1 {}
#[doc = "AES Key 1"]
pub mod aes_key1;
#[doc = "AES Key 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key2](aes_key2) module"]
pub type AES_KEY2 = crate::Reg<u32, _AES_KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY2;
#[doc = "`read()` method returns [aes_key2::R](aes_key2::R) reader structure"]
impl crate::Readable for AES_KEY2 {}
#[doc = "`write(|w| ..)` method takes [aes_key2::W](aes_key2::W) writer structure"]
impl crate::Writable for AES_KEY2 {}
#[doc = "AES Key 2"]
pub mod aes_key2;
#[doc = "AES Key 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key3](aes_key3) module"]
pub type AES_KEY3 = crate::Reg<u32, _AES_KEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY3;
#[doc = "`read()` method returns [aes_key3::R](aes_key3::R) reader structure"]
impl crate::Readable for AES_KEY3 {}
#[doc = "`write(|w| ..)` method takes [aes_key3::W](aes_key3::W) writer structure"]
impl crate::Writable for AES_KEY3 {}
#[doc = "AES Key 3"]
pub mod aes_key3;
