#[doc = "Register `UART_FIFOSTS` reader"]
pub struct R(crate::R<UART_FIFOSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FIFOSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_FIFOSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_FIFOSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_FIFOSTS` writer"]
pub struct W(crate::W<UART_FIFOSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FIFOSTS_SPEC>;
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
impl From<crate::W<UART_FIFOSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_FIFOSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RX Overflow Error Interrupt Flag\nThis bit is set when RX FIFO overflow.\nIf the number of bytes of received data is greater than RX_FIFO (UART_DAT) size 16 bytes, this bit will be set.\nNote: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVIF_A {
    #[doc = "0: RX FIFO is not overflow"]
    _0 = 0,
    #[doc = "1: RX FIFO is overflow"]
    _1 = 1,
}
impl From<RXOVIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVIF` reader - RX Overflow Error Interrupt Flag\nThis bit is set when RX FIFO overflow.\nIf the number of bytes of received data is greater than RX_FIFO (UART_DAT) size 16 bytes, this bit will be set.\nNote: This bit can be cleared by writing '1' to it."]
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
#[doc = "Field `RXOVIF` writer - RX Overflow Error Interrupt Flag\nThis bit is set when RX FIFO overflow.\nIf the number of bytes of received data is greater than RX_FIFO (UART_DAT) size 16 bytes, this bit will be set.\nNote: This bit can be cleared by writing '1' to it."]
pub struct RXOVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOVIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RX FIFO is not overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOVIF_A::_0)
    }
    #[doc = "RX FIFO is overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOVIF_A::_1)
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
#[doc = "Auto-baud Rate Detect Interrupt Flag\nThis bit is set to logic '1' when auto-baud rate detect function is finished.\nNote: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRDIF_A {
    #[doc = "0: Auto-baud rate detect function is not finished"]
    _0 = 0,
    #[doc = "1: Auto-baud rate detect function is finished"]
    _1 = 1,
}
impl From<ABRDIF_A> for bool {
    #[inline(always)]
    fn from(variant: ABRDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRDIF` reader - Auto-baud Rate Detect Interrupt Flag\nThis bit is set to logic '1' when auto-baud rate detect function is finished.\nNote: This bit can be cleared by writing '1' to it."]
pub struct ABRDIF_R(crate::FieldReader<bool, ABRDIF_A>);
impl ABRDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRDIF_A {
        match self.bits {
            false => ABRDIF_A::_0,
            true => ABRDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABRDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABRDIF_A::_1
    }
}
impl core::ops::Deref for ABRDIF_R {
    type Target = crate::FieldReader<bool, ABRDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRDIF` writer - Auto-baud Rate Detect Interrupt Flag\nThis bit is set to logic '1' when auto-baud rate detect function is finished.\nNote: This bit can be cleared by writing '1' to it."]
pub struct ABRDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABRDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-baud rate detect function is not finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABRDIF_A::_0)
    }
    #[doc = "Auto-baud rate detect function is finished"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABRDIF_A::_1)
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
#[doc = "Auto-baud Rate Detect Time-out Interrupt Flag\nThis bit is set to logic '1' in Auto-baud Rate Detect mode when the baud rate counter is overflow.\nNote: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRDTOIF_A {
    #[doc = "0: Auto-baud rate counter is underflow"]
    _0 = 0,
    #[doc = "1: Auto-baud rate counter is overflow"]
    _1 = 1,
}
impl From<ABRDTOIF_A> for bool {
    #[inline(always)]
    fn from(variant: ABRDTOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRDTOIF` reader - Auto-baud Rate Detect Time-out Interrupt Flag\nThis bit is set to logic '1' in Auto-baud Rate Detect mode when the baud rate counter is overflow.\nNote: This bit can be cleared by writing '1' to it."]
pub struct ABRDTOIF_R(crate::FieldReader<bool, ABRDTOIF_A>);
impl ABRDTOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRDTOIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRDTOIF_A {
        match self.bits {
            false => ABRDTOIF_A::_0,
            true => ABRDTOIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABRDTOIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABRDTOIF_A::_1
    }
}
impl core::ops::Deref for ABRDTOIF_R {
    type Target = crate::FieldReader<bool, ABRDTOIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRDTOIF` writer - Auto-baud Rate Detect Time-out Interrupt Flag\nThis bit is set to logic '1' in Auto-baud Rate Detect mode when the baud rate counter is overflow.\nNote: This bit can be cleared by writing '1' to it."]
pub struct ABRDTOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRDTOIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABRDTOIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-baud rate counter is underflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABRDTOIF_A::_0)
    }
    #[doc = "Auto-baud rate counter is overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABRDTOIF_A::_1)
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
#[doc = "RS-485 Address Byte Detect Flag\nNote 1: This field is used for RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1 to enable Address detection mode.\nNote 2: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRDETF_A {
    #[doc = "0: Receiver detects a data that is not an address bit (bit 9 ='0')"]
    _0 = 0,
    #[doc = "1: Receiver detects a data that is an address bit (bit 9 ='1')"]
    _1 = 1,
}
impl From<ADDRDETF_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRDETF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRDETF` reader - RS-485 Address Byte Detect Flag\nNote 1: This field is used for RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1 to enable Address detection mode.\nNote 2: This bit can be cleared by writing '1' to it."]
pub struct ADDRDETF_R(crate::FieldReader<bool, ADDRDETF_A>);
impl ADDRDETF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRDETF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRDETF_A {
        match self.bits {
            false => ADDRDETF_A::_0,
            true => ADDRDETF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADDRDETF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADDRDETF_A::_1
    }
}
impl core::ops::Deref for ADDRDETF_R {
    type Target = crate::FieldReader<bool, ADDRDETF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRDETF` writer - RS-485 Address Byte Detect Flag\nNote 1: This field is used for RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1 to enable Address detection mode.\nNote 2: This bit can be cleared by writing '1' to it."]
