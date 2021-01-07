#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART enabled, to enable UART block, it is used to drive a gated clock in order to save power consumption when UART is not used. FIFOs are flushed when UART is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: UART disabled. FIFOs are flushed. Clock is gated off for power savings."]
    DIS = 0,
    #[doc = "1: UART enabled."]
    EN = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DIS,
            true => ENABLE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENABLE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ENABLE_A::EN
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART disabled. FIFOs are flushed. Clock is gated off for power savings."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENABLE_A::DIS)
    }
    #[doc = "UART enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ENABLE_A::EN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Enable/disable Parity bit (9th character).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITY_EN_A {
    #[doc = "0: No Parity"]
    DIS = 0,
    #[doc = "1: Parity enabled as 9th bit"]
    EN = 1,
}
impl From<PARITY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PARITY_EN`"]
pub type PARITY_EN_R = crate::R<bool, PARITY_EN_A>;
impl PARITY_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_EN_A {
        match self.bits {
            false => PARITY_EN_A::DIS,
            true => PARITY_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PARITY_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PARITY_EN_A::EN
    }
}
#[doc = "Write proxy for field `PARITY_EN`"]
pub struct PARITY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Parity"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PARITY_EN_A::DIS)
    }
    #[doc = "Parity enabled as 9th bit"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PARITY_EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "When PARITY_EN=1, selects odd, even, Mark or Space parity. Mark parity = always 1; Space parity = always 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PARITY_A {
    #[doc = "0: Even parity selected."]
    EVEN = 0,
    #[doc = "1: Odd parity selected."]
    ODD = 1,
    #[doc = "2: Mark parity selected."]
    MARK = 2,
    #[doc = "3: Space parity selected."]
    SPACE = 3,
}
impl From<PARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PARITY`"]
pub type PARITY_R = crate::R<u8, PARITY_A>;
impl PARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_A {
        match self.bits {
            0 => PARITY_A::EVEN,
            1 => PARITY_A::ODD,
            2 => PARITY_A::MARK,
            3 => PARITY_A::SPACE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY_A::ODD
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == PARITY_A::MARK
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == PARITY_A::SPACE
    }
}
#[doc = "Write proxy for field `PARITY`"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Even parity selected."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARITY_A::EVEN)
    }
    #[doc = "Odd parity selected."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARITY_A::ODD)
    }
    #[doc = "Mark parity selected."]
    #[inline(always)]
    pub fn mark(self) -> &'a mut W {
        self.variant(PARITY_A::MARK)
    }
    #[doc = "Space parity selected."]
    #[inline(always)]
    pub fn space(self) -> &'a mut W {
        self.variant(PARITY_A::SPACE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Selects parity based on 1s or 0s count (when PARITY_EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARMD_A {
    #[doc = "0: Parity calculation is based on number of 1s in frame."]
    _1 = 0,
    #[doc = "1: Parity calculation is based on number of 0s in frame."]
    _0 = 1,
}
impl From<PARMD_A> for bool {
    #[inline(always)]
    fn from(variant: PARMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PARMD`"]
pub type PARMD_R = crate::R<bool, PARMD_A>;
impl PARMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARMD_A {
        match self.bits {
            false => PARMD_A::_1,
            true => PARMD_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PARMD_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PARMD_A::_0
    }
}
#[doc = "Write proxy for field `PARMD`"]
pub struct PARMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PARMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Parity calculation is based on number of 1s in frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PARMD_A::_1)
    }
    #[doc = "Parity calculation is based on number of 0s in frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PARMD_A::_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TX_FLUSH`"]
pub type TX_FLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FLUSH`"]
pub struct TX_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RX_FLUSH`"]
pub type RX_FLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_FLUSH`"]
pub struct RX_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "If set, bit accuracy is selected, in this case the bit duration is the same for all the bits with the optimal accuracy. But the frame duration can have a significant deviation from the expected baudrate.If clear, frame accuracy is selected, therefore bits can have different duration in order to guarantee the minimum frame deviation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITACC_A {
    #[doc = "0: Frame accuracy."]
    FRAME = 0,
    #[doc = "1: Bit accuracy."]
    BIT = 1,
}
impl From<BITACC_A> for bool {
    #[inline(always)]
    fn from(variant: BITACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BITACC`"]
pub type BITACC_R = crate::R<bool, BITACC_A>;
impl BITACC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITACC_A {
        match self.bits {
            false => BITACC_A::FRAME,
            true => BITACC_A::BIT,
        }
    }
    #[doc = "Checks if the value of the field is `FRAME`"]
    #[inline(always)]
    pub fn is_frame(&self) -> bool {
        *self == BITACC_A::FRAME
    }
    #[doc = "Checks if the value of the field is `BIT`"]
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        *self == BITACC_A::BIT
    }
}
#[doc = "Write proxy for field `BITACC`"]
pub struct BITACC_W<'a> {
    w: &'a mut W,
}
impl<'a> BITACC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITACC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame accuracy."]
    #[inline(always)]
    pub fn frame(self) -> &'a mut W {
        self.variant(BITACC_A::FRAME)
    }
    #[doc = "Bit accuracy."]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut W {
        self.variant(BITACC_A::BIT)
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
#[doc = "Selects UART character size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHAR_SIZE_A {
    #[doc = "0: 5 bits."]
    _5 = 0,
    #[doc = "1: 6 bits."]
    _6 = 1,
    #[doc = "2: 7 bits."]
    _7 = 2,
    #[doc = "3: 8 bits."]
    _8 = 3,
}
impl From<CHAR_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHAR_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHAR_SIZE`"]
pub type CHAR_SIZE_R = crate::R<u8, CHAR_SIZE_A>;
impl CHAR_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAR_SIZE_A {
        match self.bits {
            0 => CHAR_SIZE_A::_5,
            1 => CHAR_SIZE_A::_6,
            2 => CHAR_SIZE_A::_7,
            3 => CHAR_SIZE_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == CHAR_SIZE_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == CHAR_SIZE_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == CHAR_SIZE_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CHAR_SIZE_A::_8
    }
}
#[doc = "Write proxy for field `CHAR_SIZE`"]
pub struct CHAR_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAR_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHAR_SIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "5 bits."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(CHAR_SIZE_A::_5)
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(CHAR_SIZE_A::_6)
    }
    #[doc = "7 bits."]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(CHAR_SIZE_A::_7)
    }
    #[doc = "8 bits."]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHAR_SIZE_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Selects the number of stop bits that will be generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPBITS_A {
    #[doc = "0: 1 stop bit."]
    _1 = 0,
    #[doc = "1: 1.5 stop bits."]
    _1_5 = 1,
}
impl From<STOPBITS_A> for bool {
    #[inline(always)]
    fn from(variant: STOPBITS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOPBITS`"]
