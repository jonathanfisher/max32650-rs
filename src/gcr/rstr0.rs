#[doc = "Reader of register RSTR0"]
pub type R = crate::R<u32, super::RSTR0>;
#[doc = "Writer for register RSTR0"]
pub type W = crate::W<u32, super::RSTR0>;
#[doc = "Register RSTR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, DMA_A>;
impl DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::RESET_DONE,
            true => DMA_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == DMA_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DMA_A::BUSY
    }
}
#[doc = "DMA Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<DMA_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(DMA_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA_AW::RESET)
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
#[doc = "Watchdog Timer Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, WDT_A>;
impl WDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::RESET_DONE,
            true => WDT_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == WDT_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == WDT_A::BUSY
    }
}
#[doc = "Watchdog Timer Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<WDT_AW> for bool {
    #[inline(always)]
    fn from(variant: WDT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `WDT`"]
pub struct WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(WDT_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDT_AW::RESET)
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
#[doc = "GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<GPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO0`"]
pub type GPIO0_R = crate::R<bool, GPIO0_A>;
impl GPIO0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_A {
        match self.bits {
            false => GPIO0_A::RESET_DONE,
            true => GPIO0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == GPIO0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == GPIO0_A::BUSY
    }
}
#[doc = "GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<GPIO0_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `GPIO0`"]
pub struct GPIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(GPIO0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIO0_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<GPIO1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO1`"]
pub type GPIO1_R = crate::R<bool, GPIO1_A>;
impl GPIO1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1_A {
        match self.bits {
            false => GPIO1_A::RESET_DONE,
            true => GPIO1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == GPIO1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == GPIO1_A::BUSY
    }
}
#[doc = "GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<GPIO1_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `GPIO1`"]
pub struct GPIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(GPIO1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIO1_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "GPIO2 Reset. Setting this bit to 1 resets GPIO2 pins to their default states.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<GPIO2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO2`"]
pub type GPIO2_R = crate::R<bool, GPIO2_A>;
impl GPIO2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2_A {
        match self.bits {
            false => GPIO2_A::RESET_DONE,
            true => GPIO2_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == GPIO2_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == GPIO2_A::BUSY
    }
}
#[doc = "GPIO2 Reset. Setting this bit to 1 resets GPIO2 pins to their default states.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<GPIO2_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIO2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `GPIO2`"]
pub struct GPIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(GPIO2_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIO2_AW::RESET)
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
#[doc = "Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TIMER0_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER0`"]
pub type TIMER0_R = crate::R<bool, TIMER0_A>;
impl TIMER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_A {
        match self.bits {
            false => TIMER0_A::RESET_DONE,
            true => TIMER0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == TIMER0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TIMER0_A::BUSY
    }
}
#[doc = "Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TIMER0_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMER0`"]
pub struct TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TIMER0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER0_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TIMER1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER1`"]
pub type TIMER1_R = crate::R<bool, TIMER1_A>;
impl TIMER1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1_A {
        match self.bits {
            false => TIMER1_A::RESET_DONE,
            true => TIMER1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == TIMER1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TIMER1_A::BUSY
    }
}
#[doc = "Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TIMER1_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMER1`"]
pub struct TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TIMER1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER1_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TIMER2_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER2`"]
pub type TIMER2_R = crate::R<bool, TIMER2_A>;
impl TIMER2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER2_A {
        match self.bits {
            false => TIMER2_A::RESET_DONE,
            true => TIMER2_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == TIMER2_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TIMER2_A::BUSY
    }
}
#[doc = "Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TIMER2_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMER2`"]
pub struct TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TIMER2_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER2_AW::RESET)
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
#[doc = "Timer3 Reset. Setting this bit to 1 resets Timer 3 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TIMER3_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER3`"]
pub type TIMER3_R = crate::R<bool, TIMER3_A>;
impl TIMER3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER3_A {
        match self.bits {
            false => TIMER3_A::RESET_DONE,
            true => TIMER3_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == TIMER3_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TIMER3_A::BUSY
    }
}
#[doc = "Timer3 Reset. Setting this bit to 1 resets Timer 3 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TIMER3_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMER3`"]
pub struct TIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TIMER3_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER3_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Timer3 Reset. Setting this bit to 1 resets Timer 4 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TIMER4_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER4`"]
pub type TIMER4_R = crate::R<bool, TIMER4_A>;
impl TIMER4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER4_A {
        match self.bits {
            false => TIMER4_A::RESET_DONE,
            true => TIMER4_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == TIMER4_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TIMER4_A::BUSY
    }
}
#[doc = "Timer3 Reset. Setting this bit to 1 resets Timer 4 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TIMER4_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMER4`"]
pub struct TIMER4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TIMER4_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER4_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Timer3 Reset. Setting this bit to 1 resets Timer 5 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER5_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TIMER5_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER5`"]
pub type TIMER5_R = crate::R<bool, TIMER5_A>;
impl TIMER5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER5_A {
        match self.bits {
            false => TIMER5_A::RESET_DONE,
            true => TIMER5_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == TIMER5_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TIMER5_A::BUSY
    }
}
#[doc = "Timer3 Reset. Setting this bit to 1 resets Timer 5 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER5_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TIMER5_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMER5`"]
pub struct TIMER5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TIMER5_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER5_AW::RESET)
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
#[doc = "UART0 Reset. Setting this bit to 1 resets all UART 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0`"]
pub type UART0_R = crate::R<bool, UART0_A>;
impl UART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::RESET_DONE,
            true => UART0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == UART0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == UART0_A::BUSY
    }
}
#[doc = "UART0 Reset. Setting this bit to 1 resets all UART 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<UART0_AW> for bool {
    #[inline(always)]
    fn from(variant: UART0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `UART0`"]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(UART0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART0_AW::RESET)
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
#[doc = "UART1 Reset. Setting this bit to 1 resets all UART 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART1`"]
pub type UART1_R = crate::R<bool, UART1_A>;
impl UART1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::RESET_DONE,
            true => UART1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == UART1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == UART1_A::BUSY
    }
}
#[doc = "UART1 Reset. Setting this bit to 1 resets all UART 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<UART1_AW> for bool {
    #[inline(always)]
    fn from(variant: UART1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `UART1`"]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(UART1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART1_AW::RESET)
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
#[doc = "SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0`"]
pub type SPI0_R = crate::R<bool, SPI0_A>;
impl SPI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::RESET_DONE,
            true => SPI0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPI0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPI0_A::BUSY
    }
}
#[doc = "SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPI0_AW> for bool {
    #[inline(always)]
    fn from(variant: SPI0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPI0`"]
pub struct SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPI0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI0_AW::RESET)
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
#[doc = "SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1`"]
pub type SPI1_R = crate::R<bool, SPI1_A>;
impl SPI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::RESET_DONE,
            true => SPI1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPI1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPI1_A::BUSY
    }
}
#[doc = "SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPI1_AW> for bool {
    #[inline(always)]
    fn from(variant: SPI1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPI1`"]
pub struct SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPI1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI1_AW::RESET)
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
#[doc = "SPI2 Reset. Setting this bit to 1 resets all SPI 2 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI2_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<SPI2_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI2`"]
pub type SPI2_R = crate::R<bool, SPI2_A>;
impl SPI2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2_A {
        match self.bits {
            false => SPI2_A::RESET_DONE,
            true => SPI2_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SPI2_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SPI2_A::BUSY
    }
}
#[doc = "SPI2 Reset. Setting this bit to 1 resets all SPI 2 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI2_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPI2_AW> for bool {
    #[inline(always)]
    fn from(variant: SPI2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPI2`"]
pub struct SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPI2_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI2_AW::RESET)
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
#[doc = "I2C0 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<I2C0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0`"]
pub type I2C0_R = crate::R<bool, I2C0_A>;
impl I2C0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::RESET_DONE,
            true => I2C0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == I2C0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == I2C0_A::BUSY
    }
}
#[doc = "I2C0 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<I2C0_AW> for bool {
    #[inline(always)]
    fn from(variant: I2C0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `I2C0`"]
pub struct I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(I2C0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C0_AW::RESET)
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
#[doc = "Real Time Clock Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::RESET_DONE,
            true => RTC_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RTC_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RTC_A::BUSY
    }
}
#[doc = "Real Time Clock Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<RTC_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(RTC_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RTC_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Cryptographic Reset. Setting this bit to 1 resets the AES block, the SHA block and the DES block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTO_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<CRYPTO_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYPTO`"]
pub type CRYPTO_R = crate::R<bool, CRYPTO_A>;
impl CRYPTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPTO_A {
        match self.bits {
            false => CRYPTO_A::RESET_DONE,
            true => CRYPTO_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == CRYPTO_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == CRYPTO_A::BUSY
    }
}
#[doc = "Cryptographic Reset. Setting this bit to 1 resets the AES block, the SHA block and the DES block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTO_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<CRYPTO_AW> for bool {
    #[inline(always)]
    fn from(variant: CRYPTO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CRYPTO`"]
pub struct CRYPTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(CRYPTO_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRYPTO_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Hyper Bus Controller Reset. Setting this bit to 1 resets the HBC clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HBC_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<HBC_A> for bool {
    #[inline(always)]
    fn from(variant: HBC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HBC`"]
pub type HBC_R = crate::R<bool, HBC_A>;
impl HBC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HBC_A {
        match self.bits {
            false => HBC_A::RESET_DONE,
            true => HBC_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == HBC_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == HBC_A::BUSY
    }
}
#[doc = "Hyper Bus Controller Reset. Setting this bit to 1 resets the HBC clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HBC_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<HBC_AW> for bool {
    #[inline(always)]
    fn from(variant: HBC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `HBC`"]
pub struct HBC_W<'a> {
    w: &'a mut W,
}
impl<'a> HBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HBC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(HBC_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HBC_AW::RESET)
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
#[doc = "CLCD Reset. Setting this bit to 1 resets the CLCD clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLCD_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<CLCD_A> for bool {
    #[inline(always)]
    fn from(variant: CLCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLCD`"]
pub type CLCD_R = crate::R<bool, CLCD_A>;
impl CLCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLCD_A {
        match self.bits {
            false => CLCD_A::RESET_DONE,
            true => CLCD_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == CLCD_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == CLCD_A::BUSY
    }
}
#[doc = "CLCD Reset. Setting this bit to 1 resets the CLCD clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLCD_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<CLCD_AW> for bool {
    #[inline(always)]
    fn from(variant: CLCD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLCD`"]
pub struct CLCD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLCD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(CLCD_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLCD_AW::RESET)
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
#[doc = "USB Reset. Setting this bit resets both USB blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<USB_A> for bool {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB`"]
pub type USB_R = crate::R<bool, USB_A>;
impl USB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_A {
        match self.bits {
            false => USB_A::RESET_DONE,
            true => USB_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == USB_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == USB_A::BUSY
    }
}
#[doc = "USB Reset. Setting this bit resets both USB blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<USB_AW> for bool {
    #[inline(always)]
    fn from(variant: USB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `USB`"]
pub struct USB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(USB_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USB_AW::RESET)
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
#[doc = "TRNG Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TRNG_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRNG`"]
pub type TRNG_R = crate::R<bool, TRNG_A>;
impl TRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNG_A {
        match self.bits {
            false => TRNG_A::RESET_DONE,
            true => TRNG_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == TRNG_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TRNG_A::BUSY
    }
}
#[doc = "TRNG Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TRNG_AW> for bool {
    #[inline(always)]
    fn from(variant: TRNG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TRNG`"]
pub struct TRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRNG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TRNG_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TRNG_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RSV_0x19`"]
pub type RSV_0X19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSV_0x19`"]
pub struct RSV_0X19_W<'a> {
    w: &'a mut W,
}
impl<'a> RSV_0X19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Analog to Digital Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, ADC_A>;
impl ADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::RESET_DONE,
            true => ADC_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == ADC_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == ADC_A::BUSY
    }
}
#[doc = "Analog to Digital Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<ADC_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ADC`"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(ADC_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ADC_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "UART2 Reset. Setting this bit to 1 resets all UART 2 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<UART2_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART2`"]
pub type UART2_R = crate::R<bool, UART2_A>;
impl UART2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_A {
        match self.bits {
            false => UART2_A::RESET_DONE,
            true => UART2_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == UART2_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == UART2_A::BUSY
    }
}
#[doc = "UART2 Reset. Setting this bit to 1 resets all UART 2 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<UART2_AW> for bool {
    #[inline(always)]
    fn from(variant: UART2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `UART2`"]
pub struct UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(UART2_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART2_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRST_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<SRST_A> for bool {
    #[inline(always)]
    fn from(variant: SRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRST`"]
pub type SRST_R = crate::R<bool, SRST_A>;
impl SRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRST_A {
        match self.bits {
            false => SRST_A::RESET_DONE,
            true => SRST_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SRST_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SRST_A::BUSY
    }
}
#[doc = "Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRST_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SRST_AW> for bool {
    #[inline(always)]
    fn from(variant: SRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SRST`"]
pub struct SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SRST_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SRST_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRST_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<PRST_A> for bool {
    #[inline(always)]
    fn from(variant: PRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRST`"]
pub type PRST_R = crate::R<bool, PRST_A>;
impl PRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRST_A {
        match self.bits {
            false => PRST_A::RESET_DONE,
            true => PRST_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == PRST_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PRST_A::BUSY
    }
}
#[doc = "Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRST_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<PRST_AW> for bool {
    #[inline(always)]
    fn from(variant: PRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PRST`"]
pub struct PRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(PRST_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PRST_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTEM_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<SYSTEM_A> for bool {
    #[inline(always)]
    fn from(variant: SYSTEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSTEM`"]
pub type SYSTEM_R = crate::R<bool, SYSTEM_A>;
impl SYSTEM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSTEM_A {
        match self.bits {
            false => SYSTEM_A::RESET_DONE,
            true => SYSTEM_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == SYSTEM_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SYSTEM_A::BUSY
    }
}
#[doc = "System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTEM_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SYSTEM_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSTEM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SYSTEM`"]
pub struct SYSTEM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSTEM_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SYSTEM_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSTEM_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO2 Reset. Setting this bit to 1 resets GPIO2 pins to their default states."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer3 Reset. Setting this bit to 1 resets Timer 4 blocks."]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer3 Reset. Setting this bit to 1 resets Timer 5 blocks."]
    #[inline(always)]
    pub fn timer5(&self) -> TIMER5_R {
        TIMER5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - UART0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPI2 Reset. Setting this bit to 1 resets all SPI 2 blocks."]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C0 Reset."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Cryptographic Reset. Setting this bit to 1 resets the AES block, the SHA block and the DES block."]
    #[inline(always)]
    pub fn crypto(&self) -> CRYPTO_R {
        CRYPTO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Hyper Bus Controller Reset. Setting this bit to 1 resets the HBC clock."]
    #[inline(always)]
    pub fn hbc(&self) -> HBC_R {
        HBC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CLCD Reset. Setting this bit to 1 resets the CLCD clock."]
    #[inline(always)]
    pub fn clcd(&self) -> CLCD_R {
        CLCD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - USB Reset. Setting this bit resets both USB blocks."]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TRNG Reset."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rsv_0x19(&self) -> RSV_0X19_R {
        RSV_0X19_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Analog to Digital Reset."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    pub fn system(&self) -> SYSTEM_R {
        SYSTEM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 1 - Watchdog Timer Reset."]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W {
        WDT_W { w: self }
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W {
        GPIO0_W { w: self }
    }
    #[doc = "Bit 3 - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO1_W {
        GPIO1_W { w: self }
    }
    #[doc = "Bit 4 - GPIO2 Reset. Setting this bit to 1 resets GPIO2 pins to their default states."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> GPIO2_W {
        GPIO2_W { w: self }
    }
    #[doc = "Bit 5 - Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    pub fn timer0(&mut self) -> TIMER0_W {
        TIMER0_W { w: self }
    }
    #[doc = "Bit 6 - Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    pub fn timer1(&mut self) -> TIMER1_W {
        TIMER1_W { w: self }
    }
    #[doc = "Bit 7 - Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    pub fn timer2(&mut self) -> TIMER2_W {
        TIMER2_W { w: self }
    }
    #[doc = "Bit 8 - Timer3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
    #[inline(always)]
    pub fn timer3(&mut self) -> TIMER3_W {
        TIMER3_W { w: self }
    }
    #[doc = "Bit 9 - Timer3 Reset. Setting this bit to 1 resets Timer 4 blocks."]
    #[inline(always)]
    pub fn timer4(&mut self) -> TIMER4_W {
        TIMER4_W { w: self }
    }
    #[doc = "Bit 10 - Timer3 Reset. Setting this bit to 1 resets Timer 5 blocks."]
    #[inline(always)]
    pub fn timer5(&mut self) -> TIMER5_W {
        TIMER5_W { w: self }
    }
    #[doc = "Bit 11 - UART0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 12 - UART1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 13 - SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks."]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W {
        SPI0_W { w: self }
    }
    #[doc = "Bit 14 - SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W {
        SPI1_W { w: self }
    }
    #[doc = "Bit 15 - SPI2 Reset. Setting this bit to 1 resets all SPI 2 blocks."]
    #[inline(always)]
    pub fn spi2(&mut self) -> SPI2_W {
        SPI2_W { w: self }
    }
    #[doc = "Bit 16 - I2C0 Reset."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W { w: self }
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 18 - Cryptographic Reset. Setting this bit to 1 resets the AES block, the SHA block and the DES block."]
    #[inline(always)]
    pub fn crypto(&mut self) -> CRYPTO_W {
        CRYPTO_W { w: self }
    }
    #[doc = "Bit 21 - Hyper Bus Controller Reset. Setting this bit to 1 resets the HBC clock."]
    #[inline(always)]
    pub fn hbc(&mut self) -> HBC_W {
        HBC_W { w: self }
    }
    #[doc = "Bit 22 - CLCD Reset. Setting this bit to 1 resets the CLCD clock."]
    #[inline(always)]
    pub fn clcd(&mut self) -> CLCD_W {
        CLCD_W { w: self }
    }
    #[doc = "Bit 23 - USB Reset. Setting this bit resets both USB blocks."]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W { w: self }
    }
    #[doc = "Bit 24 - TRNG Reset."]
    #[inline(always)]
    pub fn trng(&mut self) -> TRNG_W {
        TRNG_W { w: self }
    }
    #[doc = "Bit 25 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rsv_0x19(&mut self) -> RSV_0X19_W {
        RSV_0X19_W { w: self }
    }
    #[doc = "Bit 26 - Analog to Digital Reset."]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 28 - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W {
        UART2_W { w: self }
    }
    #[doc = "Bit 29 - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
    #[inline(always)]
    pub fn srst(&mut self) -> SRST_W {
        SRST_W { w: self }
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W {
        PRST_W { w: self }
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    pub fn system(&mut self) -> SYSTEM_W {
        SYSTEM_W { w: self }
    }
}
