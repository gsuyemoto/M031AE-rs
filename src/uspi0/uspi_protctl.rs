#[doc = "Register `USPI_PROTCTL` reader"]
pub struct R(crate::R<USPI_PROTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USPI_PROTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USPI_PROTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USPI_PROTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USPI_PROTCTL` writer"]
pub struct W(crate::W<USPI_PROTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USPI_PROTCTL_SPEC>;
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
impl From<crate::W<USPI_PROTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USPI_PROTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Mode Selection\n\nValue on reset: 0"]
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
#[doc = "Field `SLAVE` reader - Slave Mode Selection"]
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
#[doc = "Field `SLAVE` writer - Slave Mode Selection"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Slave 3-wire Mode Selection (Slave Only)\nThe SPI protocol can work with 3-wire interface (without slave select signal) in Slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLV3WIRE_A {
    #[doc = "0: 4-wire bi-direction interface"]
    _0 = 0,
    #[doc = "1: 3-wire bi-direction interface"]
    _1 = 1,
}
impl From<SLV3WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: SLV3WIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLV3WIRE` reader - Slave 3-wire Mode Selection (Slave Only)\nThe SPI protocol can work with 3-wire interface (without slave select signal) in Slave mode."]
pub struct SLV3WIRE_R(crate::FieldReader<bool, SLV3WIRE_A>);
impl SLV3WIRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV3WIRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLV3WIRE_A {
        match self.bits {
            false => SLV3WIRE_A::_0,
            true => SLV3WIRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLV3WIRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLV3WIRE_A::_1
    }
}
impl core::ops::Deref for SLV3WIRE_R {
    type Target = crate::FieldReader<bool, SLV3WIRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV3WIRE` writer - Slave 3-wire Mode Selection (Slave Only)\nThe SPI protocol can work with 3-wire interface (without slave select signal) in Slave mode."]
pub struct SLV3WIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV3WIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLV3WIRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4-wire bi-direction interface"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLV3WIRE_A::_0)
    }
    #[doc = "3-wire bi-direction interface"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLV3WIRE_A::_1)
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
#[doc = "Field `SS` reader - Slave Select Control (Master Only)\nIf AUTOSS bit is cleared, setting this bit to 1 will set the slave select signal to active state, and setting this bit to 0 will set the slave select back to inactive state.\nNote: In SPI protocol, the internal slave select signal is active high."]
pub struct SS_R(crate::FieldReader<bool, bool>);
impl SS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS` writer - Slave Select Control (Master Only)\nIf AUTOSS bit is cleared, setting this bit to 1 will set the slave select signal to active state, and setting this bit to 0 will set the slave select back to inactive state.\nNote: In SPI protocol, the internal slave select signal is active high."]
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
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
#[doc = "Automatic Slave Select Function Enable (Master Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOSS_A {
    #[doc = "0: Slave select signal will be controlled by the setting value of SS (USPI_PROTCTL\\[2\\]) bit"]
    _0 = 0,
    #[doc = "1: Slave select signal will be generated automatically. The slave select signal will be asserted by the SPI controller when transmit/receive is started, and will be de-asserted after each transmit/receive is finished"]
    _1 = 1,
}
impl From<AUTOSS_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOSS` reader - Automatic Slave Select Function Enable (Master Only)"]
pub struct AUTOSS_R(crate::FieldReader<bool, AUTOSS_A>);
impl AUTOSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTOSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOSS_A {
        match self.bits {
            false => AUTOSS_A::_0,
            true => AUTOSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AUTOSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AUTOSS_A::_1
    }
}
impl core::ops::Deref for AUTOSS_R {
    type Target = crate::FieldReader<bool, AUTOSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOSS` writer - Automatic Slave Select Function Enable (Master Only)"]
pub struct AUTOSS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOSS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave select signal will be controlled by the setting value of SS (USPI_PROTCTL\\[2\\]) bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUTOSS_A::_0)
    }
    #[doc = "Slave select signal will be generated automatically. The slave select signal will be asserted by the SPI controller when transmit/receive is started, and will be de-asserted after each transmit/receive is finished"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUTOSS_A::_1)
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
#[doc = "Field `SCLKMODE` reader - Serial Bus Clock Mode\nThis bit field defines the SCLK idle status, data transmit, and data receive edge."]
pub struct SCLKMODE_R(crate::FieldReader<u8, u8>);
impl SCLKMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLKMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLKMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLKMODE` writer - Serial Bus Clock Mode\nThis bit field defines the SCLK idle status, data transmit, and data receive edge."]
pub struct SCLKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `SUSPITV` reader - Suspend Interval (Master Only)\nThis bit field provides the configurable suspend interval between two successive transmit/receive transaction in a transfer. The definition of the suspend interval is the interval between the last clock edge of the preceding transaction word and the first clock edge of the following transaction word. The default value is 0x3. The period of the suspend interval is obtained according to the following equation.\n (SUSPITV\\[3:0\\]
+ 0.5) * period of SPI_CLK clock cycle\nExample:"]
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
#[doc = "Field `SUSPITV` writer - Suspend Interval (Master Only)\nThis bit field provides the configurable suspend interval between two successive transmit/receive transaction in a transfer. The definition of the suspend interval is the interval between the last clock edge of the preceding transaction word and the first clock edge of the following transaction word. The default value is 0x3. The period of the suspend interval is obtained according to the following equation.\n (SUSPITV\\[3:0\\]
+ 0.5) * period of SPI_CLK clock cycle\nExample:"]
pub struct SUSPITV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPITV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TSMSEL` reader - Transmit Data Mode Selection\nThis bit field describes how receive and transmit data is shifted in and out.\nOther values are reserved.\nNote: Changing the value of this bit field will produce the TXRST and RXRST to clear the TX/RX data buffer automatically."]
pub struct TSMSEL_R(crate::FieldReader<u8, u8>);
impl TSMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSMSEL` writer - Transmit Data Mode Selection\nThis bit field describes how receive and transmit data is shifted in and out.\nOther values are reserved.\nNote: Changing the value of this bit field will produce the TXRST and RXRST to clear the TX/RX data buffer automatically."]
pub struct TSMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `SLVTOCNT` reader - Slave Mode Time-out Period (Slave Only)\nIn Slave mode, this bit field is used for Slave time-out period. This bit field indicates how many clock periods (selected by TMCNTSRC, USPI_BRGEN\\[5\\]) between the two edges of input SCLK will assert the Slave time-out event. Writing 0x0 into this bit field will disable the Slave time-out function.\nExample: Assume SLVTOCNT is 0x0A and TMCNTSRC (USPI_BRGEN\\[5\\]) is 1, it means the time-out event will occur if the state of SPI bus clock pin is not changed more than (10+1) periods of fDIV_CLK."]
pub struct SLVTOCNT_R(crate::FieldReader<u16, u16>);
impl SLVTOCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        SLVTOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLVTOCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVTOCNT` writer - Slave Mode Time-out Period (Slave Only)\nIn Slave mode, this bit field is used for Slave time-out period. This bit field indicates how many clock periods (selected by TMCNTSRC, USPI_BRGEN\\[5\\]) between the two edges of input SCLK will assert the Slave time-out event. Writing 0x0 into this bit field will disable the Slave time-out function.\nExample: Assume SLVTOCNT is 0x0A and TMCNTSRC (USPI_BRGEN\\[5\\]) is 1, it means the time-out event will occur if the state of SPI bus clock pin is not changed more than (10+1) periods of fDIV_CLK."]
pub struct SLVTOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVTOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Transmit Under-run Data Polarity (for Slave)\nThis bit defines the transmitting data value of USCIx_DAT1 when no data is available for transferring.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUDRPOL_A {
    #[doc = "0: The output data value is 0 if TX under run event occurs"]
    _0 = 0,
    #[doc = "1: The output data value is 1 if TX under run event occurs"]
    _1 = 1,
}
impl From<TXUDRPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TXUDRPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUDRPOL` reader - Transmit Under-run Data Polarity (for Slave)\nThis bit defines the transmitting data value of USCIx_DAT1 when no data is available for transferring."]
pub struct TXUDRPOL_R(crate::FieldReader<bool, TXUDRPOL_A>);
impl TXUDRPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUDRPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUDRPOL_A {
        match self.bits {
            false => TXUDRPOL_A::_0,
            true => TXUDRPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXUDRPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXUDRPOL_A::_1
    }
}
impl core::ops::Deref for TXUDRPOL_R {
    type Target = crate::FieldReader<bool, TXUDRPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUDRPOL` writer - Transmit Under-run Data Polarity (for Slave)\nThis bit defines the transmitting data value of USCIx_DAT1 when no data is available for transferring."]
