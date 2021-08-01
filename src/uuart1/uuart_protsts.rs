#[doc = "Register `UUART_PROTSTS` reader"]
pub struct R(crate::R<UUART_PROTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_PROTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_PROTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_PROTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUART_PROTSTS` writer"]
pub struct W(crate::W<UUART_PROTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUART_PROTSTS_SPEC>;
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
impl From<crate::W<UUART_PROTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUART_PROTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Start Interrupt Flag\nNote 1: It is cleared by software writing one into this bit.\nNote 2: Used for user to load next transmit data when there is no data in transmit buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTIF_A {
    #[doc = "0: A transmit start interrupt status has not occurred"]
    _0 = 0,
    #[doc = "1: A transmit start interrupt status has occurred"]
    _1 = 1,
}
impl From<TXSTIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTIF` reader - Transmit Start Interrupt Flag\nNote 1: It is cleared by software writing one into this bit.\nNote 2: Used for user to load next transmit data when there is no data in transmit buffer."]
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
#[doc = "Field `TXSTIF` writer - Transmit Start Interrupt Flag\nNote 1: It is cleared by software writing one into this bit.\nNote 2: Used for user to load next transmit data when there is no data in transmit buffer."]
pub struct TXSTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSTIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A transmit start interrupt status has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXSTIF_A::_0)
    }
    #[doc = "A transmit start interrupt status has occurred"]
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
#[doc = "Transmit End Interrupt Flag\nNote: It is cleared by software writing 1 into this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDIF_A {
    #[doc = "0: A transmit end interrupt status has not occurred"]
    _0 = 0,
    #[doc = "1: A transmit end interrupt status has occurred"]
    _1 = 1,
}
impl From<TXENDIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXENDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXENDIF` reader - Transmit End Interrupt Flag\nNote: It is cleared by software writing 1 into this bit."]
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
#[doc = "Field `TXENDIF` writer - Transmit End Interrupt Flag\nNote: It is cleared by software writing 1 into this bit."]
pub struct TXENDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXENDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A transmit end interrupt status has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXENDIF_A::_0)
    }
    #[doc = "A transmit end interrupt status has occurred"]
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
#[doc = "Receive Start Interrupt Flag\nNote: It is cleared by software writing 1 into this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTIF_A {
    #[doc = "0: A receive start interrupt status has not occurred"]
    _0 = 0,
    #[doc = "1: A receive start interrupt status has occurred"]
    _1 = 1,
}
impl From<RXSTIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTIF` reader - Receive Start Interrupt Flag\nNote: It is cleared by software writing 1 into this bit."]
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
#[doc = "Field `RXSTIF` writer - Receive Start Interrupt Flag\nNote: It is cleared by software writing 1 into this bit."]
pub struct RXSTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSTIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A receive start interrupt status has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXSTIF_A::_0)
    }
    #[doc = "A receive start interrupt status has occurred"]
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
#[doc = "Receive End Interrupt Flag\nNote: It is cleared by software writing 1 into this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXENDIF_A {
    #[doc = "0: A receive finish interrupt status has not occurred"]
    _0 = 0,
    #[doc = "1: A receive finish interrupt status has occurred"]
    _1 = 1,
}
impl From<RXENDIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXENDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXENDIF` reader - Receive End Interrupt Flag\nNote: It is cleared by software writing 1 into this bit."]
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
#[doc = "Field `RXENDIF` writer - Receive End Interrupt Flag\nNote: It is cleared by software writing 1 into this bit."]
pub struct RXENDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXENDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A receive finish interrupt status has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXENDIF_A::_0)
    }
    #[doc = "A receive finish interrupt status has occurred"]
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
#[doc = "Parity Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'parity bit'.\nNote: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYERR_A {
    #[doc = "0: No parity error is generated"]
    _0 = 0,
    #[doc = "1: Parity error is generated"]
    _1 = 1,
}
impl From<PARITYERR_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITYERR` reader - Parity Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'parity bit'.\nNote: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
pub struct PARITYERR_R(crate::FieldReader<bool, PARITYERR_A>);
impl PARITYERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITYERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYERR_A {
        match self.bits {
            false => PARITYERR_A::_0,
            true => PARITYERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PARITYERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PARITYERR_A::_1
    }
}
impl core::ops::Deref for PARITYERR_R {
    type Target = crate::FieldReader<bool, PARITYERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITYERR` writer - Parity Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'parity bit'.\nNote: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
pub struct PARITYERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITYERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PARITYERR_A::_0)
    }
    #[doc = "Parity error is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PARITYERR_A::_1)
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
#[doc = "Framing Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0).\nNote: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMERR_A {
    #[doc = "0: No framing error is generated"]
    _0 = 0,
    #[doc = "1: Framing error is generated"]
    _1 = 1,
}
impl From<FRMERR_A> for bool {
    #[inline(always)]
    fn from(variant: FRMERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMERR` reader - Framing Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0).\nNote: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
