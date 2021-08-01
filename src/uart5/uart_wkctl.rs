#[doc = "Register `UART_WKCTL` reader"]
pub struct R(crate::R<UART_WKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_WKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_WKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_WKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_WKCTL` writer"]
pub struct W(crate::W<UART_WKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_WKCTL_SPEC>;
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
impl From<crate::W<UART_WKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_WKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "nCTS Wake-up Enable Bit\nNote:When the system is in Power-down mode, an external.nCTS change will wake up system from Power-down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKCTSEN_A {
    #[doc = "0: nCTS Wake-up system function Disabled"]
    _0 = 0,
    #[doc = "1: nCTS Wake-up system function Enabled"]
    _1 = 1,
}
impl From<WKCTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKCTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKCTSEN` reader - nCTS Wake-up Enable Bit\nNote:When the system is in Power-down mode, an external.nCTS change will wake up system from Power-down mode."]
pub struct WKCTSEN_R(crate::FieldReader<bool, WKCTSEN_A>);
impl WKCTSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKCTSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKCTSEN_A {
        match self.bits {
            false => WKCTSEN_A::_0,
            true => WKCTSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKCTSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKCTSEN_A::_1
    }
}
impl core::ops::Deref for WKCTSEN_R {
    type Target = crate::FieldReader<bool, WKCTSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKCTSEN` writer - nCTS Wake-up Enable Bit\nNote:When the system is in Power-down mode, an external.nCTS change will wake up system from Power-down mode."]
pub struct WKCTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKCTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKCTSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "nCTS Wake-up system function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKCTSEN_A::_0)
    }
    #[doc = "nCTS Wake-up system function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKCTSEN_A::_1)
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
#[doc = "Incoming Data Wake-up Enable Bit\nNote:When the system is in Power-down mode, incoming data will wake-up system from Power-down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKDATEN_A {
    #[doc = "0: Incoming data wake-up system function Disabled"]
    _0 = 0,
    #[doc = "1: Incoming data wake-up system function Enabled"]
    _1 = 1,
}
impl From<WKDATEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKDATEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKDATEN` reader - Incoming Data Wake-up Enable Bit\nNote:When the system is in Power-down mode, incoming data will wake-up system from Power-down mode."]
pub struct WKDATEN_R(crate::FieldReader<bool, WKDATEN_A>);
impl WKDATEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKDATEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKDATEN_A {
        match self.bits {
            false => WKDATEN_A::_0,
            true => WKDATEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKDATEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKDATEN_A::_1
    }
}
impl core::ops::Deref for WKDATEN_R {
    type Target = crate::FieldReader<bool, WKDATEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKDATEN` writer - Incoming Data Wake-up Enable Bit\nNote:When the system is in Power-down mode, incoming data will wake-up system from Power-down mode."]
pub struct WKDATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKDATEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKDATEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Incoming data wake-up system function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKDATEN_A::_0)
    }
    #[doc = "Incoming data wake-up system function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKDATEN_A::_1)
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
#[doc = "Received Data FIFO Reached Threshold Wake-up Enable Bit\nNote: When the system is in Power-down mode, Received Data FIFO reached threshold will wake-up system from Power-down mode.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKRFRTEN_A {
    #[doc = "0: Received Data FIFO reached threshold wake-up system function Disabled"]
    _0 = 0,
    #[doc = "1: Received Data FIFO reached threshold wake-up system function Enabled"]
    _1 = 1,
}
impl From<WKRFRTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKRFRTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKRFRTEN` reader - Received Data FIFO Reached Threshold Wake-up Enable Bit\nNote: When the system is in Power-down mode, Received Data FIFO reached threshold will wake-up system from Power-down mode.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct WKRFRTEN_R(crate::FieldReader<bool, WKRFRTEN_A>);
impl WKRFRTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKRFRTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKRFRTEN_A {
        match self.bits {
            false => WKRFRTEN_A::_0,
            true => WKRFRTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKRFRTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKRFRTEN_A::_1
    }
}
impl core::ops::Deref for WKRFRTEN_R {
    type Target = crate::FieldReader<bool, WKRFRTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKRFRTEN` writer - Received Data FIFO Reached Threshold Wake-up Enable Bit\nNote: When the system is in Power-down mode, Received Data FIFO reached threshold will wake-up system from Power-down mode.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct WKRFRTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKRFRTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKRFRTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Received Data FIFO reached threshold wake-up system function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKRFRTEN_A::_0)
    }
    #[doc = "Received Data FIFO reached threshold wake-up system function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKRFRTEN_A::_1)
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
#[doc = "RS-485 Address Match (AAD Mode) Wake-up Enable Bit\nNote 1: When the system is in.Power-down mode, RS-485 Address Match will wake-up system from Power-down mode.\nNote 2: This bit is used for RS-485 Auto Address Detection (AAD) mode in RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKRS485EN_A {
    #[doc = "0: RS-485 Address Match (AAD mode) wake-up system function Disabled"]
    _0 = 0,
    #[doc = "1: RS-485 Address Match (AAD mode) wake-up system function Enabled"]
    _1 = 1,
}
impl From<WKRS485EN_A> for bool {
    #[inline(always)]
    fn from(variant: WKRS485EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKRS485EN` reader - RS-485 Address Match (AAD Mode) Wake-up Enable Bit\nNote 1: When the system is in.Power-down mode, RS-485 Address Match will wake-up system from Power-down mode.\nNote 2: This bit is used for RS-485 Auto Address Detection (AAD) mode in RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct WKRS485EN_R(crate::FieldReader<bool, WKRS485EN_A>);
impl WKRS485EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKRS485EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKRS485EN_A {
        match self.bits {
            false => WKRS485EN_A::_0,
            true => WKRS485EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKRS485EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKRS485EN_A::_1
    }
}
impl core::ops::Deref for WKRS485EN_R {
    type Target = crate::FieldReader<bool, WKRS485EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKRS485EN` writer - RS-485 Address Match (AAD Mode) Wake-up Enable Bit\nNote 1: When the system is in.Power-down mode, RS-485 Address Match will wake-up system from Power-down mode.\nNote 2: This bit is used for RS-485 Auto Address Detection (AAD) mode in RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct WKRS485EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKRS485EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKRS485EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RS-485 Address Match (AAD mode) wake-up system function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKRS485EN_A::_0)
    }
    #[doc = "RS-485 Address Match (AAD mode) wake-up system function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKRS485EN_A::_1)
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
#[doc = "Received Data FIFO Reached Threshold Time-out Wake-up Enable Bit\nNote 1: When the system is in Power-down mode, Received Data FIFO reached threshold time-out will wake up system from Power-down mode.\nNote 2: It is suggested the function is enabled when the WKRFRTEN (UART_WKCTL\\[2\\]) is set to 1.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKTOUTEN_A {
    #[doc = "0: Received Data FIFO reached threshold time-out wake-up system function Disabled"]
    _0 = 0,
    #[doc = "1: Received Data FIFO reached threshold time-out wake-up system function Enabled"]
    _1 = 1,
}
impl From<WKTOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKTOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKTOUTEN` reader - Received Data FIFO Reached Threshold Time-out Wake-up Enable Bit\nNote 1: When the system is in Power-down mode, Received Data FIFO reached threshold time-out will wake up system from Power-down mode.\nNote 2: It is suggested the function is enabled when the WKRFRTEN (UART_WKCTL\\[2\\]) is set to 1.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct WKTOUTEN_R(crate::FieldReader<bool, WKTOUTEN_A>);
impl WKTOUTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKTOUTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKTOUTEN_A {
        match self.bits {
            false => WKTOUTEN_A::_0,
            true => WKTOUTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKTOUTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKTOUTEN_A::_1
    }
}
impl core::ops::Deref for WKTOUTEN_R {
    type Target = crate::FieldReader<bool, WKTOUTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKTOUTEN` writer - Received Data FIFO Reached Threshold Time-out Wake-up Enable Bit\nNote 1: When the system is in Power-down mode, Received Data FIFO reached threshold time-out will wake up system from Power-down mode.\nNote 2: It is suggested the function is enabled when the WKRFRTEN (UART_WKCTL\\[2\\]) is set to 1.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct WKTOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKTOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKTOUTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Received Data FIFO reached threshold time-out wake-up system function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKTOUTEN_A::_0)
    }
    #[doc = "Received Data FIFO reached threshold time-out wake-up system function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKTOUTEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - nCTS Wake-up Enable Bit Note:When the system is in Power-down mode, an external.nCTS change will wake up system from Power-down mode."]
    #[inline(always)]
    pub fn wkctsen(&self) -> WKCTSEN_R {
        WKCTSEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Incoming Data Wake-up Enable Bit Note:When the system is in Power-down mode, incoming data will wake-up system from Power-down mode."]
    #[inline(always)]
    pub fn wkdaten(&self) -> WKDATEN_R {
        WKDATEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Received Data FIFO Reached Threshold Wake-up Enable Bit Note: When the system is in Power-down mode, Received Data FIFO reached threshold will wake-up system from Power-down mode. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn wkrfrten(&self) -> WKRFRTEN_R {
        WKRFRTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RS-485 Address Match (AAD Mode) Wake-up Enable Bit Note 1: When the system is in.Power-down mode, RS-485 Address Match will wake-up system from Power-down mode. Note 2: This bit is used for RS-485 Auto Address Detection (AAD) mode in RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn wkrs485en(&self) -> WKRS485EN_R {
        WKRS485EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Received Data FIFO Reached Threshold Time-out Wake-up Enable Bit Note 1: When the system is in Power-down mode, Received Data FIFO reached threshold time-out will wake up system from Power-down mode. Note 2: It is suggested the function is enabled when the WKRFRTEN (UART_WKCTL\\[2\\]) is set to 1. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn wktouten(&self) -> WKTOUTEN_R {
        WKTOUTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nCTS Wake-up Enable Bit Note:When the system is in Power-down mode, an external.nCTS change will wake up system from Power-down mode."]
    #[inline(always)]
    pub fn wkctsen(&mut self) -> WKCTSEN_W {
        WKCTSEN_W { w: self }
    }
    #[doc = "Bit 1 - Incoming Data Wake-up Enable Bit Note:When the system is in Power-down mode, incoming data will wake-up system from Power-down mode."]
    #[inline(always)]
    pub fn wkdaten(&mut self) -> WKDATEN_W {
        WKDATEN_W { w: self }
    }
    #[doc = "Bit 2 - Received Data FIFO Reached Threshold Wake-up Enable Bit Note: When the system is in Power-down mode, Received Data FIFO reached threshold will wake-up system from Power-down mode. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn wkrfrten(&mut self) -> WKRFRTEN_W {
        WKRFRTEN_W { w: self }
    }
    #[doc = "Bit 3 - RS-485 Address Match (AAD Mode) Wake-up Enable Bit Note 1: When the system is in.Power-down mode, RS-485 Address Match will wake-up system from Power-down mode. Note 2: This bit is used for RS-485 Auto Address Detection (AAD) mode in RS-485 function mode and ADDRDEN (UART_ALTCTL\\[15\\]) is set to 1. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn wkrs485en(&mut self) -> WKRS485EN_W {
        WKRS485EN_W { w: self }
    }
    #[doc = "Bit 4 - Received Data FIFO Reached Threshold Time-out Wake-up Enable Bit Note 1: When the system is in Power-down mode, Received Data FIFO reached threshold time-out will wake up system from Power-down mode. Note 2: It is suggested the function is enabled when the WKRFRTEN (UART_WKCTL\\[2\\]) is set to 1. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn wktouten(&mut self) -> WKTOUTEN_W {
        WKTOUTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Wake-up Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_wkctl](index.html) module"]
pub struct UART_WKCTL_SPEC;
impl crate::RegisterSpec for UART_WKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_wkctl::R](R) reader structure"]
impl crate::Readable for UART_WKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_wkctl::W](W) writer structure"]
impl crate::Writable for UART_WKCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_WKCTL to value 0"]
impl crate::Resettable for UART_WKCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
