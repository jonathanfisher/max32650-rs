#[doc = "Reader of register REG0"]
pub type R = crate::R<u32, super::REG0>;
#[doc = "Writer for register REG0"]
pub type W = crate::W<u32, super::REG0>;
#[doc = "Register REG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB External Core Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRCKSEL_A {
    #[doc = "0: Generated clock from system clock."]
    SYS = 0,
    #[doc = "1: Digital clock from a GPIO."]
    DIG = 1,
}
impl From<USBRCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: USBRCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBRCKSEL`"]
pub type USBRCKSEL_R = crate::R<bool, USBRCKSEL_A>;
impl USBRCKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRCKSEL_A {
        match self.bits {
            false => USBRCKSEL_A::SYS,
            true => USBRCKSEL_A::DIG,
        }
    }
    #[doc = "Checks if the value of the field is `SYS`"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == USBRCKSEL_A::SYS
    }
    #[doc = "Checks if the value of the field is `DIG`"]
    #[inline(always)]
    pub fn is_dig(&self) -> bool {
        *self == USBRCKSEL_A::DIG
    }
}
#[doc = "Write proxy for field `USBRCKSEL`"]
pub struct USBRCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRCKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generated clock from system clock."]
    #[inline(always)]
    pub fn sys(self) -> &'a mut W {
        self.variant(USBRCKSEL_A::SYS)
    }
    #[doc = "Digital clock from a GPIO."]
    #[inline(always)]
    pub fn dig(self) -> &'a mut W {
        self.variant(USBRCKSEL_A::DIG)
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
#[doc = "I2C0 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0DGEN0_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C0DGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0DGEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0DGEN0`"]
pub type I2C0DGEN0_R = crate::R<bool, I2C0DGEN0_A>;
impl I2C0DGEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0DGEN0_A {
        match self.bits {
            false => I2C0DGEN0_A::DIS,
            true => I2C0DGEN0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C0DGEN0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C0DGEN0_A::EN
    }
}
#[doc = "Write proxy for field `I2C0DGEN0`"]
pub struct I2C0DGEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0DGEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0DGEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C0DGEN0_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C0DGEN0_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "I2C0 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0DGEN1_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C0DGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0DGEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0DGEN1`"]
pub type I2C0DGEN1_R = crate::R<bool, I2C0DGEN1_A>;
impl I2C0DGEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0DGEN1_A {
        match self.bits {
            false => I2C0DGEN1_A::DIS,
            true => I2C0DGEN1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C0DGEN1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C0DGEN1_A::EN
    }
}
#[doc = "Write proxy for field `I2C0DGEN1`"]
pub struct I2C0DGEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0DGEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0DGEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C0DGEN1_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C0DGEN1_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "I2C1 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1DGEN0_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C1DGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1DGEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C1DGEN0`"]
pub type I2C1DGEN0_R = crate::R<bool, I2C1DGEN0_A>;
impl I2C1DGEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1DGEN0_A {
        match self.bits {
            false => I2C1DGEN0_A::DIS,
            true => I2C1DGEN0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C1DGEN0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C1DGEN0_A::EN
    }
}
#[doc = "Write proxy for field `I2C1DGEN0`"]
pub struct I2C1DGEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1DGEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1DGEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C1DGEN0_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C1DGEN0_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "I2C1 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1DGEN1_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C1DGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1DGEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C1DGEN1`"]
pub type I2C1DGEN1_R = crate::R<bool, I2C1DGEN1_A>;
impl I2C1DGEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1DGEN1_A {
        match self.bits {
            false => I2C1DGEN1_A::DIS,
            true => I2C1DGEN1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C1DGEN1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C1DGEN1_A::EN
    }
}
#[doc = "Write proxy for field `I2C1DGEN1`"]
pub struct I2C1DGEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1DGEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1DGEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C1DGEN1_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C1DGEN1_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - USB External Core Clock Select."]
    #[inline(always)]
    pub fn usbrcksel(&self) -> USBRCKSEL_R {
        USBRCKSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - I2C0 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen0(&self) -> I2C0DGEN0_R {
        I2C0DGEN0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C0 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen1(&self) -> I2C0DGEN1_R {
        I2C0DGEN1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C1 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen0(&self) -> I2C1DGEN0_R {
        I2C1DGEN0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C1 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen1(&self) -> I2C1DGEN1_R {
        I2C1DGEN1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - USB External Core Clock Select."]
    #[inline(always)]
    pub fn usbrcksel(&mut self) -> USBRCKSEL_W {
        USBRCKSEL_W { w: self }
    }
    #[doc = "Bit 20 - I2C0 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen0(&mut self) -> I2C0DGEN0_W {
        I2C0DGEN0_W { w: self }
    }
    #[doc = "Bit 21 - I2C0 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen1(&mut self) -> I2C0DGEN1_W {
        I2C0DGEN1_W { w: self }
    }
    #[doc = "Bit 22 - I2C1 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen0(&mut self) -> I2C1DGEN0_W {
        I2C1DGEN0_W { w: self }
    }
    #[doc = "Bit 23 - I2C1 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen1(&mut self) -> I2C1DGEN1_W {
        I2C1DGEN1_W { w: self }
    }
}