pub struct FRMERR_R(crate::FieldReader<bool, FRMERR_A>);
impl FRMERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRMERR_A {
        match self.bits {
            false => FRMERR_A::_0,
            true => FRMERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRMERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRMERR_A::_1
    }
}
impl core::ops::Deref for FRMERR_R {
    type Target = crate::FieldReader<bool, FRMERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMERR` writer - Framing Error Flag\nThis bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0).\nNote: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
pub struct FRMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRMERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No framing error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRMERR_A::_0)
    }
    #[doc = "Framing error is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRMERR_A::_1)
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
#[doc = "Break Flag\nThis bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits).\nNote: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREAK_A {
    #[doc = "0: No Break is generated"]
    _0 = 0,
    #[doc = "1: Break is generated in the receiver bus"]
    _1 = 1,
}
impl From<BREAK_A> for bool {
    #[inline(always)]
    fn from(variant: BREAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREAK` reader - Break Flag\nThis bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits).\nNote: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
pub struct BREAK_R(crate::FieldReader<bool, BREAK_A>);
impl BREAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BREAK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREAK_A {
        match self.bits {
            false => BREAK_A::_0,
            true => BREAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BREAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BREAK_A::_1
    }
}
impl core::ops::Deref for BREAK_R {
    type Target = crate::FieldReader<bool, BREAK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREAK` writer - Break Flag\nThis bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits).\nNote: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
pub struct BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BREAK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Break is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BREAK_A::_0)
    }
    #[doc = "Break is generated in the receiver bus"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BREAK_A::_1)
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
#[doc = "Auto-baud Rate Interrupt Flag \nThis bit is set when auto-baud rate detection is done among the falling edge of the input data. If the ABRIEN (UUART_PROTCTL\\[6\\]) is set, the auto-baud rate interrupt will be generated. This bit can be set 4 times when the input data pattern is 0x55 and it is cleared before the next falling edge of the input bus.\nNote: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRDETIF_A {
    #[doc = "0: Auto-baud rate detect function is not done"]
    _0 = 0,
    #[doc = "1: One Bit auto-baud rate detect function is done"]
    _1 = 1,
}
impl From<ABRDETIF_A> for bool {
    #[inline(always)]
    fn from(variant: ABRDETIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRDETIF` reader - Auto-baud Rate Interrupt Flag \nThis bit is set when auto-baud rate detection is done among the falling edge of the input data. If the ABRIEN (UUART_PROTCTL\\[6\\]) is set, the auto-baud rate interrupt will be generated. This bit can be set 4 times when the input data pattern is 0x55 and it is cleared before the next falling edge of the input bus.\nNote: This bit can be cleared by writing '1' to it."]
pub struct ABRDETIF_R(crate::FieldReader<bool, ABRDETIF_A>);
impl ABRDETIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRDETIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRDETIF_A {
        match self.bits {
            false => ABRDETIF_A::_0,
            true => ABRDETIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABRDETIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABRDETIF_A::_1
    }
}
impl core::ops::Deref for ABRDETIF_R {
    type Target = crate::FieldReader<bool, ABRDETIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRDETIF` writer - Auto-baud Rate Interrupt Flag \nThis bit is set when auto-baud rate detection is done among the falling edge of the input data. If the ABRIEN (UUART_PROTCTL\\[6\\]) is set, the auto-baud rate interrupt will be generated. This bit can be set 4 times when the input data pattern is 0x55 and it is cleared before the next falling edge of the input bus.\nNote: This bit can be cleared by writing '1' to it."]
pub struct ABRDETIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRDETIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABRDETIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-baud rate detect function is not done"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABRDETIF_A::_0)
    }
    #[doc = "One Bit auto-baud rate detect function is done"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABRDETIF_A::_1)
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
#[doc = "RX Bus Status Flag (Read Only) \nThis bit indicates the busy status of the receiver.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUSY_A {
    #[doc = "0: The receiver is Idle"]
    _0 = 0,
    #[doc = "1: The receiver is BUSY"]
    _1 = 1,
}
impl From<RXBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: RXBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBUSY` reader - RX Bus Status Flag (Read Only) \nThis bit indicates the busy status of the receiver."]
pub struct RXBUSY_R(crate::FieldReader<bool, RXBUSY_A>);
impl RXBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBUSY_A {
        match self.bits {
            false => RXBUSY_A::_0,
            true => RXBUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXBUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXBUSY_A::_1
    }
}
impl core::ops::Deref for RXBUSY_R {
    type Target = crate::FieldReader<bool, RXBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Auto-baud Rate Error Status \nThis bit is set when auto-baud rate detection counter overrun. When the auto-baud rate counter overrun, the user shall revise the CLKDIV (UUART_BRGEN\\[25:16\\]) value and enable ABREN (UUART_PROTCTL\\[6\\]) to detect the correct baud rate again.\nNote 1: This bit is set at the same time of ABRDETIF.\nNote 2: This bit can be cleared by writing '1' to ABRDETIF or ABERRSTS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABERRSTS_A {
    #[doc = "0: Auto-baud rate detect counter is not overrun"]
    _0 = 0,
    #[doc = "1: Auto-baud rate detect counter is overrun"]
    _1 = 1,
}
impl From<ABERRSTS_A> for bool {
    #[inline(always)]
    fn from(variant: ABERRSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABERRSTS` reader - Auto-baud Rate Error Status \nThis bit is set when auto-baud rate detection counter overrun. When the auto-baud rate counter overrun, the user shall revise the CLKDIV (UUART_BRGEN\\[25:16\\]) value and enable ABREN (UUART_PROTCTL\\[6\\]) to detect the correct baud rate again.\nNote 1: This bit is set at the same time of ABRDETIF.\nNote 2: This bit can be cleared by writing '1' to ABRDETIF or ABERRSTS."]
pub struct ABERRSTS_R(crate::FieldReader<bool, ABERRSTS_A>);
impl ABERRSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABERRSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABERRSTS_A {
        match self.bits {
            false => ABERRSTS_A::_0,
            true => ABERRSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABERRSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABERRSTS_A::_1
    }
}
impl core::ops::Deref for ABERRSTS_R {
    type Target = crate::FieldReader<bool, ABERRSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABERRSTS` writer - Auto-baud Rate Error Status \nThis bit is set when auto-baud rate detection counter overrun. When the auto-baud rate counter overrun, the user shall revise the CLKDIV (UUART_BRGEN\\[25:16\\]) value and enable ABREN (UUART_PROTCTL\\[6\\]) to detect the correct baud rate again.\nNote 1: This bit is set at the same time of ABRDETIF.\nNote 2: This bit can be cleared by writing '1' to ABRDETIF or ABERRSTS."]
pub struct ABERRSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> ABERRSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABERRSTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-baud rate detect counter is not overrun"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABERRSTS_A::_0)
    }
    #[doc = "Auto-baud rate detect counter is overrun"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABERRSTS_A::_1)
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
#[doc = "nCTS Synchronized Level Status (Read Only)\nThis bit used to indicate the current status of the internal synchronized nCTS signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSSYNCLV_A {
    #[doc = "0: The internal synchronized nCTS is low"]
    _0 = 0,
    #[doc = "1: The internal synchronized nCTS is high"]
    _1 = 1,
}
impl From<CTSSYNCLV_A> for bool {
    #[inline(always)]
    fn from(variant: CTSSYNCLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSSYNCLV` reader - nCTS Synchronized Level Status (Read Only)\nThis bit used to indicate the current status of the internal synchronized nCTS signal."]
pub struct CTSSYNCLV_R(crate::FieldReader<bool, CTSSYNCLV_A>);
impl CTSSYNCLV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSSYNCLV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSSYNCLV_A {
        match self.bits {
            false => CTSSYNCLV_A::_0,
            true => CTSSYNCLV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTSSYNCLV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTSSYNCLV_A::_1
    }
}
impl core::ops::Deref for CTSSYNCLV_R {
    type Target = crate::FieldReader<bool, CTSSYNCLV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nCTS Pin Status (Read Only)\nThis bit used to monitor the current status of nCTS pin input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSLV_A {
    #[doc = "0: nCTS pin input is low level voltage logic state"]
    _0 = 0,
    #[doc = "1: nCTS pin input is high level voltage logic state"]
    _1 = 1,
}
impl From<CTSLV_A> for bool {
    #[inline(always)]
    fn from(variant: CTSLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSLV` reader - nCTS Pin Status (Read Only)\nThis bit used to monitor the current status of nCTS pin input."]
pub struct CTSLV_R(crate::FieldReader<bool, CTSLV_A>);
impl CTSLV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSLV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSLV_A {
        match self.bits {
            false => CTSLV_A::_0,
            true => CTSLV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTSLV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTSLV_A::_1
    }
}
impl core::ops::Deref for CTSLV_R {
    type Target = crate::FieldReader<bool, CTSLV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Transmit Start Interrupt Flag Note 1: It is cleared by software writing one into this bit. Note 2: Used for user to load next transmit data when there is no data in transmit buffer."]
    #[inline(always)]
    pub fn txstif(&self) -> TXSTIF_R {
        TXSTIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit End Interrupt Flag Note: It is cleared by software writing 1 into this bit."]
    #[inline(always)]
    pub fn txendif(&self) -> TXENDIF_R {
        TXENDIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Start Interrupt Flag Note: It is cleared by software writing 1 into this bit."]
    #[inline(always)]
    pub fn rxstif(&self) -> RXSTIF_R {
        RXSTIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive End Interrupt Flag Note: It is cleared by software writing 1 into this bit."]
    #[inline(always)]
    pub fn rxendif(&self) -> RXENDIF_R {
        RXENDIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Parity Error Flag This bit is set to logic 1 whenever the received character does not have a valid 'parity bit'. Note: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
    #[inline(always)]
    pub fn parityerr(&self) -> PARITYERR_R {
        PARITYERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Framing Error Flag This bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0). Note: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
    #[inline(always)]
    pub fn frmerr(&self) -> FRMERR_R {
        FRMERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Break Flag This bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits). Note: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Auto-baud Rate Interrupt Flag This bit is set when auto-baud rate detection is done among the falling edge of the input data. If the ABRIEN (UUART_PROTCTL\\[6\\]) is set, the auto-baud rate interrupt will be generated. This bit can be set 4 times when the input data pattern is 0x55 and it is cleared before the next falling edge of the input bus. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn abrdetif(&self) -> ABRDETIF_R {
        ABRDETIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RX Bus Status Flag (Read Only) This bit indicates the busy status of the receiver."]
    #[inline(always)]
    pub fn rxbusy(&self) -> RXBUSY_R {
        RXBUSY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Auto-baud Rate Error Status This bit is set when auto-baud rate detection counter overrun. When the auto-baud rate counter overrun, the user shall revise the CLKDIV (UUART_BRGEN\\[25:16\\]) value and enable ABREN (UUART_PROTCTL\\[6\\]) to detect the correct baud rate again. Note 1: This bit is set at the same time of ABRDETIF. Note 2: This bit can be cleared by writing '1' to ABRDETIF or ABERRSTS."]
    #[inline(always)]
    pub fn aberrsts(&self) -> ABERRSTS_R {
        ABERRSTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - nCTS Synchronized Level Status (Read Only) This bit used to indicate the current status of the internal synchronized nCTS signal."]
    #[inline(always)]
    pub fn ctssynclv(&self) -> CTSSYNCLV_R {
        CTSSYNCLV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - nCTS Pin Status (Read Only) This bit used to monitor the current status of nCTS pin input."]
    #[inline(always)]
    pub fn ctslv(&self) -> CTSLV_R {
        CTSLV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit Start Interrupt Flag Note 1: It is cleared by software writing one into this bit. Note 2: Used for user to load next transmit data when there is no data in transmit buffer."]
    #[inline(always)]
    pub fn txstif(&mut self) -> TXSTIF_W {
        TXSTIF_W { w: self }
    }
    #[doc = "Bit 2 - Transmit End Interrupt Flag Note: It is cleared by software writing 1 into this bit."]
    #[inline(always)]
    pub fn txendif(&mut self) -> TXENDIF_W {
        TXENDIF_W { w: self }
    }
    #[doc = "Bit 3 - Receive Start Interrupt Flag Note: It is cleared by software writing 1 into this bit."]
    #[inline(always)]
    pub fn rxstif(&mut self) -> RXSTIF_W {
        RXSTIF_W { w: self }
    }
    #[doc = "Bit 4 - Receive End Interrupt Flag Note: It is cleared by software writing 1 into this bit."]
    #[inline(always)]
    pub fn rxendif(&mut self) -> RXENDIF_W {
        RXENDIF_W { w: self }
    }
    #[doc = "Bit 5 - Parity Error Flag This bit is set to logic 1 whenever the received character does not have a valid 'parity bit'. Note: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
    #[inline(always)]
    pub fn parityerr(&mut self) -> PARITYERR_W {
        PARITYERR_W { w: self }
    }
    #[doc = "Bit 6 - Framing Error Flag This bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0). Note: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
    #[inline(always)]
    pub fn frmerr(&mut self) -> FRMERR_W {
        FRMERR_W { w: self }
    }
    #[doc = "Bit 7 - Break Flag This bit is set to logic 1 whenever the received data input (RX) is held in the 'spacing state' (logic 0) for longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits). Note: This bit can be cleared by writing '1' among the BREAK, FRMERR and PARITYERR bits."]
    #[inline(always)]
    pub fn break_(&mut self) -> BREAK_W {
        BREAK_W { w: self }
    }
    #[doc = "Bit 9 - Auto-baud Rate Interrupt Flag This bit is set when auto-baud rate detection is done among the falling edge of the input data. If the ABRIEN (UUART_PROTCTL\\[6\\]) is set, the auto-baud rate interrupt will be generated. This bit can be set 4 times when the input data pattern is 0x55 and it is cleared before the next falling edge of the input bus. Note: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn abrdetif(&mut self) -> ABRDETIF_W {
        ABRDETIF_W { w: self }
    }
    #[doc = "Bit 11 - Auto-baud Rate Error Status This bit is set when auto-baud rate detection counter overrun. When the auto-baud rate counter overrun, the user shall revise the CLKDIV (UUART_BRGEN\\[25:16\\]) value and enable ABREN (UUART_PROTCTL\\[6\\]) to detect the correct baud rate again. Note 1: This bit is set at the same time of ABRDETIF. Note 2: This bit can be cleared by writing '1' to ABRDETIF or ABERRSTS."]
    #[inline(always)]
    pub fn aberrsts(&mut self) -> ABERRSTS_W {
        ABERRSTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_protsts](index.html) module"]
pub struct UUART_PROTSTS_SPEC;
impl crate::RegisterSpec for UUART_PROTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_protsts::R](R) reader structure"]
impl crate::Readable for UUART_PROTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuart_protsts::W](W) writer structure"]
impl crate::Writable for UUART_PROTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UUART_PROTSTS to value 0"]
impl crate::Resettable for UUART_PROTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
