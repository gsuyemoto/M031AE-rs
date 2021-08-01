#[doc = "Register `UUART_INTEN` reader"]
pub struct R(crate::R<UUART_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUART_INTEN` writer"]
pub struct W(crate::W<UUART_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUART_INTEN_SPEC>;
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
impl From<crate::W<UUART_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUART_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Start Interrupt Enable Bit\nThis bit enables the interrupt generation in case of a transmit start event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTIEN_A {
    #[doc = "0: The transmit start interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The transmit start interrupt Enabled"]
    _1 = 1,
}
impl From<TXSTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTIEN` reader - Transmit Start Interrupt Enable Bit\nThis bit enables the interrupt generation in case of a transmit start event."]
pub struct TXSTIEN_R(crate::FieldReader<bool, TXSTIEN_A>);
impl TXSTIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTIEN_A {
        match self.bits {
            false => TXSTIEN_A::_0,
            true => TXSTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXSTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXSTIEN_A::_1
    }
}
impl core::ops::Deref for TXSTIEN_R {
    type Target = crate::FieldReader<bool, TXSTIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTIEN` writer - Transmit Start Interrupt Enable Bit\nThis bit enables the interrupt generation in case of a transmit start event."]
pub struct TXSTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSTIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transmit start interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXSTIEN_A::_0)
    }
    #[doc = "The transmit start interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXSTIEN_A::_1)
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
#[doc = "Transmit End Interrupt Enable Bit\nThis bit enables the interrupt generation in case of a transmit finish event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDIEN_A {
    #[doc = "0: The transmit finish interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The transmit finish interrupt Enabled"]
    _1 = 1,
}
impl From<TXENDIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXENDIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXENDIEN` reader - Transmit End Interrupt Enable Bit\nThis bit enables the interrupt generation in case of a transmit finish event."]
pub struct TXENDIEN_R(crate::FieldReader<bool, TXENDIEN_A>);
impl TXENDIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXENDIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENDIEN_A {
        match self.bits {
            false => TXENDIEN_A::_0,
            true => TXENDIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXENDIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXENDIEN_A::_1
    }
}
impl core::ops::Deref for TXENDIEN_R {
    type Target = crate::FieldReader<bool, TXENDIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXENDIEN` writer - Transmit End Interrupt Enable Bit\nThis bit enables the interrupt generation in case of a transmit finish event."]
pub struct TXENDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXENDIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transmit finish interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXENDIEN_A::_0)
    }
    #[doc = "The transmit finish interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXENDIEN_A::_1)
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
#[doc = "Receive Start Interrupt Enable BIt\nThis bit enables the interrupt generation in case of a receive start event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTIEN_A {
    #[doc = "0: The receive start interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The receive start interrupt Enabled"]
    _1 = 1,
}
impl From<RXSTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTIEN` reader - Receive Start Interrupt Enable BIt\nThis bit enables the interrupt generation in case of a receive start event."]
pub struct RXSTIEN_R(crate::FieldReader<bool, RXSTIEN_A>);
impl RXSTIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTIEN_A {
        match self.bits {
            false => RXSTIEN_A::_0,
            true => RXSTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXSTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXSTIEN_A::_1
    }
}
impl core::ops::Deref for RXSTIEN_R {
    type Target = crate::FieldReader<bool, RXSTIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTIEN` writer - Receive Start Interrupt Enable BIt\nThis bit enables the interrupt generation in case of a receive start event."]
pub struct RXSTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSTIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The receive start interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXSTIEN_A::_0)
    }
    #[doc = "The receive start interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXSTIEN_A::_1)
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
#[doc = "Receive End Interrupt Enable Bit\nThis bit enables the interrupt generation in case of a receive finish event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXENDIEN_A {
    #[doc = "0: The receive end interrupt Disabled"]
    _0 = 0,
    #[doc = "1: The receive end interrupt Enabled"]
    _1 = 1,
}
impl From<RXENDIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXENDIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXENDIEN` reader - Receive End Interrupt Enable Bit\nThis bit enables the interrupt generation in case of a receive finish event."]
pub struct RXENDIEN_R(crate::FieldReader<bool, RXENDIEN_A>);
impl RXENDIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXENDIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXENDIEN_A {
        match self.bits {
            false => RXENDIEN_A::_0,
            true => RXENDIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXENDIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXENDIEN_A::_1
    }
}
impl core::ops::Deref for RXENDIEN_R {
    type Target = crate::FieldReader<bool, RXENDIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXENDIEN` writer - Receive End Interrupt Enable Bit\nThis bit enables the interrupt generation in case of a receive finish event."]
pub struct RXENDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENDIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXENDIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The receive end interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXENDIEN_A::_0)
    }
    #[doc = "The receive end interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXENDIEN_A::_1)
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
    #[doc = "Bit 1 - Transmit Start Interrupt Enable Bit This bit enables the interrupt generation in case of a transmit start event."]
    #[inline(always)]
    pub fn txstien(&self) -> TXSTIEN_R {
        TXSTIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit End Interrupt Enable Bit This bit enables the interrupt generation in case of a transmit finish event."]
    #[inline(always)]
    pub fn txendien(&self) -> TXENDIEN_R {
        TXENDIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Start Interrupt Enable BIt This bit enables the interrupt generation in case of a receive start event."]
    #[inline(always)]
    pub fn rxstien(&self) -> RXSTIEN_R {
        RXSTIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive End Interrupt Enable Bit This bit enables the interrupt generation in case of a receive finish event."]
    #[inline(always)]
    pub fn rxendien(&self) -> RXENDIEN_R {
        RXENDIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit Start Interrupt Enable Bit This bit enables the interrupt generation in case of a transmit start event."]
    #[inline(always)]
    pub fn txstien(&mut self) -> TXSTIEN_W {
        TXSTIEN_W { w: self }
    }
    #[doc = "Bit 2 - Transmit End Interrupt Enable Bit This bit enables the interrupt generation in case of a transmit finish event."]
    #[inline(always)]
    pub fn txendien(&mut self) -> TXENDIEN_W {
        TXENDIEN_W { w: self }
    }
    #[doc = "Bit 3 - Receive Start Interrupt Enable BIt This bit enables the interrupt generation in case of a receive start event."]
    #[inline(always)]
    pub fn rxstien(&mut self) -> RXSTIEN_W {
        RXSTIEN_W { w: self }
    }
    #[doc = "Bit 4 - Receive End Interrupt Enable Bit This bit enables the interrupt generation in case of a receive finish event."]
    #[inline(always)]
    pub fn rxendien(&mut self) -> RXENDIEN_W {
        RXENDIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_inten](index.html) module"]
pub struct UUART_INTEN_SPEC;
impl crate::RegisterSpec for UUART_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_inten::R](R) reader structure"]
impl crate::Readable for UUART_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuart_inten::W](W) writer structure"]
impl crate::Writable for UUART_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UUART_INTEN to value 0"]
impl crate::Resettable for UUART_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
