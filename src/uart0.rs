#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Threshold Control register."]
    pub thresh_ctrl: THRESH_CTRL,
    #[doc = "0x08 - Status Register."]
    pub status: STATUS,
    #[doc = "0x0c - Interrupt Enable Register."]
    pub int_en: INT_EN,
    #[doc = "0x10 - Interrupt Status Flags."]
    pub int_fl: INT_FL,
    #[doc = "0x14 - Baud rate register. Integer portion."]
    pub baud0: BAUD0,
    #[doc = "0x18 - Baud rate register. Decimal Setting."]
    pub baud1: BAUD1,
    #[doc = "0x1c - FIFO Data buffer."]
    pub fifo: FIFO,
    #[doc = "0x20 - DMA Configuration."]
    pub dma: DMA,
    #[doc = "0x24 - Transmit FIFO Status register."]
    pub tx_fifo: TX_FIFO,
}
#[doc = "Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register."]
pub mod ctrl;
#[doc = "Threshold Control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thresh_ctrl](thresh_ctrl) module"]
pub type THRESH_CTRL = crate::Reg<u32, _THRESH_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THRESH_CTRL;
#[doc = "`read()` method returns [thresh_ctrl::R](thresh_ctrl::R) reader structure"]
impl crate::Readable for THRESH_CTRL {}
#[doc = "`write(|w| ..)` method takes [thresh_ctrl::W](thresh_ctrl::W) writer structure"]
impl crate::Writable for THRESH_CTRL {}
#[doc = "Threshold Control register."]
pub mod thresh_ctrl;
#[doc = "Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register."]
pub mod status;
#[doc = "Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](int_en) module"]
pub type INT_EN = crate::Reg<u32, _INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN;
#[doc = "`read()` method returns [int_en::R](int_en::R) reader structure"]
impl crate::Readable for INT_EN {}
#[doc = "`write(|w| ..)` method takes [int_en::W](int_en::W) writer structure"]
impl crate::Writable for INT_EN {}
#[doc = "Interrupt Enable Register."]
pub mod int_en;
#[doc = "Interrupt Status Flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_fl](int_fl) module"]
pub type INT_FL = crate::Reg<u32, _INT_FL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_FL;
#[doc = "`read()` method returns [int_fl::R](int_fl::R) reader structure"]
impl crate::Readable for INT_FL {}
#[doc = "`write(|w| ..)` method takes [int_fl::W](int_fl::W) writer structure"]
impl crate::Writable for INT_FL {}
#[doc = "Interrupt Status Flags."]
pub mod int_fl;
#[doc = "Baud rate register. Integer portion.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud0](baud0) module"]
pub type BAUD0 = crate::Reg<u32, _BAUD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD0;
#[doc = "`read()` method returns [baud0::R](baud0::R) reader structure"]
impl crate::Readable for BAUD0 {}
#[doc = "`write(|w| ..)` method takes [baud0::W](baud0::W) writer structure"]
impl crate::Writable for BAUD0 {}
#[doc = "Baud rate register. Integer portion."]
pub mod baud0;
#[doc = "Baud rate register. Decimal Setting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud1](baud1) module"]
pub type BAUD1 = crate::Reg<u32, _BAUD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD1;
#[doc = "`read()` method returns [baud1::R](baud1::R) reader structure"]
impl crate::Readable for BAUD1 {}
#[doc = "`write(|w| ..)` method takes [baud1::W](baud1::W) writer structure"]
impl crate::Writable for BAUD1 {}
#[doc = "Baud rate register. Decimal Setting."]
pub mod baud1;
#[doc = "FIFO Data buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "FIFO Data buffer."]
pub mod fifo;
#[doc = "DMA Configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](dma) module"]
pub type DMA = crate::Reg<u32, _DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA;
#[doc = "`read()` method returns [dma::R](dma::R) reader structure"]
impl crate::Readable for DMA {}
#[doc = "`write(|w| ..)` method takes [dma::W](dma::W) writer structure"]
impl crate::Writable for DMA {}
#[doc = "DMA Configuration."]
pub mod dma;
#[doc = "Transmit FIFO Status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo](tx_fifo) module"]
pub type TX_FIFO = crate::Reg<u32, _TX_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_FIFO;
#[doc = "`read()` method returns [tx_fifo::R](tx_fifo::R) reader structure"]
impl crate::Readable for TX_FIFO {}
#[doc = "`write(|w| ..)` method takes [tx_fifo::W](tx_fifo::W) writer structure"]
impl crate::Writable for TX_FIFO {}
#[doc = "Transmit FIFO Status register."]
pub mod tx_fifo;