pub struct TXUDRPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDRPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUDRPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output data value is 0 if TX under run event occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXUDRPOL_A::_0)
    }
    #[doc = "The output data value is 1 if TX under run event occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXUDRPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "SPI Protocol Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTEN_A {
    #[doc = "0: SPI Protocol Disabled"]
    _0 = 0,
    #[doc = "1: SPI Protocol Enabled"]
    _1 = 1,
}
impl From<PROTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PROTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTEN` reader - SPI Protocol Enable Bit"]
pub struct PROTEN_R(crate::FieldReader<bool, PROTEN_A>);
impl PROTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTEN_A {
        match self.bits {
            false => PROTEN_A::_0,
            true => PROTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PROTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PROTEN_A::_1
    }
}
impl core::ops::Deref for PROTEN_R {
    type Target = crate::FieldReader<bool, PROTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROTEN` writer - SPI Protocol Enable Bit"]
pub struct PROTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI Protocol Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROTEN_A::_0)
    }
    #[doc = "SPI Protocol Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Mode Selection"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave 3-wire Mode Selection (Slave Only) The SPI protocol can work with 3-wire interface (without slave select signal) in Slave mode."]
    #[inline(always)]
    pub fn slv3wire(&self) -> SLV3WIRE_R {
        SLV3WIRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave Select Control (Master Only) If AUTOSS bit is cleared, setting this bit to 1 will set the slave select signal to active state, and setting this bit to 0 will set the slave select back to inactive state. Note: In SPI protocol, the internal slave select signal is active high."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Automatic Slave Select Function Enable (Master Only)"]
    #[inline(always)]
    pub fn autoss(&self) -> AUTOSS_R {
        AUTOSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Serial Bus Clock Mode This bit field defines the SCLK idle status, data transmit, and data receive edge."]
    #[inline(always)]
    pub fn sclkmode(&self) -> SCLKMODE_R {
        SCLKMODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Suspend Interval (Master Only) This bit field provides the configurable suspend interval between two successive transmit/receive transaction in a transfer. The definition of the suspend interval is the interval between the last clock edge of the preceding transaction word and the first clock edge of the following transaction word. The default value is 0x3. The period of the suspend interval is obtained according to the following equation. (SUSPITV\\[3:0\\]
+ 0.5) * period of SPI_CLK clock cycle Example:"]
    #[inline(always)]
    pub fn suspitv(&self) -> SUSPITV_R {
        SUSPITV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Transmit Data Mode Selection This bit field describes how receive and transmit data is shifted in and out. Other values are reserved. Note: Changing the value of this bit field will produce the TXRST and RXRST to clear the TX/RX data buffer automatically."]
    #[inline(always)]
    pub fn tsmsel(&self) -> TSMSEL_R {
        TSMSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:25 - Slave Mode Time-out Period (Slave Only) In Slave mode, this bit field is used for Slave time-out period. This bit field indicates how many clock periods (selected by TMCNTSRC, USPI_BRGEN\\[5\\]) between the two edges of input SCLK will assert the Slave time-out event. Writing 0x0 into this bit field will disable the Slave time-out function. Example: Assume SLVTOCNT is 0x0A and TMCNTSRC (USPI_BRGEN\\[5\\]) is 1, it means the time-out event will occur if the state of SPI bus clock pin is not changed more than (10+1) periods of fDIV_CLK."]
    #[inline(always)]
    pub fn slvtocnt(&self) -> SLVTOCNT_R {
        SLVTOCNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - Transmit Under-run Data Polarity (for Slave) This bit defines the transmitting data value of USCIx_DAT1 when no data is available for transferring."]
    #[inline(always)]
    pub fn txudrpol(&self) -> TXUDRPOL_R {
        TXUDRPOL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPI Protocol Enable Bit"]
    #[inline(always)]
    pub fn proten(&self) -> PROTEN_R {
        PROTEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Mode Selection"]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Bit 1 - Slave 3-wire Mode Selection (Slave Only) The SPI protocol can work with 3-wire interface (without slave select signal) in Slave mode."]
    #[inline(always)]
    pub fn slv3wire(&mut self) -> SLV3WIRE_W {
        SLV3WIRE_W { w: self }
    }
    #[doc = "Bit 2 - Slave Select Control (Master Only) If AUTOSS bit is cleared, setting this bit to 1 will set the slave select signal to active state, and setting this bit to 0 will set the slave select back to inactive state. Note: In SPI protocol, the internal slave select signal is active high."]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
    #[doc = "Bit 3 - Automatic Slave Select Function Enable (Master Only)"]
    #[inline(always)]
    pub fn autoss(&mut self) -> AUTOSS_W {
        AUTOSS_W { w: self }
    }
    #[doc = "Bits 6:7 - Serial Bus Clock Mode This bit field defines the SCLK idle status, data transmit, and data receive edge."]
    #[inline(always)]
    pub fn sclkmode(&mut self) -> SCLKMODE_W {
        SCLKMODE_W { w: self }
    }
    #[doc = "Bits 8:11 - Suspend Interval (Master Only) This bit field provides the configurable suspend interval between two successive transmit/receive transaction in a transfer. The definition of the suspend interval is the interval between the last clock edge of the preceding transaction word and the first clock edge of the following transaction word. The default value is 0x3. The period of the suspend interval is obtained according to the following equation. (SUSPITV\\[3:0\\]
+ 0.5) * period of SPI_CLK clock cycle Example:"]
    #[inline(always)]
    pub fn suspitv(&mut self) -> SUSPITV_W {
        SUSPITV_W { w: self }
    }
    #[doc = "Bits 12:14 - Transmit Data Mode Selection This bit field describes how receive and transmit data is shifted in and out. Other values are reserved. Note: Changing the value of this bit field will produce the TXRST and RXRST to clear the TX/RX data buffer automatically."]
    #[inline(always)]
    pub fn tsmsel(&mut self) -> TSMSEL_W {
        TSMSEL_W { w: self }
    }
    #[doc = "Bits 16:25 - Slave Mode Time-out Period (Slave Only) In Slave mode, this bit field is used for Slave time-out period. This bit field indicates how many clock periods (selected by TMCNTSRC, USPI_BRGEN\\[5\\]) between the two edges of input SCLK will assert the Slave time-out event. Writing 0x0 into this bit field will disable the Slave time-out function. Example: Assume SLVTOCNT is 0x0A and TMCNTSRC (USPI_BRGEN\\[5\\]) is 1, it means the time-out event will occur if the state of SPI bus clock pin is not changed more than (10+1) periods of fDIV_CLK."]
    #[inline(always)]
    pub fn slvtocnt(&mut self) -> SLVTOCNT_W {
        SLVTOCNT_W { w: self }
    }
    #[doc = "Bit 28 - Transmit Under-run Data Polarity (for Slave) This bit defines the transmitting data value of USCIx_DAT1 when no data is available for transferring."]
    #[inline(always)]
    pub fn txudrpol(&mut self) -> TXUDRPOL_W {
        TXUDRPOL_W { w: self }
    }
    #[doc = "Bit 31 - SPI Protocol Enable Bit"]
    #[inline(always)]
    pub fn proten(&mut self) -> PROTEN_W {
        PROTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Protocol Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uspi_protctl](index.html) module"]
pub struct USPI_PROTCTL_SPEC;
impl crate::RegisterSpec for USPI_PROTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uspi_protctl::R](R) reader structure"]
impl crate::Readable for USPI_PROTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uspi_protctl::W](W) writer structure"]
impl crate::Writable for USPI_PROTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USPI_PROTCTL to value 0x0300"]
impl crate::Resettable for USPI_PROTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300
    }
}
