#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CHEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHEN`"]
pub type CHEN_R = crate::R<bool, CHEN_A>;
impl CHEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN_A {
        match self.bits {
            false => CHEN_A::DIS,
            true => CHEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CHEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CHEN_A::EN
    }
}
#[doc = "Write proxy for field `CHEN`"]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CHEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CHEN_A::EN)
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
#[doc = "Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLDEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RLDEN`"]
pub type RLDEN_R = crate::R<bool, RLDEN_A>;
impl RLDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLDEN_A {
        match self.bits {
            false => RLDEN_A::DIS,
            true => RLDEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RLDEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RLDEN_A::EN
    }
}
#[doc = "Write proxy for field `RLDEN`"]
pub struct RLDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RLDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RLDEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RLDEN_A::EN)
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
#[doc = "DMA Priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRI_A {
    #[doc = "0: Highest Priority."]
    HIGH = 0,
    #[doc = "1: Medium High Priority."]
    MEDHIGH = 1,
    #[doc = "2: Medium Low Priority."]
    MEDLOW = 2,
    #[doc = "3: Lowest Priority."]
    LOW = 3,
}
impl From<PRI_A> for u8 {
    #[inline(always)]
    fn from(variant: PRI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRI`"]
pub type PRI_R = crate::R<u8, PRI_A>;
impl PRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRI_A {
        match self.bits {
            0 => PRI_A::HIGH,
            1 => PRI_A::MEDHIGH,
            2 => PRI_A::MEDLOW,
            3 => PRI_A::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PRI_A::HIGH
    }
    #[doc = "Checks if the value of the field is `MEDHIGH`"]
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        *self == PRI_A::MEDHIGH
    }
    #[doc = "Checks if the value of the field is `MEDLOW`"]
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        *self == PRI_A::MEDLOW
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PRI_A::LOW
    }
}
#[doc = "Write proxy for field `PRI`"]
pub struct PRI_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRI_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Highest Priority."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PRI_A::HIGH)
    }
    #[doc = "Medium High Priority."]
    #[inline(always)]
    pub fn med_high(self) -> &'a mut W {
        self.variant(PRI_A::MEDHIGH)
    }
    #[doc = "Medium Low Priority."]
    #[inline(always)]
    pub fn med_low(self) -> &'a mut W {
        self.variant(PRI_A::MEDLOW)
    }
    #[doc = "Lowest Priority."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PRI_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REQSEL_A {
    #[doc = "0: Memory To Memory"]
    MEMTOMEM = 0,
    #[doc = "1: SPI0 RX"]
    SPI0RX = 1,
    #[doc = "2: SPI1 RX"]
    SPI1RX = 2,
    #[doc = "3: SPI2 RX"]
    SPI2RX = 3,
    #[doc = "4: UART0 RX"]
    UART0RX = 4,
    #[doc = "5: UART1 RX"]
    UART1RX = 5,
    #[doc = "7: I2C0 RX"]
    I2C0RX = 7,
    #[doc = "8: I2C1 RX"]
    I2C1RX = 8,
    #[doc = "9: Analog-to-Digital Converter Channel"]
    ADC = 9,
    #[doc = "14: UART2 RX"]
    UART2RX = 14,
    #[doc = "15: SPI3 RX"]
    SPI3RX = 15,
    #[doc = "16: SPI MSS0 RX"]
    SPI_MSS0RX = 16,
    #[doc = "17: USB Endpoint 1 RX"]
    USBRXEP1 = 17,
    #[doc = "18: USB Endpoint 2 RX"]
    USBRXEP2 = 18,
    #[doc = "19: USB Endpoint 3 RX"]
    USBRXEP3 = 19,
    #[doc = "20: USB Endpoint 4 RX"]
    USBRXEP4 = 20,
    #[doc = "21: USB Endpoint 5 RX"]
    USBRXEP5 = 21,
    #[doc = "22: USB Endpoint 6 RX"]
    USBRXEP6 = 22,
    #[doc = "23: USB Endpoint 7 RX"]
    USBRXEP7 = 23,
    #[doc = "24: USB Endpoint 8 RX"]
    USBRXEP8 = 24,
    #[doc = "25: USB Endpoint 9 RX"]
    USBRXEP9 = 25,
    #[doc = "26: USB Endpoint 10 RX"]
    USBRXEP10 = 26,
    #[doc = "27: USB Endpoint 11 RX"]
    USBRXEP11 = 27,
    #[doc = "33: SPI0 TX"]
    SPI0TX = 33,
    #[doc = "34: SPI1 TX"]
    SPI1TX = 34,
    #[doc = "35: SPI2 TX"]
    SPI2TX = 35,
    #[doc = "36: UART0 TX"]
    UART0TX = 36,
    #[doc = "37: UART1 TX"]
    UART1TX = 37,
    #[doc = "39: I2C0 TX"]
    I2C0TX = 39,
    #[doc = "40: I2C1 TX"]
    I2C1TX = 40,
    #[doc = "46: UART2 TX"]
    UART2TX = 46,
    #[doc = "47: SPI3 TX"]
    SPI3TX = 47,
    #[doc = "48: SPI MSS0 TX"]
    SPI_MSS0TX = 48,
    #[doc = "49: USB Endpoint 1 TX"]
    USBTXEP1 = 49,
    #[doc = "50: USB Endpoint 2 TX"]
    USBTXEP2 = 50,
    #[doc = "51: USB Endpoint 3 TX"]
    USBTXEP3 = 51,
    #[doc = "52: USB Endpoint 4 TX"]
    USBTXEP4 = 52,
    #[doc = "53: USB Endpoint 5 TX"]
    USBTXEP5 = 53,
    #[doc = "54: USB Endpoint 6 TX"]
    USBTXEP6 = 54,
    #[doc = "55: USB Endpoint 7 TX"]
    USBTXEP7 = 55,
    #[doc = "56: USB Endpoint 8 TX"]
    USBTXEP8 = 56,
    #[doc = "57: USB Endpoint 9 TX"]
    USBTXEP9 = 57,
    #[doc = "58: USB Endpoint 10 TX"]
    USBTXEP10 = 58,
    #[doc = "59: USB Endpoint 11 TX"]
    USBTXEP11 = 59,
}
impl From<REQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REQSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REQSEL`"]
pub type REQSEL_R = crate::R<u8, REQSEL_A>;
impl REQSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REQSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REQSEL_A::MEMTOMEM),
            1 => Val(REQSEL_A::SPI0RX),
            2 => Val(REQSEL_A::SPI1RX),
            3 => Val(REQSEL_A::SPI2RX),
            4 => Val(REQSEL_A::UART0RX),
            5 => Val(REQSEL_A::UART1RX),
            7 => Val(REQSEL_A::I2C0RX),
            8 => Val(REQSEL_A::I2C1RX),
            9 => Val(REQSEL_A::ADC),
            14 => Val(REQSEL_A::UART2RX),
            15 => Val(REQSEL_A::SPI3RX),
            16 => Val(REQSEL_A::SPI_MSS0RX),
            17 => Val(REQSEL_A::USBRXEP1),
            18 => Val(REQSEL_A::USBRXEP2),
            19 => Val(REQSEL_A::USBRXEP3),
            20 => Val(REQSEL_A::USBRXEP4),
            21 => Val(REQSEL_A::USBRXEP5),
            22 => Val(REQSEL_A::USBRXEP6),
            23 => Val(REQSEL_A::USBRXEP7),
            24 => Val(REQSEL_A::USBRXEP8),
            25 => Val(REQSEL_A::USBRXEP9),
            26 => Val(REQSEL_A::USBRXEP10),
            27 => Val(REQSEL_A::USBRXEP11),
            33 => Val(REQSEL_A::SPI0TX),
            34 => Val(REQSEL_A::SPI1TX),
            35 => Val(REQSEL_A::SPI2TX),
            36 => Val(REQSEL_A::UART0TX),
            37 => Val(REQSEL_A::UART1TX),
            39 => Val(REQSEL_A::I2C0TX),
            40 => Val(REQSEL_A::I2C1TX),
            46 => Val(REQSEL_A::UART2TX),
            47 => Val(REQSEL_A::SPI3TX),
            48 => Val(REQSEL_A::SPI_MSS0TX),
            49 => Val(REQSEL_A::USBTXEP1),
            50 => Val(REQSEL_A::USBTXEP2),
            51 => Val(REQSEL_A::USBTXEP3),
            52 => Val(REQSEL_A::USBTXEP4),
            53 => Val(REQSEL_A::USBTXEP5),
            54 => Val(REQSEL_A::USBTXEP6),
            55 => Val(REQSEL_A::USBTXEP7),
            56 => Val(REQSEL_A::USBTXEP8),
            57 => Val(REQSEL_A::USBTXEP9),
            58 => Val(REQSEL_A::USBTXEP10),
            59 => Val(REQSEL_A::USBTXEP11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MEMTOMEM`"]
    #[inline(always)]
    pub fn is_memtomem(&self) -> bool {
        *self == REQSEL_A::MEMTOMEM
    }
    #[doc = "Checks if the value of the field is `SPI0RX`"]
    #[inline(always)]
    pub fn is_spi0rx(&self) -> bool {
        *self == REQSEL_A::SPI0RX
    }
    #[doc = "Checks if the value of the field is `SPI1RX`"]
    #[inline(always)]
    pub fn is_spi1rx(&self) -> bool {
        *self == REQSEL_A::SPI1RX
    }
    #[doc = "Checks if the value of the field is `SPI2RX`"]
    #[inline(always)]
    pub fn is_spi2rx(&self) -> bool {
        *self == REQSEL_A::SPI2RX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == REQSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == REQSEL_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `I2C0RX`"]
    #[inline(always)]
    pub fn is_i2c0rx(&self) -> bool {
        *self == REQSEL_A::I2C0RX
    }
    #[doc = "Checks if the value of the field is `I2C1RX`"]
    #[inline(always)]
    pub fn is_i2c1rx(&self) -> bool {
        *self == REQSEL_A::I2C1RX
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == REQSEL_A::ADC
    }
    #[doc = "Checks if the value of the field is `UART2RX`"]
    #[inline(always)]
    pub fn is_uart2rx(&self) -> bool {
        *self == REQSEL_A::UART2RX
    }
    #[doc = "Checks if the value of the field is `SPI3RX`"]
    #[inline(always)]
    pub fn is_spi3rx(&self) -> bool {
        *self == REQSEL_A::SPI3RX
    }
    #[doc = "Checks if the value of the field is `SPI_MSS0RX`"]
    #[inline(always)]
    pub fn is_spi_mss0rx(&self) -> bool {
        *self == REQSEL_A::SPI_MSS0RX
    }
    #[doc = "Checks if the value of the field is `USBRXEP1`"]
    #[inline(always)]
    pub fn is_usbrxep1(&self) -> bool {
        *self == REQSEL_A::USBRXEP1
    }
    #[doc = "Checks if the value of the field is `USBRXEP2`"]
    #[inline(always)]
    pub fn is_usbrxep2(&self) -> bool {
        *self == REQSEL_A::USBRXEP2
    }
    #[doc = "Checks if the value of the field is `USBRXEP3`"]
    #[inline(always)]
    pub fn is_usbrxep3(&self) -> bool {
        *self == REQSEL_A::USBRXEP3
    }
    #[doc = "Checks if the value of the field is `USBRXEP4`"]
    #[inline(always)]
    pub fn is_usbrxep4(&self) -> bool {
        *self == REQSEL_A::USBRXEP4
    }
    #[doc = "Checks if the value of the field is `USBRXEP5`"]
    #[inline(always)]
    pub fn is_usbrxep5(&self) -> bool {
        *self == REQSEL_A::USBRXEP5
    }
    #[doc = "Checks if the value of the field is `USBRXEP6`"]
    #[inline(always)]
    pub fn is_usbrxep6(&self) -> bool {
        *self == REQSEL_A::USBRXEP6
    }
    #[doc = "Checks if the value of the field is `USBRXEP7`"]
    #[inline(always)]
    pub fn is_usbrxep7(&self) -> bool {
        *self == REQSEL_A::USBRXEP7
    }
    #[doc = "Checks if the value of the field is `USBRXEP8`"]
    #[inline(always)]
    pub fn is_usbrxep8(&self) -> bool {
        *self == REQSEL_A::USBRXEP8
    }
    #[doc = "Checks if the value of the field is `USBRXEP9`"]
    #[inline(always)]
    pub fn is_usbrxep9(&self) -> bool {
        *self == REQSEL_A::USBRXEP9
    }
    #[doc = "Checks if the value of the field is `USBRXEP10`"]
    #[inline(always)]
    pub fn is_usbrxep10(&self) -> bool {
        *self == REQSEL_A::USBRXEP10
    }
    #[doc = "Checks if the value of the field is `USBRXEP11`"]
    #[inline(always)]
    pub fn is_usbrxep11(&self) -> bool {
        *self == REQSEL_A::USBRXEP11
    }
    #[doc = "Checks if the value of the field is `SPI0TX`"]
    #[inline(always)]
    pub fn is_spi0tx(&self) -> bool {
        *self == REQSEL_A::SPI0TX
    }
    #[doc = "Checks if the value of the field is `SPI1TX`"]
    #[inline(always)]
    pub fn is_spi1tx(&self) -> bool {
        *self == REQSEL_A::SPI1TX
    }
    #[doc = "Checks if the value of the field is `SPI2TX`"]
    #[inline(always)]
    pub fn is_spi2tx(&self) -> bool {
        *self == REQSEL_A::SPI2TX
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == REQSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == REQSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `I2C0TX`"]
    #[inline(always)]
    pub fn is_i2c0tx(&self) -> bool {
        *self == REQSEL_A::I2C0TX
    }
    #[doc = "Checks if the value of the field is `I2C1TX`"]
    #[inline(always)]
    pub fn is_i2c1tx(&self) -> bool {
        *self == REQSEL_A::I2C1TX
    }
    #[doc = "Checks if the value of the field is `UART2TX`"]
    #[inline(always)]
    pub fn is_uart2tx(&self) -> bool {
        *self == REQSEL_A::UART2TX
    }
    #[doc = "Checks if the value of the field is `SPI3TX`"]
    #[inline(always)]
    pub fn is_spi3tx(&self) -> bool {
        *self == REQSEL_A::SPI3TX
    }
    #[doc = "Checks if the value of the field is `SPI_MSS0TX`"]
    #[inline(always)]
    pub fn is_spi_mss0tx(&self) -> bool {
        *self == REQSEL_A::SPI_MSS0TX
    }
    #[doc = "Checks if the value of the field is `USBTXEP1`"]
    #[inline(always)]
    pub fn is_usbtxep1(&self) -> bool {
        *self == REQSEL_A::USBTXEP1
    }
    #[doc = "Checks if the value of the field is `USBTXEP2`"]
    #[inline(always)]
    pub fn is_usbtxep2(&self) -> bool {
        *self == REQSEL_A::USBTXEP2
    }
    #[doc = "Checks if the value of the field is `USBTXEP3`"]
    #[inline(always)]
    pub fn is_usbtxep3(&self) -> bool {
        *self == REQSEL_A::USBTXEP3
    }
    #[doc = "Checks if the value of the field is `USBTXEP4`"]
    #[inline(always)]
    pub fn is_usbtxep4(&self) -> bool {
        *self == REQSEL_A::USBTXEP4
    }
    #[doc = "Checks if the value of the field is `USBTXEP5`"]
    #[inline(always)]
    pub fn is_usbtxep5(&self) -> bool {
        *self == REQSEL_A::USBTXEP5
    }
    #[doc = "Checks if the value of the field is `USBTXEP6`"]
    #[inline(always)]
    pub fn is_usbtxep6(&self) -> bool {
        *self == REQSEL_A::USBTXEP6
    }
    #[doc = "Checks if the value of the field is `USBTXEP7`"]
    #[inline(always)]
    pub fn is_usbtxep7(&self) -> bool {
        *self == REQSEL_A::USBTXEP7
    }
    #[doc = "Checks if the value of the field is `USBTXEP8`"]
    #[inline(always)]
    pub fn is_usbtxep8(&self) -> bool {
        *self == REQSEL_A::USBTXEP8
    }
    #[doc = "Checks if the value of the field is `USBTXEP9`"]
    #[inline(always)]
    pub fn is_usbtxep9(&self) -> bool {
        *self == REQSEL_A::USBTXEP9
    }
    #[doc = "Checks if the value of the field is `USBTXEP10`"]
    #[inline(always)]
    pub fn is_usbtxep10(&self) -> bool {
        *self == REQSEL_A::USBTXEP10
    }
    #[doc = "Checks if the value of the field is `USBTXEP11`"]
    #[inline(always)]
    pub fn is_usbtxep11(&self) -> bool {
        *self == REQSEL_A::USBTXEP11
    }
}
#[doc = "Write proxy for field `REQSEL`"]
pub struct REQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Memory To Memory"]
    #[inline(always)]
    pub fn memtomem(self) -> &'a mut W {
        self.variant(REQSEL_A::MEMTOMEM)
    }
    #[doc = "SPI0 RX"]
    #[inline(always)]
    pub fn spi0rx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI0RX)
    }
    #[doc = "SPI1 RX"]
    #[inline(always)]
    pub fn spi1rx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI1RX)
    }
    #[doc = "SPI2 RX"]
    #[inline(always)]
    pub fn spi2rx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI2RX)
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART0RX)
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART1RX)
    }
    #[doc = "I2C0 RX"]
    #[inline(always)]
    pub fn i2c0rx(self) -> &'a mut W {
        self.variant(REQSEL_A::I2C0RX)
    }
    #[doc = "I2C1 RX"]
    #[inline(always)]
    pub fn i2c1rx(self) -> &'a mut W {
        self.variant(REQSEL_A::I2C1RX)
    }
    #[doc = "Analog-to-Digital Converter Channel"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(REQSEL_A::ADC)
    }
    #[doc = "UART2 RX"]
    #[inline(always)]
    pub fn uart2rx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART2RX)
    }
    #[doc = "SPI3 RX"]
    #[inline(always)]
    pub fn spi3rx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI3RX)
    }
    #[doc = "SPI MSS0 RX"]
    #[inline(always)]
    pub fn spi_mss0rx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI_MSS0RX)
    }
    #[doc = "USB Endpoint 1 RX"]
    #[inline(always)]
    pub fn usbrxep1(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP1)
    }
    #[doc = "USB Endpoint 2 RX"]
    #[inline(always)]
    pub fn usbrxep2(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP2)
    }
    #[doc = "USB Endpoint 3 RX"]
    #[inline(always)]
    pub fn usbrxep3(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP3)
    }
    #[doc = "USB Endpoint 4 RX"]
    #[inline(always)]
    pub fn usbrxep4(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP4)
    }
    #[doc = "USB Endpoint 5 RX"]
    #[inline(always)]
    pub fn usbrxep5(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP5)
    }
    #[doc = "USB Endpoint 6 RX"]
    #[inline(always)]
    pub fn usbrxep6(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP6)
    }
    #[doc = "USB Endpoint 7 RX"]
    #[inline(always)]
    pub fn usbrxep7(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP7)
    }
    #[doc = "USB Endpoint 8 RX"]
    #[inline(always)]
    pub fn usbrxep8(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP8)
    }
    #[doc = "USB Endpoint 9 RX"]
    #[inline(always)]
    pub fn usbrxep9(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP9)
    }
    #[doc = "USB Endpoint 10 RX"]
    #[inline(always)]
    pub fn usbrxep10(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP10)
    }
    #[doc = "USB Endpoint 11 RX"]
    #[inline(always)]
    pub fn usbrxep11(self) -> &'a mut W {
        self.variant(REQSEL_A::USBRXEP11)
    }
    #[doc = "SPI0 TX"]
    #[inline(always)]
    pub fn spi0tx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI0TX)
    }
    #[doc = "SPI1 TX"]
    #[inline(always)]
    pub fn spi1tx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI1TX)
    }
    #[doc = "SPI2 TX"]
    #[inline(always)]
    pub fn spi2tx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI2TX)
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART0TX)
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART1TX)
    }
    #[doc = "I2C0 TX"]
    #[inline(always)]
    pub fn i2c0tx(self) -> &'a mut W {
        self.variant(REQSEL_A::I2C0TX)
    }
    #[doc = "I2C1 TX"]
    #[inline(always)]
    pub fn i2c1tx(self) -> &'a mut W {
        self.variant(REQSEL_A::I2C1TX)
    }
    #[doc = "UART2 TX"]
    #[inline(always)]
    pub fn uart2tx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART2TX)
    }
    #[doc = "SPI3 TX"]
    #[inline(always)]
    pub fn spi3tx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI3TX)
    }
    #[doc = "SPI MSS0 TX"]
    #[inline(always)]
    pub fn spi_mss0tx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI_MSS0TX)
    }
    #[doc = "USB Endpoint 1 TX"]
    #[inline(always)]
    pub fn usbtxep1(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP1)
    }
    #[doc = "USB Endpoint 2 TX"]
    #[inline(always)]
    pub fn usbtxep2(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP2)
    }
    #[doc = "USB Endpoint 3 TX"]
    #[inline(always)]
    pub fn usbtxep3(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP3)
    }
    #[doc = "USB Endpoint 4 TX"]
    #[inline(always)]
    pub fn usbtxep4(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP4)
    }
    #[doc = "USB Endpoint 5 TX"]
    #[inline(always)]
    pub fn usbtxep5(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP5)
    }
    #[doc = "USB Endpoint 6 TX"]
    #[inline(always)]
    pub fn usbtxep6(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP6)
    }
    #[doc = "USB Endpoint 7 TX"]
    #[inline(always)]
    pub fn usbtxep7(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP7)
    }
    #[doc = "USB Endpoint 8 TX"]
    #[inline(always)]
    pub fn usbtxep8(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP8)
    }
    #[doc = "USB Endpoint 9 TX"]
    #[inline(always)]
    pub fn usbtxep9(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP9)
    }
    #[doc = "USB Endpoint 10 TX"]
    #[inline(always)]
    pub fn usbtxep10(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP10)
    }
    #[doc = "USB Endpoint 11 TX"]
    #[inline(always)]
    pub fn usbtxep11(self) -> &'a mut W {
        self.variant(REQSEL_A::USBTXEP11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQWAIT_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<REQWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: REQWAIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REQWAIT`"]
pub type REQWAIT_R = crate::R<bool, REQWAIT_A>;
impl REQWAIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQWAIT_A {
        match self.bits {
            false => REQWAIT_A::DIS,
            true => REQWAIT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == REQWAIT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == REQWAIT_A::EN
    }
}
#[doc = "Write proxy for field `REQWAIT`"]
pub struct REQWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> REQWAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQWAIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(REQWAIT_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(REQWAIT_A::EN)
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
#[doc = "Time-Out Select. Selects the number of prescale clocks seen by the channel timer before a time-out conditions is generated for this channel. Important note: since the prescaler runs independent of the individual channel timers, the actual number of Pre-Scale clock edges seen has a margin of error equal to a single Pre-Scale clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOSEL_A {
    #[doc = "0: Timeout of 3 to 4 prescale clocks."]
    TO4 = 0,
    #[doc = "1: Timeout of 7 to 8 prescale clocks."]
    TO8 = 1,
    #[doc = "2: Timeout of 15 to 16 prescale clocks."]
    TO16 = 2,
    #[doc = "3: Timeout of 31 to 32 prescale clocks."]
    TO32 = 3,
    #[doc = "4: Timeout of 63 to 64 prescale clocks."]
    TO64 = 4,
    #[doc = "5: Timeout of 127 to 128 prescale clocks."]
    TO128 = 5,
    #[doc = "6: Timeout of 255 to 256 prescale clocks."]
    TO256 = 6,
    #[doc = "7: Timeout of 511 to 512 prescale clocks."]
    TO512 = 7,
}
impl From<TOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TOSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TOSEL`"]
pub type TOSEL_R = crate::R<u8, TOSEL_A>;
impl TOSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOSEL_A {
        match self.bits {
            0 => TOSEL_A::TO4,
            1 => TOSEL_A::TO8,
            2 => TOSEL_A::TO16,
            3 => TOSEL_A::TO32,
            4 => TOSEL_A::TO64,
            5 => TOSEL_A::TO128,
            6 => TOSEL_A::TO256,
            7 => TOSEL_A::TO512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TO4`"]
    #[inline(always)]
    pub fn is_to4(&self) -> bool {
        *self == TOSEL_A::TO4
    }
    #[doc = "Checks if the value of the field is `TO8`"]
    #[inline(always)]
    pub fn is_to8(&self) -> bool {
        *self == TOSEL_A::TO8
    }
    #[doc = "Checks if the value of the field is `TO16`"]
    #[inline(always)]
    pub fn is_to16(&self) -> bool {
        *self == TOSEL_A::TO16
    }
    #[doc = "Checks if the value of the field is `TO32`"]
    #[inline(always)]
    pub fn is_to32(&self) -> bool {
        *self == TOSEL_A::TO32
    }
    #[doc = "Checks if the value of the field is `TO64`"]
    #[inline(always)]
    pub fn is_to64(&self) -> bool {
        *self == TOSEL_A::TO64
    }
    #[doc = "Checks if the value of the field is `TO128`"]
    #[inline(always)]
    pub fn is_to128(&self) -> bool {
        *self == TOSEL_A::TO128
    }
    #[doc = "Checks if the value of the field is `TO256`"]
    #[inline(always)]
    pub fn is_to256(&self) -> bool {
        *self == TOSEL_A::TO256
    }
    #[doc = "Checks if the value of the field is `TO512`"]
    #[inline(always)]
    pub fn is_to512(&self) -> bool {
        *self == TOSEL_A::TO512
    }
}
#[doc = "Write proxy for field `TOSEL`"]
pub struct TOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timeout of 3 to 4 prescale clocks."]
    #[inline(always)]
    pub fn to4(self) -> &'a mut W {
        self.variant(TOSEL_A::TO4)
    }
    #[doc = "Timeout of 7 to 8 prescale clocks."]
    #[inline(always)]
    pub fn to8(self) -> &'a mut W {
        self.variant(TOSEL_A::TO8)
    }
    #[doc = "Timeout of 15 to 16 prescale clocks."]
    #[inline(always)]
    pub fn to16(self) -> &'a mut W {
        self.variant(TOSEL_A::TO16)
    }
    #[doc = "Timeout of 31 to 32 prescale clocks."]
    #[inline(always)]
    pub fn to32(self) -> &'a mut W {
        self.variant(TOSEL_A::TO32)
    }
    #[doc = "Timeout of 63 to 64 prescale clocks."]
    #[inline(always)]
    pub fn to64(self) -> &'a mut W {
        self.variant(TOSEL_A::TO64)
    }
    #[doc = "Timeout of 127 to 128 prescale clocks."]
    #[inline(always)]
    pub fn to128(self) -> &'a mut W {
        self.variant(TOSEL_A::TO128)
    }
    #[doc = "Timeout of 255 to 256 prescale clocks."]
    #[inline(always)]
    pub fn to256(self) -> &'a mut W {
        self.variant(TOSEL_A::TO256)
    }
    #[doc = "Timeout of 511 to 512 prescale clocks."]
    #[inline(always)]
    pub fn to512(self) -> &'a mut W {
        self.variant(TOSEL_A::TO512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pre-Scale Select. Selects the Pre-Scale divider for timer clock input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSSEL_A {
    #[doc = "0: Disable timer."]
    DIS = 0,
    #[doc = "1: hclk / 256."]
    DIV256 = 1,
    #[doc = "2: hclk / 64k."]
    DIV64K = 2,
    #[doc = "3: hclk / 16M."]
    DIV16M = 3,
}
impl From<PSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSSEL`"]
pub type PSSEL_R = crate::R<u8, PSSEL_A>;
impl PSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSSEL_A {
        match self.bits {
            0 => PSSEL_A::DIS,
            1 => PSSEL_A::DIV256,
            2 => PSSEL_A::DIV64K,
            3 => PSSEL_A::DIV16M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PSSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PSSEL_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV64K`"]
    #[inline(always)]
    pub fn is_div64k(&self) -> bool {
        *self == PSSEL_A::DIV64K
    }
    #[doc = "Checks if the value of the field is `DIV16M`"]
    #[inline(always)]
    pub fn is_div16m(&self) -> bool {
        *self == PSSEL_A::DIV16M
    }
}
#[doc = "Write proxy for field `PSSEL`"]
pub struct PSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable timer."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PSSEL_A::DIS)
    }
    #[doc = "hclk / 256."]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PSSEL_A::DIV256)
    }
    #[doc = "hclk / 64k."]
    #[inline(always)]
    pub fn div64k(self) -> &'a mut W {
        self.variant(PSSEL_A::DIV64K)
    }
    #[doc = "hclk / 16M."]
    #[inline(always)]
    pub fn div16m(self) -> &'a mut W {
        self.variant(PSSEL_A::DIV16M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRCWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Halfword."]
    HALFWORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<SRCWD_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCWD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRCWD`"]
pub type SRCWD_R = crate::R<u8, SRCWD_A>;
impl SRCWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRCWD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRCWD_A::BYTE),
            1 => Val(SRCWD_A::HALFWORD),
            2 => Val(SRCWD_A::WORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SRCWD_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SRCWD_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SRCWD_A::WORD
    }
}
#[doc = "Write proxy for field `SRCWD`"]
pub struct SRCWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCWD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SRCWD_A::BYTE)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SRCWD_A::HALFWORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SRCWD_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCINC_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<SRCINC_A> for bool {
    #[inline(always)]
    fn from(variant: SRCINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRCINC`"]
pub type SRCINC_R = crate::R<bool, SRCINC_A>;
impl SRCINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCINC_A {
        match self.bits {
            false => SRCINC_A::DIS,
            true => SRCINC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SRCINC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SRCINC_A::EN
    }
}
#[doc = "Write proxy for field `SRCINC`"]
pub struct SRCINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRCINC_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SRCINC_A::EN)
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
#[doc = "Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSTWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Halfword."]
    HALFWORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<DSTWD_A> for u8 {
    #[inline(always)]
    fn from(variant: DSTWD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSTWD`"]
pub type DSTWD_R = crate::R<u8, DSTWD_A>;
impl DSTWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSTWD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSTWD_A::BYTE),
            1 => Val(DSTWD_A::HALFWORD),
            2 => Val(DSTWD_A::WORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DSTWD_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DSTWD_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DSTWD_A::WORD
    }
}
#[doc = "Write proxy for field `DSTWD`"]
pub struct DSTWD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTWD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DSTWD_A::BYTE)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(DSTWD_A::HALFWORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DSTWD_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTINC_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<DSTINC_A> for bool {
    #[inline(always)]
    fn from(variant: DSTINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSTINC`"]
pub type DSTINC_R = crate::R<bool, DSTINC_A>;
impl DSTINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTINC_A {
        match self.bits {
            false => DSTINC_A::DIS,
            true => DSTINC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DSTINC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DSTINC_A::EN
    }
}
#[doc = "Write proxy for field `DSTINC`"]
pub struct DSTINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DSTINC_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DSTINC_A::EN)
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
#[doc = "Reader of field `BRST`"]
pub type BRST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRST`"]
pub struct BRST_W<'a> {
    w: &'a mut W,
}
impl<'a> BRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHDIEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CHDIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHDIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHDIEN`"]
pub type CHDIEN_R = crate::R<bool, CHDIEN_A>;
impl CHDIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHDIEN_A {
        match self.bits {
            false => CHDIEN_A::DIS,
            true => CHDIEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CHDIEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CHDIEN_A::EN
    }
}
#[doc = "Write proxy for field `CHDIEN`"]
pub struct CHDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHDIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CHDIEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CHDIEN_A::EN)
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
#[doc = "Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTZIEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CTZIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTZIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTZIEN`"]
pub type CTZIEN_R = crate::R<bool, CTZIEN_A>;
impl CTZIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTZIEN_A {
        match self.bits {
            false => CTZIEN_A::DIS,
            true => CTZIEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CTZIEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CTZIEN_A::EN
    }
}
#[doc = "Write proxy for field `CTZIEN`"]
pub struct CTZIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTZIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTZIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CTZIEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CTZIEN_A::EN)
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
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    pub fn reqwait(&self) -> REQWAIT_R {
        REQWAIT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Time-Out Select. Selects the number of prescale clocks seen by the channel timer before a time-out conditions is generated for this channel. Important note: since the prescaler runs independent of the individual channel timers, the actual number of Pre-Scale clock edges seen has a margin of error equal to a single Pre-Scale clock."]
    #[inline(always)]
    pub fn tosel(&self) -> TOSEL_R {
        TOSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    pub fn pssel(&self) -> PSSEL_R {
        PSSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    pub fn srcwd(&self) -> SRCWD_R {
        SRCWD_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    pub fn dstwd(&self) -> DSTWD_R {
        DSTWD_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    pub fn brst(&self) -> BRST_R {
        BRST_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn chdien(&self) -> CHDIEN_R {
        CHDIEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    pub fn ctzien(&self) -> CTZIEN_R {
        CTZIEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    pub fn rlden(&mut self) -> RLDEN_W {
        RLDEN_W { w: self }
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    pub fn pri(&mut self) -> PRI_W {
        PRI_W { w: self }
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    pub fn reqsel(&mut self) -> REQSEL_W {
        REQSEL_W { w: self }
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    pub fn reqwait(&mut self) -> REQWAIT_W {
        REQWAIT_W { w: self }
    }
    #[doc = "Bits 11:13 - Time-Out Select. Selects the number of prescale clocks seen by the channel timer before a time-out conditions is generated for this channel. Important note: since the prescaler runs independent of the individual channel timers, the actual number of Pre-Scale clock edges seen has a margin of error equal to a single Pre-Scale clock."]
    #[inline(always)]
    pub fn tosel(&mut self) -> TOSEL_W {
        TOSEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    pub fn pssel(&mut self) -> PSSEL_W {
        PSSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    pub fn srcwd(&mut self) -> SRCWD_W {
        SRCWD_W { w: self }
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    pub fn srcinc(&mut self) -> SRCINC_W {
        SRCINC_W { w: self }
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    pub fn dstwd(&mut self) -> DSTWD_W {
        DSTWD_W { w: self }
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    pub fn dstinc(&mut self) -> DSTINC_W {
        DSTINC_W { w: self }
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    pub fn brst(&mut self) -> BRST_W {
        BRST_W { w: self }
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn chdien(&mut self) -> CHDIEN_W {
        CHDIEN_W { w: self }
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    pub fn ctzien(&mut self) -> CTZIEN_W {
        CTZIEN_W { w: self }
    }
}
