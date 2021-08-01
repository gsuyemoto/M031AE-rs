#[doc = "Register `UART_WKSTS` reader"]
pub struct R(crate::R<UART_WKSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_WKSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_WKSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_WKSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_WKSTS` writer"]
pub struct W(crate::W<UART_WKSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_WKSTS_SPEC>;
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
impl From<crate::W<UART_WKSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_WKSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "nCTS Wake-up Flag\nThis bit is set if chip wake-up from power-down state by nCTS wake-up.\nNote 1: If WKCTSEN (UART_WKCTL\\[0\\]) is enabled, the nCTS wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSWKF_A {
    #[doc = "0: Chip stays in power-down state"]
    _0 = 0,
    #[doc = "1: Chip wake-up from power-down state by nCTS wake-up"]
    _1 = 1,
}
impl From<CTSWKF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSWKF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSWKF` reader - nCTS Wake-up Flag\nThis bit is set if chip wake-up from power-down state by nCTS wake-up.\nNote 1: If WKCTSEN (UART_WKCTL\\[0\\]) is enabled, the nCTS wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it."]
pub struct CTSWKF_R(crate::FieldReader<bool, CTSWKF_A>);
impl CTSWKF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSWKF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSWKF_A {
        match self.bits {
            false => CTSWKF_A::_0,
            true => CTSWKF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTSWKF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTSWKF_A::_1
    }
}
impl core::ops::Deref for CTSWKF_R {
    type Target = crate::FieldReader<bool, CTSWKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSWKF` writer - nCTS Wake-up Flag\nThis bit is set if chip wake-up from power-down state by nCTS wake-up.\nNote 1: If WKCTSEN (UART_WKCTL\\[0\\]) is enabled, the nCTS wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it."]
pub struct CTSWKF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSWKF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSWKF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip stays in power-down state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSWKF_A::_0)
    }
    #[doc = "Chip wake-up from power-down state by nCTS wake-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSWKF_A::_1)
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
#[doc = "Incoming Data Wake-up Flag\nThis bit is set if chip wake-up from power-down state by data wake-up.\nNote 1: If WKDATEN (UART_WKCTL\\[1\\]) is enabled, the Incoming Data wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATWKF_A {
    #[doc = "0: Chip stays in power-down state"]
    _0 = 0,
    #[doc = "1: Chip wake-up from power-down state by Incoming Data wake-up"]
    _1 = 1,
}
impl From<DATWKF_A> for bool {
    #[inline(always)]
    fn from(variant: DATWKF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATWKF` reader - Incoming Data Wake-up Flag\nThis bit is set if chip wake-up from power-down state by data wake-up.\nNote 1: If WKDATEN (UART_WKCTL\\[1\\]) is enabled, the Incoming Data wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it."]
pub struct DATWKF_R(crate::FieldReader<bool, DATWKF_A>);
impl DATWKF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATWKF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATWKF_A {
        match self.bits {
            false => DATWKF_A::_0,
            true => DATWKF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATWKF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATWKF_A::_1
    }
}
impl core::ops::Deref for DATWKF_R {
    type Target = crate::FieldReader<bool, DATWKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATWKF` writer - Incoming Data Wake-up Flag\nThis bit is set if chip wake-up from power-down state by data wake-up.\nNote 1: If WKDATEN (UART_WKCTL\\[1\\]) is enabled, the Incoming Data wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it."]
pub struct DATWKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DATWKF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATWKF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip stays in power-down state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATWKF_A::_0)
    }
    #[doc = "Chip wake-up from power-down state by Incoming Data wake-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATWKF_A::_1)
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
#[doc = "Received Data FIFO Reached Threshold Wake-up Flag\nThis bit is set if chip wake-up from power-down state by Received Data FIFO reached threshold\nwake-up .\nNote 1: If WKRFRTEN (UART_WKCTL\\[2\\]) is enabled, the Received Data FIFO Reached Threshold wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFRTWKF_A {
    #[doc = "0: Chip stays in power-down state"]
    _0 = 0,
    #[doc = "1: Chip wake-up from power-down state by Received Data FIFO Reached Threshold wake-up"]
    _1 = 1,
}
impl From<RFRTWKF_A> for bool {
    #[inline(always)]
    fn from(variant: RFRTWKF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFRTWKF` reader - Received Data FIFO Reached Threshold Wake-up Flag\nThis bit is set if chip wake-up from power-down state by Received Data FIFO reached threshold\nwake-up .\nNote 1: If WKRFRTEN (UART_WKCTL\\[2\\]) is enabled, the Received Data FIFO Reached Threshold wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct RFRTWKF_R(crate::FieldReader<bool, RFRTWKF_A>);
impl RFRTWKF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFRTWKF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRTWKF_A {
        match self.bits {
            false => RFRTWKF_A::_0,
            true => RFRTWKF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RFRTWKF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RFRTWKF_A::_1
    }
}
impl core::ops::Deref for RFRTWKF_R {
    type Target = crate::FieldReader<bool, RFRTWKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFRTWKF` writer - Received Data FIFO Reached Threshold Wake-up Flag\nThis bit is set if chip wake-up from power-down state by Received Data FIFO reached threshold\nwake-up .\nNote 1: If WKRFRTEN (UART_WKCTL\\[2\\]) is enabled, the Received Data FIFO Reached Threshold wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct RFRTWKF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRTWKF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFRTWKF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip stays in power-down state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFRTWKF_A::_0)
    }
    #[doc = "Chip wake-up from power-down state by Received Data FIFO Reached Threshold wake-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFRTWKF_A::_1)
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
#[doc = "RS-485 Address Match (AAD Mode) Wake-up Flag\nThis bit is set if chip wake-up from power-down state by RS-485 Address Match (AAD mode).\nNote 1: If WKRS485EN (UART_WKCTL\\[3\\]) is enabled, the RS-485 Address Match (AAD mode) wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RS485WKF_A {
    #[doc = "0: Chip stays in power-down state"]
    _0 = 0,
    #[doc = "1: Chip wake-up from power-down state by RS-485 Address Match (AAD mode) wake-up"]
    _1 = 1,
}
impl From<RS485WKF_A> for bool {
    #[inline(always)]
    fn from(variant: RS485WKF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS485WKF` reader - RS-485 Address Match (AAD Mode) Wake-up Flag\nThis bit is set if chip wake-up from power-down state by RS-485 Address Match (AAD mode).\nNote 1: If WKRS485EN (UART_WKCTL\\[3\\]) is enabled, the RS-485 Address Match (AAD mode) wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct RS485WKF_R(crate::FieldReader<bool, RS485WKF_A>);
impl RS485WKF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485WKF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS485WKF_A {
        match self.bits {
            false => RS485WKF_A::_0,
            true => RS485WKF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RS485WKF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RS485WKF_A::_1
    }
}
impl core::ops::Deref for RS485WKF_R {
    type Target = crate::FieldReader<bool, RS485WKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485WKF` writer - RS-485 Address Match (AAD Mode) Wake-up Flag\nThis bit is set if chip wake-up from power-down state by RS-485 Address Match (AAD mode).\nNote 1: If WKRS485EN (UART_WKCTL\\[3\\]) is enabled, the RS-485 Address Match (AAD mode) wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct RS485WKF_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485WKF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RS485WKF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip stays in power-down state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RS485WKF_A::_0)
    }
    #[doc = "Chip wake-up from power-down state by RS-485 Address Match (AAD mode) wake-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RS485WKF_A::_1)
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
#[doc = "Received Data FIFO Threshold Time-out Wake-up Flag\nThis bit is set if chip wake-up from power-down state by Received Data FIFO Threshold Time-out\nwake-up.\nNote 1: If WKTOUTEN (UART_WKCTL\\[4\\]) is enabled, the Received Data FIFO reached threshold time-out wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUTWKF_A {
    #[doc = "0: Chip stays in power-down state"]
    _0 = 0,
    #[doc = "1: Chip wake-up from power-down state by Received Data FIFO reached threshold time-out"]
    _1 = 1,
}
impl From<TOUTWKF_A> for bool {
    #[inline(always)]
    fn from(variant: TOUTWKF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOUTWKF` reader - Received Data FIFO Threshold Time-out Wake-up Flag\nThis bit is set if chip wake-up from power-down state by Received Data FIFO Threshold Time-out\nwake-up.\nNote 1: If WKTOUTEN (UART_WKCTL\\[4\\]) is enabled, the Received Data FIFO reached threshold time-out wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct TOUTWKF_R(crate::FieldReader<bool, TOUTWKF_A>);
impl TOUTWKF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUTWKF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTWKF_A {
        match self.bits {
            false => TOUTWKF_A::_0,
            true => TOUTWKF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTWKF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTWKF_A::_1
    }
}
impl core::ops::Deref for TOUTWKF_R {
    type Target = crate::FieldReader<bool, TOUTWKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTWKF` writer - Received Data FIFO Threshold Time-out Wake-up Flag\nThis bit is set if chip wake-up from power-down state by Received Data FIFO Threshold Time-out\nwake-up.\nNote 1: If WKTOUTEN (UART_WKCTL\\[4\\]) is enabled, the Received Data FIFO reached threshold time-out wake-up cause this bit is set to '1'.\nNote 2: This bit can be cleared by writing '1' to it.\nNote 3: This bit is valid in UART0, UART1, UART4 and UART5."]
pub struct TOUTWKF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTWKF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTWKF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip stays in power-down state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTWKF_A::_0)
    }
    #[doc = "Chip wake-up from power-down state by Received Data FIFO reached threshold time-out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTWKF_A::_1)
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
    #[doc = "Bit 0 - nCTS Wake-up Flag This bit is set if chip wake-up from power-down state by nCTS wake-up. Note 1: If WKCTSEN (UART_WKCTL\\[0\\]) is enabled, the nCTS wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn ctswkf(&self) -> CTSWKF_R {
        CTSWKF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Incoming Data Wake-up Flag This bit is set if chip wake-up from power-down state by data wake-up. Note 1: If WKDATEN (UART_WKCTL\\[1\\]) is enabled, the Incoming Data wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn datwkf(&self) -> DATWKF_R {
        DATWKF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Received Data FIFO Reached Threshold Wake-up Flag This bit is set if chip wake-up from power-down state by Received Data FIFO reached threshold wake-up . Note 1: If WKRFRTEN (UART_WKCTL\\[2\\]) is enabled, the Received Data FIFO Reached Threshold wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn rfrtwkf(&self) -> RFRTWKF_R {
        RFRTWKF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RS-485 Address Match (AAD Mode) Wake-up Flag This bit is set if chip wake-up from power-down state by RS-485 Address Match (AAD mode). Note 1: If WKRS485EN (UART_WKCTL\\[3\\]) is enabled, the RS-485 Address Match (AAD mode) wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn rs485wkf(&self) -> RS485WKF_R {
        RS485WKF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Received Data FIFO Threshold Time-out Wake-up Flag This bit is set if chip wake-up from power-down state by Received Data FIFO Threshold Time-out wake-up. Note 1: If WKTOUTEN (UART_WKCTL\\[4\\]) is enabled, the Received Data FIFO reached threshold time-out wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn toutwkf(&self) -> TOUTWKF_R {
        TOUTWKF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nCTS Wake-up Flag This bit is set if chip wake-up from power-down state by nCTS wake-up. Note 1: If WKCTSEN (UART_WKCTL\\[0\\]) is enabled, the nCTS wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn ctswkf(&mut self) -> CTSWKF_W {
        CTSWKF_W { w: self }
    }
    #[doc = "Bit 1 - Incoming Data Wake-up Flag This bit is set if chip wake-up from power-down state by data wake-up. Note 1: If WKDATEN (UART_WKCTL\\[1\\]) is enabled, the Incoming Data wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn datwkf(&mut self) -> DATWKF_W {
        DATWKF_W { w: self }
    }
    #[doc = "Bit 2 - Received Data FIFO Reached Threshold Wake-up Flag This bit is set if chip wake-up from power-down state by Received Data FIFO reached threshold wake-up . Note 1: If WKRFRTEN (UART_WKCTL\\[2\\]) is enabled, the Received Data FIFO Reached Threshold wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn rfrtwkf(&mut self) -> RFRTWKF_W {
        RFRTWKF_W { w: self }
    }
    #[doc = "Bit 3 - RS-485 Address Match (AAD Mode) Wake-up Flag This bit is set if chip wake-up from power-down state by RS-485 Address Match (AAD mode). Note 1: If WKRS485EN (UART_WKCTL\\[3\\]) is enabled, the RS-485 Address Match (AAD mode) wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn rs485wkf(&mut self) -> RS485WKF_W {
        RS485WKF_W { w: self }
    }
    #[doc = "Bit 4 - Received Data FIFO Threshold Time-out Wake-up Flag This bit is set if chip wake-up from power-down state by Received Data FIFO Threshold Time-out wake-up. Note 1: If WKTOUTEN (UART_WKCTL\\[4\\]) is enabled, the Received Data FIFO reached threshold time-out wake-up cause this bit is set to '1'. Note 2: This bit can be cleared by writing '1' to it. Note 3: This bit is valid in UART0, UART1, UART4 and UART5."]
    #[inline(always)]
    pub fn toutwkf(&mut self) -> TOUTWKF_W {
        TOUTWKF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Wake-up Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_wksts](index.html) module"]
pub struct UART_WKSTS_SPEC;
impl crate::RegisterSpec for UART_WKSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_wksts::R](R) reader structure"]
impl crate::Readable for UART_WKSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_wksts::W](W) writer structure"]
impl crate::Writable for UART_WKSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_WKSTS to value 0"]
impl crate::Resettable for UART_WKSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
