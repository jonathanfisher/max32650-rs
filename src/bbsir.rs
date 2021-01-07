#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - System Init. Configuration Register 2."]
    pub bb_sir2: BB_SIR2,
    #[doc = "0x0c - System Init. Configuration Register 3."]
    pub bb_sir3: BB_SIR3,
}
#[doc = "System Init. Configuration Register 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_sir2](bb_sir2) module"]
pub type BB_SIR2 = crate::Reg<u32, _BB_SIR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_SIR2;
#[doc = "`read()` method returns [bb_sir2::R](bb_sir2::R) reader structure"]
impl crate::Readable for BB_SIR2 {}
#[doc = "System Init. Configuration Register 2."]
pub mod bb_sir2;
#[doc = "System Init. Configuration Register 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_sir3](bb_sir3) module"]
pub type BB_SIR3 = crate::Reg<u32, _BB_SIR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_SIR3;
#[doc = "`read()` method returns [bb_sir3::R](bb_sir3::R) reader structure"]
impl crate::Readable for BB_SIR3 {}
#[doc = "System Init. Configuration Register 3."]
pub mod bb_sir3;
