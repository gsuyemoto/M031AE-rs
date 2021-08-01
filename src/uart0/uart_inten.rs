#[doc = "Register `UART_INTEN` reader"]
pub struct R(crate::R<UART_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_INTEN` writer"]
pub struct W(crate::W<UART_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_INTEN_SPEC>;
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
impl From<crate::W<UART_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Data Available Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDAIEN_A {
    #[doc = "0: Receive data available interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Receive data available interrupt Enabled"]
    _1 = 1,
}
impl From<RDAIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RDAIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDAIEN` reader - Receive Data Available Interrupt Enable Bit"]
pub struct RDAIEN_R(crate::FieldReader<bool, RDAIEN_A>);
impl RDAIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDAIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDAIEN_A {
        match self.bits {
            false => RDAIEN_A::_0,
            true => RDAIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RDAIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RDAIEN_A::_1
    }
}
impl core::ops::Deref for RDAIEN_R {
    type Target = crate::FieldReader<bool, RDAIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDAIEN` writer - Receive Data Available Interrupt Enable Bit"]
pub struct RDAIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDAIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDAIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive data available interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDAIEN_A::_0)
    }
    #[doc = "Receive data available interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDAIEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Transmit Holding Register Empty Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREIEN_A {
    #[doc = "0: Transmit holding register empty interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Transmit holding register empty interrupt Enabled"]
    _1 = 1,
}
impl From<THREIEN_A> for bool {
    #[inline(always)]
    fn from(variant: THREIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREIEN` reader - Transmit Holding Register Empty Interrupt Enable Bit"]
pub struct THREIEN_R(crate::FieldReader<bool, THREIEN_A>);
impl THREIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        THREIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREIEN_A {
        match self.bits {
            false => THREIEN_A::_0,
            true => THREIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == THREIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == THREIEN_A::_1
    }
}
impl core::ops::Deref for THREIEN_R {
    type Target = crate::FieldReader<bool, THREIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THREIEN` writer - Transmit Holding Register Empty Interrupt Enable Bit"]
pub struct THREIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THREIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit holding register empty interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THREIEN_A::_0)
    }
    #[doc = "Transmit holding register empty interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THREIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Receive Line Status Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLSIEN_A {
    #[doc = "0: Receive Line Status interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Receive Line Status interrupt Enabled"]
    _1 = 1,
}
impl From<RLSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLSIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLSIEN` reader - Receive Line Status Interrupt Enable Bit"]
pub struct RLSIEN_R(crate::FieldReader<bool, RLSIEN_A>);
impl RLSIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLSIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLSIEN_A {
        match self.bits {
            false => RLSIEN_A::_0,
            true => RLSIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RLSIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RLSIEN_A::_1
    }
}
impl core::ops::Deref for RLSIEN_R {
    type Target = crate::FieldReader<bool, RLSIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLSIEN` writer - Receive Line Status Interrupt Enable Bit"]
pub struct RLSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RLSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLSIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive Line Status interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RLSIEN_A::_0)
    }
    #[doc = "Receive Line Status interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RLSIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Modem Status Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEMIEN_A {
    #[doc = "0: Modem status interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Modem status interrupt Enabled"]
    _1 = 1,
}
impl From<MODEMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODEMIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODEMIEN` reader - Modem Status Interrupt Enable Bit"]
pub struct MODEMIEN_R(crate::FieldReader<bool, MODEMIEN_A>);
impl MODEMIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODEMIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEMIEN_A {
        match self.bits {
            false => MODEMIEN_A::_0,
            true => MODEMIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODEMIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODEMIEN_A::_1
    }
}
impl core::ops::Deref for MODEMIEN_R {
    type Target = crate::FieldReader<bool, MODEMIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODEMIEN` writer - Modem Status Interrupt Enable Bit"]
pub struct MODEMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODEMIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Modem status interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODEMIEN_A::_0)
    }
    #[doc = "Modem status interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODEMIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "RX Time-out Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOIEN_A {
    #[doc = "0: RX time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: RX time-out interrupt Enabled"]
    _1 = 1,
}
impl From<RXTOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOIEN` reader - RX Time-out Interrupt Enable Bit"]
pub struct RXTOIEN_R(crate::FieldReader<bool, RXTOIEN_A>);
impl RXTOIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOIEN_A {
        match self.bits {
            false => RXTOIEN_A::_0,
            true => RXTOIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXTOIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXTOIEN_A::_1
    }
}
impl core::ops::Deref for RXTOIEN_R {
    type Target = crate::FieldReader<bool, RXTOIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTOIEN` writer - RX Time-out Interrupt Enable Bit"]
pub struct RXTOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTOIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RX time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTOIEN_A::_0)
    }
    #[doc = "RX time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTOIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Buffer Error Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFERRIEN_A {
    #[doc = "0: Buffer error interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Buffer error interrupt Enabled"]
    _1 = 1,
}
impl From<BUFERRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFERRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFERRIEN` reader - Buffer Error Interrupt Enable Bit"]
pub struct BUFERRIEN_R(crate::FieldReader<bool, BUFERRIEN_A>);
impl BUFERRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFERRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFERRIEN_A {
        match self.bits {
            false => BUFERRIEN_A::_0,
            true => BUFERRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUFERRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUFERRIEN_A::_1
    }
}
impl core::ops::Deref for BUFERRIEN_R {
    type Target = crate::FieldReader<bool, BUFERRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFERRIEN` writer - Buffer Error Interrupt Enable Bit"]
pub struct BUFERRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFERRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFERRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Buffer error interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFERRIEN_A::_0)
    }
    #[doc = "Buffer error interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFERRIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Wake-up Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKIEN_A {
    #[doc = "0: Wake-up Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Wake-up Interrupt Enabled"]
    _1 = 1,
}
impl From<WKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKIEN` reader - Wake-up Interrupt Enable Bit"]
pub struct WKIEN_R(crate::FieldReader<bool, WKIEN_A>);
impl WKIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKIEN_A {
        match self.bits {
            false => WKIEN_A::_0,
            true => WKIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKIEN_A::_1
    }
}
impl core::ops::Deref for WKIEN_R {
    type Target = crate::FieldReader<bool, WKIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKIEN` writer - Wake-up Interrupt Enable Bit"]
pub struct WKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wake-up Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKIEN_A::_0)
    }
    #[doc = "Wake-up Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Receive Buffer Time-out Counter Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOCNTEN_A {
    #[doc = "0: Receive Buffer Time-out counter Disabled"]
    _0 = 0,
    #[doc = "1: Receive Buffer Time-out counter Enabled"]
    _1 = 1,
}
impl From<TOCNTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOCNTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOCNTEN` reader - Receive Buffer Time-out Counter Enable Bit"]
pub struct TOCNTEN_R(crate::FieldReader<bool, TOCNTEN_A>);
impl TOCNTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOCNTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOCNTEN_A {
        match self.bits {
            false => TOCNTEN_A::_0,
            true => TOCNTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOCNTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOCNTEN_A::_1
    }
}
impl core::ops::Deref for TOCNTEN_R {
    type Target = crate::FieldReader<bool, TOCNTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOCNTEN` writer - Receive Buffer Time-out Counter Enable Bit"]
pub struct TOCNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOCNTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOCNTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive Buffer Time-out counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOCNTEN_A::_0)
    }
    #[doc = "Receive Buffer Time-out counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOCNTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "nRTS Auto-flow Control Enable Bit\nNote: When nRTS auto-flow is enabled, if the number of bytes in the RX FIFO equals the RTSTRGLV (UART_FIFO\\[19:16\\]), the UART will de-assert nRTS signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATORTSEN_A {
    #[doc = "0: nRTS auto-flow control Disabled"]
    _0 = 0,
    #[doc = "1: nRTS auto-flow control Enabled"]
    _1 = 1,
}
impl From<ATORTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: ATORTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATORTSEN` reader - nRTS Auto-flow Control Enable Bit\nNote: When nRTS auto-flow is enabled, if the number of bytes in the RX FIFO equals the RTSTRGLV (UART_FIFO\\[19:16\\]), the UART will de-assert nRTS signal."]
pub struct ATORTSEN_R(crate::FieldReader<bool, ATORTSEN_A>);
impl ATORTSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATORTSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATORTSEN_A {
        match self.bits {
            false => ATORTSEN_A::_0,
            true => ATORTSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ATORTSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ATORTSEN_A::_1
    }
}
impl core::ops::Deref for ATORTSEN_R {
    type Target = crate::FieldReader<bool, ATORTSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATORTSEN` writer - nRTS Auto-flow Control Enable Bit\nNote: When nRTS auto-flow is enabled, if the number of bytes in the RX FIFO equals the RTSTRGLV (UART_FIFO\\[19:16\\]), the UART will de-assert nRTS signal."]
pub struct ATORTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATORTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATORTSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "nRTS auto-flow control Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATORTSEN_A::_0)
    }
    #[doc = "nRTS auto-flow control Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATORTSEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "nCTS Auto-flow Control Enable Bit\nNote: When nCTS auto-flow is enabled, the UART will send data to external device if nCTS input assert (UART will not send data to device until nCTS is asserted).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATOCTSEN_A {
    #[doc = "0: nCTS auto-flow control Disabled"]
    _0 = 0,
    #[doc = "1: nCTS auto-flow control Enabled"]
    _1 = 1,
}
impl From<ATOCTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: ATOCTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATOCTSEN` reader - nCTS Auto-flow Control Enable Bit\nNote: When nCTS auto-flow is enabled, the UART will send data to external device if nCTS input assert (UART will not send data to device until nCTS is asserted)."]
pub struct ATOCTSEN_R(crate::FieldReader<bool, ATOCTSEN_A>);
impl ATOCTSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATOCTSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATOCTSEN_A {
        match self.bits {
            false => ATOCTSEN_A::_0,
            true => ATOCTSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ATOCTSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ATOCTSEN_A::_1
    }
}
impl core::ops::Deref for ATOCTSEN_R {
    type Target = crate::FieldReader<bool, ATOCTSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOCTSEN` writer - nCTS Auto-flow Control Enable Bit\nNote: When nCTS auto-flow is enabled, the UART will send data to external device if nCTS input assert (UART will not send data to device until nCTS is asserted)."]
pub struct ATOCTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOCTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATOCTSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "nCTS auto-flow control Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATOCTSEN_A::_0)
    }
    #[doc = "nCTS auto-flow control Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATOCTSEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "TX PDMA Enable Bit\nNote: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA transmit request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA transmit request operation continue.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPDMAEN_A {
    #[doc = "0: TX PDMA Disabled"]
    _0 = 0,
    #[doc = "1: TX PDMA Enabled"]
    _1 = 1,
}
impl From<TXPDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXPDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPDMAEN` reader - TX PDMA Enable Bit\nNote: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA transmit request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA transmit request operation continue."]
pub struct TXPDMAEN_R(crate::FieldReader<bool, TXPDMAEN_A>);
impl TXPDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPDMAEN_A {
        match self.bits {
            false => TXPDMAEN_A::_0,
            true => TXPDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXPDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXPDMAEN_A::_1
    }
}
impl core::ops::Deref for TXPDMAEN_R {
    type Target = crate::FieldReader<bool, TXPDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPDMAEN` writer - TX PDMA Enable Bit\nNote: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA transmit request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA transmit request operation continue."]
pub struct TXPDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TX PDMA Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXPDMAEN_A::_0)
    }
    #[doc = "TX PDMA Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXPDMAEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "RX PDMA Enable Bit\nThis bit can enable or disable RX PDMA service.\nNote: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA receive request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA receive request operation continue.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPDMAEN_A {
    #[doc = "0: RX PDMA Disabled"]
    _0 = 0,
    #[doc = "1: RX PDMA Enabled"]
    _1 = 1,
}
impl From<RXPDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXPDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPDMAEN` reader - RX PDMA Enable Bit\nThis bit can enable or disable RX PDMA service.\nNote: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA receive request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA receive request operation continue."]
pub struct RXPDMAEN_R(crate::FieldReader<bool, RXPDMAEN_A>);
impl RXPDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXPDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPDMAEN_A {
        match self.bits {
            false => RXPDMAEN_A::_0,
            true => RXPDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXPDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXPDMAEN_A::_1
    }
}
impl core::ops::Deref for RXPDMAEN_R {
    type Target = crate::FieldReader<bool, RXPDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPDMAEN` writer - RX PDMA Enable Bit\nThis bit can enable or disable RX PDMA service.\nNote: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA receive request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA receive request operation continue."]
pub struct RXPDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RX PDMA Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXPDMAEN_A::_0)
    }
    #[doc = "RX PDMA Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXPDMAEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Single-wire Bit Error Detection Interrupt Enable Bit\nSet this bit, the Single-wire Half Duplex Bit Error Detection Interrupt SWBEINT(UART_INTSTS\\[24\\]) is generated when Single-wire Bit Error Detection SWBEIF(UART_INTSTS\\[16\\]) is set.\nNote: This bit is valid when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWBEIEN_A {
    #[doc = "0: Single-wire Bit Error Detect Inerrupt Disabled"]
    _0 = 0,
    #[doc = "1: Single-wire Bit Error Detect Inerrupt Enabled"]
    _1 = 1,
}
impl From<SWBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SWBEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWBEIEN` reader - Single-wire Bit Error Detection Interrupt Enable Bit\nSet this bit, the Single-wire Half Duplex Bit Error Detection Interrupt SWBEINT(UART_INTSTS\\[24\\]) is generated when Single-wire Bit Error Detection SWBEIF(UART_INTSTS\\[16\\]) is set.\nNote: This bit is valid when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode."]
pub struct SWBEIEN_R(crate::FieldReader<bool, SWBEIEN_A>);
impl SWBEIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWBEIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWBEIEN_A {
        match self.bits {
            false => SWBEIEN_A::_0,
            true => SWBEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWBEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWBEIEN_A::_1
    }
}
impl core::ops::Deref for SWBEIEN_R {
    type Target = crate::FieldReader<bool, SWBEIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWBEIEN` writer - Single-wire Bit Error Detection Interrupt Enable Bit\nSet this bit, the Single-wire Half Duplex Bit Error Detection Interrupt SWBEINT(UART_INTSTS\\[24\\]) is generated when Single-wire Bit Error Detection SWBEIF(UART_INTSTS\\[16\\]) is set.\nNote: This bit is valid when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode."]
pub struct SWBEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWBEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWBEIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single-wire Bit Error Detect Inerrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWBEIEN_A::_0)
    }
    #[doc = "Single-wire Bit Error Detect Inerrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWBEIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Auto-baud Rate Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRIEN_A {
    #[doc = "0: Auto-baud rate interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Auto-baud rate interrupt Enabled"]
    _1 = 1,
}
impl From<ABRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRIEN` reader - Auto-baud Rate Interrupt Enable Bit"]
pub struct ABRIEN_R(crate::FieldReader<bool, ABRIEN_A>);
impl ABRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRIEN_A {
        match self.bits {
            false => ABRIEN_A::_0,
            true => ABRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABRIEN_A::_1
    }
}
impl core::ops::Deref for ABRIEN_R {
    type Target = crate::FieldReader<bool, ABRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRIEN` writer - Auto-baud Rate Interrupt Enable Bit"]
pub struct ABRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-baud rate interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABRIEN_A::_0)
    }
    #[doc = "Auto-baud rate interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABRIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Transmitter Empty Interrupt Enable Bit\nIf TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt TXENDINT (UART_INTSTS\\[30\\]) will be generated when TXENDIF (UART_INTSTS\\[22\\]) is set (TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDIEN_A {
    #[doc = "0: Transmitter empty interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Transmitter empty interrupt Enabled"]
    _1 = 1,
}
impl From<TXENDIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXENDIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXENDIEN` reader - Transmitter Empty Interrupt Enable Bit\nIf TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt TXENDINT (UART_INTSTS\\[30\\]) will be generated when TXENDIF (UART_INTSTS\\[22\\]) is set (TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted)."]
pub struct TXENDIEN_R(crate::FieldReader<bool, TXENDIEN_A>);
impl TXENDIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXENDIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENDIEN_A {
        match self.bits {
            false => TXENDIEN_A::_0,
            true => TXENDIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXENDIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXENDIEN_A::_1
    }
}
impl core::ops::Deref for TXENDIEN_R {
    type Target = crate::FieldReader<bool, TXENDIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXENDIEN` writer - Transmitter Empty Interrupt Enable Bit\nIf TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt TXENDINT (UART_INTSTS\\[30\\]) will be generated when TXENDIF (UART_INTSTS\\[22\\]) is set (TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted)."]
pub struct TXENDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXENDIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitter empty interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXENDIEN_A::_0)
    }
    #[doc = "Transmitter empty interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXENDIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Available Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rdaien(&self) -> RDAIEN_R {
        RDAIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Holding Register Empty Interrupt Enable Bit"]
    #[inline(always)]
    pub fn threien(&self) -> THREIEN_R {
        THREIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Line Status Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rlsien(&self) -> RLSIEN_R {
        RLSIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable Bit"]
    #[inline(always)]
    pub fn modemien(&self) -> MODEMIEN_R {
        MODEMIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RX Time-out Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxtoien(&self) -> RXTOIEN_R {
        RXTOIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn buferrien(&self) -> BUFERRIEN_R {
        BUFERRIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake-up Interrupt Enable Bit"]
    #[inline(always)]
    pub fn wkien(&self) -> WKIEN_R {
        WKIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive Buffer Time-out Counter Enable Bit"]
    #[inline(always)]
    pub fn tocnten(&self) -> TOCNTEN_R {
        TOCNTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - nRTS Auto-flow Control Enable Bit Note: When nRTS auto-flow is enabled, if the number of bytes in the RX FIFO equals the RTSTRGLV (UART_FIFO\\[19:16\\]), the UART will de-assert nRTS signal."]
    #[inline(always)]
    pub fn atortsen(&self) -> ATORTSEN_R {
        ATORTSEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - nCTS Auto-flow Control Enable Bit Note: When nCTS auto-flow is enabled, the UART will send data to external device if nCTS input assert (UART will not send data to device until nCTS is asserted)."]
    #[inline(always)]
    pub fn atoctsen(&self) -> ATOCTSEN_R {
        ATOCTSEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TX PDMA Enable Bit Note: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA transmit request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA transmit request operation continue."]
    #[inline(always)]
    pub fn txpdmaen(&self) -> TXPDMAEN_R {
        TXPDMAEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RX PDMA Enable Bit This bit can enable or disable RX PDMA service. Note: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA receive request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA receive request operation continue."]
    #[inline(always)]
    pub fn rxpdmaen(&self) -> RXPDMAEN_R {
        RXPDMAEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Single-wire Bit Error Detection Interrupt Enable Bit Set this bit, the Single-wire Half Duplex Bit Error Detection Interrupt SWBEINT(UART_INTSTS\\[24\\]) is generated when Single-wire Bit Error Detection SWBEIF(UART_INTSTS\\[16\\]) is set. Note: This bit is valid when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode."]
    #[inline(always)]
    pub fn swbeien(&self) -> SWBEIEN_R {
        SWBEIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Auto-baud Rate Interrupt Enable Bit"]
    #[inline(always)]
    pub fn abrien(&self) -> ABRIEN_R {
        ABRIEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Transmitter Empty Interrupt Enable Bit If TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt TXENDINT (UART_INTSTS\\[30\\]) will be generated when TXENDIF (UART_INTSTS\\[22\\]) is set (TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted)."]
    #[inline(always)]
    pub fn txendien(&self) -> TXENDIEN_R {
        TXENDIEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Available Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rdaien(&mut self) -> RDAIEN_W {
        RDAIEN_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Holding Register Empty Interrupt Enable Bit"]
    #[inline(always)]
    pub fn threien(&mut self) -> THREIEN_W {
        THREIEN_W { w: self }
    }
    #[doc = "Bit 2 - Receive Line Status Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rlsien(&mut self) -> RLSIEN_W {
        RLSIEN_W { w: self }
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable Bit"]
    #[inline(always)]
    pub fn modemien(&mut self) -> MODEMIEN_W {
        MODEMIEN_W { w: self }
    }
    #[doc = "Bit 4 - RX Time-out Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxtoien(&mut self) -> RXTOIEN_W {
        RXTOIEN_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn buferrien(&mut self) -> BUFERRIEN_W {
        BUFERRIEN_W { w: self }
    }
    #[doc = "Bit 6 - Wake-up Interrupt Enable Bit"]
    #[inline(always)]
    pub fn wkien(&mut self) -> WKIEN_W {
        WKIEN_W { w: self }
    }
    #[doc = "Bit 11 - Receive Buffer Time-out Counter Enable Bit"]
    #[inline(always)]
    pub fn tocnten(&mut self) -> TOCNTEN_W {
        TOCNTEN_W { w: self }
    }
    #[doc = "Bit 12 - nRTS Auto-flow Control Enable Bit Note: When nRTS auto-flow is enabled, if the number of bytes in the RX FIFO equals the RTSTRGLV (UART_FIFO\\[19:16\\]), the UART will de-assert nRTS signal."]
    #[inline(always)]
    pub fn atortsen(&mut self) -> ATORTSEN_W {
        ATORTSEN_W { w: self }
    }
    #[doc = "Bit 13 - nCTS Auto-flow Control Enable Bit Note: When nCTS auto-flow is enabled, the UART will send data to external device if nCTS input assert (UART will not send data to device until nCTS is asserted)."]
    #[inline(always)]
    pub fn atoctsen(&mut self) -> ATOCTSEN_W {
        ATOCTSEN_W { w: self }
    }
    #[doc = "Bit 14 - TX PDMA Enable Bit Note: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA transmit request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA transmit request operation continue."]
    #[inline(always)]
    pub fn txpdmaen(&mut self) -> TXPDMAEN_W {
        TXPDMAEN_W { w: self }
    }
    #[doc = "Bit 15 - RX PDMA Enable Bit This bit can enable or disable RX PDMA service. Note: If RLSIEN (UART_INTEN\\[2\\]) is enabled and HWRLSINT (UART_INTSTS\\[26\\]) is set to 1, the RLS (Receive Line Status) Interrupt is caused. If RLS interrupt is caused by Break Error Flag BIF(UART_FIFOSTS\\[6\\]), Frame Error Flag FEF(UART_FIFO\\[5\\]) or Parity Error Flag PEF(UART_FIFOSTS\\[4\\]), UART PDMA receive request operation is stopped. Clear Break Error Flag BIF or Frame Error Flag FEF or Parity Error Flag PEF by writing '1' to corresponding BIF, FEF and PEF to make UART PDMA receive request operation continue."]
    #[inline(always)]
    pub fn rxpdmaen(&mut self) -> RXPDMAEN_W {
        RXPDMAEN_W { w: self }
    }
    #[doc = "Bit 16 - Single-wire Bit Error Detection Interrupt Enable Bit Set this bit, the Single-wire Half Duplex Bit Error Detection Interrupt SWBEINT(UART_INTSTS\\[24\\]) is generated when Single-wire Bit Error Detection SWBEIF(UART_INTSTS\\[16\\]) is set. Note: This bit is valid when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode."]
    #[inline(always)]
    pub fn swbeien(&mut self) -> SWBEIEN_W {
        SWBEIEN_W { w: self }
    }
    #[doc = "Bit 18 - Auto-baud Rate Interrupt Enable Bit"]
    #[inline(always)]
    pub fn abrien(&mut self) -> ABRIEN_W {
        ABRIEN_W { w: self }
    }
    #[doc = "Bit 22 - Transmitter Empty Interrupt Enable Bit If TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt TXENDINT (UART_INTSTS\\[30\\]) will be generated when TXENDIF (UART_INTSTS\\[22\\]) is set (TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted)."]
    #[inline(always)]
    pub fn txendien(&mut self) -> TXENDIEN_W {
        TXENDIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_inten](index.html) module"]
pub struct UART_INTEN_SPEC;
impl crate::RegisterSpec for UART_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_inten::R](R) reader structure"]
impl crate::Readable for UART_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_inten::W](W) writer structure"]
impl crate::Writable for UART_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_INTEN to value 0"]
impl crate::Resettable for UART_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