pub struct ADDRDETF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRDETF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRDETF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receiver detects a data that is not an address bit (bit 9 ='0')"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDRDETF_A::_0)
    }
    #[doc = "Receiver detects a data that is an address bit (bit 9 ='1')"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDRDETF_A::_1)
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
#[doc = "Parity Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'parity bit'.\nNote: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEF_A {
    #[doc = "0: No parity error is generated"]
    _0 = 0,
    #[doc = "1: Parity error is generated"]
    _1 = 1,
}
impl From<PEF_A> for bool {
    #[inline(always)]
    fn from(variant: PEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEF` reader - Parity Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'parity bit'.\nNote: This bit can be cleared by writing '1' to it."]
pub struct PEF_R(crate::FieldReader<bool, PEF_A>);
impl PEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEF_A {
        match self.bits {
            false => PEF_A::_0,
            true => PEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PEF_A::_1
    }
}
impl core::ops::Deref for PEF_R {
    type Target = crate::FieldReader<bool, PEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEF` writer - Parity Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'parity bit'.\nNote: This bit can be cleared by writing '1' to it."]
pub struct PEF_W<'a> {
    w: &'a mut W,
}
impl<'a> PEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEF_A::_0)
    }
    #[doc = "Parity error is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEF_A::_1)
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
#[doc = "Framing Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0).\nNote: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: No framing error is generated"]
    _0 = 0,
    #[doc = "1: Framing error is generated"]
    _1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEF` reader - Framing Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0).\nNote: This bit can be cleared by writing '1' to it."]
pub struct FEF_R(crate::FieldReader<bool, FEF_A>);
impl FEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FEF_A::_1
    }
}
impl core::ops::Deref for FEF_R {
    type Target = crate::FieldReader<bool, FEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEF` writer - Framing Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0).\nNote: This bit can be cleared by writing '1' to it."]
