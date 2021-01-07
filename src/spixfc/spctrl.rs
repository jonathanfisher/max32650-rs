#[doc = "Reader of register SPCTRL"]
pub type R = crate::R<u32, super::SPCTRL>;
#[doc = "Writer for register SPCTRL"]
pub type W = crate::W<u32, super::SPCTRL>;
#[doc = "Register SPCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SCLK Inhibit Mode3. In SPI Mode 3, some SPI flash read timing diagrams show the last SCLK going low prior to de-assertion. The default is to support this additional falling edge of clock. When this bit is set and the device is in SPI Mode 3, the SPI clock is held high while Slave Select is de-asserted. This is to support some SPI flash write timing diagrams.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLKINH3_A {
    #[doc = "0: Allow trailing SCLK low pulse prior to Slave Select de-assertion."]
    EN = 0,
    #[doc = "1: Inhibit trailing SCLK low pulse prior to Slave Select de-assertion."]
    DIS = 1,
}
impl From<SCLKINH3_A> for bool {
    #[inline(always)]
    fn from(variant: SCLKINH3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLKINH3`"]
pub type SCLKINH3_R = crate::R<bool, SCLKINH3_A>;
impl SCLKINH3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLKINH3_A {
        match self.bits {
            false => SCLKINH3_A::EN,
            true => SCLKINH3_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SCLKINH3_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SCLKINH3_A::DIS
    }
}
#[doc = "Write proxy for field `SCLKINH3`"]
pub struct SCLKINH3_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKINH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLKINH3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow trailing SCLK low pulse prior to Slave Select de-assertion."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SCLKINH3_A::EN)
    }
    #[doc = "Inhibit trailing SCLK low pulse prior to Slave Select de-assertion."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SCLKINH3_A::DIS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - SCLK Inhibit Mode3. In SPI Mode 3, some SPI flash read timing diagrams show the last SCLK going low prior to de-assertion. The default is to support this additional falling edge of clock. When this bit is set and the device is in SPI Mode 3, the SPI clock is held high while Slave Select is de-asserted. This is to support some SPI flash write timing diagrams."]
    #[inline(always)]
    pub fn sclkinh3(&self) -> SCLKINH3_R {
        SCLKINH3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - SCLK Inhibit Mode3. In SPI Mode 3, some SPI flash read timing diagrams show the last SCLK going low prior to de-assertion. The default is to support this additional falling edge of clock. When this bit is set and the device is in SPI Mode 3, the SPI clock is held high while Slave Select is de-asserted. This is to support some SPI flash write timing diagrams."]
    #[inline(always)]
    pub fn sclkinh3(&mut self) -> SCLKINH3_W {
        SCLKINH3_W { w: self }
    }
}
