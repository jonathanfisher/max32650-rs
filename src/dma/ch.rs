#[doc = "DMA Channel Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "DMA Channel Configuration Register."]
pub mod cfg;
#[doc = "DMA Channel Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st](st) module"]
pub type ST = crate::Reg<u32, _ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST;
#[doc = "`read()` method returns [st::R](st::R) reader structure"]
impl crate::Readable for ST {}
#[doc = "`write(|w| ..)` method takes [st::W](st::W) writer structure"]
impl crate::Writable for ST {}
#[doc = "DMA Channel Status Register."]
pub mod st;
#[doc = "Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src](src) module"]
pub type SRC = crate::Reg<u32, _SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRC;
#[doc = "`read()` method returns [src::R](src::R) reader structure"]
impl crate::Readable for SRC {}
#[doc = "`write(|w| ..)` method takes [src::W](src::W) writer structure"]
impl crate::Writable for SRC {}
#[doc = "Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD."]
pub mod src;
#[doc = "Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst](dst) module"]
pub type DST = crate::Reg<u32, _DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DST;
#[doc = "`read()` method returns [dst::R](dst::R) reader structure"]
impl crate::Readable for DST {}
#[doc = "`write(|w| ..)` method takes [dst::W](dst::W) writer structure"]
impl crate::Writable for DST {}
#[doc = "Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD."]
pub mod dst;
#[doc = "DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered."]
pub mod cnt;
#[doc = "Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src_rld](src_rld) module"]
pub type SRC_RLD = crate::Reg<u32, _SRC_RLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRC_RLD;
#[doc = "`read()` method returns [src_rld::R](src_rld::R) reader structure"]
impl crate::Readable for SRC_RLD {}
#[doc = "`write(|w| ..)` method takes [src_rld::W](src_rld::W) writer structure"]
impl crate::Writable for SRC_RLD {}
#[doc = "Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition."]
pub mod src_rld;
#[doc = "Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_rld](dst_rld) module"]
pub type DST_RLD = crate::Reg<u32, _DST_RLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DST_RLD;
#[doc = "`read()` method returns [dst_rld::R](dst_rld::R) reader structure"]
impl crate::Readable for DST_RLD {}
#[doc = "`write(|w| ..)` method takes [dst_rld::W](dst_rld::W) writer structure"]
impl crate::Writable for DST_RLD {}
#[doc = "Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition."]
pub mod dst_rld;
#[doc = "DMA Channel Count Reload Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_rld](cnt_rld) module"]
pub type CNT_RLD = crate::Reg<u32, _CNT_RLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT_RLD;
#[doc = "`read()` method returns [cnt_rld::R](cnt_rld::R) reader structure"]
impl crate::Readable for CNT_RLD {}
#[doc = "`write(|w| ..)` method takes [cnt_rld::W](cnt_rld::W) writer structure"]
impl crate::Writable for CNT_RLD {}
#[doc = "DMA Channel Count Reload Register."]
pub mod cnt_rld;
