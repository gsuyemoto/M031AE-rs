#[doc = "Register `UUART_BUFSTS` reader"]
pub struct R(crate::R<UUART_BUFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_BUFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_BUFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_BUFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Receive Buffer Empty Indicator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTY_A {
    #[doc = "0: Receive buffer is not empty"]
    _0 = 0,
    #[doc = "1: Receive buffer is empty"]
    _1 = 1,
}
impl From<RXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPTY` reader - Receive Buffer Empty Indicator"]
pub struct RXEMPTY_R(crate::FieldReader<bool, RXEMPTY_A>);
impl RXEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPTY_A {
        match self.bits {
            false => RXEMPTY_A::_0,
            true => RXEMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXEMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXEMPTY_A::_1
    }
}
impl core::ops::Deref for RXEMPTY_R {
    type Target = crate::FieldReader<bool, RXEMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Buffer Full Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFULL_A {
    #[doc = "0: Receive buffer is not full"]
    _0 = 0,
    #[doc = "1: Receive buffer is full"]
    _1 = 1,
}
impl From<RXFULL_A> for bool {
    #[inline(always)]
    fn from(variant: RXFULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFULL` reader - Receive Buffer Full Indicator"]
pub struct RXFULL_R(crate::FieldReader<bool, RXFULL_A>);
impl RXFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFULL_A {
        match self.bits {
            false => RXFULL_A::_0,
            true => RXFULL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXFULL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXFULL_A::_1
    }
}
impl core::ops::Deref for RXFULL_R {
    type Target = crate::FieldReader<bool, RXFULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Buffer Over-run Error Interrupt Status\nThis bit indicates that a receive buffer overrun error event has been detected. If RXOVIEN (UUART_BUFCTL\\[14\\]) is enabled, the corresponding interrupt request is activated. It is cleared by software writes 1 to this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVIF_A {
    #[doc = "0: A receive buffer overrun error event has not been detected"]
    _0 = 0,
    #[doc = "1: A receive buffer overrun error event has been detected"]
    _1 = 1,
}
impl From<RXOVIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVIF` reader - Receive Buffer Over-run Error Interrupt Status\nThis bit indicates that a receive buffer overrun error event has been detected. If RXOVIEN (UUART_BUFCTL\\[14\\]) is enabled, the corresponding interrupt request is activated. It is cleared by software writes 1 to this bit."]
pub struct RXOVIF_R(crate::FieldReader<bool, RXOVIF_A>);
impl RXOVIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVIF_A {
        match self.bits {
            false => RXOVIF_A::_0,
            true => RXOVIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXOVIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXOVIF_A::_1
    }
}
impl core::ops::Deref for RXOVIF_R {
    type Target = crate::FieldReader<bool, RXOVIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Buffer Empty Indicator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTY_A {
    #[doc = "0: Transmit buffer is not empty"]
    _0 = 0,
    #[doc = "1: Transmit buffer is empty"]
    _1 = 1,
}
impl From<TXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - Transmit Buffer Empty Indicator"]
pub struct TXEMPTY_R(crate::FieldReader<bool, TXEMPTY_A>);
impl TXEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPTY_A {
        match self.bits {
            false => TXEMPTY_A::_0,
            true => TXEMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXEMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXEMPTY_A::_1
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, TXEMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Buffer Full Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFULL_A {
    #[doc = "0: Transmit buffer is not full"]
    _0 = 0,
    #[doc = "1: Transmit buffer is full"]
    _1 = 1,
}
impl From<TXFULL_A> for bool {
    #[inline(always)]
    fn from(variant: TXFULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFULL` reader - Transmit Buffer Full Indicator"]
pub struct TXFULL_R(crate::FieldReader<bool, TXFULL_A>);
impl TXFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFULL_A {
        match self.bits {
            false => TXFULL_A::_0,
            true => TXFULL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXFULL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXFULL_A::_1
    }
}
impl core::ops::Deref for TXFULL_R {
    type Target = crate::FieldReader<bool, TXFULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive Buffer Empty Indicator"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Buffer Full Indicator"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Buffer Over-run Error Interrupt Status This bit indicates that a receive buffer overrun error event has been detected. If RXOVIEN (UUART_BUFCTL\\[14\\]) is enabled, the corresponding interrupt request is activated. It is cleared by software writes 1 to this bit."]
    #[inline(always)]
    pub fn rxovif(&self) -> RXOVIF_R {
        RXOVIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Buffer Empty Indicator"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Buffer Full Indicator"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "USCI Transmit/Receive Buffer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_bufsts](index.html) module"]
pub struct UUART_BUFSTS_SPEC;
impl crate::RegisterSpec for UUART_BUFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_bufsts::R](R) reader structure"]
impl crate::Readable for UUART_BUFSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UUART_BUFSTS to value 0x0101"]
impl crate::Resettable for UUART_BUFSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101
    }
}
