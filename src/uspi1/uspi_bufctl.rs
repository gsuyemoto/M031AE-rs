#[doc = "Register `USPI_BUFCTL` reader"]
pub struct R(crate::R<USPI_BUFCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USPI_BUFCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USPI_BUFCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USPI_BUFCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USPI_BUFCTL` writer"]
pub struct W(crate::W<USPI_BUFCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USPI_BUFCTL_SPEC>;
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
impl From<crate::W<USPI_BUFCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USPI_BUFCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Transmit Under Run Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUDRIEN_A {
    #[doc = "0: Transmit under-run interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Transmit under-run interrupt Enabled"]
    _1 = 1,
}
impl From<TXUDRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXUDRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUDRIEN` reader - Slave Transmit Under Run Interrupt Enable Bit"]
pub struct TXUDRIEN_R(crate::FieldReader<bool, TXUDRIEN_A>);
impl TXUDRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUDRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUDRIEN_A {
        match self.bits {
            false => TXUDRIEN_A::_0,
            true => TXUDRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXUDRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXUDRIEN_A::_1
    }
}
impl core::ops::Deref for TXUDRIEN_R {
    type Target = crate::FieldReader<bool, TXUDRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUDRIEN` writer - Slave Transmit Under Run Interrupt Enable Bit"]
pub struct TXUDRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUDRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit under-run interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXUDRIEN_A::_0)
    }
    #[doc = "Transmit under-run interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXUDRIEN_A::_1)
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
#[doc = "Clear Transmit Buffer \nNote: It is cleared automatically after one PCLK cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: The transmit buffer is cleared. Should only be used while the buffer is not taking part in data traffic"]
    _1 = 1,
}
impl From<TXCLR_A> for bool {
    #[inline(always)]
    fn from(variant: TXCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXCLR` reader - Clear Transmit Buffer \nNote: It is cleared automatically after one PCLK cycle."]
pub struct TXCLR_R(crate::FieldReader<bool, TXCLR_A>);
impl TXCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCLR_A {
        match self.bits {
            false => TXCLR_A::_0,
            true => TXCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXCLR_A::_1
    }
}
impl core::ops::Deref for TXCLR_R {
    type Target = crate::FieldReader<bool, TXCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCLR` writer - Clear Transmit Buffer \nNote: It is cleared automatically after one PCLK cycle."]
pub struct TXCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXCLR_A::_0)
    }
    #[doc = "The transmit buffer is cleared. Should only be used while the buffer is not taking part in data traffic"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXCLR_A::_1)
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
#[doc = "Receive Buffer Overrun Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVIEN_A {
    #[doc = "0: Receive overrun interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Receive overrun interrupt Enabled"]
    _1 = 1,
}
impl From<RXOVIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVIEN` reader - Receive Buffer Overrun Interrupt Enable Bit"]
pub struct RXOVIEN_R(crate::FieldReader<bool, RXOVIEN_A>);
impl RXOVIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVIEN_A {
        match self.bits {
            false => RXOVIEN_A::_0,
            true => RXOVIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXOVIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXOVIEN_A::_1
    }
}
impl core::ops::Deref for RXOVIEN_R {
    type Target = crate::FieldReader<bool, RXOVIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVIEN` writer - Receive Buffer Overrun Interrupt Enable Bit"]
pub struct RXOVIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOVIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive overrun interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOVIEN_A::_0)
    }
    #[doc = "Receive overrun interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOVIEN_A::_1)
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
#[doc = "Clear Receive Buffer\nNote: It is cleared automatically after one PCLK cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: The receive buffer is cleared. Should only be used while the buffer is not taking part in data traffic"]
    _1 = 1,
}
impl From<RXCLR_A> for bool {
    #[inline(always)]
    fn from(variant: RXCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXCLR` reader - Clear Receive Buffer\nNote: It is cleared automatically after one PCLK cycle."]
pub struct RXCLR_R(crate::FieldReader<bool, RXCLR_A>);
impl RXCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXCLR_A {
        match self.bits {
            false => RXCLR_A::_0,
            true => RXCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXCLR_A::_1
    }
}
impl core::ops::Deref for RXCLR_R {
    type Target = crate::FieldReader<bool, RXCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCLR` writer - Clear Receive Buffer\nNote: It is cleared automatically after one PCLK cycle."]
pub struct RXCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXCLR_A::_0)
    }
    #[doc = "The receive buffer is cleared. Should only be used while the buffer is not taking part in data traffic"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXCLR_A::_1)
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
#[doc = "Transmit Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the transmit-related counters, state machine, and the content of transmit shift register and data buffer"]
    _1 = 1,
}
impl From<TXRST_A> for bool {
    #[inline(always)]
    fn from(variant: TXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRST` reader - Transmit Reset"]
pub struct TXRST_R(crate::FieldReader<bool, TXRST_A>);
impl TXRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRST_A {
        match self.bits {
            false => TXRST_A::_0,
            true => TXRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRST_A::_1
    }
}
impl core::ops::Deref for TXRST_R {
    type Target = crate::FieldReader<bool, TXRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRST` writer - Transmit Reset"]
pub struct TXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRST_A::_0)
    }
    #[doc = "Reset the transmit-related counters, state machine, and the content of transmit shift register and data buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRST_A::_1)
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
#[doc = "Receive Reset\nNote: It is cleared automatically after one PCLK cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the receive-related counters, state machine, and the content of receive shift register and data buffer"]
    _1 = 1,
}
impl From<RXRST_A> for bool {
    #[inline(always)]
    fn from(variant: RXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRST` reader - Receive Reset\nNote: It is cleared automatically after one PCLK cycle."]
pub struct RXRST_R(crate::FieldReader<bool, RXRST_A>);
impl RXRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRST_A {
        match self.bits {
            false => RXRST_A::_0,
            true => RXRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXRST_A::_1
    }
}
impl core::ops::Deref for RXRST_R {
    type Target = crate::FieldReader<bool, RXRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRST` writer - Receive Reset\nNote: It is cleared automatically after one PCLK cycle."]
pub struct RXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRST_A::_0)
    }
    #[doc = "Reset the receive-related counters, state machine, and the content of receive shift register and data buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRST_A::_1)
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
impl R {
    #[doc = "Bit 6 - Slave Transmit Under Run Interrupt Enable Bit"]
    #[inline(always)]
    pub fn txudrien(&self) -> TXUDRIEN_R {
        TXUDRIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clear Transmit Buffer Note: It is cleared automatically after one PCLK cycle."]
    #[inline(always)]
    pub fn txclr(&self) -> TXCLR_R {
        TXCLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Buffer Overrun Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxovien(&self) -> RXOVIEN_R {
        RXOVIEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clear Receive Buffer Note: It is cleared automatically after one PCLK cycle."]
    #[inline(always)]
    pub fn rxclr(&self) -> RXCLR_R {
        RXCLR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit Reset"]
    #[inline(always)]
    pub fn txrst(&self) -> TXRST_R {
        TXRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive Reset Note: It is cleared automatically after one PCLK cycle."]
    #[inline(always)]
    pub fn rxrst(&self) -> RXRST_R {
        RXRST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Slave Transmit Under Run Interrupt Enable Bit"]
    #[inline(always)]
    pub fn txudrien(&mut self) -> TXUDRIEN_W {
        TXUDRIEN_W { w: self }
    }
    #[doc = "Bit 7 - Clear Transmit Buffer Note: It is cleared automatically after one PCLK cycle."]
    #[inline(always)]
    pub fn txclr(&mut self) -> TXCLR_W {
        TXCLR_W { w: self }
    }
    #[doc = "Bit 14 - Receive Buffer Overrun Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxovien(&mut self) -> RXOVIEN_W {
        RXOVIEN_W { w: self }
    }
    #[doc = "Bit 15 - Clear Receive Buffer Note: It is cleared automatically after one PCLK cycle."]
    #[inline(always)]
    pub fn rxclr(&mut self) -> RXCLR_W {
        RXCLR_W { w: self }
    }
    #[doc = "Bit 16 - Transmit Reset"]
    #[inline(always)]
    pub fn txrst(&mut self) -> TXRST_W {
        TXRST_W { w: self }
    }
    #[doc = "Bit 17 - Receive Reset Note: It is cleared automatically after one PCLK cycle."]
    #[inline(always)]
    pub fn rxrst(&mut self) -> RXRST_W {
        RXRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Transmit/Receive Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uspi_bufctl](index.html) module"]
pub struct USPI_BUFCTL_SPEC;
impl crate::RegisterSpec for USPI_BUFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uspi_bufctl::R](R) reader structure"]
impl crate::Readable for USPI_BUFCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uspi_bufctl::W](W) writer structure"]
impl crate::Writable for USPI_BUFCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USPI_BUFCTL to value 0"]
impl crate::Resettable for USPI_BUFCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
