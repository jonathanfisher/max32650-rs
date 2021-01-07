#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register0."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register."]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Status Register."]
    pub int_fl0: INT_FL0,
    #[doc = "0x0c - Interrupt Enable Register."]
    pub int_en0: INT_EN0,
    #[doc = "0x10 - Interrupt Status Register 1."]
    pub int_fl1: INT_FL1,
    #[doc = "0x14 - Interrupt Staus Register 1."]
    pub int_en1: INT_EN1,
    #[doc = "0x18 - FIFO Configuration Register."]
    pub fifo_len: FIFO_LEN,
    #[doc = "0x1c - Receive Control Register 0."]
    pub rx_ctrl0: RX_CTRL0,
    #[doc = "0x20 - Receive Control Register 1."]
    pub rx_ctrl1: RX_CTRL1,
    #[doc = "0x24 - Transmit Control Register 0."]
    pub tx_ctrl0: TX_CTRL0,
    #[doc = "0x28 - Transmit Control Register 1."]
    pub tx_ctrl1: TX_CTRL1,
    #[doc = "0x2c - Data Register."]
    pub fifo: FIFO,
    #[doc = "0x30 - Master Control Register."]
    pub master_ctrl: MASTER_CTRL,
    #[doc = "0x34 - Clock Low Register."]
    pub clk_lo: CLK_LO,
    #[doc = "0x38 - Clock high Register."]
    pub clk_hi: CLK_HI,
    #[doc = "0x3c - HS-Mode Clock Control Register"]
    pub hs_clk: HS_CLK,
    #[doc = "0x40 - Timeout Register"]
    pub timeout: TIMEOUT,
    #[doc = "0x44 - Slave Address Register."]
    pub slave_addr: SLAVE_ADDR,
    #[doc = "0x48 - DMA Register."]
    pub dma: DMA,
}
#[doc = "Control Register0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register0."]
pub mod ctrl;
#[doc = "Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status Register."]
pub mod status;
#[doc = "Interrupt Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_fl0](int_fl0) module"]
pub type INT_FL0 = crate::Reg<u32, _INT_FL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_FL0;
#[doc = "`read()` method returns [int_fl0::R](int_fl0::R) reader structure"]
impl crate::Readable for INT_FL0 {}
#[doc = "`write(|w| ..)` method takes [int_fl0::W](int_fl0::W) writer structure"]
impl crate::Writable for INT_FL0 {}
#[doc = "Interrupt Status Register."]
pub mod int_fl0;
#[doc = "Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en0](int_en0) module"]
pub type INT_EN0 = crate::Reg<u32, _INT_EN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN0;
#[doc = "`read()` method returns [int_en0::R](int_en0::R) reader structure"]
impl crate::Readable for INT_EN0 {}
#[doc = "`write(|w| ..)` method takes [int_en0::W](int_en0::W) writer structure"]
impl crate::Writable for INT_EN0 {}
#[doc = "Interrupt Enable Register."]
pub mod int_en0;
#[doc = "Interrupt Status Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_fl1](int_fl1) module"]
pub type INT_FL1 = crate::Reg<u32, _INT_FL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_FL1;
#[doc = "`read()` method returns [int_fl1::R](int_fl1::R) reader structure"]
impl crate::Readable for INT_FL1 {}
#[doc = "`write(|w| ..)` method takes [int_fl1::W](int_fl1::W) writer structure"]
impl crate::Writable for INT_FL1 {}
#[doc = "Interrupt Status Register 1."]
pub mod int_fl1;
#[doc = "Interrupt Staus Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en1](int_en1) module"]
pub type INT_EN1 = crate::Reg<u32, _INT_EN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN1;
#[doc = "`read()` method returns [int_en1::R](int_en1::R) reader structure"]
impl crate::Readable for INT_EN1 {}
#[doc = "`write(|w| ..)` method takes [int_en1::W](int_en1::W) writer structure"]
impl crate::Writable for INT_EN1 {}
#[doc = "Interrupt Staus Register 1."]
pub mod int_en1;
#[doc = "FIFO Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_len](fifo_len) module"]
pub type FIFO_LEN = crate::Reg<u32, _FIFO_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_LEN;
#[doc = "`read()` method returns [fifo_len::R](fifo_len::R) reader structure"]
impl crate::Readable for FIFO_LEN {}
#[doc = "FIFO Configuration Register."]
pub mod fifo_len;
#[doc = "Receive Control Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ctrl0](rx_ctrl0) module"]
pub type RX_CTRL0 = crate::Reg<u32, _RX_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CTRL0;
#[doc = "`read()` method returns [rx_ctrl0::R](rx_ctrl0::R) reader structure"]
impl crate::Readable for RX_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [rx_ctrl0::W](rx_ctrl0::W) writer structure"]
impl crate::Writable for RX_CTRL0 {}
#[doc = "Receive Control Register 0."]
pub mod rx_ctrl0;
#[doc = "Receive Control Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ctrl1](rx_ctrl1) module"]
pub type RX_CTRL1 = crate::Reg<u32, _RX_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CTRL1;
#[doc = "`read()` method returns [rx_ctrl1::R](rx_ctrl1::R) reader structure"]
impl crate::Readable for RX_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [rx_ctrl1::W](rx_ctrl1::W) writer structure"]
impl crate::Writable for RX_CTRL1 {}
#[doc = "Receive Control Register 1."]
pub mod rx_ctrl1;
#[doc = "Transmit Control Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl0](tx_ctrl0) module"]
pub type TX_CTRL0 = crate::Reg<u32, _TX_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CTRL0;
#[doc = "`read()` method returns [tx_ctrl0::R](tx_ctrl0::R) reader structure"]
impl crate::Readable for TX_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [tx_ctrl0::W](tx_ctrl0::W) writer structure"]
impl crate::Writable for TX_CTRL0 {}
#[doc = "Transmit Control Register 0."]
pub mod tx_ctrl0;
#[doc = "Transmit Control Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl1](tx_ctrl1) module"]
pub type TX_CTRL1 = crate::Reg<u32, _TX_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CTRL1;
#[doc = "`read()` method returns [tx_ctrl1::R](tx_ctrl1::R) reader structure"]
impl crate::Readable for TX_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [tx_ctrl1::W](tx_ctrl1::W) writer structure"]
impl crate::Writable for TX_CTRL1 {}
#[doc = "Transmit Control Register 1."]
pub mod tx_ctrl1;
#[doc = "Data Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "Data Register."]
pub mod fifo;
#[doc = "Master Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master_ctrl](master_ctrl) module"]
pub type MASTER_CTRL = crate::Reg<u32, _MASTER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASTER_CTRL;
#[doc = "`read()` method returns [master_ctrl::R](master_ctrl::R) reader structure"]
impl crate::Readable for MASTER_CTRL {}
#[doc = "`write(|w| ..)` method takes [master_ctrl::W](master_ctrl::W) writer structure"]
impl crate::Writable for MASTER_CTRL {}
#[doc = "Master Control Register."]
pub mod master_ctrl;
#[doc = "Clock Low Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_lo](clk_lo) module"]
pub type CLK_LO = crate::Reg<u32, _CLK_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_LO;
#[doc = "`read()` method returns [clk_lo::R](clk_lo::R) reader structure"]
impl crate::Readable for CLK_LO {}
#[doc = "`write(|w| ..)` method takes [clk_lo::W](clk_lo::W) writer structure"]
impl crate::Writable for CLK_LO {}
#[doc = "Clock Low Register."]
pub mod clk_lo;
#[doc = "Clock high Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_hi](clk_hi) module"]
pub type CLK_HI = crate::Reg<u32, _CLK_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_HI;
#[doc = "`read()` method returns [clk_hi::R](clk_hi::R) reader structure"]
impl crate::Readable for CLK_HI {}
#[doc = "`write(|w| ..)` method takes [clk_hi::W](clk_hi::W) writer structure"]
impl crate::Writable for CLK_HI {}
#[doc = "Clock high Register."]
pub mod clk_hi;
#[doc = "HS-Mode Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs_clk](hs_clk) module"]
pub type HS_CLK = crate::Reg<u32, _HS_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HS_CLK;
#[doc = "`read()` method returns [hs_clk::R](hs_clk::R) reader structure"]
impl crate::Readable for HS_CLK {}
#[doc = "`write(|w| ..)` method takes [hs_clk::W](hs_clk::W) writer structure"]
impl crate::Writable for HS_CLK {}
#[doc = "HS-Mode Clock Control Register"]
pub mod hs_clk;
#[doc = "Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout](timeout) module"]
pub type TIMEOUT = crate::Reg<u32, _TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUT;
#[doc = "`read()` method returns [timeout::R](timeout::R) reader structure"]
impl crate::Readable for TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [timeout::W](timeout::W) writer structure"]
impl crate::Writable for TIMEOUT {}
#[doc = "Timeout Register"]
pub mod timeout;
#[doc = "Slave Address Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_addr](slave_addr) module"]
pub type SLAVE_ADDR = crate::Reg<u32, _SLAVE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE_ADDR;
#[doc = "`read()` method returns [slave_addr::R](slave_addr::R) reader structure"]
impl crate::Readable for SLAVE_ADDR {}
#[doc = "`write(|w| ..)` method takes [slave_addr::W](slave_addr::W) writer structure"]
impl crate::Writable for SLAVE_ADDR {}
#[doc = "Slave Address Register."]
pub mod slave_addr;
#[doc = "DMA Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](dma) module"]
pub type DMA = crate::Reg<u32, _DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA;
#[doc = "`read()` method returns [dma::R](dma::R) reader structure"]
impl crate::Readable for DMA {}
#[doc = "`write(|w| ..)` method takes [dma::W](dma::W) writer structure"]
impl crate::Writable for DMA {}
#[doc = "DMA Register."]
pub mod dma;
