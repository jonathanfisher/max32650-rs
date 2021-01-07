#[doc = "Reader of register INTRIN"]
pub type R = crate::R<u16, super::INTRIN>;
#[doc = "Reader of field `EP15_IN_INT`"]
pub type EP15_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP14_IN_INT`"]
pub type EP14_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP13_IN_INT`"]
pub type EP13_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP12_IN_INT`"]
pub type EP12_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP11_IN_INT`"]
pub type EP11_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP10_IN_INT`"]
pub type EP10_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP9_IN_INT`"]
pub type EP9_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP8_IN_INT`"]
pub type EP8_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP7_IN_INT`"]
pub type EP7_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP6_IN_INT`"]
pub type EP6_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP5_IN_INT`"]
pub type EP5_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP4_IN_INT`"]
pub type EP4_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP3_IN_INT`"]
pub type EP3_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2_IN_INT`"]
pub type EP2_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1_IN_INT`"]
pub type EP1_IN_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP0_IN_INT`"]
pub type EP0_IN_INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - Endpoint 15 interrupt."]
    #[inline(always)]
    pub fn ep15_in_int(&self) -> EP15_IN_INT_R {
        EP15_IN_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt."]
    #[inline(always)]
    pub fn ep14_in_int(&self) -> EP14_IN_INT_R {
        EP14_IN_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt."]
    #[inline(always)]
    pub fn ep13_in_int(&self) -> EP13_IN_INT_R {
        EP13_IN_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt."]
    #[inline(always)]
    pub fn ep12_in_int(&self) -> EP12_IN_INT_R {
        EP12_IN_INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt."]
    #[inline(always)]
    pub fn ep11_in_int(&self) -> EP11_IN_INT_R {
        EP11_IN_INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt."]
    #[inline(always)]
    pub fn ep10_in_int(&self) -> EP10_IN_INT_R {
        EP10_IN_INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt."]
    #[inline(always)]
    pub fn ep9_in_int(&self) -> EP9_IN_INT_R {
        EP9_IN_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt."]
    #[inline(always)]
    pub fn ep8_in_int(&self) -> EP8_IN_INT_R {
        EP8_IN_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt."]
    #[inline(always)]
    pub fn ep7_in_int(&self) -> EP7_IN_INT_R {
        EP7_IN_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt."]
    #[inline(always)]
    pub fn ep6_in_int(&self) -> EP6_IN_INT_R {
        EP6_IN_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt."]
    #[inline(always)]
    pub fn ep5_in_int(&self) -> EP5_IN_INT_R {
        EP5_IN_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt."]
    #[inline(always)]
    pub fn ep4_in_int(&self) -> EP4_IN_INT_R {
        EP4_IN_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt."]
    #[inline(always)]
    pub fn ep3_in_int(&self) -> EP3_IN_INT_R {
        EP3_IN_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt."]
    #[inline(always)]
    pub fn ep2_in_int(&self) -> EP2_IN_INT_R {
        EP2_IN_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 interrupt."]
    #[inline(always)]
    pub fn ep1_in_int(&self) -> EP1_IN_INT_R {
        EP1_IN_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Endpoint 0 interrupt."]
    #[inline(always)]
    pub fn ep0_in_int(&self) -> EP0_IN_INT_R {
        EP0_IN_INT_R::new((self.bits & 0x01) != 0)
    }
}
