#[doc = "Register `QSPIx_CTL` reader"]
pub struct R(crate::R<QSPIX_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPIX_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPIX_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPIX_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPIx_CTL` writer"]
pub struct W(crate::W<QSPIX_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPIX_CTL_SPEC>;
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
impl From<crate::W<QSPIX_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPIX_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "QSPI Transfer Control Enable Bit\nIn Master mode, the transfer will start when there is data in the FIFO buffer after this bit is set to 1. In Slave mode, this device is ready to receive data when this bit is set to 1.\nNote: Before changing the configurations of QSPIx_CTL, QSPIx_CLKDIV, QSPIx_SSCTL and QSPIx_FIFOCTL registers, user shall clear the SPIEN (QSPIx_CTL\\[0\\]) and confirm the SPIENSTS (QSPIx_STATUS\\[15\\]) is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIEN_A {
    #[doc = "0: Transfer control Disabled"]
    _0 = 0,
    #[doc = "1: Transfer control Enabled"]
    _1 = 1,
}
impl From<SPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIEN` reader - QSPI Transfer Control Enable Bit\nIn Master mode, the transfer will start when there is data in the FIFO buffer after this bit is set to 1. In Slave mode, this device is ready to receive data when this bit is set to 1.\nNote: Before changing the configurations of QSPIx_CTL, QSPIx_CLKDIV, QSPIx_SSCTL and QSPIx_FIFOCTL registers, user shall clear the SPIEN (QSPIx_CTL\\[0\\]) and confirm the SPIENSTS (QSPIx_STATUS\\[15\\]) is 0."]
pub struct SPIEN_R(crate::FieldReader<bool, SPIEN_A>);
impl SPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIEN_A {
        match self.bits {
            false => SPIEN_A::_0,
            true => SPIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPIEN_A::_1
    }
}
impl core::ops::Deref for SPIEN_R {
    type Target = crate::FieldReader<bool, SPIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIEN` writer - QSPI Transfer Control Enable Bit\nIn Master mode, the transfer will start when there is data in the FIFO buffer after this bit is set to 1. In Slave mode, this device is ready to receive data when this bit is set to 1.\nNote: Before changing the configurations of QSPIx_CTL, QSPIx_CLKDIV, QSPIx_SSCTL and QSPIx_FIFOCTL registers, user shall clear the SPIEN (QSPIx_CTL\\[0\\]) and confirm the SPIENSTS (QSPIx_STATUS\\[15\\]) is 0."]
pub struct SPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transfer control Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIEN_A::_0)
    }
    #[doc = "Transfer control Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIEN_A::_1)
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
#[doc = "Receive on Negative Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEG_A {
    #[doc = "0: Received data input signal is latched on the rising edge of QSPI bus clock"]
    _0 = 0,
    #[doc = "1: Received data input signal is latched on the falling edge of QSPI bus clock"]
    _1 = 1,
}
impl From<RXNEG_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEG` reader - Receive on Negative Edge"]
pub struct RXNEG_R(crate::FieldReader<bool, RXNEG_A>);
impl RXNEG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNEG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNEG_A {
        match self.bits {
            false => RXNEG_A::_0,
            true => RXNEG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXNEG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXNEG_A::_1
    }
}
impl core::ops::Deref for RXNEG_R {
    type Target = crate::FieldReader<bool, RXNEG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNEG` writer - Receive on Negative Edge"]
pub struct RXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXNEG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Received data input signal is latched on the rising edge of QSPI bus clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXNEG_A::_0)
    }
    #[doc = "Received data input signal is latched on the falling edge of QSPI bus clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXNEG_A::_1)
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
#[doc = "Transmit on Negative Edge\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXNEG_A {
    #[doc = "0: Transmitted data output signal is changed on the rising edge of QSPI bus clock"]
    _0 = 0,
    #[doc = "1: Transmitted data output signal is changed on the falling edge of QSPI bus clock"]
    _1 = 1,
}
impl From<TXNEG_A> for bool {
    #[inline(always)]
    fn from(variant: TXNEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXNEG` reader - Transmit on Negative Edge"]
pub struct TXNEG_R(crate::FieldReader<bool, TXNEG_A>);
impl TXNEG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXNEG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXNEG_A {
        match self.bits {
            false => TXNEG_A::_0,
            true => TXNEG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXNEG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXNEG_A::_1
    }
}
impl core::ops::Deref for TXNEG_R {
    type Target = crate::FieldReader<bool, TXNEG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXNEG` writer - Transmit on Negative Edge"]
pub struct TXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXNEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXNEG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitted data output signal is changed on the rising edge of QSPI bus clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXNEG_A::_0)
    }
    #[doc = "Transmitted data output signal is changed on the falling edge of QSPI bus clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXNEG_A::_1)
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
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKPOL_A {
    #[doc = "0: QSPI bus clock is idle low"]
    _0 = 0,
    #[doc = "1: QSPI bus clock is idle high"]
    _1 = 1,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity"]
pub struct CLKPOL_R(crate::FieldReader<bool, CLKPOL_A>);
impl CLKPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKPOL_A {
        match self.bits {
            false => CLKPOL_A::_0,
            true => CLKPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKPOL_A::_1
    }
}
impl core::ops::Deref for CLKPOL_R {
    type Target = crate::FieldReader<bool, CLKPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKPOL` writer - Clock Polarity"]
pub struct CLKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "QSPI bus clock is idle low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKPOL_A::_0)
    }
    #[doc = "QSPI bus clock is idle high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKPOL_A::_1)
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
#[doc = "Field `SUSPITV` reader - Suspend Interval (Master Only)\nThe four bits provide configurable suspend interval between two successive transmit/receive transaction in a transfer. The definition of the suspend interval is the interval between the last clock edge of the preceding transaction word and the first clock edge of the following transaction word. The default value is 0x3. The period of the suspend interval is obtained according to the following equation.\n (SUSPITV + 0.5) * period of QSPICLK clock cycle\nExample:"]
pub struct SUSPITV_R(crate::FieldReader<u8, u8>);
impl SUSPITV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUSPITV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPITV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPITV` writer - Suspend Interval (Master Only)\nThe four bits provide configurable suspend interval between two successive transmit/receive transaction in a transfer. The definition of the suspend interval is the interval between the last clock edge of the preceding transaction word and the first clock edge of the following transaction word. The default value is 0x3. The period of the suspend interval is obtained according to the following equation.\n (SUSPITV + 0.5) * period of QSPICLK clock cycle\nExample:"]
pub struct SUSPITV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPITV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DWIDTH` reader - Data Width\nThis field specifies how many bits can be transmitted / received in one transaction. The minimum bit length is 8 bits and can up to 32 bits."]
pub struct DWIDTH_R(crate::FieldReader<u8, u8>);
impl DWIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DWIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DWIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DWIDTH` writer - Data Width\nThis field specifies how many bits can be transmitted / received in one transaction. The minimum bit length is 8 bits and can up to 32 bits."]
pub struct DWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Send LSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSB_A {
    #[doc = "0: The MSB, which bit of transmit/receive register depends on the setting of DWIDTH, is transmitted/received first"]
    _0 = 0,
    #[doc = "1: The LSB, bit 0 of the QSPIx TX register, is sent first to the QSPI data output pin, and the first bit received from the QSPI data input pin will be put in the LSB position of the RX register (bit 0 of QSPIx_RX)"]
    _1 = 1,
}
impl From<LSB_A> for bool {
    #[inline(always)]
    fn from(variant: LSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSB` reader - Send LSB First"]
pub struct LSB_R(crate::FieldReader<bool, LSB_A>);
impl LSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSB_A {
        match self.bits {
            false => LSB_A::_0,
            true => LSB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LSB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LSB_A::_1
    }
}
impl core::ops::Deref for LSB_R {
    type Target = crate::FieldReader<bool, LSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSB` writer - Send LSB First"]
pub struct LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> LSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The MSB, which bit of transmit/receive register depends on the setting of DWIDTH, is transmitted/received first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSB_A::_0)
    }
    #[doc = "The LSB, bit 0 of the QSPIx TX register, is sent first to the QSPI data output pin, and the first bit received from the QSPI data input pin will be put in the LSB position of the RX register (bit 0 of QSPIx_RX)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSB_A::_1)
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
#[doc = "QSPI Half-duplex Transfer Enable Bit\nThis bit is used to select full-duplex or half-duplex for QSPI transfer. The bit field DATDIR (QSPIx_CTL\\[20\\]) can be used to set the data direction in half-duplex transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALFDPX_A {
    #[doc = "0: QSPI operates in full-duplex transfer"]
    _0 = 0,
    #[doc = "1: QSPI operates in half-duplex transfer"]
    _1 = 1,
}
impl From<HALFDPX_A> for bool {
    #[inline(always)]
    fn from(variant: HALFDPX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALFDPX` reader - QSPI Half-duplex Transfer Enable Bit\nThis bit is used to select full-duplex or half-duplex for QSPI transfer. The bit field DATDIR (QSPIx_CTL\\[20\\]) can be used to set the data direction in half-duplex transfer."]
pub struct HALFDPX_R(crate::FieldReader<bool, HALFDPX_A>);
impl HALFDPX_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALFDPX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALFDPX_A {
        match self.bits {
            false => HALFDPX_A::_0,
            true => HALFDPX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HALFDPX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HALFDPX_A::_1
    }
}
impl core::ops::Deref for HALFDPX_R {
    type Target = crate::FieldReader<bool, HALFDPX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALFDPX` writer - QSPI Half-duplex Transfer Enable Bit\nThis bit is used to select full-duplex or half-duplex for QSPI transfer. The bit field DATDIR (QSPIx_CTL\\[20\\]) can be used to set the data direction in half-duplex transfer."]
pub struct HALFDPX_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFDPX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALFDPX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "QSPI operates in full-duplex transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALFDPX_A::_0)
    }
    #[doc = "QSPI operates in half-duplex transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALFDPX_A::_1)
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
#[doc = "Receive-only Mode Enable Bit (Master Only)\nThis bit field is only available in Master mode. In receive-only mode, QSPI Master will generate QSPI bus clock continuously for receiving data bit from QSPI slave device and assert the BUSY status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXONLY_A {
    #[doc = "0: Receive-only mode Disabled"]
    _0 = 0,
    #[doc = "1: Receive-only mode Enabled"]
    _1 = 1,
}
impl From<RXONLY_A> for bool {
    #[inline(always)]
    fn from(variant: RXONLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXONLY` reader - Receive-only Mode Enable Bit (Master Only)\nThis bit field is only available in Master mode. In receive-only mode, QSPI Master will generate QSPI bus clock continuously for receiving data bit from QSPI slave device and assert the BUSY status."]
pub struct RXONLY_R(crate::FieldReader<bool, RXONLY_A>);
impl RXONLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXONLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXONLY_A {
        match self.bits {
            false => RXONLY_A::_0,
            true => RXONLY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXONLY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXONLY_A::_1
    }
}
impl core::ops::Deref for RXONLY_R {
    type Target = crate::FieldReader<bool, RXONLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXONLY` writer - Receive-only Mode Enable Bit (Master Only)\nThis bit field is only available in Master mode. In receive-only mode, QSPI Master will generate QSPI bus clock continuously for receiving data bit from QSPI slave device and assert the BUSY status."]
pub struct RXONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXONLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXONLY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive-only mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXONLY_A::_0)
    }
    #[doc = "Receive-only mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXONLY_A::_1)
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
#[doc = "2-bit Transfer Mode Enable Bit\nNote: When 2-bit Transfer mode is enabled, the first serial transmitted bit data is from the first FIFO buffer data, and the 2nd serial transmitted bit data is from the second FIFO buffer data. As the same as transmitted function, the first received bit data is stored into the first FIFO buffer and the 2nd received bit data is stored into the second FIFO buffer at the same time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWOBIT_A {
    #[doc = "0: 2-bit Transfer mode Disabled"]
    _0 = 0,
    #[doc = "1: 2-bit Transfer mode Enabled"]
    _1 = 1,
}
impl From<TWOBIT_A> for bool {
    #[inline(always)]
    fn from(variant: TWOBIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWOBIT` reader - 2-bit Transfer Mode Enable Bit\nNote: When 2-bit Transfer mode is enabled, the first serial transmitted bit data is from the first FIFO buffer data, and the 2nd serial transmitted bit data is from the second FIFO buffer data. As the same as transmitted function, the first received bit data is stored into the first FIFO buffer and the 2nd received bit data is stored into the second FIFO buffer at the same time."]
pub struct TWOBIT_R(crate::FieldReader<bool, TWOBIT_A>);
impl TWOBIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWOBIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWOBIT_A {
        match self.bits {
            false => TWOBIT_A::_0,
            true => TWOBIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TWOBIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TWOBIT_A::_1
    }
}
impl core::ops::Deref for TWOBIT_R {
    type Target = crate::FieldReader<bool, TWOBIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWOBIT` writer - 2-bit Transfer Mode Enable Bit\nNote: When 2-bit Transfer mode is enabled, the first serial transmitted bit data is from the first FIFO buffer data, and the 2nd serial transmitted bit data is from the second FIFO buffer data. As the same as transmitted function, the first received bit data is stored into the first FIFO buffer and the 2nd received bit data is stored into the second FIFO buffer at the same time."]
pub struct TWOBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TWOBIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWOBIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "2-bit Transfer mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWOBIT_A::_0)
    }
    #[doc = "2-bit Transfer mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWOBIT_A::_1)
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
#[doc = "Unit Transfer Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNITIEN_A {
    #[doc = "0: QSPI unit transfer interrupt Disabled"]
    _0 = 0,
    #[doc = "1: QSPI unit transfer interrupt Enabled"]
    _1 = 1,
}
impl From<UNITIEN_A> for bool {
    #[inline(always)]
    fn from(variant: UNITIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNITIEN` reader - Unit Transfer Interrupt Enable Bit"]
pub struct UNITIEN_R(crate::FieldReader<bool, UNITIEN_A>);
impl UNITIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNITIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNITIEN_A {
        match self.bits {
            false => UNITIEN_A::_0,
            true => UNITIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UNITIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UNITIEN_A::_1
    }
}
impl core::ops::Deref for UNITIEN_R {
    type Target = crate::FieldReader<bool, UNITIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNITIEN` writer - Unit Transfer Interrupt Enable Bit"]
pub struct UNITIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNITIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNITIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "QSPI unit transfer interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNITIEN_A::_0)
    }
    #[doc = "QSPI unit transfer interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNITIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Slave Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_A {
    #[doc = "0: Master mode"]
    _0 = 0,
    #[doc = "1: Slave mode"]
    _1 = 1,
}
impl From<SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE` reader - Slave Mode Control"]
pub struct SLAVE_R(crate::FieldReader<bool, SLAVE_A>);
impl SLAVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE_A {
        match self.bits {
            false => SLAVE_A::_0,
            true => SLAVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLAVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLAVE_A::_1
    }
}
impl core::ops::Deref for SLAVE_R {
    type Target = crate::FieldReader<bool, SLAVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE` writer - Slave Mode Control"]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLAVE_A::_0)
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLAVE_A::_1)
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
#[doc = "Byte Reorder Function Enable Bit\nNote: Byte Reorder function is only available if DWIDTH is defined as 16, 24, and 32 bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REORDER_A {
    #[doc = "0: Byte Reorder function Disabled"]
    _0 = 0,
    #[doc = "1: Byte Reorder function Enabled. A byte suspend interval will be inserted among each byte. The period of the byte suspend interval depends on the setting of SUSPITV"]
    _1 = 1,
}
impl From<REORDER_A> for bool {
    #[inline(always)]
    fn from(variant: REORDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REORDER` reader - Byte Reorder Function Enable Bit\nNote: Byte Reorder function is only available if DWIDTH is defined as 16, 24, and 32 bits."]
pub struct REORDER_R(crate::FieldReader<bool, REORDER_A>);
impl REORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        REORDER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REORDER_A {
        match self.bits {
            false => REORDER_A::_0,
            true => REORDER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REORDER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REORDER_A::_1
    }
}
impl core::ops::Deref for REORDER_R {
    type Target = crate::FieldReader<bool, REORDER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REORDER` writer - Byte Reorder Function Enable Bit\nNote: Byte Reorder function is only available if DWIDTH is defined as 16, 24, and 32 bits."]
pub struct REORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> REORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REORDER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Byte Reorder function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REORDER_A::_0)
    }
    #[doc = "Byte Reorder function Enabled. A byte suspend interval will be inserted among each byte. The period of the byte suspend interval depends on the setting of SUSPITV"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REORDER_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Data Port Direction Control\nThis bit is used to select the data input/output direction in half-duplex transfer and Dual/Quad transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATDIR_A {
    #[doc = "0: QSPI data is input direction"]
    _0 = 0,
    #[doc = "1: QSPI data is output direction"]
    _1 = 1,
}
impl From<DATDIR_A> for bool {
    #[inline(always)]
    fn from(variant: DATDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATDIR` reader - Data Port Direction Control\nThis bit is used to select the data input/output direction in half-duplex transfer and Dual/Quad transfer"]
pub struct DATDIR_R(crate::FieldReader<bool, DATDIR_A>);
impl DATDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATDIR_A {
        match self.bits {
            false => DATDIR_A::_0,
            true => DATDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATDIR_A::_1
    }
}
impl core::ops::Deref for DATDIR_R {
    type Target = crate::FieldReader<bool, DATDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATDIR` writer - Data Port Direction Control\nThis bit is used to select the data input/output direction in half-duplex transfer and Dual/Quad transfer"]
pub struct DATDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATDIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "QSPI data is input direction"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATDIR_A::_0)
    }
    #[doc = "QSPI data is output direction"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATDIR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Dual I/O Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALIOEN_A {
    #[doc = "0: Dual I/O mode Disabled"]
    _0 = 0,
    #[doc = "1: Dual I/O mode Enabled"]
    _1 = 1,
}
impl From<DUALIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: DUALIOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUALIOEN` reader - Dual I/O Mode Enable Bit"]
pub struct DUALIOEN_R(crate::FieldReader<bool, DUALIOEN_A>);
impl DUALIOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUALIOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUALIOEN_A {
        match self.bits {
            false => DUALIOEN_A::_0,
            true => DUALIOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DUALIOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DUALIOEN_A::_1
    }
}
impl core::ops::Deref for DUALIOEN_R {
    type Target = crate::FieldReader<bool, DUALIOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUALIOEN` writer - Dual I/O Mode Enable Bit"]
pub struct DUALIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALIOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUALIOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Dual I/O mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DUALIOEN_A::_0)
    }
    #[doc = "Dual I/O mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DUALIOEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Quad I/O Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADIOEN_A {
    #[doc = "0: Quad I/O mode Disabled"]
    _0 = 0,
    #[doc = "1: Quad I/O mode Enabled"]
    _1 = 1,
}
impl From<QUADIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: QUADIOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QUADIOEN` reader - Quad I/O Mode Enable Bit"]
pub struct QUADIOEN_R(crate::FieldReader<bool, QUADIOEN_A>);
impl QUADIOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        QUADIOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADIOEN_A {
        match self.bits {
            false => QUADIOEN_A::_0,
            true => QUADIOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == QUADIOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == QUADIOEN_A::_1
    }
}
impl core::ops::Deref for QUADIOEN_R {
    type Target = crate::FieldReader<bool, QUADIOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUADIOEN` writer - Quad I/O Mode Enable Bit"]
pub struct QUADIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADIOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUADIOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Quad I/O mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADIOEN_A::_0)
    }
    #[doc = "Quad I/O mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADIOEN_A::_1)
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
    #[doc = "Bit 0 - QSPI Transfer Control Enable Bit In Master mode, the transfer will start when there is data in the FIFO buffer after this bit is set to 1. In Slave mode, this device is ready to receive data when this bit is set to 1. Note: Before changing the configurations of QSPIx_CTL, QSPIx_CLKDIV, QSPIx_SSCTL and QSPIx_FIFOCTL registers, user shall clear the SPIEN (QSPIx_CTL\\[0\\]) and confirm the SPIENSTS (QSPIx_STATUS\\[15\\]) is 0."]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive on Negative Edge"]
    #[inline(always)]
    pub fn rxneg(&self) -> RXNEG_R {
        RXNEG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit on Negative Edge"]
    #[inline(always)]
    pub fn txneg(&self) -> TXNEG_R {
        TXNEG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Suspend Interval (Master Only) The four bits provide configurable suspend interval between two successive transmit/receive transaction in a transfer. The definition of the suspend interval is the interval between the last clock edge of the preceding transaction word and the first clock edge of the following transaction word. The default value is 0x3. The period of the suspend interval is obtained according to the following equation. (SUSPITV + 0.5) * period of QSPICLK clock cycle Example:"]
    #[inline(always)]
    pub fn suspitv(&self) -> SUSPITV_R {
        SUSPITV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data Width This field specifies how many bits can be transmitted / received in one transaction. The minimum bit length is 8 bits and can up to 32 bits."]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Send LSB First"]
    #[inline(always)]
    pub fn lsb(&self) -> LSB_R {
        LSB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QSPI Half-duplex Transfer Enable Bit This bit is used to select full-duplex or half-duplex for QSPI transfer. The bit field DATDIR (QSPIx_CTL\\[20\\]) can be used to set the data direction in half-duplex transfer."]
    #[inline(always)]
    pub fn halfdpx(&self) -> HALFDPX_R {
        HALFDPX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive-only Mode Enable Bit (Master Only) This bit field is only available in Master mode. In receive-only mode, QSPI Master will generate QSPI bus clock continuously for receiving data bit from QSPI slave device and assert the BUSY status."]
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 2-bit Transfer Mode Enable Bit Note: When 2-bit Transfer mode is enabled, the first serial transmitted bit data is from the first FIFO buffer data, and the 2nd serial transmitted bit data is from the second FIFO buffer data. As the same as transmitted function, the first received bit data is stored into the first FIFO buffer and the 2nd received bit data is stored into the second FIFO buffer at the same time."]
    #[inline(always)]
    pub fn twobit(&self) -> TWOBIT_R {
        TWOBIT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Unit Transfer Interrupt Enable Bit"]
    #[inline(always)]
    pub fn unitien(&self) -> UNITIEN_R {
        UNITIEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Slave Mode Control"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Byte Reorder Function Enable Bit Note: Byte Reorder function is only available if DWIDTH is defined as 16, 24, and 32 bits."]
    #[inline(always)]
    pub fn reorder(&self) -> REORDER_R {
        REORDER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Port Direction Control This bit is used to select the data input/output direction in half-duplex transfer and Dual/Quad transfer"]
    #[inline(always)]
    pub fn datdir(&self) -> DATDIR_R {
        DATDIR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Dual I/O Mode Enable Bit"]
    #[inline(always)]
    pub fn dualioen(&self) -> DUALIOEN_R {
        DUALIOEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Quad I/O Mode Enable Bit"]
    #[inline(always)]
    pub fn quadioen(&self) -> QUADIOEN_R {
        QUADIOEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QSPI Transfer Control Enable Bit In Master mode, the transfer will start when there is data in the FIFO buffer after this bit is set to 1. In Slave mode, this device is ready to receive data when this bit is set to 1. Note: Before changing the configurations of QSPIx_CTL, QSPIx_CLKDIV, QSPIx_SSCTL and QSPIx_FIFOCTL registers, user shall clear the SPIEN (QSPIx_CTL\\[0\\]) and confirm the SPIENSTS (QSPIx_STATUS\\[15\\]) is 0."]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W {
        SPIEN_W { w: self }
    }
    #[doc = "Bit 1 - Receive on Negative Edge"]
    #[inline(always)]
    pub fn rxneg(&mut self) -> RXNEG_W {
        RXNEG_W { w: self }
    }
    #[doc = "Bit 2 - Transmit on Negative Edge"]
    #[inline(always)]
    pub fn txneg(&mut self) -> TXNEG_W {
        TXNEG_W { w: self }
    }
    #[doc = "Bit 3 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W {
        CLKPOL_W { w: self }
    }
    #[doc = "Bits 4:7 - Suspend Interval (Master Only) The four bits provide configurable suspend interval between two successive transmit/receive transaction in a transfer. The definition of the suspend interval is the interval between the last clock edge of the preceding transaction word and the first clock edge of the following transaction word. The default value is 0x3. The period of the suspend interval is obtained according to the following equation. (SUSPITV + 0.5) * period of QSPICLK clock cycle Example:"]
    #[inline(always)]
    pub fn suspitv(&mut self) -> SUSPITV_W {
        SUSPITV_W { w: self }
    }
    #[doc = "Bits 8:12 - Data Width This field specifies how many bits can be transmitted / received in one transaction. The minimum bit length is 8 bits and can up to 32 bits."]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DWIDTH_W {
        DWIDTH_W { w: self }
    }
    #[doc = "Bit 13 - Send LSB First"]
    #[inline(always)]
    pub fn lsb(&mut self) -> LSB_W {
        LSB_W { w: self }
    }
    #[doc = "Bit 14 - QSPI Half-duplex Transfer Enable Bit This bit is used to select full-duplex or half-duplex for QSPI transfer. The bit field DATDIR (QSPIx_CTL\\[20\\]) can be used to set the data direction in half-duplex transfer."]
    #[inline(always)]
    pub fn halfdpx(&mut self) -> HALFDPX_W {
        HALFDPX_W { w: self }
    }
    #[doc = "Bit 15 - Receive-only Mode Enable Bit (Master Only) This bit field is only available in Master mode. In receive-only mode, QSPI Master will generate QSPI bus clock continuously for receiving data bit from QSPI slave device and assert the BUSY status."]
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W {
        RXONLY_W { w: self }
    }
    #[doc = "Bit 16 - 2-bit Transfer Mode Enable Bit Note: When 2-bit Transfer mode is enabled, the first serial transmitted bit data is from the first FIFO buffer data, and the 2nd serial transmitted bit data is from the second FIFO buffer data. As the same as transmitted function, the first received bit data is stored into the first FIFO buffer and the 2nd received bit data is stored into the second FIFO buffer at the same time."]
    #[inline(always)]
    pub fn twobit(&mut self) -> TWOBIT_W {
        TWOBIT_W { w: self }
    }
    #[doc = "Bit 17 - Unit Transfer Interrupt Enable Bit"]
    #[inline(always)]
    pub fn unitien(&mut self) -> UNITIEN_W {
        UNITIEN_W { w: self }
    }
    #[doc = "Bit 18 - Slave Mode Control"]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Bit 19 - Byte Reorder Function Enable Bit Note: Byte Reorder function is only available if DWIDTH is defined as 16, 24, and 32 bits."]
    #[inline(always)]
    pub fn reorder(&mut self) -> REORDER_W {
        REORDER_W { w: self }
    }
    #[doc = "Bit 20 - Data Port Direction Control This bit is used to select the data input/output direction in half-duplex transfer and Dual/Quad transfer"]
    #[inline(always)]
    pub fn datdir(&mut self) -> DATDIR_W {
        DATDIR_W { w: self }
    }
    #[doc = "Bit 21 - Dual I/O Mode Enable Bit"]
    #[inline(always)]
    pub fn dualioen(&mut self) -> DUALIOEN_W {
        DUALIOEN_W { w: self }
    }
    #[doc = "Bit 22 - Quad I/O Mode Enable Bit"]
    #[inline(always)]
    pub fn quadioen(&mut self) -> QUADIOEN_W {
        QUADIOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspix_ctl](index.html) module"]
pub struct QSPIX_CTL_SPEC;
impl crate::RegisterSpec for QSPIX_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspix_ctl::R](R) reader structure"]
impl crate::Readable for QSPIX_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspix_ctl::W](W) writer structure"]
impl crate::Writable for QSPIX_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPIx_CTL to value 0x34"]
impl crate::Resettable for QSPIX_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x34
    }
}