pub type STOPBITS_R = crate::R<bool, STOPBITS_A>;
impl STOPBITS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPBITS_A {
        match self.bits {
            false => STOPBITS_A::_1,
            true => STOPBITS_A::_1_5,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOPBITS_A::_1
    }
    #[doc = "Checks if the value of the field is `_1_5`"]
    #[inline(always)]
    pub fn is_1_5(&self) -> bool {
        *self == STOPBITS_A::_1_5
    }
}
#[doc = "Write proxy for field `STOPBITS`"]
pub struct STOPBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPBITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPBITS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPBITS_A::_1)
    }
    #[doc = "1.5 stop bits."]
    #[inline(always)]
    pub fn _1_5(self) -> &'a mut W {
        self.variant(STOPBITS_A::_1_5)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Enables/disables hardware flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLOW_CTRL_A {
    #[doc = "1: HW Flow Control with RTS/CTS enabled"]
    EN = 1,
    #[doc = "0: HW Flow Control disabled"]
    DIS = 0,
}
impl From<FLOW_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FLOW_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLOW_CTRL`"]
pub type FLOW_CTRL_R = crate::R<bool, FLOW_CTRL_A>;
impl FLOW_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLOW_CTRL_A {
        match self.bits {
            true => FLOW_CTRL_A::EN,
            false => FLOW_CTRL_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLOW_CTRL_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLOW_CTRL_A::DIS
    }
}
#[doc = "Write proxy for field `FLOW_CTRL`"]
pub struct FLOW_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLOW_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLOW_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HW Flow Control with RTS/CTS enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLOW_CTRL_A::EN)
    }
    #[doc = "HW Flow Control disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLOW_CTRL_A::DIS)
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
#[doc = "RTS/CTS polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLOW_POL_A {
    #[doc = "0: RTS/CTS asserted is logic 0."]
    _0 = 0,
    #[doc = "1: RTS/CTS asserted is logic 1."]
    _1 = 1,
}
impl From<FLOW_POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLOW_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLOW_POL`"]
pub type FLOW_POL_R = crate::R<bool, FLOW_POL_A>;
impl FLOW_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLOW_POL_A {
        match self.bits {
            false => FLOW_POL_A::_0,
            true => FLOW_POL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLOW_POL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLOW_POL_A::_1
    }
}
#[doc = "Write proxy for field `FLOW_POL`"]
pub struct FLOW_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLOW_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLOW_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTS/CTS asserted is logic 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLOW_POL_A::_0)
    }
    #[doc = "RTS/CTS asserted is logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLOW_POL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "NULL Modem Support (RTS/CTS and TXD/RXD swap).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NULL_MODEM_A {
    #[doc = "0: Direct convention."]
    DIS = 0,
    #[doc = "1: Null Modem Mode."]
    EN = 1,
}
impl From<NULL_MODEM_A> for bool {
    #[inline(always)]
    fn from(variant: NULL_MODEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NULL_MODEM`"]
pub type NULL_MODEM_R = crate::R<bool, NULL_MODEM_A>;
impl NULL_MODEM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NULL_MODEM_A {
        match self.bits {
            false => NULL_MODEM_A::DIS,
            true => NULL_MODEM_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == NULL_MODEM_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == NULL_MODEM_A::EN
    }
}
#[doc = "Write proxy for field `NULL_MODEM`"]
pub struct NULL_MODEM_W<'a> {
    w: &'a mut W,
}
impl<'a> NULL_MODEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NULL_MODEM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Direct convention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(NULL_MODEM_A::DIS)
    }
    #[doc = "Null Modem Mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(NULL_MODEM_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Break control bit. It causes a break condition to be transmitted to receiving UART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREAK_A {
    #[doc = "0: Break characters are not generated."]
    DIS = 0,
    #[doc = "1: Break characters are sent(all the bits are at '0' including start/parity/stop)."]
    EN = 1,
}
impl From<BREAK_A> for bool {
    #[inline(always)]
    fn from(variant: BREAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BREAK`"]
pub type BREAK_R = crate::R<bool, BREAK_A>;
impl BREAK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREAK_A {
        match self.bits {
            false => BREAK_A::DIS,
            true => BREAK_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BREAK_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BREAK_A::EN
    }
}
#[doc = "Write proxy for field `BREAK`"]
pub struct BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BREAK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break characters are not generated."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BREAK_A::DIS)
    }
    #[doc = "Break characters are sent(all the bits are at '0' including start/parity/stop)."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BREAK_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Baud Rate Clock Source Select. Selects the baud rate clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSEL_A {
    #[doc = "0: System clock."]
    SYSTEM = 0,
    #[doc = "1: Alternate 7.3727MHz internal clock. Useful in low power modes when the system clock is slow."]
    ALTERNATE = 1,
}
impl From<CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<bool, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            false => CLKSEL_A::SYSTEM,
            true => CLKSEL_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == CLKSEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == CLKSEL_A::ALTERNATE
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "System clock."]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(CLKSEL_A::SYSTEM)
    }
    #[doc = "Alternate 7.3727MHz internal clock. Useful in low power modes when the system clock is slow."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(CLKSEL_A::ALTERNATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RX_TO`"]
