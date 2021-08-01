#[doc = "Register `UART_INTSTS` reader"]
pub struct R(crate::R<UART_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_INTSTS` writer"]
pub struct W(crate::W<UART_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_INTSTS_SPEC>;
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
impl From<crate::W<UART_INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Data Available Interrupt Flag \nWhen the number of bytes in the RX FIFO equals the RFITL then the RDAIF(UART_INTSTS\\[0\\]) will be set. If RDAIEN (UART_INTEN \\[0\\]) is enabled, the RDA interrupt will be generated.\nNote: This bit is read only and it will be cleared when the number of unread bytes of RX FIFO drops below the threshold level (RFITL(UART_FIFO\\[7:4\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDAIF_A {
    #[doc = "0: No RDA interrupt flag is generated"]
    _0 = 0,
    #[doc = "1: RDA interrupt flag is generated"]
    _1 = 1,
}
impl From<RDAIF_A> for bool {
    #[inline(always)]
    fn from(variant: RDAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDAIF` reader - Receive Data Available Interrupt Flag \nWhen the number of bytes in the RX FIFO equals the RFITL then the RDAIF(UART_INTSTS\\[0\\]) will be set. If RDAIEN (UART_INTEN \\[0\\]) is enabled, the RDA interrupt will be generated.\nNote: This bit is read only and it will be cleared when the number of unread bytes of RX FIFO drops below the threshold level (RFITL(UART_FIFO\\[7:4\\])."]
pub struct RDAIF_R(crate::FieldReader<bool, RDAIF_A>);
impl RDAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDAIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDAIF_A {
        match self.bits {
            false => RDAIF_A::_0,
            true => RDAIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RDAIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RDAIF_A::_1
    }
}
impl core::ops::Deref for RDAIF_R {
    type Target = crate::FieldReader<bool, RDAIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDAIF` writer - Receive Data Available Interrupt Flag \nWhen the number of bytes in the RX FIFO equals the RFITL then the RDAIF(UART_INTSTS\\[0\\]) will be set. If RDAIEN (UART_INTEN \\[0\\]) is enabled, the RDA interrupt will be generated.\nNote: This bit is read only and it will be cleared when the number of unread bytes of RX FIFO drops below the threshold level (RFITL(UART_FIFO\\[7:4\\])."]
pub struct RDAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RDAIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDAIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No RDA interrupt flag is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDAIF_A::_0)
    }
    #[doc = "RDA interrupt flag is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDAIF_A::_1)
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
#[doc = "Transmit Holding Register Empty Interrupt Flag\nThis bit is set when the last data of TX FIFO is transferred to Transmitter Shift Register. If THREIEN (UART_INTEN\\[1\\]) is enabled, the THRE interrupt will be generated.\nNote: This bit is read only and it will be cleared when writing data into UART_DAT (TX FIFO not empty).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREIF_A {
    #[doc = "0: No THRE interrupt flag is generated"]
    _0 = 0,
    #[doc = "1: THRE interrupt flag is generated"]
    _1 = 1,
}
impl From<THREIF_A> for bool {
    #[inline(always)]
    fn from(variant: THREIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREIF` reader - Transmit Holding Register Empty Interrupt Flag\nThis bit is set when the last data of TX FIFO is transferred to Transmitter Shift Register. If THREIEN (UART_INTEN\\[1\\]) is enabled, the THRE interrupt will be generated.\nNote: This bit is read only and it will be cleared when writing data into UART_DAT (TX FIFO not empty)."]
pub struct THREIF_R(crate::FieldReader<bool, THREIF_A>);
impl THREIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        THREIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREIF_A {
        match self.bits {
            false => THREIF_A::_0,
            true => THREIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == THREIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == THREIF_A::_1
    }
}
impl core::ops::Deref for THREIF_R {
    type Target = crate::FieldReader<bool, THREIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THREIF` writer - Transmit Holding Register Empty Interrupt Flag\nThis bit is set when the last data of TX FIFO is transferred to Transmitter Shift Register. If THREIEN (UART_INTEN\\[1\\]) is enabled, the THRE interrupt will be generated.\nNote: This bit is read only and it will be cleared when writing data into UART_DAT (TX FIFO not empty)."]
pub struct THREIF_W<'a> {
    w: &'a mut W,
}
impl<'a> THREIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No THRE interrupt flag is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THREIF_A::_0)
    }
    #[doc = "THRE interrupt flag is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THREIF_A::_1)
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
#[doc = "Receive Line Interrupt Flag (Read Only) \nThis bit is set when the RX receive data have parity error, frame error or break error (at least one of 3 bits, BIF(UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]) and PEF(UART_FIFOSTS\\[4\\]), is set). If RLSIEN (UART_INTEN \\[2\\]) is enabled, the RLS interrupt will be generated.\nNote 2: This bit is read only and reset to 0 when all bits of BIF (UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]) and PEF(UART_FIFOSTS\\[4\\]) are cleared.\nNote 3: In RS-485 function mode, this bit is read only and reset to 0 when all bits of BIF (UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]), PEF(UART_FIFOSTS\\[4\\]) and ADDRDETF (UART_FIFOSTS\\[3\\]) are cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLSIF_A {
    #[doc = "0: No RLS interrupt flag is generated"]
    _0 = 0,
    #[doc = "1: RLS interrupt flag is generated"]
    _1 = 1,
}
impl From<RLSIF_A> for bool {
    #[inline(always)]
    fn from(variant: RLSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLSIF` reader - Receive Line Interrupt Flag (Read Only) \nThis bit is set when the RX receive data have parity error, frame error or break error (at least one of 3 bits, BIF(UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]) and PEF(UART_FIFOSTS\\[4\\]), is set). If RLSIEN (UART_INTEN \\[2\\]) is enabled, the RLS interrupt will be generated.\nNote 2: This bit is read only and reset to 0 when all bits of BIF (UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]) and PEF(UART_FIFOSTS\\[4\\]) are cleared.\nNote 3: In RS-485 function mode, this bit is read only and reset to 0 when all bits of BIF (UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]), PEF(UART_FIFOSTS\\[4\\]) and ADDRDETF (UART_FIFOSTS\\[3\\]) are cleared."]
pub struct RLSIF_R(crate::FieldReader<bool, RLSIF_A>);
impl RLSIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLSIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLSIF_A {
        match self.bits {
            false => RLSIF_A::_0,
            true => RLSIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RLSIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RLSIF_A::_1
    }
}
impl core::ops::Deref for RLSIF_R {
    type Target = crate::FieldReader<bool, RLSIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MODEM Interrupt Flag (Read Only)\nNote: This bit is read only and reset to 0 when bit CTSDETF is cleared by a write 1 on CTSDETF(UART_MODEMSTS\\[0\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEMIF_A {
    #[doc = "0: No Modem interrupt flag is generated"]
    _0 = 0,
    #[doc = "1: Modem interrupt flag is generated"]
    _1 = 1,
}
impl From<MODEMIF_A> for bool {
    #[inline(always)]
    fn from(variant: MODEMIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODEMIF` reader - MODEM Interrupt Flag (Read Only)\nNote: This bit is read only and reset to 0 when bit CTSDETF is cleared by a write 1 on CTSDETF(UART_MODEMSTS\\[0\\])."]
pub struct MODEMIF_R(crate::FieldReader<bool, MODEMIF_A>);
impl MODEMIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODEMIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEMIF_A {
        match self.bits {
            false => MODEMIF_A::_0,
            true => MODEMIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODEMIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODEMIF_A::_1
    }
}
impl core::ops::Deref for MODEMIF_R {
    type Target = crate::FieldReader<bool, MODEMIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RX Time-out Interrupt Flag (Read Only)\nThis bit is set when the RX FIFO is not empty and no activities occurred in the RX FIFO and the time-out counter equal to TOIC (UART_TOUT\\[7:0\\]). If RXTOIEN (UART_INTEN \\[4\\]) is enabled, the RX time-out interrupt will be generated.\nNote: This bit is read only and user can read UART_DAT (RX is in active) to clear it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOIF_A {
    #[doc = "0: No RX time-out interrupt flag is generated"]
    _0 = 0,
    #[doc = "1: RX time-out interrupt flag is generated"]
    _1 = 1,
}
impl From<RXTOIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOIF` reader - RX Time-out Interrupt Flag (Read Only)\nThis bit is set when the RX FIFO is not empty and no activities occurred in the RX FIFO and the time-out counter equal to TOIC (UART_TOUT\\[7:0\\]). If RXTOIEN (UART_INTEN \\[4\\]) is enabled, the RX time-out interrupt will be generated.\nNote: This bit is read only and user can read UART_DAT (RX is in active) to clear it."]
pub struct RXTOIF_R(crate::FieldReader<bool, RXTOIF_A>);
impl RXTOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOIF_A {
        match self.bits {
            false => RXTOIF_A::_0,
            true => RXTOIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXTOIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXTOIF_A::_1
    }
}
impl core::ops::Deref for RXTOIF_R {
    type Target = crate::FieldReader<bool, RXTOIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Buffer Error Interrupt Flag (Read Only)\nThis bit is set when the TX FIFO or RX FIFO overflows (TXOVIF (UART_FIFOSTS\\[24\\]) or RXOVIF (UART_FIFOSTS\\[0\\]) is set). When BUFERRIF (UART_INTSTS\\[5\\]) is set, the transfer is not correct. If BUFERRIEN (UART_INTEN \\[5\\]) is enabled, the buffer error interrupt will be generated.\nNote: This bit is cleared if both of RXOVIF(UART_FIFOSTS\\[0\\]) and TXOVIF(UART_FIFOSTS\\[24\\]) are cleared to 0 by writing 1 to RXOVIF(UART_FIFOSTS\\[0\\]) and TXOVIF(UART_FIFOSTS\\[24\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFERRIF_A {
    #[doc = "0: No buffer error interrupt flag is generated"]
    _0 = 0,
    #[doc = "1: Buffer error interrupt flag is generated"]
    _1 = 1,
}
impl From<BUFERRIF_A> for bool {
    #[inline(always)]
    fn from(variant: BUFERRIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFERRIF` reader - Buffer Error Interrupt Flag (Read Only)\nThis bit is set when the TX FIFO or RX FIFO overflows (TXOVIF (UART_FIFOSTS\\[24\\]) or RXOVIF (UART_FIFOSTS\\[0\\]) is set). When BUFERRIF (UART_INTSTS\\[5\\]) is set, the transfer is not correct. If BUFERRIEN (UART_INTEN \\[5\\]) is enabled, the buffer error interrupt will be generated.\nNote: This bit is cleared if both of RXOVIF(UART_FIFOSTS\\[0\\]) and TXOVIF(UART_FIFOSTS\\[24\\]) are cleared to 0 by writing 1 to RXOVIF(UART_FIFOSTS\\[0\\]) and TXOVIF(UART_FIFOSTS\\[24\\])."]
pub struct BUFERRIF_R(crate::FieldReader<bool, BUFERRIF_A>);
impl BUFERRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFERRIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFERRIF_A {
        match self.bits {
            false => BUFERRIF_A::_0,
            true => BUFERRIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUFERRIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUFERRIF_A::_1
    }
}
impl core::ops::Deref for BUFERRIF_R {
    type Target = crate::FieldReader<bool, BUFERRIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "UART Wake-up Interrupt Flag (Read Only)\nThis bit is set when TOUTWKF (UART_WKSTS\\[4\\]), RS485WKF (UART_WKSTS\\[3\\]), RFRTWKF (UART_WKSTS\\[2\\]), DATWKF (UART_WKSTS\\[1\\]) or CTSWKF(UART_WKSTS\\[0\\]) is set to 1.\nNote: This bit is cleared if all of TOUTWKF, RS485WKF, RFRTWKF, DATWKF and CTSWKF are cleared to 0 by writing 1 to the corresponding interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKIF_A {
    #[doc = "0: No UART wake-up interrupt flag is generated"]
    _0 = 0,
    #[doc = "1: UART wake-up interrupt flag is generated"]
    _1 = 1,
}
impl From<WKIF_A> for bool {
    #[inline(always)]
    fn from(variant: WKIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKIF` reader - UART Wake-up Interrupt Flag (Read Only)\nThis bit is set when TOUTWKF (UART_WKSTS\\[4\\]), RS485WKF (UART_WKSTS\\[3\\]), RFRTWKF (UART_WKSTS\\[2\\]), DATWKF (UART_WKSTS\\[1\\]) or CTSWKF(UART_WKSTS\\[0\\]) is set to 1.\nNote: This bit is cleared if all of TOUTWKF, RS485WKF, RFRTWKF, DATWKF and CTSWKF are cleared to 0 by writing 1 to the corresponding interrupt flag."]
pub struct WKIF_R(crate::FieldReader<bool, WKIF_A>);
impl WKIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKIF_A {
        match self.bits {
            false => WKIF_A::_0,
            true => WKIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKIF_A::_1
    }
}
impl core::ops::Deref for WKIF_R {
    type Target = crate::FieldReader<bool, WKIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Data Available Interrupt Indicator (Read Only)\nThis bit is set if RDAIEN (UART_INTEN\\[0\\]) and RDAIF (UART_INTSTS\\[0\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDAINT_A {
    #[doc = "0: No RDA interrupt is generated"]
    _0 = 0,
    #[doc = "1: RDA interrupt is generated"]
    _1 = 1,
}
impl From<RDAINT_A> for bool {
    #[inline(always)]
    fn from(variant: RDAINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDAINT` reader - Receive Data Available Interrupt Indicator (Read Only)\nThis bit is set if RDAIEN (UART_INTEN\\[0\\]) and RDAIF (UART_INTSTS\\[0\\]) are both set to 1."]
pub struct RDAINT_R(crate::FieldReader<bool, RDAINT_A>);
impl RDAINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDAINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDAINT_A {
        match self.bits {
            false => RDAINT_A::_0,
            true => RDAINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RDAINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RDAINT_A::_1
    }
}
impl core::ops::Deref for RDAINT_R {
    type Target = crate::FieldReader<bool, RDAINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Holding Register Empty Interrupt Indicator (Read Only)\nThis bit is set if THREIEN (UART_INTEN\\[1\\]) and THREIF(UART_INTSTS\\[1\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREINT_A {
    #[doc = "0: No THRE interrupt is generated"]
    _0 = 0,
    #[doc = "1: THRE interrupt is generated"]
    _1 = 1,
}
impl From<THREINT_A> for bool {
    #[inline(always)]
    fn from(variant: THREINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREINT` reader - Transmit Holding Register Empty Interrupt Indicator (Read Only)\nThis bit is set if THREIEN (UART_INTEN\\[1\\]) and THREIF(UART_INTSTS\\[1\\]) are both set to 1."]
pub struct THREINT_R(crate::FieldReader<bool, THREINT_A>);
impl THREINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        THREINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREINT_A {
        match self.bits {
            false => THREINT_A::_0,
            true => THREINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == THREINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == THREINT_A::_1
    }
}
impl core::ops::Deref for THREINT_R {
    type Target = crate::FieldReader<bool, THREINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Line Status Interrupt Indicator (Read Only) \nThis bit is set if RLSIEN (UART_INTEN\\[2\\]) and RLSIF(UART_INTSTS\\[2\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLSINT_A {
    #[doc = "0: No RLS interrupt is generated"]
    _0 = 0,
    #[doc = "1: RLS interrupt is generated"]
    _1 = 1,
}
impl From<RLSINT_A> for bool {
    #[inline(always)]
    fn from(variant: RLSINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLSINT` reader - Receive Line Status Interrupt Indicator (Read Only) \nThis bit is set if RLSIEN (UART_INTEN\\[2\\]) and RLSIF(UART_INTSTS\\[2\\]) are both set to 1."]
pub struct RLSINT_R(crate::FieldReader<bool, RLSINT_A>);
impl RLSINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLSINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLSINT_A {
        match self.bits {
            false => RLSINT_A::_0,
            true => RLSINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RLSINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RLSINT_A::_1
    }
}
impl core::ops::Deref for RLSINT_R {
    type Target = crate::FieldReader<bool, RLSINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MODEM Status Interrupt Indicator (Read Only)\nThis bit is set if MODEMIEN(UART_INTEN\\[3\\]) and MODEMIF(UART_INTSTS\\[3\\]) are both set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEMINT_A {
    #[doc = "0: No Modem interrupt is generated"]
    _0 = 0,
    #[doc = "1: Modem interrupt is generated."]
    _1 = 1,
}
impl From<MODEMINT_A> for bool {
    #[inline(always)]
    fn from(variant: MODEMINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODEMINT` reader - MODEM Status Interrupt Indicator (Read Only)\nThis bit is set if MODEMIEN(UART_INTEN\\[3\\]) and MODEMIF(UART_INTSTS\\[3\\]) are both set to 1"]
pub struct MODEMINT_R(crate::FieldReader<bool, MODEMINT_A>);
impl MODEMINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODEMINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEMINT_A {
        match self.bits {
            false => MODEMINT_A::_0,
            true => MODEMINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODEMINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODEMINT_A::_1
    }
}
impl core::ops::Deref for MODEMINT_R {
    type Target = crate::FieldReader<bool, MODEMINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RX Time-out Interrupt Indicator (Read Only)\nThis bit is set if RXTOIEN (UART_INTEN\\[4\\]) and RXTOIF(UART_INTSTS\\[4\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOINT_A {
    #[doc = "0: No RX time-out interrupt is generated"]
    _0 = 0,
    #[doc = "1: RX time-out interrupt is generated"]
    _1 = 1,
}
impl From<RXTOINT_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOINT` reader - RX Time-out Interrupt Indicator (Read Only)\nThis bit is set if RXTOIEN (UART_INTEN\\[4\\]) and RXTOIF(UART_INTSTS\\[4\\]) are both set to 1."]
pub struct RXTOINT_R(crate::FieldReader<bool, RXTOINT_A>);
impl RXTOINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOINT_A {
        match self.bits {
            false => RXTOINT_A::_0,
            true => RXTOINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXTOINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXTOINT_A::_1
    }
}
impl core::ops::Deref for RXTOINT_R {
    type Target = crate::FieldReader<bool, RXTOINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Buffer Error Interrupt Indicator (Read Only)\nThis bit is set if BUFERRIEN(UART_INTEN\\[5\\]) and BUFERRIF(UART_ INTSTS\\[5\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFERRINT_A {
    #[doc = "0: No buffer error interrupt is generated"]
    _0 = 0,
    #[doc = "1: Buffer error interrupt is generated"]
    _1 = 1,
}
impl From<BUFERRINT_A> for bool {
    #[inline(always)]
    fn from(variant: BUFERRINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFERRINT` reader - Buffer Error Interrupt Indicator (Read Only)\nThis bit is set if BUFERRIEN(UART_INTEN\\[5\\]) and BUFERRIF(UART_ INTSTS\\[5\\]) are both set to 1."]
pub struct BUFERRINT_R(crate::FieldReader<bool, BUFERRINT_A>);
impl BUFERRINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFERRINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFERRINT_A {
        match self.bits {
            false => BUFERRINT_A::_0,
            true => BUFERRINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUFERRINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUFERRINT_A::_1
    }
}
impl core::ops::Deref for BUFERRINT_R {
    type Target = crate::FieldReader<bool, BUFERRINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "UART Wake-up Interrupt Indicator (Read Only)\nThis bit is set if WKIEN (UART_INTEN\\[6\\]) and WKIF (UART_INTSTS\\[6\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKINT_A {
    #[doc = "0: No UART wake-up interrupt is generated"]
    _0 = 0,
    #[doc = "1: UART wake-up interrupt is generated"]
    _1 = 1,
}
impl From<WKINT_A> for bool {
    #[inline(always)]
    fn from(variant: WKINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKINT` reader - UART Wake-up Interrupt Indicator (Read Only)\nThis bit is set if WKIEN (UART_INTEN\\[6\\]) and WKIF (UART_INTSTS\\[6\\]) are both set to 1."]
pub struct WKINT_R(crate::FieldReader<bool, WKINT_A>);
impl WKINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKINT_A {
        match self.bits {
            false => WKINT_A::_0,
            true => WKINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKINT_A::_1
    }
}
impl core::ops::Deref for WKINT_R {
    type Target = crate::FieldReader<bool, WKINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Single-wire Bit Error Detection Interrupt Flag\nThis bit is set when the single wire bus state not equals to UART controller TX state in Single-wire mode.\nNote 1: This bit is active when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode.\nNote 2: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWBEIF_A {
    #[doc = "0: No single-wire bit error detection interrupt flag is generated"]
    _0 = 0,
    #[doc = "1: Single-wire bit error detection interrupt flag is generated"]
    _1 = 1,
}
impl From<SWBEIF_A> for bool {
    #[inline(always)]
    fn from(variant: SWBEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWBEIF` reader - Single-wire Bit Error Detection Interrupt Flag\nThis bit is set when the single wire bus state not equals to UART controller TX state in Single-wire mode.\nNote 1: This bit is active when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode.\nNote 2: This bit can be cleared by writing '1' to it."]
pub struct SWBEIF_R(crate::FieldReader<bool, SWBEIF_A>);
impl SWBEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWBEIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWBEIF_A {
        match self.bits {
            false => SWBEIF_A::_0,
            true => SWBEIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWBEIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWBEIF_A::_1
    }
}
impl core::ops::Deref for SWBEIF_R {
    type Target = crate::FieldReader<bool, SWBEIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWBEIF` writer - Single-wire Bit Error Detection Interrupt Flag\nThis bit is set when the single wire bus state not equals to UART controller TX state in Single-wire mode.\nNote 1: This bit is active when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode.\nNote 2: This bit can be cleared by writing '1' to it."]
pub struct SWBEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SWBEIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWBEIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No single-wire bit error detection interrupt flag is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWBEIF_A::_0)
    }
    #[doc = "Single-wire bit error detection interrupt flag is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWBEIF_A::_1)
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
#[doc = "PDMA Mode Receive Line Status Flag (Read Only)\nThis bit is set when the RX receive data have parity error, frame error or break error (at least one of 3 bits, BIF (UART_FIFOSTS\\[6\\]), FEF (UART_FIFOSTS\\[5\\]) and PEF (UART_FIFOSTS\\[4\\]) is set). If RLSIEN (UART_INTEN \\[2\\]) is enabled, the RLS interrupt will be generated.\nNote 2: In UART function mode, this bit is read only and reset to 0 when all bits of BIF(UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]) and PEF(UART_FIFOSTS\\[4\\]) are cleared. \nNote 3: In RS-485 function mode, this bit is read only and reset to 0 when all bits of BIF(UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]), PEF(UART_FIFOSTS\\[4\\]) and ADDRDETF (UART_FIFOSTS\\[3\\]) are cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWRLSIF_A {
    #[doc = "0: No RLS interrupt flag is generated in PDMA mode"]
    _0 = 0,
    #[doc = "1: RLS interrupt flag is generated in PDMA mode"]
    _1 = 1,
}
impl From<HWRLSIF_A> for bool {
    #[inline(always)]
    fn from(variant: HWRLSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWRLSIF` reader - PDMA Mode Receive Line Status Flag (Read Only)\nThis bit is set when the RX receive data have parity error, frame error or break error (at least one of 3 bits, BIF (UART_FIFOSTS\\[6\\]), FEF (UART_FIFOSTS\\[5\\]) and PEF (UART_FIFOSTS\\[4\\]) is set). If RLSIEN (UART_INTEN \\[2\\]) is enabled, the RLS interrupt will be generated.\nNote 2: In UART function mode, this bit is read only and reset to 0 when all bits of BIF(UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]) and PEF(UART_FIFOSTS\\[4\\]) are cleared. \nNote 3: In RS-485 function mode, this bit is read only and reset to 0 when all bits of BIF(UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]), PEF(UART_FIFOSTS\\[4\\]) and ADDRDETF (UART_FIFOSTS\\[3\\]) are cleared."]
pub struct HWRLSIF_R(crate::FieldReader<bool, HWRLSIF_A>);
impl HWRLSIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWRLSIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWRLSIF_A {
        match self.bits {
            false => HWRLSIF_A::_0,
            true => HWRLSIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HWRLSIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HWRLSIF_A::_1
    }
}
impl core::ops::Deref for HWRLSIF_R {
    type Target = crate::FieldReader<bool, HWRLSIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Mode MODEM Interrupt Flag (Read Only)\nNote: This bit is read only and reset to 0 when the bit CTSDETF (UART_MODEMSTS\\[0\\]) is cleared by writing 1 on CTSDETF (UART_MODEMSTS \\[0\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWMODIF_A {
    #[doc = "0: No Modem interrupt flag is generated in PDMA mode"]
    _0 = 0,
    #[doc = "1: Modem interrupt flag is generated in PDMA mode"]
    _1 = 1,
}
impl From<HWMODIF_A> for bool {
    #[inline(always)]
    fn from(variant: HWMODIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWMODIF` reader - PDMA Mode MODEM Interrupt Flag (Read Only)\nNote: This bit is read only and reset to 0 when the bit CTSDETF (UART_MODEMSTS\\[0\\]) is cleared by writing 1 on CTSDETF (UART_MODEMSTS \\[0\\])."]
pub struct HWMODIF_R(crate::FieldReader<bool, HWMODIF_A>);
impl HWMODIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWMODIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWMODIF_A {
        match self.bits {
            false => HWMODIF_A::_0,
            true => HWMODIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HWMODIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HWMODIF_A::_1
    }
}
impl core::ops::Deref for HWMODIF_R {
    type Target = crate::FieldReader<bool, HWMODIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Mode RX Time-out Interrupt Flag (Read Only)\nThis bit is set when the RX FIFO is not empty and no activities occurred in the RX FIFO and the time-out counter equal to TOIC (UART_TOUT\\[7:0\\]). If RXTOIEN (UART_INTEN \\[4\\]) is enabled, the RX time-out interrupt will be generated . \nNote: This bit is read only and user can read UART_DAT (RX is in active) to clear it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWTOIF_A {
    #[doc = "0: No RX time-out interrupt flag is generated in PDMA mode"]
    _0 = 0,
    #[doc = "1: RX time-out interrupt flag is generated in PDMA mode"]
    _1 = 1,
}
impl From<HWTOIF_A> for bool {
    #[inline(always)]
    fn from(variant: HWTOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWTOIF` reader - PDMA Mode RX Time-out Interrupt Flag (Read Only)\nThis bit is set when the RX FIFO is not empty and no activities occurred in the RX FIFO and the time-out counter equal to TOIC (UART_TOUT\\[7:0\\]). If RXTOIEN (UART_INTEN \\[4\\]) is enabled, the RX time-out interrupt will be generated . \nNote: This bit is read only and user can read UART_DAT (RX is in active) to clear it."]
pub struct HWTOIF_R(crate::FieldReader<bool, HWTOIF_A>);
impl HWTOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWTOIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWTOIF_A {
        match self.bits {
            false => HWTOIF_A::_0,
            true => HWTOIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HWTOIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HWTOIF_A::_1
    }
}
impl core::ops::Deref for HWTOIF_R {
    type Target = crate::FieldReader<bool, HWTOIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Mode Buffer Error Interrupt Flag (Read Only)\nThis bit is set when the TX or RX FIFO overflows (TXOVIF (UART_FIFOSTS \\[24\\]) or RXOVIF (UART_FIFOSTS\\[0\\]) is set). When BUFERRIF (UART_INTSTS\\[5\\]) is set, the transfer maybe is not correct. If BUFERRIEN (UART_INTEN \\[5\\]) is enabled, the buffer error interrupt will be generated.\nNote: This bit is cleared when both TXOVIF (UART_FIFOSTS\\[24\\]\\]) and RXOVIF (UART_FIFOSTS\\[0\\]) are cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWBUFEIF_A {
    #[doc = "0: No buffer error interrupt flag is generated in PDMA mode"]
    _0 = 0,
    #[doc = "1: Buffer error interrupt flag is generated in PDMA mode"]
    _1 = 1,
}
impl From<HWBUFEIF_A> for bool {
    #[inline(always)]
    fn from(variant: HWBUFEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWBUFEIF` reader - PDMA Mode Buffer Error Interrupt Flag (Read Only)\nThis bit is set when the TX or RX FIFO overflows (TXOVIF (UART_FIFOSTS \\[24\\]) or RXOVIF (UART_FIFOSTS\\[0\\]) is set). When BUFERRIF (UART_INTSTS\\[5\\]) is set, the transfer maybe is not correct. If BUFERRIEN (UART_INTEN \\[5\\]) is enabled, the buffer error interrupt will be generated.\nNote: This bit is cleared when both TXOVIF (UART_FIFOSTS\\[24\\]\\]) and RXOVIF (UART_FIFOSTS\\[0\\]) are cleared."]
pub struct HWBUFEIF_R(crate::FieldReader<bool, HWBUFEIF_A>);
impl HWBUFEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWBUFEIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWBUFEIF_A {
        match self.bits {
            false => HWBUFEIF_A::_0,
            true => HWBUFEIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HWBUFEIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HWBUFEIF_A::_1
    }
}
impl core::ops::Deref for HWBUFEIF_R {
    type Target = crate::FieldReader<bool, HWBUFEIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmitter Empty Interrupt Flag\nThis bit is set when TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted (TXEMPTYF (UART_FIFOSTS\\[28\\]) is set). If TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt will be generated.\nNote: This bit is cleared automatically when TX FIFO is not empty or the last byte transmission has not completed.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDIF_A {
    #[doc = "0: No transmitter empty interrupt flag is generated"]
    _0 = 0,
    #[doc = "1: Transmitter empty interrupt flag is generated"]
    _1 = 1,
}
impl From<TXENDIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXENDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXENDIF` reader - Transmitter Empty Interrupt Flag\nThis bit is set when TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted (TXEMPTYF (UART_FIFOSTS\\[28\\]) is set). If TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt will be generated.\nNote: This bit is cleared automatically when TX FIFO is not empty or the last byte transmission has not completed."]
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
#[doc = "Field `TXENDIF` writer - Transmitter Empty Interrupt Flag\nThis bit is set when TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted (TXEMPTYF (UART_FIFOSTS\\[28\\]) is set). If TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt will be generated.\nNote: This bit is cleared automatically when TX FIFO is not empty or the last byte transmission has not completed."]
pub struct TXENDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXENDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No transmitter empty interrupt flag is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXENDIF_A::_0)
    }
    #[doc = "Transmitter empty interrupt flag is generated"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Single-wire Bit Error Detect Interrupt Indicator (Read Only)\nThis bit is set if SWBEIEN (UART_INTEN\\[16\\]) and SWBEIF (UART_INTSTS\\[16\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWBEINT_A {
    #[doc = "0: No Single-wire Bit Error Detection Interrupt generated"]
    _0 = 0,
    #[doc = "1: Single-wire Bit Error Detection Interrupt generated"]
    _1 = 1,
}
impl From<SWBEINT_A> for bool {
    #[inline(always)]
    fn from(variant: SWBEINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWBEINT` reader - Single-wire Bit Error Detect Interrupt Indicator (Read Only)\nThis bit is set if SWBEIEN (UART_INTEN\\[16\\]) and SWBEIF (UART_INTSTS\\[16\\]) are both set to 1."]
pub struct SWBEINT_R(crate::FieldReader<bool, SWBEINT_A>);
impl SWBEINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWBEINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWBEINT_A {
        match self.bits {
            false => SWBEINT_A::_0,
            true => SWBEINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWBEINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWBEINT_A::_1
    }
}
impl core::ops::Deref for SWBEINT_R {
    type Target = crate::FieldReader<bool, SWBEINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Mode Receive Line Status Interrupt Indicator (Read Only)\nThis bit is set if RLSIEN (UART_INTEN\\[2\\]) and HWRLSIF(UART_INTSTS\\[18\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWRLSINT_A {
    #[doc = "0: No RLS interrupt is generated in PDMA mode"]
    _0 = 0,
    #[doc = "1: RLS interrupt is generated in PDMA mode"]
    _1 = 1,
}
impl From<HWRLSINT_A> for bool {
    #[inline(always)]
    fn from(variant: HWRLSINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWRLSINT` reader - PDMA Mode Receive Line Status Interrupt Indicator (Read Only)\nThis bit is set if RLSIEN (UART_INTEN\\[2\\]) and HWRLSIF(UART_INTSTS\\[18\\]) are both set to 1."]
pub struct HWRLSINT_R(crate::FieldReader<bool, HWRLSINT_A>);
impl HWRLSINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWRLSINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWRLSINT_A {
        match self.bits {
            false => HWRLSINT_A::_0,
            true => HWRLSINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HWRLSINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HWRLSINT_A::_1
    }
}
impl core::ops::Deref for HWRLSINT_R {
    type Target = crate::FieldReader<bool, HWRLSINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Mode MODEM Status Interrupt Indicator (Read Only)\nThis bit is set if MODEMIEN (UART_INTEN\\[3\\]) and HWMODIF(UART_INTSTS\\[19\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWMODINT_A {
    #[doc = "0: No Modem interrupt is generated in PDMA mode"]
    _0 = 0,
    #[doc = "1: Modem interrupt is generated in PDMA mode"]
    _1 = 1,
}
impl From<HWMODINT_A> for bool {
    #[inline(always)]
    fn from(variant: HWMODINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWMODINT` reader - PDMA Mode MODEM Status Interrupt Indicator (Read Only)\nThis bit is set if MODEMIEN (UART_INTEN\\[3\\]) and HWMODIF(UART_INTSTS\\[19\\]) are both set to 1."]
pub struct HWMODINT_R(crate::FieldReader<bool, HWMODINT_A>);
impl HWMODINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWMODINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWMODINT_A {
        match self.bits {
            false => HWMODINT_A::_0,
            true => HWMODINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HWMODINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HWMODINT_A::_1
    }
}
impl core::ops::Deref for HWMODINT_R {
    type Target = crate::FieldReader<bool, HWMODINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Mode RX Time-out Interrupt Indicator (Read Only)\nThis bit is set if RXTOIEN (UART_INTEN\\[4\\]) and HWTOIF(UART_INTSTS\\[20\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWTOINT_A {
    #[doc = "0: No RX time-out interrupt is generated in PDMA mode"]
    _0 = 0,
    #[doc = "1: RX time-out interrupt is generated in PDMA mode"]
    _1 = 1,
}
impl From<HWTOINT_A> for bool {
    #[inline(always)]
    fn from(variant: HWTOINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWTOINT` reader - PDMA Mode RX Time-out Interrupt Indicator (Read Only)\nThis bit is set if RXTOIEN (UART_INTEN\\[4\\]) and HWTOIF(UART_INTSTS\\[20\\]) are both set to 1."]
pub struct HWTOINT_R(crate::FieldReader<bool, HWTOINT_A>);
impl HWTOINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWTOINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWTOINT_A {
        match self.bits {
            false => HWTOINT_A::_0,
            true => HWTOINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HWTOINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HWTOINT_A::_1
    }
}
impl core::ops::Deref for HWTOINT_R {
    type Target = crate::FieldReader<bool, HWTOINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA Mode Buffer Error Interrupt Indicator (Read Only)\nThis bit is set if BUFERRIEN (UART_INTEN\\[5\\]) and HWBUFEIF (UART_INTSTS\\[21\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWBUFEINT_A {
    #[doc = "0: No buffer error interrupt is generated in PDMA mode"]
    _0 = 0,
    #[doc = "1: Buffer error interrupt is generated in PDMA mode"]
    _1 = 1,
}
impl From<HWBUFEINT_A> for bool {
    #[inline(always)]
    fn from(variant: HWBUFEINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWBUFEINT` reader - PDMA Mode Buffer Error Interrupt Indicator (Read Only)\nThis bit is set if BUFERRIEN (UART_INTEN\\[5\\]) and HWBUFEIF (UART_INTSTS\\[21\\]) are both set to 1."]
pub struct HWBUFEINT_R(crate::FieldReader<bool, HWBUFEINT_A>);
impl HWBUFEINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWBUFEINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWBUFEINT_A {
        match self.bits {
            false => HWBUFEINT_A::_0,
            true => HWBUFEINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HWBUFEINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HWBUFEINT_A::_1
    }
}
impl core::ops::Deref for HWBUFEINT_R {
    type Target = crate::FieldReader<bool, HWBUFEINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmitter Empty Interrupt Indicator (Read Only) \nThis bit is set if TXENDIEN (UART_INTEN\\[22\\]) and TXENDIF(UART_INTSTS\\[22\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDINT_A {
    #[doc = "0: No Transmitter Empty interrupt is generated"]
    _0 = 0,
    #[doc = "1: Transmitter Empty interrupt is generated"]
    _1 = 1,
}
impl From<TXENDINT_A> for bool {
    #[inline(always)]
    fn from(variant: TXENDINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXENDINT` reader - Transmitter Empty Interrupt Indicator (Read Only) \nThis bit is set if TXENDIEN (UART_INTEN\\[22\\]) and TXENDIF(UART_INTSTS\\[22\\]) are both set to 1."]
pub struct TXENDINT_R(crate::FieldReader<bool, TXENDINT_A>);
impl TXENDINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXENDINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENDINT_A {
        match self.bits {
            false => TXENDINT_A::_0,
            true => TXENDINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXENDINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXENDINT_A::_1
    }
}
impl core::ops::Deref for TXENDINT_R {
    type Target = crate::FieldReader<bool, TXENDINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Auto-baud Rate Interrupt Indicator (Read Only)\nThis bit is set if ABRIEN (UART_INTEN\\[18\\]) and ABRIF (UART_ALTCTL\\[17\\]) are both set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRINT_A {
    #[doc = "0: No Auto-baud Rate interrupt is generated"]
    _0 = 0,
    #[doc = "1: The Auto-baud Rate interrupt is generated"]
    _1 = 1,
}
impl From<ABRINT_A> for bool {
    #[inline(always)]
    fn from(variant: ABRINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRINT` reader - Auto-baud Rate Interrupt Indicator (Read Only)\nThis bit is set if ABRIEN (UART_INTEN\\[18\\]) and ABRIF (UART_ALTCTL\\[17\\]) are both set to 1."]
pub struct ABRINT_R(crate::FieldReader<bool, ABRINT_A>);
impl ABRINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRINT_A {
        match self.bits {
            false => ABRINT_A::_0,
            true => ABRINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABRINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABRINT_A::_1
    }
}
impl core::ops::Deref for ABRINT_R {
    type Target = crate::FieldReader<bool, ABRINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Available Interrupt Flag When the number of bytes in the RX FIFO equals the RFITL then the RDAIF(UART_INTSTS\\[0\\]) will be set. If RDAIEN (UART_INTEN \\[0\\]) is enabled, the RDA interrupt will be generated. Note: This bit is read only and it will be cleared when the number of unread bytes of RX FIFO drops below the threshold level (RFITL(UART_FIFO\\[7:4\\])."]
    #[inline(always)]
    pub fn rdaif(&self) -> RDAIF_R {
        RDAIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Holding Register Empty Interrupt Flag This bit is set when the last data of TX FIFO is transferred to Transmitter Shift Register. If THREIEN (UART_INTEN\\[1\\]) is enabled, the THRE interrupt will be generated. Note: This bit is read only and it will be cleared when writing data into UART_DAT (TX FIFO not empty)."]
    #[inline(always)]
    pub fn threif(&self) -> THREIF_R {
        THREIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Line Interrupt Flag (Read Only) This bit is set when the RX receive data have parity error, frame error or break error (at least one of 3 bits, BIF(UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]) and PEF(UART_FIFOSTS\\[4\\]), is set). If RLSIEN (UART_INTEN \\[2\\]) is enabled, the RLS interrupt will be generated. Note 2: This bit is read only and reset to 0 when all bits of BIF (UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]) and PEF(UART_FIFOSTS\\[4\\]) are cleared. Note 3: In RS-485 function mode, this bit is read only and reset to 0 when all bits of BIF (UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]), PEF(UART_FIFOSTS\\[4\\]) and ADDRDETF (UART_FIFOSTS\\[3\\]) are cleared."]
    #[inline(always)]
    pub fn rlsif(&self) -> RLSIF_R {
        RLSIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MODEM Interrupt Flag (Read Only) Note: This bit is read only and reset to 0 when bit CTSDETF is cleared by a write 1 on CTSDETF(UART_MODEMSTS\\[0\\])."]
    #[inline(always)]
    pub fn modemif(&self) -> MODEMIF_R {
        MODEMIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RX Time-out Interrupt Flag (Read Only) This bit is set when the RX FIFO is not empty and no activities occurred in the RX FIFO and the time-out counter equal to TOIC (UART_TOUT\\[7:0\\]). If RXTOIEN (UART_INTEN \\[4\\]) is enabled, the RX time-out interrupt will be generated. Note: This bit is read only and user can read UART_DAT (RX is in active) to clear it."]
    #[inline(always)]
    pub fn rxtoif(&self) -> RXTOIF_R {
        RXTOIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Error Interrupt Flag (Read Only) This bit is set when the TX FIFO or RX FIFO overflows (TXOVIF (UART_FIFOSTS\\[24\\]) or RXOVIF (UART_FIFOSTS\\[0\\]) is set). When BUFERRIF (UART_INTSTS\\[5\\]) is set, the transfer is not correct. If BUFERRIEN (UART_INTEN \\[5\\]) is enabled, the buffer error interrupt will be generated. Note: This bit is cleared if both of RXOVIF(UART_FIFOSTS\\[0\\]) and TXOVIF(UART_FIFOSTS\\[24\\]) are cleared to 0 by writing 1 to RXOVIF(UART_FIFOSTS\\[0\\]) and TXOVIF(UART_FIFOSTS\\[24\\])."]
    #[inline(always)]
    pub fn buferrif(&self) -> BUFERRIF_R {
        BUFERRIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Wake-up Interrupt Flag (Read Only) This bit is set when TOUTWKF (UART_WKSTS\\[4\\]), RS485WKF (UART_WKSTS\\[3\\]), RFRTWKF (UART_WKSTS\\[2\\]), DATWKF (UART_WKSTS\\[1\\]) or CTSWKF(UART_WKSTS\\[0\\]) is set to 1. Note: This bit is cleared if all of TOUTWKF, RS485WKF, RFRTWKF, DATWKF and CTSWKF are cleared to 0 by writing 1 to the corresponding interrupt flag."]
    #[inline(always)]
    pub fn wkif(&self) -> WKIF_R {
        WKIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Data Available Interrupt Indicator (Read Only) This bit is set if RDAIEN (UART_INTEN\\[0\\]) and RDAIF (UART_INTSTS\\[0\\]) are both set to 1."]
    #[inline(always)]
    pub fn rdaint(&self) -> RDAINT_R {
        RDAINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Holding Register Empty Interrupt Indicator (Read Only) This bit is set if THREIEN (UART_INTEN\\[1\\]) and THREIF(UART_INTSTS\\[1\\]) are both set to 1."]
    #[inline(always)]
    pub fn threint(&self) -> THREINT_R {
        THREINT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Line Status Interrupt Indicator (Read Only) This bit is set if RLSIEN (UART_INTEN\\[2\\]) and RLSIF(UART_INTSTS\\[2\\]) are both set to 1."]
    #[inline(always)]
    pub fn rlsint(&self) -> RLSINT_R {
        RLSINT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MODEM Status Interrupt Indicator (Read Only) This bit is set if MODEMIEN(UART_INTEN\\[3\\]) and MODEMIF(UART_INTSTS\\[3\\]) are both set to 1"]
    #[inline(always)]
    pub fn modemint(&self) -> MODEMINT_R {
        MODEMINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RX Time-out Interrupt Indicator (Read Only) This bit is set if RXTOIEN (UART_INTEN\\[4\\]) and RXTOIF(UART_INTSTS\\[4\\]) are both set to 1."]
    #[inline(always)]
    pub fn rxtoint(&self) -> RXTOINT_R {
        RXTOINT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Buffer Error Interrupt Indicator (Read Only) This bit is set if BUFERRIEN(UART_INTEN\\[5\\]) and BUFERRIF(UART_ INTSTS\\[5\\]) are both set to 1."]
    #[inline(always)]
    pub fn buferrint(&self) -> BUFERRINT_R {
        BUFERRINT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - UART Wake-up Interrupt Indicator (Read Only) This bit is set if WKIEN (UART_INTEN\\[6\\]) and WKIF (UART_INTSTS\\[6\\]) are both set to 1."]
    #[inline(always)]
    pub fn wkint(&self) -> WKINT_R {
        WKINT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Single-wire Bit Error Detection Interrupt Flag This bit is set when the single wire bus state not equals to UART controller TX state in Single-wire mode. Note 1: This bit is active when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode. Note 2: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn swbeif(&self) -> SWBEIF_R {
        SWBEIF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PDMA Mode Receive Line Status Flag (Read Only) This bit is set when the RX receive data have parity error, frame error or break error (at least one of 3 bits, BIF (UART_FIFOSTS\\[6\\]), FEF (UART_FIFOSTS\\[5\\]) and PEF (UART_FIFOSTS\\[4\\]) is set). If RLSIEN (UART_INTEN \\[2\\]) is enabled, the RLS interrupt will be generated. Note 2: In UART function mode, this bit is read only and reset to 0 when all bits of BIF(UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]) and PEF(UART_FIFOSTS\\[4\\]) are cleared. Note 3: In RS-485 function mode, this bit is read only and reset to 0 when all bits of BIF(UART_FIFOSTS\\[6\\]), FEF(UART_FIFOSTS\\[5\\]), PEF(UART_FIFOSTS\\[4\\]) and ADDRDETF (UART_FIFOSTS\\[3\\]) are cleared."]
    #[inline(always)]
    pub fn hwrlsif(&self) -> HWRLSIF_R {
        HWRLSIF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PDMA Mode MODEM Interrupt Flag (Read Only) Note: This bit is read only and reset to 0 when the bit CTSDETF (UART_MODEMSTS\\[0\\]) is cleared by writing 1 on CTSDETF (UART_MODEMSTS \\[0\\])."]
    #[inline(always)]
    pub fn hwmodif(&self) -> HWMODIF_R {
        HWMODIF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PDMA Mode RX Time-out Interrupt Flag (Read Only) This bit is set when the RX FIFO is not empty and no activities occurred in the RX FIFO and the time-out counter equal to TOIC (UART_TOUT\\[7:0\\]). If RXTOIEN (UART_INTEN \\[4\\]) is enabled, the RX time-out interrupt will be generated . Note: This bit is read only and user can read UART_DAT (RX is in active) to clear it."]
    #[inline(always)]
    pub fn hwtoif(&self) -> HWTOIF_R {
        HWTOIF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PDMA Mode Buffer Error Interrupt Flag (Read Only) This bit is set when the TX or RX FIFO overflows (TXOVIF (UART_FIFOSTS \\[24\\]) or RXOVIF (UART_FIFOSTS\\[0\\]) is set). When BUFERRIF (UART_INTSTS\\[5\\]) is set, the transfer maybe is not correct. If BUFERRIEN (UART_INTEN \\[5\\]) is enabled, the buffer error interrupt will be generated. Note: This bit is cleared when both TXOVIF (UART_FIFOSTS\\[24\\]\\]) and RXOVIF (UART_FIFOSTS\\[0\\]) are cleared."]
    #[inline(always)]
    pub fn hwbufeif(&self) -> HWBUFEIF_R {
        HWBUFEIF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Transmitter Empty Interrupt Flag This bit is set when TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted (TXEMPTYF (UART_FIFOSTS\\[28\\]) is set). If TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt will be generated. Note: This bit is cleared automatically when TX FIFO is not empty or the last byte transmission has not completed."]
    #[inline(always)]
    pub fn txendif(&self) -> TXENDIF_R {
        TXENDIF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Single-wire Bit Error Detect Interrupt Indicator (Read Only) This bit is set if SWBEIEN (UART_INTEN\\[16\\]) and SWBEIF (UART_INTSTS\\[16\\]) are both set to 1."]
    #[inline(always)]
    pub fn swbeint(&self) -> SWBEINT_R {
        SWBEINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PDMA Mode Receive Line Status Interrupt Indicator (Read Only) This bit is set if RLSIEN (UART_INTEN\\[2\\]) and HWRLSIF(UART_INTSTS\\[18\\]) are both set to 1."]
    #[inline(always)]
    pub fn hwrlsint(&self) -> HWRLSINT_R {
        HWRLSINT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PDMA Mode MODEM Status Interrupt Indicator (Read Only) This bit is set if MODEMIEN (UART_INTEN\\[3\\]) and HWMODIF(UART_INTSTS\\[19\\]) are both set to 1."]
    #[inline(always)]
    pub fn hwmodint(&self) -> HWMODINT_R {
        HWMODINT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PDMA Mode RX Time-out Interrupt Indicator (Read Only) This bit is set if RXTOIEN (UART_INTEN\\[4\\]) and HWTOIF(UART_INTSTS\\[20\\]) are both set to 1."]
    #[inline(always)]
    pub fn hwtoint(&self) -> HWTOINT_R {
        HWTOINT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - PDMA Mode Buffer Error Interrupt Indicator (Read Only) This bit is set if BUFERRIEN (UART_INTEN\\[5\\]) and HWBUFEIF (UART_INTSTS\\[21\\]) are both set to 1."]
    #[inline(always)]
    pub fn hwbufeint(&self) -> HWBUFEINT_R {
        HWBUFEINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmitter Empty Interrupt Indicator (Read Only) This bit is set if TXENDIEN (UART_INTEN\\[22\\]) and TXENDIF(UART_INTSTS\\[22\\]) are both set to 1."]
    #[inline(always)]
    pub fn txendint(&self) -> TXENDINT_R {
        TXENDINT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Auto-baud Rate Interrupt Indicator (Read Only) This bit is set if ABRIEN (UART_INTEN\\[18\\]) and ABRIF (UART_ALTCTL\\[17\\]) are both set to 1."]
    #[inline(always)]
    pub fn abrint(&self) -> ABRINT_R {
        ABRINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Available Interrupt Flag When the number of bytes in the RX FIFO equals the RFITL then the RDAIF(UART_INTSTS\\[0\\]) will be set. If RDAIEN (UART_INTEN \\[0\\]) is enabled, the RDA interrupt will be generated. Note: This bit is read only and it will be cleared when the number of unread bytes of RX FIFO drops below the threshold level (RFITL(UART_FIFO\\[7:4\\])."]
    #[inline(always)]
    pub fn rdaif(&mut self) -> RDAIF_W {
        RDAIF_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Holding Register Empty Interrupt Flag This bit is set when the last data of TX FIFO is transferred to Transmitter Shift Register. If THREIEN (UART_INTEN\\[1\\]) is enabled, the THRE interrupt will be generated. Note: This bit is read only and it will be cleared when writing data into UART_DAT (TX FIFO not empty)."]
    #[inline(always)]
    pub fn threif(&mut self) -> THREIF_W {
        THREIF_W { w: self }
    }
    #[doc = "Bit 16 - Single-wire Bit Error Detection Interrupt Flag This bit is set when the single wire bus state not equals to UART controller TX state in Single-wire mode. Note 1: This bit is active when FUNCSEL (UART_FUNCSEL\\[2:0\\]) is select UART Single-wire mode. Note 2: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn swbeif(&mut self) -> SWBEIF_W {
        SWBEIF_W { w: self }
    }
    #[doc = "Bit 22 - Transmitter Empty Interrupt Flag This bit is set when TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted (TXEMPTYF (UART_FIFOSTS\\[28\\]) is set). If TXENDIEN (UART_INTEN\\[22\\]) is enabled, the Transmitter Empty interrupt will be generated. Note: This bit is cleared automatically when TX FIFO is not empty or the last byte transmission has not completed."]
    #[inline(always)]
    pub fn txendif(&mut self) -> TXENDIF_W {
        TXENDIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_intsts](index.html) module"]
pub struct UART_INTSTS_SPEC;
impl crate::RegisterSpec for UART_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_intsts::R](R) reader structure"]
impl crate::Readable for UART_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_intsts::W](W) writer structure"]
impl crate::Writable for UART_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_INTSTS to value 0x0040_0002"]
impl crate::Resettable for UART_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0002
    }
}
