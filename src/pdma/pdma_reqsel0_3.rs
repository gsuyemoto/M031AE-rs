#[doc = "Register `PDMA_REQSEL0_3` reader"]
pub struct R(crate::R<PDMA_REQSEL0_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_REQSEL0_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_REQSEL0_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_REQSEL0_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_REQSEL0_3` writer"]
pub struct W(crate::W<PDMA_REQSEL0_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_REQSEL0_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PDMA_REQSEL0_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_REQSEL0_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 0. User can configure the peripheral by setting REQSRC0.\n\nNote 1: A peripheral cannot be assigned to two channels at the same time.\nNote 2: This field is useless when transfer between memory and memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REQSRC0_A {
    #[doc = "0: Disable PDMA peripheral request"]
    _0 = 0,
    #[doc = "1: Reserved."]
    _1 = 1,
    #[doc = "4: Channel connects to UART0_TX"]
    _4 = 4,
    #[doc = "5: Channel connects to UART0_RX"]
    _5 = 5,
    #[doc = "6: Channel connects to UART1_TX"]
    _6 = 6,
    #[doc = "7: Channel connects to UART1_RX"]
    _7 = 7,
    #[doc = "8: Channel connects to UART2_TX"]
    _8 = 8,
    #[doc = "9: Channel connects to UART2_RX"]
    _9 = 9,
    #[doc = "10: Channel connects to USCI0_TX"]
    _10 = 10,
    #[doc = "11: Channel connects to USCI0_RX"]
    _11 = 11,
    #[doc = "12: Channel connects to USCI1_TX"]
    _12 = 12,
    #[doc = "13: Channel connects to USCI1_RX"]
    _13 = 13,
    #[doc = "14: Reserved."]
    _14 = 14,
    #[doc = "15: Reserved."]
    _15 = 15,
    #[doc = "16: Channel connects to QSPI0_TX"]
    _16 = 16,
    #[doc = "17: Channel connects to QSPI0_RX"]
    _17 = 17,
    #[doc = "18: Channel connects to SPI0_TX"]
    _18 = 18,
    #[doc = "19: Channel connects to SPI0_RX"]
    _19 = 19,
    #[doc = "20: Channel connects to ADC_RX"]
    _20 = 20,
    #[doc = "21: Channel connects to PWM0_P1_RX"]
    _21 = 21,
    #[doc = "22: Channel connects to PWM0_P2_RX"]
    _22 = 22,
    #[doc = "23: Channel connects to PWM0_P3_RX"]
    _23 = 23,
    #[doc = "24: Channel connects to PWM1_P1_RX"]
    _24 = 24,
    #[doc = "25: Channel connects to PWM1_P2_RX"]
    _25 = 25,
    #[doc = "26: Channel connects to PWM1_P3_RX"]
    _26 = 26,
    #[doc = "27: Reserved."]
    _27 = 27,
    #[doc = "28: Channel connects to I2C0_TX"]
    _28 = 28,
    #[doc = "29: Channel connects to I2C0_RX"]
    _29 = 29,
    #[doc = "30: Channel connects to I2C1_TX"]
    _30 = 30,
    #[doc = "31: Channel connects to I2C1_RX"]
    _31 = 31,
    #[doc = "32: Channel connects to TMR0"]
    _32 = 32,
    #[doc = "33: Channel connects to TMR1"]
    _33 = 33,
    #[doc = "34: Channel connects to TMR2"]
    _34 = 34,
    #[doc = "35: Channel connects to TMR3"]
    _35 = 35,
    #[doc = "36: Channel connects to UART3_TX"]
    _36 = 36,
    #[doc = "37: Channel connects to UART3_RX"]
    _37 = 37,
    #[doc = "38: Channel connects to UART4_TX"]
    _38 = 38,
    #[doc = "39: Channel connects to UART4_RX"]
    _39 = 39,
    #[doc = "40: Channel connects to UART5_TX"]
    _40 = 40,
    #[doc = "41: Channel connects to UART5_RX"]
    _41 = 41,
    #[doc = "42: Channel connects to UART6_TX"]
    _42 = 42,
    #[doc = "43: Channel connects to UART6_RX"]
    _43 = 43,
    #[doc = "44: Channel connects to UART7_TX"]
    _44 = 44,
    #[doc = "45: Channel connects to UART7_RX"]
    _45 = 45,
}
impl From<REQSRC0_A> for u8 {
    #[inline(always)]
    fn from(variant: REQSRC0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REQSRC0` reader - Channel 0 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 0. User can configure the peripheral by setting REQSRC0.\n\nNote 1: A peripheral cannot be assigned to two channels at the same time.\nNote 2: This field is useless when transfer between memory and memory."]
pub struct REQSRC0_R(crate::FieldReader<u8, REQSRC0_A>);
impl REQSRC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REQSRC0_A> {
        match self.bits {
            0 => Some(REQSRC0_A::_0),
            1 => Some(REQSRC0_A::_1),
            4 => Some(REQSRC0_A::_4),
            5 => Some(REQSRC0_A::_5),
            6 => Some(REQSRC0_A::_6),
            7 => Some(REQSRC0_A::_7),
            8 => Some(REQSRC0_A::_8),
            9 => Some(REQSRC0_A::_9),
            10 => Some(REQSRC0_A::_10),
            11 => Some(REQSRC0_A::_11),
            12 => Some(REQSRC0_A::_12),
            13 => Some(REQSRC0_A::_13),
            14 => Some(REQSRC0_A::_14),
            15 => Some(REQSRC0_A::_15),
            16 => Some(REQSRC0_A::_16),
            17 => Some(REQSRC0_A::_17),
            18 => Some(REQSRC0_A::_18),
            19 => Some(REQSRC0_A::_19),
            20 => Some(REQSRC0_A::_20),
            21 => Some(REQSRC0_A::_21),
            22 => Some(REQSRC0_A::_22),
            23 => Some(REQSRC0_A::_23),
            24 => Some(REQSRC0_A::_24),
            25 => Some(REQSRC0_A::_25),
            26 => Some(REQSRC0_A::_26),
            27 => Some(REQSRC0_A::_27),
            28 => Some(REQSRC0_A::_28),
            29 => Some(REQSRC0_A::_29),
            30 => Some(REQSRC0_A::_30),
            31 => Some(REQSRC0_A::_31),
            32 => Some(REQSRC0_A::_32),
            33 => Some(REQSRC0_A::_33),
            34 => Some(REQSRC0_A::_34),
            35 => Some(REQSRC0_A::_35),
            36 => Some(REQSRC0_A::_36),
            37 => Some(REQSRC0_A::_37),
            38 => Some(REQSRC0_A::_38),
            39 => Some(REQSRC0_A::_39),
            40 => Some(REQSRC0_A::_40),
            41 => Some(REQSRC0_A::_41),
            42 => Some(REQSRC0_A::_42),
            43 => Some(REQSRC0_A::_43),
            44 => Some(REQSRC0_A::_44),
            45 => Some(REQSRC0_A::_45),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSRC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSRC0_A::_1
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == REQSRC0_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == REQSRC0_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == REQSRC0_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == REQSRC0_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == REQSRC0_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == REQSRC0_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == REQSRC0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == REQSRC0_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == REQSRC0_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == REQSRC0_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == REQSRC0_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == REQSRC0_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == REQSRC0_A::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        **self == REQSRC0_A::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        **self == REQSRC0_A::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline(always)]
    pub fn is_19(&self) -> bool {
        **self == REQSRC0_A::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        **self == REQSRC0_A::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline(always)]
    pub fn is_21(&self) -> bool {
        **self == REQSRC0_A::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline(always)]
    pub fn is_22(&self) -> bool {
        **self == REQSRC0_A::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline(always)]
    pub fn is_23(&self) -> bool {
        **self == REQSRC0_A::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        **self == REQSRC0_A::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        **self == REQSRC0_A::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        **self == REQSRC0_A::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        **self == REQSRC0_A::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        **self == REQSRC0_A::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        **self == REQSRC0_A::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        **self == REQSRC0_A::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        **self == REQSRC0_A::_31
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        **self == REQSRC0_A::_32
    }
    #[doc = "Checks if the value of the field is `_33`"]
    #[inline(always)]
    pub fn is_33(&self) -> bool {
        **self == REQSRC0_A::_33
    }
    #[doc = "Checks if the value of the field is `_34`"]
    #[inline(always)]
    pub fn is_34(&self) -> bool {
        **self == REQSRC0_A::_34
    }
    #[doc = "Checks if the value of the field is `_35`"]
    #[inline(always)]
    pub fn is_35(&self) -> bool {
        **self == REQSRC0_A::_35
    }
    #[doc = "Checks if the value of the field is `_36`"]
    #[inline(always)]
    pub fn is_36(&self) -> bool {
        **self == REQSRC0_A::_36
    }
    #[doc = "Checks if the value of the field is `_37`"]
    #[inline(always)]
    pub fn is_37(&self) -> bool {
        **self == REQSRC0_A::_37
    }
    #[doc = "Checks if the value of the field is `_38`"]
    #[inline(always)]
    pub fn is_38(&self) -> bool {
        **self == REQSRC0_A::_38
    }
    #[doc = "Checks if the value of the field is `_39`"]
    #[inline(always)]
    pub fn is_39(&self) -> bool {
        **self == REQSRC0_A::_39
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        **self == REQSRC0_A::_40
    }
    #[doc = "Checks if the value of the field is `_41`"]
    #[inline(always)]
    pub fn is_41(&self) -> bool {
        **self == REQSRC0_A::_41
    }
    #[doc = "Checks if the value of the field is `_42`"]
    #[inline(always)]
    pub fn is_42(&self) -> bool {
        **self == REQSRC0_A::_42
    }
    #[doc = "Checks if the value of the field is `_43`"]
    #[inline(always)]
    pub fn is_43(&self) -> bool {
        **self == REQSRC0_A::_43
    }
    #[doc = "Checks if the value of the field is `_44`"]
    #[inline(always)]
    pub fn is_44(&self) -> bool {
        **self == REQSRC0_A::_44
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline(always)]
    pub fn is_45(&self) -> bool {
        **self == REQSRC0_A::_45
    }
}
impl core::ops::Deref for REQSRC0_R {
    type Target = crate::FieldReader<u8, REQSRC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC0` writer - Channel 0 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 0. User can configure the peripheral by setting REQSRC0.\n\nNote 1: A peripheral cannot be assigned to two channels at the same time.\nNote 2: This field is useless when transfer between memory and memory."]
pub struct REQSRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQSRC0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable PDMA peripheral request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REQSRC0_A::_0)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REQSRC0_A::_1)
    }
    #[doc = "Channel connects to UART0_TX"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(REQSRC0_A::_4)
    }
    #[doc = "Channel connects to UART0_RX"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(REQSRC0_A::_5)
    }
    #[doc = "Channel connects to UART1_TX"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(REQSRC0_A::_6)
    }
    #[doc = "Channel connects to UART1_RX"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(REQSRC0_A::_7)
    }
    #[doc = "Channel connects to UART2_TX"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(REQSRC0_A::_8)
    }
    #[doc = "Channel connects to UART2_RX"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(REQSRC0_A::_9)
    }
    #[doc = "Channel connects to USCI0_TX"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(REQSRC0_A::_10)
    }
    #[doc = "Channel connects to USCI0_RX"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(REQSRC0_A::_11)
    }
    #[doc = "Channel connects to USCI1_TX"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(REQSRC0_A::_12)
    }
    #[doc = "Channel connects to USCI1_RX"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(REQSRC0_A::_13)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(REQSRC0_A::_14)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(REQSRC0_A::_15)
    }
    #[doc = "Channel connects to QSPI0_TX"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(REQSRC0_A::_16)
    }
    #[doc = "Channel connects to QSPI0_RX"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut W {
        self.variant(REQSRC0_A::_17)
    }
    #[doc = "Channel connects to SPI0_TX"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(REQSRC0_A::_18)
    }
    #[doc = "Channel connects to SPI0_RX"]
    #[inline(always)]
    pub fn _19(self) -> &'a mut W {
        self.variant(REQSRC0_A::_19)
    }
    #[doc = "Channel connects to ADC_RX"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(REQSRC0_A::_20)
    }
    #[doc = "Channel connects to PWM0_P1_RX"]
    #[inline(always)]
    pub fn _21(self) -> &'a mut W {
        self.variant(REQSRC0_A::_21)
    }
    #[doc = "Channel connects to PWM0_P2_RX"]
    #[inline(always)]
    pub fn _22(self) -> &'a mut W {
        self.variant(REQSRC0_A::_22)
    }
    #[doc = "Channel connects to PWM0_P3_RX"]
    #[inline(always)]
    pub fn _23(self) -> &'a mut W {
        self.variant(REQSRC0_A::_23)
    }
    #[doc = "Channel connects to PWM1_P1_RX"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(REQSRC0_A::_24)
    }
    #[doc = "Channel connects to PWM1_P2_RX"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut W {
        self.variant(REQSRC0_A::_25)
    }
    #[doc = "Channel connects to PWM1_P3_RX"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut W {
        self.variant(REQSRC0_A::_26)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _27(self) -> &'a mut W {
        self.variant(REQSRC0_A::_27)
    }
    #[doc = "Channel connects to I2C0_TX"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut W {
        self.variant(REQSRC0_A::_28)
    }
    #[doc = "Channel connects to I2C0_RX"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut W {
        self.variant(REQSRC0_A::_29)
    }
    #[doc = "Channel connects to I2C1_TX"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(REQSRC0_A::_30)
    }
    #[doc = "Channel connects to I2C1_RX"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(REQSRC0_A::_31)
    }
    #[doc = "Channel connects to TMR0"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(REQSRC0_A::_32)
    }
    #[doc = "Channel connects to TMR1"]
    #[inline(always)]
    pub fn _33(self) -> &'a mut W {
        self.variant(REQSRC0_A::_33)
    }
    #[doc = "Channel connects to TMR2"]
    #[inline(always)]
    pub fn _34(self) -> &'a mut W {
        self.variant(REQSRC0_A::_34)
    }
    #[doc = "Channel connects to TMR3"]
    #[inline(always)]
    pub fn _35(self) -> &'a mut W {
        self.variant(REQSRC0_A::_35)
    }
    #[doc = "Channel connects to UART3_TX"]
    #[inline(always)]
    pub fn _36(self) -> &'a mut W {
        self.variant(REQSRC0_A::_36)
    }
    #[doc = "Channel connects to UART3_RX"]
    #[inline(always)]
    pub fn _37(self) -> &'a mut W {
        self.variant(REQSRC0_A::_37)
    }
    #[doc = "Channel connects to UART4_TX"]
    #[inline(always)]
    pub fn _38(self) -> &'a mut W {
        self.variant(REQSRC0_A::_38)
    }
    #[doc = "Channel connects to UART4_RX"]
    #[inline(always)]
    pub fn _39(self) -> &'a mut W {
        self.variant(REQSRC0_A::_39)
    }
    #[doc = "Channel connects to UART5_TX"]
    #[inline(always)]
    pub fn _40(self) -> &'a mut W {
        self.variant(REQSRC0_A::_40)
    }
    #[doc = "Channel connects to UART5_RX"]
    #[inline(always)]
    pub fn _41(self) -> &'a mut W {
        self.variant(REQSRC0_A::_41)
    }
    #[doc = "Channel connects to UART6_TX"]
    #[inline(always)]
    pub fn _42(self) -> &'a mut W {
        self.variant(REQSRC0_A::_42)
    }
    #[doc = "Channel connects to UART6_RX"]
    #[inline(always)]
    pub fn _43(self) -> &'a mut W {
        self.variant(REQSRC0_A::_43)
    }
    #[doc = "Channel connects to UART7_TX"]
    #[inline(always)]
    pub fn _44(self) -> &'a mut W {
        self.variant(REQSRC0_A::_44)
    }
    #[doc = "Channel connects to UART7_RX"]
    #[inline(always)]
    pub fn _45(self) -> &'a mut W {
        self.variant(REQSRC0_A::_45)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `REQSRC1` reader - Channel 1 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 1. User can configure the peripheral setting by REQSRC1. \nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC1_R(crate::FieldReader<u8, u8>);
impl REQSRC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC1` writer - Channel 1 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 1. User can configure the peripheral setting by REQSRC1. \nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `REQSRC2` reader - Channel 2 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 2. User can configure the peripheral setting by REQSRC2. \nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC2_R(crate::FieldReader<u8, u8>);
impl REQSRC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC2` writer - Channel 2 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 2. User can configure the peripheral setting by REQSRC2. \nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `REQSRC3` reader - Channel 3 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 3. User can configure the peripheral setting by REQSRC3. \nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC3_R(crate::FieldReader<u8, u8>);
impl REQSRC3_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC3` writer - Channel 3 Request Source Selection\nThis filed defines which peripheral is connected to PDMA channel 3. User can configure the peripheral setting by REQSRC3. \nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Channel 0 Request Source Selection This filed defines which peripheral is connected to PDMA channel 0. User can configure the peripheral by setting REQSRC0. Note 1: A peripheral cannot be assigned to two channels at the same time. Note 2: This field is useless when transfer between memory and memory."]
    #[inline(always)]
    pub fn reqsrc0(&self) -> REQSRC0_R {
        REQSRC0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Channel 1 Request Source Selection This filed defines which peripheral is connected to PDMA channel 1. User can configure the peripheral setting by REQSRC1. Note: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc1(&self) -> REQSRC1_R {
        REQSRC1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Channel 2 Request Source Selection This filed defines which peripheral is connected to PDMA channel 2. User can configure the peripheral setting by REQSRC2. Note: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc2(&self) -> REQSRC2_R {
        REQSRC2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Channel 3 Request Source Selection This filed defines which peripheral is connected to PDMA channel 3. User can configure the peripheral setting by REQSRC3. Note: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc3(&self) -> REQSRC3_R {
        REQSRC3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Channel 0 Request Source Selection This filed defines which peripheral is connected to PDMA channel 0. User can configure the peripheral by setting REQSRC0. Note 1: A peripheral cannot be assigned to two channels at the same time. Note 2: This field is useless when transfer between memory and memory."]
    #[inline(always)]
    pub fn reqsrc0(&mut self) -> REQSRC0_W {
        REQSRC0_W { w: self }
    }
    #[doc = "Bits 8:13 - Channel 1 Request Source Selection This filed defines which peripheral is connected to PDMA channel 1. User can configure the peripheral setting by REQSRC1. Note: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc1(&mut self) -> REQSRC1_W {
        REQSRC1_W { w: self }
    }
    #[doc = "Bits 16:21 - Channel 2 Request Source Selection This filed defines which peripheral is connected to PDMA channel 2. User can configure the peripheral setting by REQSRC2. Note: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc2(&mut self) -> REQSRC2_W {
        REQSRC2_W { w: self }
    }
    #[doc = "Bits 24:29 - Channel 3 Request Source Selection This filed defines which peripheral is connected to PDMA channel 3. User can configure the peripheral setting by REQSRC3. Note: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc3(&mut self) -> REQSRC3_W {
        REQSRC3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Request Source Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_reqsel0_3](index.html) module"]
pub struct PDMA_REQSEL0_3_SPEC;
impl crate::RegisterSpec for PDMA_REQSEL0_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_reqsel0_3::R](R) reader structure"]
impl crate::Readable for PDMA_REQSEL0_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_reqsel0_3::W](W) writer structure"]
impl crate::Writable for PDMA_REQSEL0_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_REQSEL0_3 to value 0"]
impl crate::Resettable for PDMA_REQSEL0_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