pub type RX_TO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_TO`"]
pub struct RX_TO_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - UART enabled, to enable UART block, it is used to drive a gated clock in order to save power consumption when UART is not used. FIFOs are flushed when UART is disabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable/disable Parity bit (9th character)."]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - When PARITY_EN=1, selects odd, even, Mark or Space parity. Mark parity = always 1; Space parity = always 0."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Selects parity based on 1s or 0s count (when PARITY_EN=1)."]
    #[inline(always)]
    pub fn parmd(&self) -> PARMD_R {
        PARMD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flushes the TX FIFO buffer."]
    #[inline(always)]
    pub fn tx_flush(&self) -> TX_FLUSH_R {
        TX_FLUSH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Flushes the RX FIFO buffer."]
    #[inline(always)]
    pub fn rx_flush(&self) -> RX_FLUSH_R {
        RX_FLUSH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If set, bit accuracy is selected, in this case the bit duration is the same for all the bits with the optimal accuracy. But the frame duration can have a significant deviation from the expected baudrate.If clear, frame accuracy is selected, therefore bits can have different duration in order to guarantee the minimum frame deviation."]
    #[inline(always)]
    pub fn bitacc(&self) -> BITACC_R {
        BITACC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Selects UART character size."]
    #[inline(always)]
    pub fn char_size(&self) -> CHAR_SIZE_R {
        CHAR_SIZE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Selects the number of stop bits that will be generated."]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables/disables hardware flow control."]
    #[inline(always)]
    pub fn flow_ctrl(&self) -> FLOW_CTRL_R {
        FLOW_CTRL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RTS/CTS polarity."]
    #[inline(always)]
    pub fn flow_pol(&self) -> FLOW_POL_R {
        FLOW_POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NULL Modem Support (RTS/CTS and TXD/RXD swap)."]
    #[inline(always)]
    pub fn null_modem(&self) -> NULL_MODEM_R {
        NULL_MODEM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Break control bit. It causes a break condition to be transmitted to receiving UART."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Baud Rate Clock Source Select. Selects the baud rate clock."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - RX Time Out. RX time out interrupt will occur after RXTO Uart characters if RX-FIFO is not empty and RX FIFO has not been read."]
    #[inline(always)]
    pub fn rx_to(&self) -> RX_TO_R {
        RX_TO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UART enabled, to enable UART block, it is used to drive a gated clock in order to save power consumption when UART is not used. FIFOs are flushed when UART is disabled."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enable/disable Parity bit (9th character)."]
    #[inline(always)]
    pub fn parity_en(&mut self) -> PARITY_EN_W {
        PARITY_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - When PARITY_EN=1, selects odd, even, Mark or Space parity. Mark parity = always 1; Space parity = always 0."]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Bit 4 - Selects parity based on 1s or 0s count (when PARITY_EN=1)."]
    #[inline(always)]
    pub fn parmd(&mut self) -> PARMD_W {
        PARMD_W { w: self }
    }
    #[doc = "Bit 5 - Flushes the TX FIFO buffer."]
    #[inline(always)]
    pub fn tx_flush(&mut self) -> TX_FLUSH_W {
        TX_FLUSH_W { w: self }
    }
    #[doc = "Bit 6 - Flushes the RX FIFO buffer."]
    #[inline(always)]
    pub fn rx_flush(&mut self) -> RX_FLUSH_W {
        RX_FLUSH_W { w: self }
    }
    #[doc = "Bit 7 - If set, bit accuracy is selected, in this case the bit duration is the same for all the bits with the optimal accuracy. But the frame duration can have a significant deviation from the expected baudrate.If clear, frame accuracy is selected, therefore bits can have different duration in order to guarantee the minimum frame deviation."]
    #[inline(always)]
    pub fn bitacc(&mut self) -> BITACC_W {
        BITACC_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects UART character size."]
    #[inline(always)]
    pub fn char_size(&mut self) -> CHAR_SIZE_W {
        CHAR_SIZE_W { w: self }
    }
    #[doc = "Bit 10 - Selects the number of stop bits that will be generated."]
    #[inline(always)]
    pub fn stopbits(&mut self) -> STOPBITS_W {
        STOPBITS_W { w: self }
    }
    #[doc = "Bit 11 - Enables/disables hardware flow control."]
    #[inline(always)]
    pub fn flow_ctrl(&mut self) -> FLOW_CTRL_W {
        FLOW_CTRL_W { w: self }
    }
    #[doc = "Bit 12 - RTS/CTS polarity."]
    #[inline(always)]
    pub fn flow_pol(&mut self) -> FLOW_POL_W {
        FLOW_POL_W { w: self }
    }
    #[doc = "Bit 13 - NULL Modem Support (RTS/CTS and TXD/RXD swap)."]
    #[inline(always)]
    pub fn null_modem(&mut self) -> NULL_MODEM_W {
        NULL_MODEM_W { w: self }
    }
    #[doc = "Bit 14 - Break control bit. It causes a break condition to be transmitted to receiving UART."]
    #[inline(always)]
    pub fn break_(&mut self) -> BREAK_W {
        BREAK_W { w: self }
    }
    #[doc = "Bit 15 - Baud Rate Clock Source Select. Selects the baud rate clock."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bits 16:23 - RX Time Out. RX time out interrupt will occur after RXTO Uart characters if RX-FIFO is not empty and RX FIFO has not been read."]
    #[inline(always)]
    pub fn rx_to(&mut self) -> RX_TO_W {
        RX_TO_W { w: self }
    }
}
