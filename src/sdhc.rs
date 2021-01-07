#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMA System Address / Argument 2."]
    pub sdma: SDMA,
    #[doc = "0x04 - Block Size."]
    pub blk_size: BLK_SIZE,
    #[doc = "0x06 - Block Count."]
    pub blk_cnt: BLK_CNT,
    #[doc = "0x08 - Argument 1."]
    pub arg_1: ARG_1,
    #[doc = "0x0c - Transfer Mode."]
    pub trans: TRANS,
    #[doc = "0x0e - Command."]
    pub cmd: CMD,
    #[doc = "0x10 - Response 0 Register 0-15."]
    pub resp: [RESP; 4],
    #[doc = "0x20 - Buffer Data Port."]
    pub buffer: BUFFER,
    #[doc = "0x24 - Present State."]
    pub present: PRESENT,
    #[doc = "0x28 - Host Control 1."]
    pub host_cn_1: HOST_CN_1,
    #[doc = "0x29 - Power Control."]
    pub pwr: PWR,
    #[doc = "0x2a - Block Gap Control."]
    pub blk_gap: BLK_GAP,
    #[doc = "0x2b - Wakeup Control."]
    pub wakeup: WAKEUP,
    #[doc = "0x2c - Clock Control."]
    pub clk_cn: CLK_CN,
    #[doc = "0x2e - Timeout Control."]
    pub to: TO,
    #[doc = "0x2f - Software Reset."]
    pub sw_reset: SW_RESET,
    #[doc = "0x30 - Normal Interrupt Status."]
    pub int_stat: INT_STAT,
    #[doc = "0x32 - Error Interrupt Status."]
    pub er_int_stat: ER_INT_STAT,
    #[doc = "0x34 - Normal Interrupt Status Enable."]
    pub int_en: INT_EN,
    #[doc = "0x36 - Error Interrupt Status Enable."]
    pub er_int_en: ER_INT_EN,
    #[doc = "0x38 - Normal Interrupt Signal Enable."]
    pub int_signal: INT_SIGNAL,
    #[doc = "0x3a - Error Interrupt Signal Enable."]
    pub er_int_signal: ER_INT_SIGNAL,
    #[doc = "0x3c - Auto CMD Error Status."]
    pub auto_cmd_er: AUTO_CMD_ER,
    #[doc = "0x3e - Host Control 2."]
    pub host_cn_2: HOST_CN_2,
    #[doc = "0x40 - Capabilities 0-31."]
    pub cfg_0: CFG_0,
    #[doc = "0x44 - Capabilities 32-63."]
    pub cfg_1: CFG_1,
    #[doc = "0x48 - Maximum Current Capabilities."]
    pub max_curr_cfg: MAX_CURR_CFG,
    _reserved27: [u8; 4usize],
    #[doc = "0x50 - Force Event for Auto CMD Error Status."]
    pub force_cmd: FORCE_CMD,
    #[doc = "0x52 - Force Event for Error Interrupt Status."]
    pub force_event_int_stat: FORCE_EVENT_INT_STAT,
    #[doc = "0x54 - ADMA Error Status."]
    pub adma_er: ADMA_ER,
    _reserved30: [u8; 3usize],
    #[doc = "0x58 - ADMA System Address 0-31."]
    pub adma_addr_0: ADMA_ADDR_0,
    #[doc = "0x5c - ADMA System Address 32-63."]
    pub adma_addr_1: ADMA_ADDR_1,
    #[doc = "0x60 - Preset Value for Initialization."]
    pub preset_0: PRESET_0,
    #[doc = "0x62 - Preset Value for Default Speed."]
    pub preset_1: PRESET_1,
    #[doc = "0x64 - Preset Value for High Speed."]
    pub preset_2: PRESET_2,
    #[doc = "0x66 - Preset Value for SDR12."]
    pub preset_3: PRESET_3,
    #[doc = "0x68 - Preset Value for SDR25."]
    pub preset_4: PRESET_4,
    #[doc = "0x6a - Preset Value for SDR50."]
    pub preset_5: PRESET_5,
    #[doc = "0x6c - Preset Value for SDR104."]
    pub preset_6: PRESET_6,
    #[doc = "0x6e - Preset Value for DDR50."]
    pub preset_7: PRESET_7,
    _reserved40: [u8; 112usize],
    #[doc = "0xe0 - Shared Bus Control."]
    pub shared_bus: SHARED_BUS,
    _reserved41: [u8; 24usize],
    #[doc = "0xfc - Slot Interrupt Status."]
    pub slot_int: SLOT_INT,
    #[doc = "0xfe - Host Controller Version."]
    pub host_cn_ver: HOST_CN_VER,
}
#[doc = "SDMA System Address / Argument 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdma](sdma) module"]
pub type SDMA = crate::Reg<u32, _SDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMA;
#[doc = "`read()` method returns [sdma::R](sdma::R) reader structure"]
impl crate::Readable for SDMA {}
#[doc = "`write(|w| ..)` method takes [sdma::W](sdma::W) writer structure"]
impl crate::Writable for SDMA {}
#[doc = "SDMA System Address / Argument 2."]
pub mod sdma;
#[doc = "Block Size.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_size](blk_size) module"]
pub type BLK_SIZE = crate::Reg<u16, _BLK_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK_SIZE;
#[doc = "`read()` method returns [blk_size::R](blk_size::R) reader structure"]
impl crate::Readable for BLK_SIZE {}
#[doc = "`write(|w| ..)` method takes [blk_size::W](blk_size::W) writer structure"]
impl crate::Writable for BLK_SIZE {}
#[doc = "Block Size."]
pub mod blk_size;
#[doc = "Block Count.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_cnt](blk_cnt) module"]
pub type BLK_CNT = crate::Reg<u16, _BLK_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK_CNT;
#[doc = "`read()` method returns [blk_cnt::R](blk_cnt::R) reader structure"]
impl crate::Readable for BLK_CNT {}
#[doc = "`write(|w| ..)` method takes [blk_cnt::W](blk_cnt::W) writer structure"]
impl crate::Writable for BLK_CNT {}
#[doc = "Block Count."]
pub mod blk_cnt;
#[doc = "Argument 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arg_1](arg_1) module"]
pub type ARG_1 = crate::Reg<u32, _ARG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARG_1;
#[doc = "`read()` method returns [arg_1::R](arg_1::R) reader structure"]
impl crate::Readable for ARG_1 {}
#[doc = "`write(|w| ..)` method takes [arg_1::W](arg_1::W) writer structure"]
impl crate::Writable for ARG_1 {}
#[doc = "Argument 1."]
pub mod arg_1;
#[doc = "Transfer Mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trans](trans) module"]
pub type TRANS = crate::Reg<u16, _TRANS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRANS;
#[doc = "`read()` method returns [trans::R](trans::R) reader structure"]
impl crate::Readable for TRANS {}
#[doc = "`write(|w| ..)` method takes [trans::W](trans::W) writer structure"]
impl crate::Writable for TRANS {}
#[doc = "Transfer Mode."]
pub mod trans;
#[doc = "Command.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u16, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command."]
pub mod cmd;
#[doc = "Response 0 Register 0-15.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp](resp) module"]
pub type RESP = crate::Reg<u32, _RESP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP;
#[doc = "`read()` method returns [resp::R](resp::R) reader structure"]
impl crate::Readable for RESP {}
#[doc = "`write(|w| ..)` method takes [resp::W](resp::W) writer structure"]
impl crate::Writable for RESP {}
#[doc = "Response 0 Register 0-15."]
pub mod resp;
#[doc = "Buffer Data Port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buffer](buffer) module"]
pub type BUFFER = crate::Reg<u32, _BUFFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFFER;
#[doc = "`read()` method returns [buffer::R](buffer::R) reader structure"]
impl crate::Readable for BUFFER {}
#[doc = "`write(|w| ..)` method takes [buffer::W](buffer::W) writer structure"]
impl crate::Writable for BUFFER {}
#[doc = "Buffer Data Port."]
pub mod buffer;
#[doc = "Present State.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [present](present) module"]
pub type PRESENT = crate::Reg<u32, _PRESENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESENT;
#[doc = "`read()` method returns [present::R](present::R) reader structure"]
impl crate::Readable for PRESENT {}
#[doc = "Present State."]
pub mod present;
#[doc = "Host Control 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_cn_1](host_cn_1) module"]
pub type HOST_CN_1 = crate::Reg<u8, _HOST_CN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CN_1;
#[doc = "`read()` method returns [host_cn_1::R](host_cn_1::R) reader structure"]
impl crate::Readable for HOST_CN_1 {}
#[doc = "`write(|w| ..)` method takes [host_cn_1::W](host_cn_1::W) writer structure"]
impl crate::Writable for HOST_CN_1 {}
#[doc = "Host Control 1."]
pub mod host_cn_1;
#[doc = "Power Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr](pwr) module"]
pub type PWR = crate::Reg<u8, _PWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR;
#[doc = "`read()` method returns [pwr::R](pwr::R) reader structure"]
impl crate::Readable for PWR {}
#[doc = "`write(|w| ..)` method takes [pwr::W](pwr::W) writer structure"]
impl crate::Writable for PWR {}
#[doc = "Power Control."]
pub mod pwr;
#[doc = "Block Gap Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_gap](blk_gap) module"]
pub type BLK_GAP = crate::Reg<u8, _BLK_GAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK_GAP;
#[doc = "`read()` method returns [blk_gap::R](blk_gap::R) reader structure"]
impl crate::Readable for BLK_GAP {}
#[doc = "`write(|w| ..)` method takes [blk_gap::W](blk_gap::W) writer structure"]
impl crate::Writable for BLK_GAP {}
#[doc = "Block Gap Control."]
pub mod blk_gap;
#[doc = "Wakeup Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup](wakeup) module"]
pub type WAKEUP = crate::Reg<u8, _WAKEUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEUP;
#[doc = "`read()` method returns [wakeup::R](wakeup::R) reader structure"]
impl crate::Readable for WAKEUP {}
#[doc = "`write(|w| ..)` method takes [wakeup::W](wakeup::W) writer structure"]
impl crate::Writable for WAKEUP {}
#[doc = "Wakeup Control."]
pub mod wakeup;
#[doc = "Clock Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cn](clk_cn) module"]
pub type CLK_CN = crate::Reg<u16, _CLK_CN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CN;
#[doc = "`read()` method returns [clk_cn::R](clk_cn::R) reader structure"]
impl crate::Readable for CLK_CN {}
#[doc = "`write(|w| ..)` method takes [clk_cn::W](clk_cn::W) writer structure"]
impl crate::Writable for CLK_CN {}
#[doc = "Clock Control."]
pub mod clk_cn;
#[doc = "Timeout Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [to](to) module"]
pub type TO = crate::Reg<u8, _TO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TO;
#[doc = "`read()` method returns [to::R](to::R) reader structure"]
impl crate::Readable for TO {}
#[doc = "`write(|w| ..)` method takes [to::W](to::W) writer structure"]
impl crate::Writable for TO {}
#[doc = "Timeout Control."]
pub mod to;
#[doc = "Software Reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_reset](sw_reset) module"]
pub type SW_RESET = crate::Reg<u8, _SW_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_RESET;
#[doc = "`read()` method returns [sw_reset::R](sw_reset::R) reader structure"]
impl crate::Readable for SW_RESET {}
#[doc = "`write(|w| ..)` method takes [sw_reset::W](sw_reset::W) writer structure"]
impl crate::Writable for SW_RESET {}
#[doc = "Software Reset."]
pub mod sw_reset;
#[doc = "Normal Interrupt Status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_stat](int_stat) module"]
pub type INT_STAT = crate::Reg<u16, _INT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STAT;
#[doc = "`read()` method returns [int_stat::R](int_stat::R) reader structure"]
impl crate::Readable for INT_STAT {}
#[doc = "`write(|w| ..)` method takes [int_stat::W](int_stat::W) writer structure"]
impl crate::Writable for INT_STAT {}
#[doc = "Normal Interrupt Status."]
pub mod int_stat;
#[doc = "Error Interrupt Status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [er_int_stat](er_int_stat) module"]
pub type ER_INT_STAT = crate::Reg<u16, _ER_INT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ER_INT_STAT;
#[doc = "`read()` method returns [er_int_stat::R](er_int_stat::R) reader structure"]
impl crate::Readable for ER_INT_STAT {}
#[doc = "`write(|w| ..)` method takes [er_int_stat::W](er_int_stat::W) writer structure"]
impl crate::Writable for ER_INT_STAT {}
#[doc = "Error Interrupt Status."]
pub mod er_int_stat;
#[doc = "Normal Interrupt Status Enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](int_en) module"]
pub type INT_EN = crate::Reg<u16, _INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN;
#[doc = "`read()` method returns [int_en::R](int_en::R) reader structure"]
impl crate::Readable for INT_EN {}
#[doc = "`write(|w| ..)` method takes [int_en::W](int_en::W) writer structure"]
impl crate::Writable for INT_EN {}
#[doc = "Normal Interrupt Status Enable."]
pub mod int_en;
#[doc = "Error Interrupt Status Enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [er_int_en](er_int_en) module"]
pub type ER_INT_EN = crate::Reg<u16, _ER_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ER_INT_EN;
#[doc = "`read()` method returns [er_int_en::R](er_int_en::R) reader structure"]
impl crate::Readable for ER_INT_EN {}
#[doc = "`write(|w| ..)` method takes [er_int_en::W](er_int_en::W) writer structure"]
impl crate::Writable for ER_INT_EN {}
#[doc = "Error Interrupt Status Enable."]
pub mod er_int_en;
#[doc = "Normal Interrupt Signal Enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_signal](int_signal) module"]
pub type INT_SIGNAL = crate::Reg<u16, _INT_SIGNAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_SIGNAL;
#[doc = "`read()` method returns [int_signal::R](int_signal::R) reader structure"]
impl crate::Readable for INT_SIGNAL {}
#[doc = "`write(|w| ..)` method takes [int_signal::W](int_signal::W) writer structure"]
impl crate::Writable for INT_SIGNAL {}
#[doc = "Normal Interrupt Signal Enable."]
pub mod int_signal;
#[doc = "Error Interrupt Signal Enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [er_int_signal](er_int_signal) module"]
pub type ER_INT_SIGNAL = crate::Reg<u16, _ER_INT_SIGNAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ER_INT_SIGNAL;
#[doc = "`read()` method returns [er_int_signal::R](er_int_signal::R) reader structure"]
impl crate::Readable for ER_INT_SIGNAL {}
#[doc = "`write(|w| ..)` method takes [er_int_signal::W](er_int_signal::W) writer structure"]
impl crate::Writable for ER_INT_SIGNAL {}
#[doc = "Error Interrupt Signal Enable."]
pub mod er_int_signal;
#[doc = "Auto CMD Error Status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auto_cmd_er](auto_cmd_er) module"]
pub type AUTO_CMD_ER = crate::Reg<u16, _AUTO_CMD_ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTO_CMD_ER;
#[doc = "`read()` method returns [auto_cmd_er::R](auto_cmd_er::R) reader structure"]
impl crate::Readable for AUTO_CMD_ER {}
#[doc = "`write(|w| ..)` method takes [auto_cmd_er::W](auto_cmd_er::W) writer structure"]
impl crate::Writable for AUTO_CMD_ER {}
#[doc = "Auto CMD Error Status."]
pub mod auto_cmd_er;
#[doc = "Host Control 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_cn_2](host_cn_2) module"]
pub type HOST_CN_2 = crate::Reg<u16, _HOST_CN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CN_2;
#[doc = "`read()` method returns [host_cn_2::R](host_cn_2::R) reader structure"]
impl crate::Readable for HOST_CN_2 {}
#[doc = "`write(|w| ..)` method takes [host_cn_2::W](host_cn_2::W) writer structure"]
impl crate::Writable for HOST_CN_2 {}
#[doc = "Host Control 2."]
pub mod host_cn_2;
#[doc = "Capabilities 0-31.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_0](cfg_0) module"]
pub type CFG_0 = crate::Reg<u32, _CFG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_0;
#[doc = "`read()` method returns [cfg_0::R](cfg_0::R) reader structure"]
impl crate::Readable for CFG_0 {}
#[doc = "Capabilities 0-31."]
pub mod cfg_0;
#[doc = "Capabilities 32-63.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_1](cfg_1) module"]
pub type CFG_1 = crate::Reg<u32, _CFG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_1;
#[doc = "`read()` method returns [cfg_1::R](cfg_1::R) reader structure"]
impl crate::Readable for CFG_1 {}
#[doc = "Capabilities 32-63."]
pub mod cfg_1;
#[doc = "Maximum Current Capabilities.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_curr_cfg](max_curr_cfg) module"]
pub type MAX_CURR_CFG = crate::Reg<u32, _MAX_CURR_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAX_CURR_CFG;
#[doc = "`read()` method returns [max_curr_cfg::R](max_curr_cfg::R) reader structure"]
impl crate::Readable for MAX_CURR_CFG {}
#[doc = "Maximum Current Capabilities."]
pub mod max_curr_cfg;
#[doc = "Force Event for Auto CMD Error Status.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_cmd](force_cmd) module"]
pub type FORCE_CMD = crate::Reg<u16, _FORCE_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORCE_CMD;
#[doc = "`write(|w| ..)` method takes [force_cmd::W](force_cmd::W) writer structure"]
impl crate::Writable for FORCE_CMD {}
#[doc = "Force Event for Auto CMD Error Status."]
pub mod force_cmd;
#[doc = "Force Event for Error Interrupt Status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_event_int_stat](force_event_int_stat) module"]
pub type FORCE_EVENT_INT_STAT = crate::Reg<u16, _FORCE_EVENT_INT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORCE_EVENT_INT_STAT;
#[doc = "`read()` method returns [force_event_int_stat::R](force_event_int_stat::R) reader structure"]
impl crate::Readable for FORCE_EVENT_INT_STAT {}
#[doc = "`write(|w| ..)` method takes [force_event_int_stat::W](force_event_int_stat::W) writer structure"]
impl crate::Writable for FORCE_EVENT_INT_STAT {}
#[doc = "Force Event for Error Interrupt Status."]
pub mod force_event_int_stat;
#[doc = "ADMA Error Status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_er](adma_er) module"]
pub type ADMA_ER = crate::Reg<u8, _ADMA_ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMA_ER;
#[doc = "`read()` method returns [adma_er::R](adma_er::R) reader structure"]
impl crate::Readable for ADMA_ER {}
#[doc = "`write(|w| ..)` method takes [adma_er::W](adma_er::W) writer structure"]
impl crate::Writable for ADMA_ER {}
#[doc = "ADMA Error Status."]
pub mod adma_er;
#[doc = "ADMA System Address 0-31.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_addr_0](adma_addr_0) module"]
pub type ADMA_ADDR_0 = crate::Reg<u32, _ADMA_ADDR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMA_ADDR_0;
#[doc = "`read()` method returns [adma_addr_0::R](adma_addr_0::R) reader structure"]
impl crate::Readable for ADMA_ADDR_0 {}
#[doc = "`write(|w| ..)` method takes [adma_addr_0::W](adma_addr_0::W) writer structure"]
impl crate::Writable for ADMA_ADDR_0 {}
#[doc = "ADMA System Address 0-31."]
pub mod adma_addr_0;
#[doc = "ADMA System Address 32-63.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_addr_1](adma_addr_1) module"]
pub type ADMA_ADDR_1 = crate::Reg<u32, _ADMA_ADDR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMA_ADDR_1;
#[doc = "`read()` method returns [adma_addr_1::R](adma_addr_1::R) reader structure"]
impl crate::Readable for ADMA_ADDR_1 {}
#[doc = "`write(|w| ..)` method takes [adma_addr_1::W](adma_addr_1::W) writer structure"]
impl crate::Writable for ADMA_ADDR_1 {}
#[doc = "ADMA System Address 32-63."]
pub mod adma_addr_1;
#[doc = "Preset Value for Initialization.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_0](preset_0) module"]
pub type PRESET_0 = crate::Reg<u16, _PRESET_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESET_0;
#[doc = "`read()` method returns [preset_0::R](preset_0::R) reader structure"]
impl crate::Readable for PRESET_0 {}
#[doc = "Preset Value for Initialization."]
pub mod preset_0;
#[doc = "Preset Value for Default Speed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_1](preset_1) module"]
pub type PRESET_1 = crate::Reg<u16, _PRESET_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESET_1;
#[doc = "`read()` method returns [preset_1::R](preset_1::R) reader structure"]
impl crate::Readable for PRESET_1 {}
#[doc = "Preset Value for Default Speed."]
pub mod preset_1;
#[doc = "Preset Value for High Speed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_2](preset_2) module"]
pub type PRESET_2 = crate::Reg<u16, _PRESET_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESET_2;
#[doc = "`read()` method returns [preset_2::R](preset_2::R) reader structure"]
impl crate::Readable for PRESET_2 {}
#[doc = "Preset Value for High Speed."]
pub mod preset_2;
#[doc = "Preset Value for SDR12.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_3](preset_3) module"]
pub type PRESET_3 = crate::Reg<u16, _PRESET_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESET_3;
#[doc = "`read()` method returns [preset_3::R](preset_3::R) reader structure"]
impl crate::Readable for PRESET_3 {}
#[doc = "Preset Value for SDR12."]
pub mod preset_3;
#[doc = "Preset Value for SDR25.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_4](preset_4) module"]
pub type PRESET_4 = crate::Reg<u16, _PRESET_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESET_4;
#[doc = "`read()` method returns [preset_4::R](preset_4::R) reader structure"]
impl crate::Readable for PRESET_4 {}
#[doc = "Preset Value for SDR25."]
pub mod preset_4;
#[doc = "Preset Value for SDR50.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_5](preset_5) module"]
pub type PRESET_5 = crate::Reg<u16, _PRESET_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESET_5;
#[doc = "`read()` method returns [preset_5::R](preset_5::R) reader structure"]
impl crate::Readable for PRESET_5 {}
#[doc = "Preset Value for SDR50."]
pub mod preset_5;
#[doc = "Preset Value for SDR104.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_6](preset_6) module"]
pub type PRESET_6 = crate::Reg<u16, _PRESET_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESET_6;
#[doc = "`read()` method returns [preset_6::R](preset_6::R) reader structure"]
impl crate::Readable for PRESET_6 {}
#[doc = "Preset Value for SDR104."]
pub mod preset_6;
#[doc = "Preset Value for DDR50.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_7](preset_7) module"]
pub type PRESET_7 = crate::Reg<u16, _PRESET_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESET_7;
#[doc = "`read()` method returns [preset_7::R](preset_7::R) reader structure"]
impl crate::Readable for PRESET_7 {}
#[doc = "Preset Value for DDR50."]
pub mod preset_7;
#[doc = "Shared Bus Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shared_bus](shared_bus) module"]
pub type SHARED_BUS = crate::Reg<u32, _SHARED_BUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHARED_BUS;
#[doc = "`read()` method returns [shared_bus::R](shared_bus::R) reader structure"]
impl crate::Readable for SHARED_BUS {}
#[doc = "`write(|w| ..)` method takes [shared_bus::W](shared_bus::W) writer structure"]
impl crate::Writable for SHARED_BUS {}
#[doc = "Shared Bus Control."]
pub mod shared_bus;
#[doc = "Slot Interrupt Status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slot_int](slot_int) module"]
pub type SLOT_INT = crate::Reg<u16, _SLOT_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLOT_INT;
#[doc = "`read()` method returns [slot_int::R](slot_int::R) reader structure"]
impl crate::Readable for SLOT_INT {}
#[doc = "Slot Interrupt Status."]
pub mod slot_int;
#[doc = "Host Controller Version.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_cn_ver](host_cn_ver) module"]
pub type HOST_CN_VER = crate::Reg<u16, _HOST_CN_VER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CN_VER;
#[doc = "`read()` method returns [host_cn_ver::R](host_cn_ver::R) reader structure"]
impl crate::Readable for HOST_CN_VER {}
#[doc = "`write(|w| ..)` method takes [host_cn_ver::W](host_cn_ver::W) writer structure"]
impl crate::Writable for HOST_CN_VER {}
#[doc = "Host Controller Version."]
pub mod host_cn_ver;
