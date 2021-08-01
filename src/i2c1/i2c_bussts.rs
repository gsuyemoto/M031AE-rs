#[doc = "Register `I2C_BUSSTS` reader"]
pub struct R(crate::R<I2C_BUSSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_BUSSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_BUSSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_BUSSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_BUSSTS` writer"]
pub struct W(crate::W<I2C_BUSSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_BUSSTS_SPEC>;
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
impl From<crate::W<I2C_BUSSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_BUSSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bus Busy (Read Only)\nIndicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Bus is IDLE (both SCLK and SDA High)"]
    _0 = 0,
    #[doc = "1: Bus is busy"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Bus Busy (Read Only)\nIndicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected"]
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
#[doc = "Byte Count Transmission/Receive Done \nNote: Software can write 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCDONE_A {
    #[doc = "0: Byte count transmission/ receive is not finished when the PECEN is set"]
    _0 = 0,
    #[doc = "1: Byte count transmission/ receive is finished when the PECEN is set"]
    _1 = 1,
}
impl From<BCDONE_A> for bool {
    #[inline(always)]
    fn from(variant: BCDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCDONE` reader - Byte Count Transmission/Receive Done \nNote: Software can write 1 to clear this bit."]
pub struct BCDONE_R(crate::FieldReader<bool, BCDONE_A>);
impl BCDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCDONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCDONE_A {
        match self.bits {
            false => BCDONE_A::_0,
            true => BCDONE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BCDONE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BCDONE_A::_1
    }
}
impl core::ops::Deref for BCDONE_R {
    type Target = crate::FieldReader<bool, BCDONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDONE` writer - Byte Count Transmission/Receive Done \nNote: Software can write 1 to clear this bit."]
pub struct BCDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCDONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Byte count transmission/ receive is not finished when the PECEN is set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCDONE_A::_0)
    }
    #[doc = "Byte count transmission/ receive is finished when the PECEN is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCDONE_A::_1)
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
#[doc = "PEC Error in Reception \nNote: Software can write 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERR_A {
    #[doc = "0: PEC value equal the received PEC data packet"]
    _0 = 0,
    #[doc = "1: PEC value doesn't match the receive PEC data packet"]
    _1 = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` reader - PEC Error in Reception \nNote: Software can write 1 to clear this bit."]
pub struct PECERR_R(crate::FieldReader<bool, PECERR_A>);
impl PECERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::_0,
            true => PECERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PECERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PECERR_A::_1
    }
}
impl core::ops::Deref for PECERR_R {
    type Target = crate::FieldReader<bool, PECERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECERR` writer - PEC Error in Reception \nNote: Software can write 1 to clear this bit."]
pub struct PECERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PECERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PECERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PEC value equal the received PEC data packet"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PECERR_A::_0)
    }
    #[doc = "PEC value doesn't match the receive PEC data packet"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PECERR_A::_1)
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
#[doc = "SMBus Alert Status \nNote: 1. The SMBALERT pin is an open-drain pin, the pull-high resistor is must in the system. 2. Software can write 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERT_A {
    #[doc = "0: SMBALERT pin state is low.\\nNo SMBALERT event"]
    _0 = 0,
    #[doc = "1: SMBALERT pin state is high.\\nThere is SMBALERT event (falling edge) is detected in SMBALERT pin when the BMHEN = 1 (SMBus host configuration) and the ALERTEN = 1"]
    _1 = 1,
}
impl From<ALERT_A> for bool {
    #[inline(always)]
    fn from(variant: ALERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERT` reader - SMBus Alert Status \nNote: 1. The SMBALERT pin is an open-drain pin, the pull-high resistor is must in the system. 2. Software can write 1 to clear this bit."]
pub struct ALERT_R(crate::FieldReader<bool, ALERT_A>);
impl ALERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALERT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALERT_A {
        match self.bits {
            false => ALERT_A::_0,
            true => ALERT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALERT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALERT_A::_1
    }
}
impl core::ops::Deref for ALERT_R {
    type Target = crate::FieldReader<bool, ALERT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALERT` writer - SMBus Alert Status \nNote: 1. The SMBALERT pin is an open-drain pin, the pull-high resistor is must in the system. 2. Software can write 1 to clear this bit."]
pub struct ALERT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALERT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SMBALERT pin state is low.\nNo SMBALERT event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALERT_A::_0)
    }
    #[doc = "SMBALERT pin state is high.\nThere is SMBALERT event (falling edge) is detected in SMBALERT pin when the BMHEN = 1 (SMBus host configuration) and the ALERTEN = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALERT_A::_1)
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
#[doc = "Bus Suspend or Control Signal Input Status (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTLDIN_A {
    #[doc = "0: The input status of SUSCON pin is 0"]
    _0 = 0,
    #[doc = "1: The input status of SUSCON pin is 1"]
    _1 = 1,
}
impl From<SCTLDIN_A> for bool {
    #[inline(always)]
    fn from(variant: SCTLDIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTLDIN` reader - Bus Suspend or Control Signal Input Status (Read Only)"]
pub struct SCTLDIN_R(crate::FieldReader<bool, SCTLDIN_A>);
impl SCTLDIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCTLDIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCTLDIN_A {
        match self.bits {
            false => SCTLDIN_A::_0,
            true => SCTLDIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCTLDIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCTLDIN_A::_1
    }
}
impl core::ops::Deref for SCTLDIN_R {
    type Target = crate::FieldReader<bool, SCTLDIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bus Time-out Status \nIn bus busy, the bit indicates the total clock low time-out event occurred; otherwise, it indicates the bus idle time-out event occurred.\nNote: Software can write 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSTO_A {
    #[doc = "0: There is no any time-out or external clock time-out"]
    _0 = 0,
    #[doc = "1: A time-out or external clock time-out occurred"]
    _1 = 1,
}
impl From<BUSTO_A> for bool {
    #[inline(always)]
    fn from(variant: BUSTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSTO` reader - Bus Time-out Status \nIn bus busy, the bit indicates the total clock low time-out event occurred; otherwise, it indicates the bus idle time-out event occurred.\nNote: Software can write 1 to clear this bit."]
pub struct BUSTO_R(crate::FieldReader<bool, BUSTO_A>);
impl BUSTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSTO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSTO_A {
        match self.bits {
            false => BUSTO_A::_0,
            true => BUSTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSTO_A::_1
    }
}
impl core::ops::Deref for BUSTO_R {
    type Target = crate::FieldReader<bool, BUSTO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSTO` writer - Bus Time-out Status \nIn bus busy, the bit indicates the total clock low time-out event occurred; otherwise, it indicates the bus idle time-out event occurred.\nNote: Software can write 1 to clear this bit."]
pub struct BUSTO_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSTO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "There is no any time-out or external clock time-out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSTO_A::_0)
    }
    #[doc = "A time-out or external clock time-out occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSTO_A::_1)
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
#[doc = "Clock Low Cumulate Time-out Status \nNote: Software can write 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKTO_A {
    #[doc = "0: Cumulative clock low is no any time-out"]
    _0 = 0,
    #[doc = "1: Cumulative clock low time-out occurred"]
    _1 = 1,
}
impl From<CLKTO_A> for bool {
    #[inline(always)]
    fn from(variant: CLKTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKTO` reader - Clock Low Cumulate Time-out Status \nNote: Software can write 1 to clear this bit."]
pub struct CLKTO_R(crate::FieldReader<bool, CLKTO_A>);
impl CLKTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKTO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKTO_A {
        match self.bits {
            false => CLKTO_A::_0,
            true => CLKTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKTO_A::_1
    }
}
impl core::ops::Deref for CLKTO_R {
    type Target = crate::FieldReader<bool, CLKTO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKTO` writer - Clock Low Cumulate Time-out Status \nNote: Software can write 1 to clear this bit."]
pub struct CLKTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKTO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Cumulative clock low is no any time-out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKTO_A::_0)
    }
    #[doc = "Cumulative clock low time-out occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKTO_A::_1)
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
#[doc = "PEC Byte Transmission/Receive Done \nNote: Software can write 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECDONE_A {
    #[doc = "0: PEC transmission/ receive is not finished when the PECEN is set"]
    _0 = 0,
    #[doc = "1: PEC transmission/ receive is finished when the PECEN is set"]
    _1 = 1,
}
impl From<PECDONE_A> for bool {
    #[inline(always)]
    fn from(variant: PECDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECDONE` reader - PEC Byte Transmission/Receive Done \nNote: Software can write 1 to clear this bit."]
pub struct PECDONE_R(crate::FieldReader<bool, PECDONE_A>);
impl PECDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECDONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECDONE_A {
        match self.bits {
            false => PECDONE_A::_0,
            true => PECDONE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PECDONE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PECDONE_A::_1
    }
}
impl core::ops::Deref for PECDONE_R {
    type Target = crate::FieldReader<bool, PECDONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECDONE` writer - PEC Byte Transmission/Receive Done \nNote: Software can write 1 to clear this bit."]
pub struct PECDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> PECDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PECDONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PEC transmission/ receive is not finished when the PECEN is set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PECDONE_A::_0)
    }
    #[doc = "PEC transmission/ receive is finished when the PECEN is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PECDONE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bus Busy (Read Only) Indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Byte Count Transmission/Receive Done Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn bcdone(&self) -> BCDONE_R {
        BCDONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PEC Error in Reception Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMBus Alert Status Note: 1. The SMBALERT pin is an open-drain pin, the pull-high resistor is must in the system. 2. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bus Suspend or Control Signal Input Status (Read Only)"]
    #[inline(always)]
    pub fn sctldin(&self) -> SCTLDIN_R {
        SCTLDIN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bus Time-out Status In bus busy, the bit indicates the total clock low time-out event occurred; otherwise, it indicates the bus idle time-out event occurred. Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn busto(&self) -> BUSTO_R {
        BUSTO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clock Low Cumulate Time-out Status Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn clkto(&self) -> CLKTO_R {
        CLKTO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PEC Byte Transmission/Receive Done Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn pecdone(&self) -> PECDONE_R {
        PECDONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Byte Count Transmission/Receive Done Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn bcdone(&mut self) -> BCDONE_W {
        BCDONE_W { w: self }
    }
    #[doc = "Bit 2 - PEC Error in Reception Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W {
        PECERR_W { w: self }
    }
    #[doc = "Bit 3 - SMBus Alert Status Note: 1. The SMBALERT pin is an open-drain pin, the pull-high resistor is must in the system. 2. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn alert(&mut self) -> ALERT_W {
        ALERT_W { w: self }
    }
    #[doc = "Bit 5 - Bus Time-out Status In bus busy, the bit indicates the total clock low time-out event occurred; otherwise, it indicates the bus idle time-out event occurred. Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn busto(&mut self) -> BUSTO_W {
        BUSTO_W { w: self }
    }
    #[doc = "Bit 6 - Clock Low Cumulate Time-out Status Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn clkto(&mut self) -> CLKTO_W {
        CLKTO_W { w: self }
    }
    #[doc = "Bit 7 - PEC Byte Transmission/Receive Done Note: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn pecdone(&mut self) -> PECDONE_W {
        PECDONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Management Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_bussts](index.html) module"]
pub struct I2C_BUSSTS_SPEC;
impl crate::RegisterSpec for I2C_BUSSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_bussts::R](R) reader structure"]
impl crate::Readable for I2C_BUSSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_bussts::W](W) writer structure"]
impl crate::Writable for I2C_BUSSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_BUSSTS to value 0"]
impl crate::Resettable for I2C_BUSSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
