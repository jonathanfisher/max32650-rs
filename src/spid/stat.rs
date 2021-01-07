#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "SPI active status. In Master mode, set when transaction starts, cleared when last bit of last character is acted upon and Slave Select de-assertion would occur. In Slave mode, set when Slave Select is asserted, cleared when Slave Select is de-asserted. Not used in Timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: SPI not active."]
    NOT = 0,
    #[doc = "1: SPI active."]
    ACTIVE = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NOT,
            true => BUSY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == BUSY_A::NOT
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BUSY_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - SPI active status. In Master mode, set when transaction starts, cleared when last bit of last character is acted upon and Slave Select de-assertion would occur. In Slave mode, set when Slave Select is asserted, cleared when Slave Select is de-asserted. Not used in Timer mode."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
}