pub struct FEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No framing error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEF_A::_0)
    }
    #[doc = "Framing error is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEF_A::_1)
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
#[doc = "Break Interrupt Flag\nThis bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits).\nNote: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIF_A {
    #[doc = "0: No Break interrupt is generated"]
    _0 = 0,
    #[doc = "1: Break interrupt is generated"]
    _1 = 1,
}
impl From<BIF_A> for bool {
    #[inline(always)]
    fn from(variant: BIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIF` reader - Break Interrupt Flag\nThis bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits).\nNote: This bit can be cleared by writing '1' to it."]
pub struct BIF_R(crate::FieldReader<bool, BIF_A>);
impl BIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIF_A {
        match self.bits {
            false => BIF_A::_0,
            true => BIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BIF_A::_1
    }
}
impl core::ops::Deref for BIF_R {
    type Target = crate::FieldReader<bool, BIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIF` writer - Break Interrupt Flag\nThis bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits).\nNote: This bit can be cleared by writing '1' to it."]
pub struct BIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Break interrupt is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BIF_A::_0)
    }
    #[doc = "Break interrupt is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BIF_A::_1)
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
#[doc = "Field `RXPTR` reader - RX FIFO Pointer (Read Only)\nThis field indicates the RX FIFO Buffer Pointer. When UART receives one byte from external device, RXPTR increases one. When one byte of RX FIFO is read by CPU, RXPTR decreases one.\nThe Maximum value shown in RXPTR is 15. When the using level of RX FIFO Buffer equal to 16, the RXFULL bit is set to 1 and RXPTR will show 0. As one byte of RX FIFO is read by CPU, the RXFULL bit is cleared to 0 and RXPTR will show 15."]
pub struct RXPTR_R(crate::FieldReader<u8, u8>);
impl RXPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receiver FIFO Empty (Read Only)\nThis bit initiate RX FIFO empty or not.\nNote: When the last byte of RX FIFO has been read by CPU, hardware sets this bit high. It will be cleared when UART receives any new data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTY_A {
    #[doc = "0: RX FIFO is not empty"]
    _0 = 0,
    #[doc = "1: RX FIFO is empty"]
    _1 = 1,
}
impl From<RXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPTY` reader - Receiver FIFO Empty (Read Only)\nThis bit initiate RX FIFO empty or not.\nNote: When the last byte of RX FIFO has been read by CPU, hardware sets this bit high. It will be cleared when UART receives any new data."]
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
#[doc = "Receiver FIFO Full (Read Only)\nThis bit initiates RX FIFO full or not.\nNote: This bit is set when the number of usage in RX FIFO Buffer is equal to 16, otherwise it is cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFULL_A {
    #[doc = "0: RX FIFO is not full"]
    _0 = 0,
    #[doc = "1: RX FIFO is full"]
    _1 = 1,
}
impl From<RXFULL_A> for bool {
    #[inline(always)]
    fn from(variant: RXFULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFULL` reader - Receiver FIFO Full (Read Only)\nThis bit initiates RX FIFO full or not.\nNote: This bit is set when the number of usage in RX FIFO Buffer is equal to 16, otherwise it is cleared by hardware."]
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
#[doc = "Field `TXPTR` reader - TX FIFO Pointer (Read Only)\nThis field indicates the TX FIFO Buffer Pointer. When CPU writes one byte into UART_DAT, TXPTR increases one. When one byte of TX FIFO is transferred to Transmitter Shift Register, TXPTR decreases one.\nThe Maximum value shown in TXPTR is 15. When the using level of TX FIFO Buffer equal to 16, the TXFULL bit is set to 1 and TXPTR will show 0. As one byte of TX FIFO is transferred to Transmitter Shift Register, the TXFULL bit is cleared to 0 and TXPTR will show 15."]
pub struct TXPTR_R(crate::FieldReader<u8, u8>);
impl TXPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmitter FIFO Empty (Read Only)\nThis bit indicates TX FIFO empty or not.\nNote: When the last byte of TX FIFO has been transferred to Transmitter Shift Register, hardware sets this bit high. It will be cleared when writing data into UART_DAT (TX FIFO not empty).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTY_A {
    #[doc = "0: TX FIFO is not empty"]
    _0 = 0,
    #[doc = "1: TX FIFO is empty"]
    _1 = 1,
}
impl From<TXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - Transmitter FIFO Empty (Read Only)\nThis bit indicates TX FIFO empty or not.\nNote: When the last byte of TX FIFO has been transferred to Transmitter Shift Register, hardware sets this bit high. It will be cleared when writing data into UART_DAT (TX FIFO not empty)."]
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
#[doc = "Transmitter FIFO Full (Read Only)\nThis bit indicates TX FIFO full or not.\nNote: This bit is set when the number of usage in TX FIFO Buffer is equal to 16, otherwise it is cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFULL_A {
    #[doc = "0: TX FIFO is not full"]
    _0 = 0,
    #[doc = "1: TX FIFO is full"]
    _1 = 1,
}
impl From<TXFULL_A> for bool {
    #[inline(always)]
    fn from(variant: TXFULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFULL` reader - Transmitter FIFO Full (Read Only)\nThis bit indicates TX FIFO full or not.\nNote: This bit is set when the number of usage in TX FIFO Buffer is equal to 16, otherwise it is cleared by hardware."]
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
#[doc = "TX Overflow Error Interrupt Flag\nIf TX FIFO (UART_DAT) is full, an additional write to UART_DAT will cause this bit to logic 1.\nNote: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOVIF_A {
    #[doc = "0: TX FIFO is not overflow"]
    _0 = 0,
    #[doc = "1: TX FIFO is overflow"]
    _1 = 1,
}
impl From<TXOVIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXOVIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXOVIF` reader - TX Overflow Error Interrupt Flag\nIf TX FIFO (UART_DAT) is full, an additional write to UART_DAT will cause this bit to logic 1.\nNote: This bit can be cleared by writing '1' to it."]
pub struct TXOVIF_R(crate::FieldReader<bool, TXOVIF_A>);
impl TXOVIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOVIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOVIF_A {
        match self.bits {
            false => TXOVIF_A::_0,
            true => TXOVIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXOVIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXOVIF_A::_1
    }
}
impl core::ops::Deref for TXOVIF_R {
    type Target = crate::FieldReader<bool, TXOVIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOVIF` writer - TX Overflow Error Interrupt Flag\nIf TX FIFO (UART_DAT) is full, an additional write to UART_DAT will cause this bit to logic 1.\nNote: This bit can be cleared by writing '1' to it."]
pub struct TXOVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOVIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOVIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TX FIFO is not overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOVIF_A::_0)
    }
    #[doc = "TX FIFO is overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOVIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Transmitter Empty Flag (Read Only)\nThis bit is set by hardware when TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted.\nNote: This bit is cleared automatically when TX FIFO is not empty or the last byte transmission has not completed.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTYF_A {
    #[doc = "0: TX FIFO is not empty or the STOP bit of the last byte has been not transmitted"]
    _0 = 0,
    #[doc = "1: TX FIFO is empty and the STOP bit of the last byte has been transmitted"]
    _1 = 1,
}
impl From<TXEMPTYF_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTYF` reader - Transmitter Empty Flag (Read Only)\nThis bit is set by hardware when TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted.\nNote: This bit is cleared automatically when TX FIFO is not empty or the last byte transmission has not completed."]
pub struct TXEMPTYF_R(crate::FieldReader<bool, TXEMPTYF_A>);
impl TXEMPTYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPTYF_A {
        match self.bits {
            false => TXEMPTYF_A::_0,
            true => TXEMPTYF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXEMPTYF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXEMPTYF_A::_1
    }
}
impl core::ops::Deref for TXEMPTYF_R {
    type Target = crate::FieldReader<bool, TXEMPTYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RX Idle Status (Read Only)\nThis bit is set by hardware when RX is idle.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIDLE_A {
    #[doc = "0: RX is busy"]
    _0 = 0,
    #[doc = "1: RX is idle. (Default)"]
    _1 = 1,
}
impl From<RXIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIDLE` reader - RX Idle Status (Read Only)\nThis bit is set by hardware when RX is idle."]
pub struct RXIDLE_R(crate::FieldReader<bool, RXIDLE_A>);
impl RXIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIDLE_A {
        match self.bits {
            false => RXIDLE_A::_0,
            true => RXIDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXIDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXIDLE_A::_1
    }
}
impl core::ops::Deref for RXIDLE_R {
    type Target = crate::FieldReader<bool, RXIDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TX and RX Active Status (Read Only)\nThis bit indicates TX and RX are active or inactive.\nNote: When TXRXDIS (UART_FUNCSEL\\[3\\]) is set and both TX and RX are in idle state, this bit is cleared. The UART controller can not transmit or receive data at this moment. Otherwise this bit is set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRXACT_A {
    #[doc = "0: TX and RX are inactive"]
    _0 = 0,
    #[doc = "1: TX and RX are active. (Default)"]
    _1 = 1,
}
impl From<TXRXACT_A> for bool {
    #[inline(always)]
    fn from(variant: TXRXACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRXACT` reader - TX and RX Active Status (Read Only)\nThis bit indicates TX and RX are active or inactive.\nNote: When TXRXDIS (UART_FUNCSEL\\[3\\]) is set and both TX and RX are in idle state, this bit is cleared. The UART controller can not transmit or receive data at this moment. Otherwise this bit is set."]
pub struct TXRXACT_R(crate::FieldReader<bool, TXRXACT_A>);
impl TXRXACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRXACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRXACT_A {
        match self.bits {
            false => TXRXACT_A::_0,
            true => TXRXACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRXACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRXACT_A::_1
    }
}
impl core::ops::Deref for TXRXACT_R {
    type Target = crate::FieldReader<bool, TXRXACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RX Overflow Error Interrupt Flag This bit is set when RX FIFO overflow. If the number of bytes of received data is greater than RX_FIFO (UART_DAT) size 16 bytes, this bit will be set. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn rxovif(&self) -> RXOVIF_R {
        RXOVIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto-baud Rate Detect Interrupt Flag This bit is set to logic '1' when auto-baud rate detect function is finished. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn abrdif(&self) -> ABRDIF_R {
        ABRDIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto-baud Rate Detect Time-out Interrupt Flag This bit is set to logic '1' in Auto-baud Rate Detect mode when the baud rate counter is overflow. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn abrdtoif(&self) -> ABRDTOIF_R {
        ABRDTOIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RS-485 Address Byte Detect Flag Note 1: This field is used for RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1 to enable Address detection mode. Note 2: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn addrdetf(&self) -> ADDRDETF_R {
        ADDRDETF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error Flag This bit is set to logic 1 whenever the received character does not have a valid 'parity bit'. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Framing Error Flag This bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0). Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Break Interrupt Flag This bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits). Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - RX FIFO Pointer (Read Only) This field indicates the RX FIFO Buffer Pointer. When UART receives one byte from external device, RXPTR increases one. When one byte of RX FIFO is read by CPU, RXPTR decreases one. The Maximum value shown in RXPTR is 15. When the using level of RX FIFO Buffer equal to 16, the RXFULL bit is set to 1 and RXPTR will show 0. As one byte of RX FIFO is read by CPU, the RXFULL bit is cleared to 0 and RXPTR will show 15."]
    #[inline(always)]
    pub fn rxptr(&self) -> RXPTR_R {
        RXPTR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Receiver FIFO Empty (Read Only) This bit initiate RX FIFO empty or not. Note: When the last byte of RX FIFO has been read by CPU, hardware sets this bit high. It will be cleared when UART receives any new data."]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receiver FIFO Full (Read Only) This bit initiates RX FIFO full or not. Note: This bit is set when the number of usage in RX FIFO Buffer is equal to 16, otherwise it is cleared by hardware."]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - TX FIFO Pointer (Read Only) This field indicates the TX FIFO Buffer Pointer. When CPU writes one byte into UART_DAT, TXPTR increases one. When one byte of TX FIFO is transferred to Transmitter Shift Register, TXPTR decreases one. The Maximum value shown in TXPTR is 15. When the using level of TX FIFO Buffer equal to 16, the TXFULL bit is set to 1 and TXPTR will show 0. As one byte of TX FIFO is transferred to Transmitter Shift Register, the TXFULL bit is cleared to 0 and TXPTR will show 15."]
    #[inline(always)]
    pub fn txptr(&self) -> TXPTR_R {
        TXPTR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Transmitter FIFO Empty (Read Only) This bit indicates TX FIFO empty or not. Note: When the last byte of TX FIFO has been transferred to Transmitter Shift Register, hardware sets this bit high. It will be cleared when writing data into UART_DAT (TX FIFO not empty)."]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmitter FIFO Full (Read Only) This bit indicates TX FIFO full or not. Note: This bit is set when the number of usage in TX FIFO Buffer is equal to 16, otherwise it is cleared by hardware."]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TX Overflow Error Interrupt Flag If TX FIFO (UART_DAT) is full, an additional write to UART_DAT will cause this bit to logic 1. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn txovif(&self) -> TXOVIF_R {
        TXOVIF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Transmitter Empty Flag (Read Only) This bit is set by hardware when TX FIFO (UART_DAT) is empty and the STOP bit of the last byte has been transmitted. Note: This bit is cleared automatically when TX FIFO is not empty or the last byte transmission has not completed."]
    #[inline(always)]
    pub fn txemptyf(&self) -> TXEMPTYF_R {
        TXEMPTYF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RX Idle Status (Read Only) This bit is set by hardware when RX is idle."]
    #[inline(always)]
    pub fn rxidle(&self) -> RXIDLE_R {
        RXIDLE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TX and RX Active Status (Read Only) This bit indicates TX and RX are active or inactive. Note: When TXRXDIS (UART_FUNCSEL\\[3\\]) is set and both TX and RX are in idle state, this bit is cleared. The UART controller can not transmit or receive data at this moment. Otherwise this bit is set."]
    #[inline(always)]
    pub fn txrxact(&self) -> TXRXACT_R {
        TXRXACT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Overflow Error Interrupt Flag This bit is set when RX FIFO overflow. If the number of bytes of received data is greater than RX_FIFO (UART_DAT) size 16 bytes, this bit will be set. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn rxovif(&mut self) -> RXOVIF_W {
        RXOVIF_W { w: self }
    }
    #[doc = "Bit 1 - Auto-baud Rate Detect Interrupt Flag This bit is set to logic '1' when auto-baud rate detect function is finished. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn abrdif(&mut self) -> ABRDIF_W {
        ABRDIF_W { w: self }
    }
    #[doc = "Bit 2 - Auto-baud Rate Detect Time-out Interrupt Flag This bit is set to logic '1' in Auto-baud Rate Detect mode when the baud rate counter is overflow. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn abrdtoif(&mut self) -> ABRDTOIF_W {
        ABRDTOIF_W { w: self }
    }
    #[doc = "Bit 3 - RS-485 Address Byte Detect Flag Note 1: This field is used for RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1 to enable Address detection mode. Note 2: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn addrdetf(&mut self) -> ADDRDETF_W {
        ADDRDETF_W { w: self }
    }
    #[doc = "Bit 4 - Parity Error Flag This bit is set to logic 1 whenever the received character does not have a valid 'parity bit'. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn pef(&mut self) -> PEF_W {
        PEF_W { w: self }
    }
    #[doc = "Bit 5 - Framing Error Flag This bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0). Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
    #[doc = "Bit 6 - Break Interrupt Flag This bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits). Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W {
        BIF_W { w: self }
    }
    #[doc = "Bit 24 - TX Overflow Error Interrupt Flag If TX FIFO (UART_DAT) is full, an additional write to UART_DAT will cause this bit to logic 1. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn txovif(&mut self) -> TXOVIF_W {
        TXOVIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifosts](index.html) module"]
pub struct UART_FIFOSTS_SPEC;
impl crate::RegisterSpec for UART_FIFOSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_fifosts::R](R) reader structure"]
impl crate::Readable for UART_FIFOSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_fifosts::W](W) writer structure"]
impl crate::Writable for UART_FIFOSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_FIFOSTS to value 0xb040_4000"]
impl crate::Resettable for UART_FIFOSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb040_4000
    }
}
