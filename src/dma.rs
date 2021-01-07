#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register."]
    pub cn: CN,
    #[doc = "0x04 - DMA Interrupt Register."]
    pub intr: INTR,
    _reserved2: [u8; 248usize],
    _reserved_2_ch0: [u8; 348usize],
}
impl RegisterBlock {
    #[doc = "0x100 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch0(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const CH) }
    }
    #[doc = "0x100 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch0_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut CH) }
    }
    #[doc = "0x104 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch1(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const CH) }
    }
    #[doc = "0x104 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch1_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(260usize) as *mut CH) }
    }
    #[doc = "0x108 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch2(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const CH) }
    }
    #[doc = "0x108 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch2_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(264usize) as *mut CH) }
    }
    #[doc = "0x10c - DMA Channel registers."]
    #[inline(always)]
    pub fn ch3(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const CH) }
    }
    #[doc = "0x10c - DMA Channel registers."]
    #[inline(always)]
    pub fn ch3_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(268usize) as *mut CH) }
    }
    #[doc = "0x110 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch4(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const CH) }
    }
    #[doc = "0x110 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch4_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(272usize) as *mut CH) }
    }
    #[doc = "0x114 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch5(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const CH) }
    }
    #[doc = "0x114 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch5_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(276usize) as *mut CH) }
    }
    #[doc = "0x118 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch6(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const CH) }
    }
    #[doc = "0x118 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch6_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(280usize) as *mut CH) }
    }
    #[doc = "0x11c - DMA Channel registers."]
    #[inline(always)]
    pub fn ch7(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const CH) }
    }
    #[doc = "0x11c - DMA Channel registers."]
    #[inline(always)]
    pub fn ch7_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(284usize) as *mut CH) }
    }
    #[doc = "0x120 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch8(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(288usize) as *const CH) }
    }
    #[doc = "0x120 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch8_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(288usize) as *mut CH) }
    }
    #[doc = "0x124 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch9(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(292usize) as *const CH) }
    }
    #[doc = "0x124 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch9_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(292usize) as *mut CH) }
    }
    #[doc = "0x128 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch10(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(296usize) as *const CH) }
    }
    #[doc = "0x128 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch10_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(296usize) as *mut CH) }
    }
    #[doc = "0x12c - DMA Channel registers."]
    #[inline(always)]
    pub fn ch11(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(300usize) as *const CH) }
    }
    #[doc = "0x12c - DMA Channel registers."]
    #[inline(always)]
    pub fn ch11_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(300usize) as *mut CH) }
    }
    #[doc = "0x130 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch12(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(304usize) as *const CH) }
    }
    #[doc = "0x130 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch12_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(304usize) as *mut CH) }
    }
    #[doc = "0x134 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch13(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(308usize) as *const CH) }
    }
    #[doc = "0x134 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch13_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(308usize) as *mut CH) }
    }
    #[doc = "0x138 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch14(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(312usize) as *const CH) }
    }
    #[doc = "0x138 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch14_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(312usize) as *mut CH) }
    }
    #[doc = "0x13c - DMA Channel registers."]
    #[inline(always)]
    pub fn ch15(&self) -> &CH {
        unsafe { &*(((self as *const Self) as *const u8).add(316usize) as *const CH) }
    }
    #[doc = "0x13c - DMA Channel registers."]
    #[inline(always)]
    pub fn ch15_mut(&self) -> &mut CH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(316usize) as *mut CH) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - DMA Channel Configuration Register."]
    pub cfg: self::ch::CFG,
    #[doc = "0x104 - DMA Channel Status Register."]
    pub st: self::ch::ST,
    #[doc = "0x108 - Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD."]
    pub src: self::ch::SRC,
    #[doc = "0x10c - Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD."]
    pub dst: self::ch::DST,
    #[doc = "0x110 - DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered."]
    pub cnt: self::ch::CNT,
    #[doc = "0x114 - Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition."]
    pub src_rld: self::ch::SRC_RLD,
    #[doc = "0x118 - Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition."]
    pub dst_rld: self::ch::DST_RLD,
    #[doc = "0x11c - DMA Channel Count Reload Register."]
    pub cnt_rld: self::ch::CNT_RLD,
}
#[doc = r"Register block"]
#[doc = "DMA Channel registers."]
pub mod ch;
#[doc = "DMA Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cn](cn) module"]
pub type CN = crate::Reg<u32, _CN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CN;
#[doc = "`read()` method returns [cn::R](cn::R) reader structure"]
impl crate::Readable for CN {}
#[doc = "`write(|w| ..)` method takes [cn::W](cn::W) writer structure"]
impl crate::Writable for CN {}
#[doc = "DMA Control Register."]
pub mod cn;
#[doc = "DMA Interrupt Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "DMA Interrupt Register."]
pub mod intr;
