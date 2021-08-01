#[doc = "Register `USPI_PROTSTS` reader"]
pub struct R(crate::R<USPI_PROTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USPI_PROTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USPI_PROTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USPI_PROTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USPI_PROTSTS` writer"]
pub struct W(crate::W<USPI_PROTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USPI_PROTSTS_SPEC>;
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
impl From<crate::W<USPI_PROTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USPI_PROTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Start Interrupt Flag\nNote: It is cleared by software write 1 to this bit. The transmit start event happens when hardware starts to move TX data from data buffer to shift data unit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTIF_A {
    #[doc = "0: Transmit start event did not occur"]
    _0 = 0,
    #[doc = "1: Transmit start event occurred"]
    _1 = 1,
}
impl From<TXSTIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTIF` reader - Transmit Start Interrupt Flag\nNote: It is cleared by software write 1 to this bit. The transmit start event happens when hardware starts to move TX data from data buffer to shift data unit."]
pub struct TXSTIF_R(crate::FieldReader<bool, TXSTIF_A>);
impl TXSTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTIF_A {
        match self.bits {
            false => TXSTIF_A::_0,
            true => TXSTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXSTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXSTIF_A::_1
    }
}
impl core::ops::Deref for TXSTIF_R {
    type Target = crate::FieldReader<bool, TXSTIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTIF` writer - Transmit Start Interrupt Flag\nNote: It is cleared by software write 1 to this bit. The transmit start event happens when hardware starts to move TX data from data buffer to shift data unit."]
pub struct TXSTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSTIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit start event did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXSTIF_A::_0)
    }
    #[doc = "Transmit start event occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXSTIF_A::_1)
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
#[doc = "Transmit End Interrupt Flag\nNote: It is cleared by software write 1 to this bit. The transmit end event happens when hardware sends the last bit of TX data from shift data unit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDIF_A {
    #[doc = "0: Transmit end event did not occur"]
    _0 = 0,
    #[doc = "1: Transmit end event occurred"]
    _1 = 1,
}
impl From<TXENDIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXENDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXENDIF` reader - Transmit End Interrupt Flag\nNote: It is cleared by software write 1 to this bit. The transmit end event happens when hardware sends the last bit of TX data from shift data unit."]
pub struct TXENDIF_R(crate::FieldReader<bool, TXENDIF_A>);
impl TXENDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXENDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENDIF_A {
        match self.bits {
            false => TXENDIF_A::_0,
            true => TXENDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXENDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXENDIF_A::_1
    }
}
impl core::ops::Deref for TXENDIF_R {
    type Target = crate::FieldReader<bool, TXENDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXENDIF` writer - Transmit End Interrupt Flag\nNote: It is cleared by software write 1 to this bit. The transmit end event happens when hardware sends the last bit of TX data from shift data unit."]
pub struct TXENDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXENDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit end event did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXENDIF_A::_0)
    }
    #[doc = "Transmit end event occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXENDIF_A::_1)
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
#[doc = "Receive Start Interrupt Flag\nNote: It is cleared by software write 1 to this bit. For SPI master mode, the receive start event happens when SPI master sends slave select active and spi clock to the external SPI slave. For SPI slave mode, the receive start event happens when slave select of SPI slave is active and spi clock of SPI slave is inputed from the external SPI master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTIF_A {
    #[doc = "0: Receive start event did not occur"]
    _0 = 0,
    #[doc = "1: Receive start event occurred"]
    _1 = 1,
}
impl From<RXSTIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTIF` reader - Receive Start Interrupt Flag\nNote: It is cleared by software write 1 to this bit. For SPI master mode, the receive start event happens when SPI master sends slave select active and spi clock to the external SPI slave. For SPI slave mode, the receive start event happens when slave select of SPI slave is active and spi clock of SPI slave is inputed from the external SPI master."]
pub struct RXSTIF_R(crate::FieldReader<bool, RXSTIF_A>);
impl RXSTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTIF_A {
        match self.bits {
            false => RXSTIF_A::_0,
            true => RXSTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXSTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXSTIF_A::_1
    }
}
impl core::ops::Deref for RXSTIF_R {
    type Target = crate::FieldReader<bool, RXSTIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTIF` writer - Receive Start Interrupt Flag\nNote: It is cleared by software write 1 to this bit. For SPI master mode, the receive start event happens when SPI master sends slave select active and spi clock to the external SPI slave. For SPI slave mode, the receive start event happens when slave select of SPI slave is active and spi clock of SPI slave is inputed from the external SPI master."]
pub struct RXSTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSTIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive start event did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXSTIF_A::_0)
    }
    #[doc = "Receive start event occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXSTIF_A::_1)
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
#[doc = "Receive End Interrupt Flag\nNote: It is cleared by software write 1 to this bit. The receive end event happens when hardware receives the last bit of RX data into shift data unit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXENDIF_A {
    #[doc = "0: Receive end event did not occur"]
    _0 = 0,
    #[doc = "1: Receive end event occurred"]
    _1 = 1,
}
impl From<RXENDIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXENDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXENDIF` reader - Receive End Interrupt Flag\nNote: It is cleared by software write 1 to this bit. The receive end event happens when hardware receives the last bit of RX data into shift data unit."]
pub struct RXENDIF_R(crate::FieldReader<bool, RXENDIF_A>);
impl RXENDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXENDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXENDIF_A {
        match self.bits {
            false => RXENDIF_A::_0,
            true => RXENDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXENDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXENDIF_A::_1
    }
}
impl core::ops::Deref for RXENDIF_R {
    type Target = crate::FieldReader<bool, RXENDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXENDIF` writer - Receive End Interrupt Flag\nNote: It is cleared by software write 1 to this bit. The receive end event happens when hardware receives the last bit of RX data into shift data unit."]
pub struct RXENDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXENDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive end event did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXENDIF_A::_0)
    }
    #[doc = "Receive end event occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXENDIF_A::_1)
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
#[doc = "Slave Time-out Interrupt Flag (for Slave Only)\nNote: It is cleared by software write 1 to this bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVTOIF_A {
    #[doc = "0: Slave time-out event did not occur"]
    _0 = 0,
    #[doc = "1: Slave time-out event occurred"]
    _1 = 1,
}
impl From<SLVTOIF_A> for bool {
    #[inline(always)]
    fn from(variant: SLVTOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVTOIF` reader - Slave Time-out Interrupt Flag (for Slave Only)\nNote: It is cleared by software write 1 to this bit"]
pub struct SLVTOIF_R(crate::FieldReader<bool, SLVTOIF_A>);
impl SLVTOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVTOIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVTOIF_A {
        match self.bits {
            false => SLVTOIF_A::_0,
            true => SLVTOIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVTOIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVTOIF_A::_1
    }
}
impl core::ops::Deref for SLVTOIF_R {
    type Target = crate::FieldReader<bool, SLVTOIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVTOIF` writer - Slave Time-out Interrupt Flag (for Slave Only)\nNote: It is cleared by software write 1 to this bit"]
pub struct SLVTOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVTOIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVTOIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave time-out event did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVTOIF_A::_0)
    }
    #[doc = "Slave time-out event occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVTOIF_A::_1)
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
#[doc = "Slave Bit Count Error Interrupt Flag (for Slave Only)\nNote: It is cleared by software write 1 to this bit. If the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]), bit count error event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVBEIF_A {
    #[doc = "0: Slave bit count error event did not occur"]
    _0 = 0,
    #[doc = "1: Slave bit count error event occurred"]
    _1 = 1,
}
impl From<SLVBEIF_A> for bool {
    #[inline(always)]
    fn from(variant: SLVBEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVBEIF` reader - Slave Bit Count Error Interrupt Flag (for Slave Only)\nNote: It is cleared by software write 1 to this bit. If the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]), bit count error event occurs."]
pub struct SLVBEIF_R(crate::FieldReader<bool, SLVBEIF_A>);
impl SLVBEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVBEIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVBEIF_A {
        match self.bits {
            false => SLVBEIF_A::_0,
            true => SLVBEIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVBEIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVBEIF_A::_1
    }
}
impl core::ops::Deref for SLVBEIF_R {
    type Target = crate::FieldReader<bool, SLVBEIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVBEIF` writer - Slave Bit Count Error Interrupt Flag (for Slave Only)\nNote: It is cleared by software write 1 to this bit. If the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]), bit count error event occurs."]
pub struct SLVBEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVBEIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVBEIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave bit count error event did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVBEIF_A::_0)
    }
    #[doc = "Slave bit count error event occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVBEIF_A::_1)
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
#[doc = "Slave Select Inactive Interrupt Flag (for Slave Only)\nThis bit indicates that the internal slave select signal has changed to inactive. It is cleared by software writes 1 to this bit\nNote: The internal slave select signal is active high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSINAIF_A {
    #[doc = "0: The slave select signal has not changed to inactive"]
    _0 = 0,
    #[doc = "1: The slave select signal has changed to inactive"]
    _1 = 1,
}
impl From<SSINAIF_A> for bool {
    #[inline(always)]
    fn from(variant: SSINAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSINAIF` reader - Slave Select Inactive Interrupt Flag (for Slave Only)\nThis bit indicates that the internal slave select signal has changed to inactive. It is cleared by software writes 1 to this bit\nNote: The internal slave select signal is active high."]
pub struct SSINAIF_R(crate::FieldReader<bool, SSINAIF_A>);
impl SSINAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSINAIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSINAIF_A {
        match self.bits {
            false => SSINAIF_A::_0,
            true => SSINAIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSINAIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSINAIF_A::_1
    }
}
impl core::ops::Deref for SSINAIF_R {
    type Target = crate::FieldReader<bool, SSINAIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSINAIF` writer - Slave Select Inactive Interrupt Flag (for Slave Only)\nThis bit indicates that the internal slave select signal has changed to inactive. It is cleared by software writes 1 to this bit\nNote: The internal slave select signal is active high."]
pub struct SSINAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINAIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSINAIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The slave select signal has not changed to inactive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSINAIF_A::_0)
    }
    #[doc = "The slave select signal has changed to inactive"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSINAIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Slave Select Active Interrupt Flag (for Slave Only)\nThis bit indicates that the internal slave select signal has changed to active. It is cleared by software writes one to this bit\nNote: The internal slave select signal is active high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACTIF_A {
    #[doc = "0: The slave select signal has not changed to active"]
    _0 = 0,
    #[doc = "1: The slave select signal has changed to active"]
    _1 = 1,
}
impl From<SSACTIF_A> for bool {
    #[inline(always)]
    fn from(variant: SSACTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSACTIF` reader - Slave Select Active Interrupt Flag (for Slave Only)\nThis bit indicates that the internal slave select signal has changed to active. It is cleared by software writes one to this bit\nNote: The internal slave select signal is active high."]
pub struct SSACTIF_R(crate::FieldReader<bool, SSACTIF_A>);
impl SSACTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSACTIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACTIF_A {
        match self.bits {
            false => SSACTIF_A::_0,
            true => SSACTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSACTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSACTIF_A::_1
    }
}
impl core::ops::Deref for SSACTIF_R {
    type Target = crate::FieldReader<bool, SSACTIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSACTIF` writer - Slave Select Active Interrupt Flag (for Slave Only)\nThis bit indicates that the internal slave select signal has changed to active. It is cleared by software writes one to this bit\nNote: The internal slave select signal is active high."]
pub struct SSACTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACTIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The slave select signal has not changed to active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACTIF_A::_0)
    }
    #[doc = "The slave select signal has changed to active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACTIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Slave Select Line Bus Status (Read Only)\nThis bit is only available in Slave mode. It used to monitor the current status of the input slave select signal on the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSLINE_A {
    #[doc = "0: The slave select line status is 0"]
    _0 = 0,
    #[doc = "1: The slave select line status is 1"]
    _1 = 1,
}
impl From<SSLINE_A> for bool {
    #[inline(always)]
    fn from(variant: SSLINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSLINE` reader - Slave Select Line Bus Status (Read Only)\nThis bit is only available in Slave mode. It used to monitor the current status of the input slave select signal on the bus."]
pub struct SSLINE_R(crate::FieldReader<bool, SSLINE_A>);
impl SSLINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSLINE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSLINE_A {
        match self.bits {
            false => SSLINE_A::_0,
            true => SSLINE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSLINE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSLINE_A::_1
    }
}
impl core::ops::Deref for SSLINE_R {
    type Target = crate::FieldReader<bool, SSLINE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Busy Status (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: SPI is in idle state"]
    _0 = 0,
    #[doc = "1: SPI is in busy state"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy Status (Read Only)"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSY_A::_1
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slave Mode Transmit Under-run Status (Read Only)\nIn Slave mode, if there is no available transmit data in buffer while transmit data shift out caused by input serial bus clock, this status flag will be set to 1. This bit indicates whether the current shift-out data of word transmission is switched to TXUDRPOL (USPI_PROTCTL\\[28\\]) or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVUDR_A {
    #[doc = "0: Slave transmit under run event does not occur"]
    _0 = 0,
    #[doc = "1: Slave transmit under run event occurs"]
    _1 = 1,
}
impl From<SLVUDR_A> for bool {
    #[inline(always)]
    fn from(variant: SLVUDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVUDR` reader - Slave Mode Transmit Under-run Status (Read Only)\nIn Slave mode, if there is no available transmit data in buffer while transmit data shift out caused by input serial bus clock, this status flag will be set to 1. This bit indicates whether the current shift-out data of word transmission is switched to TXUDRPOL (USPI_PROTCTL\\[28\\]) or not."]
pub struct SLVUDR_R(crate::FieldReader<bool, SLVUDR_A>);
impl SLVUDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVUDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVUDR_A {
        match self.bits {
            false => SLVUDR_A::_0,
            true => SLVUDR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVUDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVUDR_A::_1
    }
}
impl core::ops::Deref for SLVUDR_R {
    type Target = crate::FieldReader<bool, SLVUDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Transmit Start Interrupt Flag Note: It is cleared by software write 1 to this bit. The transmit start event happens when hardware starts to move TX data from data buffer to shift data unit."]
    #[inline(always)]
    pub fn txstif(&self) -> TXSTIF_R {
        TXSTIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit End Interrupt Flag Note: It is cleared by software write 1 to this bit. The transmit end event happens when hardware sends the last bit of TX data from shift data unit."]
    #[inline(always)]
    pub fn txendif(&self) -> TXENDIF_R {
        TXENDIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Start Interrupt Flag Note: It is cleared by software write 1 to this bit. For SPI master mode, the receive start event happens when SPI master sends slave select active and spi clock to the external SPI slave. For SPI slave mode, the receive start event happens when slave select of SPI slave is active and spi clock of SPI slave is inputed from the external SPI master."]
    #[inline(always)]
    pub fn rxstif(&self) -> RXSTIF_R {
        RXSTIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive End Interrupt Flag Note: It is cleared by software write 1 to this bit. The receive end event happens when hardware receives the last bit of RX data into shift data unit."]
    #[inline(always)]
    pub fn rxendif(&self) -> RXENDIF_R {
        RXENDIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave Time-out Interrupt Flag (for Slave Only) Note: It is cleared by software write 1 to this bit"]
    #[inline(always)]
    pub fn slvtoif(&self) -> SLVTOIF_R {
        SLVTOIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Slave Bit Count Error Interrupt Flag (for Slave Only) Note: It is cleared by software write 1 to this bit. If the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]), bit count error event occurs."]
    #[inline(always)]
    pub fn slvbeif(&self) -> SLVBEIF_R {
        SLVBEIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Select Inactive Interrupt Flag (for Slave Only) This bit indicates that the internal slave select signal has changed to inactive. It is cleared by software writes 1 to this bit Note: The internal slave select signal is active high."]
    #[inline(always)]
    pub fn ssinaif(&self) -> SSINAIF_R {
        SSINAIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Slave Select Active Interrupt Flag (for Slave Only) This bit indicates that the internal slave select signal has changed to active. It is cleared by software writes one to this bit Note: The internal slave select signal is active high."]
    #[inline(always)]
    pub fn ssactif(&self) -> SSACTIF_R {
        SSACTIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slave Select Line Bus Status (Read Only) This bit is only available in Slave mode. It used to monitor the current status of the input slave select signal on the bus."]
    #[inline(always)]
    pub fn ssline(&self) -> SSLINE_R {
        SSLINE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Busy Status (Read Only)"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Slave Mode Transmit Under-run Status (Read Only) In Slave mode, if there is no available transmit data in buffer while transmit data shift out caused by input serial bus clock, this status flag will be set to 1. This bit indicates whether the current shift-out data of word transmission is switched to TXUDRPOL (USPI_PROTCTL\\[28\\]) or not."]
    #[inline(always)]
    pub fn slvudr(&self) -> SLVUDR_R {
        SLVUDR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit Start Interrupt Flag Note: It is cleared by software write 1 to this bit. The transmit start event happens when hardware starts to move TX data from data buffer to shift data unit."]
    #[inline(always)]
    pub fn txstif(&mut self) -> TXSTIF_W {
        TXSTIF_W { w: self }
    }
    #[doc = "Bit 2 - Transmit End Interrupt Flag Note: It is cleared by software write 1 to this bit. The transmit end event happens when hardware sends the last bit of TX data from shift data unit."]
    #[inline(always)]
    pub fn txendif(&mut self) -> TXENDIF_W {
        TXENDIF_W { w: self }
    }
    #[doc = "Bit 3 - Receive Start Interrupt Flag Note: It is cleared by software write 1 to this bit. For SPI master mode, the receive start event happens when SPI master sends slave select active and spi clock to the external SPI slave. For SPI slave mode, the receive start event happens when slave select of SPI slave is active and spi clock of SPI slave is inputed from the external SPI master."]
    #[inline(always)]
    pub fn rxstif(&mut self) -> RXSTIF_W {
        RXSTIF_W { w: self }
    }
    #[doc = "Bit 4 - Receive End Interrupt Flag Note: It is cleared by software write 1 to this bit. The receive end event happens when hardware receives the last bit of RX data into shift data unit."]
    #[inline(always)]
    pub fn rxendif(&mut self) -> RXENDIF_W {
        RXENDIF_W { w: self }
    }
    #[doc = "Bit 5 - Slave Time-out Interrupt Flag (for Slave Only) Note: It is cleared by software write 1 to this bit"]
    #[inline(always)]
    pub fn slvtoif(&mut self) -> SLVTOIF_W {
        SLVTOIF_W { w: self }
    }
    #[doc = "Bit 6 - Slave Bit Count Error Interrupt Flag (for Slave Only) Note: It is cleared by software write 1 to this bit. If the transmit/receive data bit count does not match the setting of DWIDTH (USPI_LINECTL\\[11:8\\]), bit count error event occurs."]
    #[inline(always)]
    pub fn slvbeif(&mut self) -> SLVBEIF_W {
        SLVBEIF_W { w: self }
    }
    #[doc = "Bit 8 - Slave Select Inactive Interrupt Flag (for Slave Only) This bit indicates that the internal slave select signal has changed to inactive. It is cleared by software writes 1 to this bit Note: The internal slave select signal is active high."]
    #[inline(always)]
    pub fn ssinaif(&mut self) -> SSINAIF_W {
        SSINAIF_W { w: self }
    }
    #[doc = "Bit 9 - Slave Select Active Interrupt Flag (for Slave Only) This bit indicates that the internal slave select signal has changed to active. It is cleared by software writes one to this bit Note: The internal slave select signal is active high."]
    #[inline(always)]
    pub fn ssactif(&mut self) -> SSACTIF_W {
        SSACTIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uspi_protsts](index.html) module"]
pub struct USPI_PROTSTS_SPEC;
impl crate::RegisterSpec for USPI_PROTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uspi_protsts::R](R) reader structure"]
impl crate::Readable for USPI_PROTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uspi_protsts::W](W) writer structure"]
impl crate::Writable for USPI_PROTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USPI_PROTSTS to value 0"]
impl crate::Resettable for USPI_PROTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
