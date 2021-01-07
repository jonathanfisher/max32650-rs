#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_IPEND_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<CH0_IPEND_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_IPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0_IPEND`"]
pub type CH0_IPEND_R = crate::R<bool, CH0_IPEND_A>;
impl CH0_IPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_IPEND_A {
        match self.bits {
            false => CH0_IPEND_A::INACTIVE,
            true => CH0_IPEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CH0_IPEND_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == CH0_IPEND_A::PENDING
    }
}
#[doc = ""]
pub type CH1_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH1_IPEND`"]
pub type CH1_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH2_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH2_IPEND`"]
pub type CH2_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH3_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH3_IPEND`"]
pub type CH3_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH4_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH4_IPEND`"]
pub type CH4_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH5_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH5_IPEND`"]
pub type CH5_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH6_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH6_IPEND`"]
pub type CH6_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH7_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH7_IPEND`"]
pub type CH7_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH8_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH8_IPEND`"]
pub type CH8_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH9_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH9_IPEND`"]
pub type CH9_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH10_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH10_IPEND`"]
pub type CH10_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH11_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH11_IPEND`"]
pub type CH11_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH12_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH12_IPEND`"]
pub type CH12_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH13_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH13_IPEND`"]
pub type CH13_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH14_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH14_IPEND`"]
pub type CH14_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
#[doc = ""]
pub type CH15_IPEND_A = crate::dma::intr::CH0_IPEND_A;
#[doc = "Reader of field `CH15_IPEND`"]
pub type CH15_IPEND_R = crate::R<bool, crate::dma::intr::CH0_IPEND_A>;
impl R {
    #[doc = "Bit 0 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch0_ipend(&self) -> CH0_IPEND_R {
        CH0_IPEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1_ipend(&self) -> CH1_IPEND_R {
        CH1_IPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2_ipend(&self) -> CH2_IPEND_R {
        CH2_IPEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3_ipend(&self) -> CH3_IPEND_R {
        CH3_IPEND_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch4_ipend(&self) -> CH4_IPEND_R {
        CH4_IPEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch5_ipend(&self) -> CH5_IPEND_R {
        CH5_IPEND_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ch6_ipend(&self) -> CH6_IPEND_R {
        CH6_IPEND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ch7_ipend(&self) -> CH7_IPEND_R {
        CH7_IPEND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ch8_ipend(&self) -> CH8_IPEND_R {
        CH8_IPEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ch9_ipend(&self) -> CH9_IPEND_R {
        CH9_IPEND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ch10_ipend(&self) -> CH10_IPEND_R {
        CH10_IPEND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ch11_ipend(&self) -> CH11_IPEND_R {
        CH11_IPEND_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ch12_ipend(&self) -> CH12_IPEND_R {
        CH12_IPEND_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ch13_ipend(&self) -> CH13_IPEND_R {
        CH13_IPEND_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ch14_ipend(&self) -> CH14_IPEND_R {
        CH14_IPEND_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ch15_ipend(&self) -> CH15_IPEND_R {
        CH15_IPEND_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
