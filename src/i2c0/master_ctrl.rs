#[doc = "Reader of register MASTER_CTRL"]
pub type R = crate::R<u32, super::MASTER_CTRL>;
#[doc = "Writer for register MASTER_CTRL"]
pub type W = crate::W<u32, super::MASTER_CTRL>;
#[doc = "Register MASTER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MASTER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RESTART`"]
pub type RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESTART`"]
pub struct RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Slave Extend Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SL_EX_ADDR_A {
    #[doc = "0: 7-bit address."]
    _7_BITS_ADDRESS = 0,
    #[doc = "1: 10-bit address."]
    _10_BITS_ADDRESS = 1,
}
impl From<SL_EX_ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: SL_EX_ADDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SL_EX_ADDR`"]
pub type SL_EX_ADDR_R = crate::R<bool, SL_EX_ADDR_A>;
impl SL_EX_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SL_EX_ADDR_A {
        match self.bits {
            false => SL_EX_ADDR_A::_7_BITS_ADDRESS,
            true => SL_EX_ADDR_A::_10_BITS_ADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `_7_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        *self == SL_EX_ADDR_A::_7_BITS_ADDRESS
    }
    #[doc = "Checks if the value of the field is `_10_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        *self == SL_EX_ADDR_A::_10_BITS_ADDRESS
    }
}
#[doc = "Write proxy for field `SL_EX_ADDR`"]
pub struct SL_EX_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SL_EX_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SL_EX_ADDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut W {
        self.variant(SL_EX_ADDR_A::_7_BITS_ADDRESS)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut W {
        self.variant(SL_EX_ADDR_A::_10_BITS_ADDRESS)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `MASTER_CODE`"]
pub type MASTER_CODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASTER_CODE`"]
pub struct MASTER_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Serial Clock speed Up. Setting this bit disables the master's monitoring of SCL state for other external masters or slaves.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_SPEED_UP_A {
    #[doc = "0: Master monitors SCL state."]
    EN = 0,
    #[doc = "1: SCL state monitoring disabled."]
    DIS = 1,
}
impl From<SCL_SPEED_UP_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_SPEED_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCL_SPEED_UP`"]
pub type SCL_SPEED_UP_R = crate::R<bool, SCL_SPEED_UP_A>;
impl SCL_SPEED_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_SPEED_UP_A {
        match self.bits {
            false => SCL_SPEED_UP_A::EN,
            true => SCL_SPEED_UP_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SCL_SPEED_UP_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SCL_SPEED_UP_A::DIS
    }
}
#[doc = "Write proxy for field `SCL_SPEED_UP`"]
pub struct SCL_SPEED_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_SPEED_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCL_SPEED_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master monitors SCL state."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SCL_SPEED_UP_A::EN)
    }
    #[doc = "SCL state monitoring disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SCL_SPEED_UP_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Setting this bit to 1 will start a master transfer."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 will generate a repeated START."]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Setting this bit to 1 will generate a STOP condition."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Slave Extend Address Select."]
    #[inline(always)]
    pub fn sl_ex_addr(&self) -> SL_EX_ADDR_R {
        SL_EX_ADDR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Master Code. These bits set the Master Code used in Hs-mode operation."]
    #[inline(always)]
    pub fn master_code(&self) -> MASTER_CODE_R {
        MASTER_CODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Serial Clock speed Up. Setting this bit disables the master's monitoring of SCL state for other external masters or slaves."]
    #[inline(always)]
    pub fn scl_speed_up(&self) -> SCL_SPEED_UP_R {
        SCL_SPEED_UP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit to 1 will start a master transfer."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - Setting this bit to 1 will generate a repeated START."]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W {
        RESTART_W { w: self }
    }
    #[doc = "Bit 2 - Setting this bit to 1 will generate a STOP condition."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 7 - Slave Extend Address Select."]
    #[inline(always)]
    pub fn sl_ex_addr(&mut self) -> SL_EX_ADDR_W {
        SL_EX_ADDR_W { w: self }
    }
    #[doc = "Bits 8:10 - Master Code. These bits set the Master Code used in Hs-mode operation."]
    #[inline(always)]
    pub fn master_code(&mut self) -> MASTER_CODE_W {
        MASTER_CODE_W { w: self }
    }
    #[doc = "Bit 11 - Serial Clock speed Up. Setting this bit disables the master's monitoring of SCL state for other external masters or slaves."]
    #[inline(always)]
    pub fn scl_speed_up(&mut self) -> SCL_SPEED_UP_W {
        SCL_SPEED_UP_W { w: self }
    }
}
