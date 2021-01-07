#[doc = "Reader of register INTROUT"]
pub type R = crate::R<u16, super::INTROUT>;
#[doc = "Reader of field `EP15_OUT_INT`"]
pub type EP15_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP14_OUT_INT`"]
pub type EP14_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP13_OUT_INT`"]
pub type EP13_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP12_OUT_INT`"]
pub type EP12_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP11_OUT_INT`"]
pub type EP11_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP10_OUT_INT`"]
pub type EP10_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP9_OUT_INT`"]
pub type EP9_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP8_OUT_INT`"]
pub type EP8_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP7_OUT_INT`"]
pub type EP7_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP6_OUT_INT`"]
pub type EP6_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP5_OUT_INT`"]
pub type EP5_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP4_OUT_INT`"]
pub type EP4_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP3_OUT_INT`"]
pub type EP3_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2_OUT_INT`"]
pub type EP2_OUT_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1_OUT_INT`"]
pub type EP1_OUT_INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - Endpoint 15 interrupt."]
    #[inline(always)]
    pub fn ep15_out_int(&self) -> EP15_OUT_INT_R {
        EP15_OUT_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt."]
    #[inline(always)]
    pub fn ep14_out_int(&self) -> EP14_OUT_INT_R {
        EP14_OUT_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt."]
    #[inline(always)]
    pub fn ep13_out_int(&self) -> EP13_OUT_INT_R {
        EP13_OUT_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt."]
    #[inline(always)]
    pub fn ep12_out_int(&self) -> EP12_OUT_INT_R {
        EP12_OUT_INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt."]
    #[inline(always)]
    pub fn ep11_out_int(&self) -> EP11_OUT_INT_R {
        EP11_OUT_INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt."]
    #[inline(always)]
    pub fn ep10_out_int(&self) -> EP10_OUT_INT_R {
        EP10_OUT_INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt."]
    #[inline(always)]
    pub fn ep9_out_int(&self) -> EP9_OUT_INT_R {
        EP9_OUT_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt."]
    #[inline(always)]
    pub fn ep8_out_int(&self) -> EP8_OUT_INT_R {
        EP8_OUT_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt."]
    #[inline(always)]
    pub fn ep7_out_int(&self) -> EP7_OUT_INT_R {
        EP7_OUT_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt."]
    #[inline(always)]
    pub fn ep6_out_int(&self) -> EP6_OUT_INT_R {
        EP6_OUT_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt."]
    #[inline(always)]
    pub fn ep5_out_int(&self) -> EP5_OUT_INT_R {
        EP5_OUT_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt."]
    #[inline(always)]
    pub fn ep4_out_int(&self) -> EP4_OUT_INT_R {
        EP4_OUT_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt."]
    #[inline(always)]
    pub fn ep3_out_int(&self) -> EP3_OUT_INT_R {
        EP3_OUT_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt."]
    #[inline(always)]
    pub fn ep2_out_int(&self) -> EP2_OUT_INT_R {
        EP2_OUT_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 interrupt."]
    #[inline(always)]
    pub fn ep1_out_int(&self) -> EP1_OUT_INT_R {
        EP1_OUT_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
