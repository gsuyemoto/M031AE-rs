#[doc = "Register `UUART_PDMACTL` reader"]
pub struct R(crate::R<UUART_PDMACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_PDMACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_PDMACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_PDMACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUART_PDMACTL` writer"]
pub struct W(crate::W<UUART_PDMACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUART_PDMACTL_SPEC>;
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
impl From<crate::W<UUART_PDMACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUART_PDMACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMARST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the USCI's PDMA control logic. This bit will be cleared to 0 automatically"]
    _1 = 1,
}
impl From<PDMARST_A> for bool {
    #[inline(always)]
    fn from(variant: PDMARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMARST` reader - PDMA Reset"]
pub struct PDMARST_R(crate::FieldReader<bool, PDMARST_A>);
impl PDMARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMARST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMARST_A {
        match self.bits {
            false => PDMARST_A::_0,
            true => PDMARST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDMARST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDMARST_A::_1
    }
}
impl core::ops::Deref for PDMARST_R {
    type Target = crate::FieldReader<bool, PDMARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMARST` writer - PDMA Reset"]
pub struct PDMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDMARST_A::_0)
    }
    #[doc = "Reset the USCI's PDMA control logic. This bit will be cleared to 0 automatically"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDMARST_A::_1)
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
#[doc = "PDMA Transmit Channel Available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPDMAEN_A {
    #[doc = "0: Transmit PDMA function Disabled"]
    _0 = 0,
    #[doc = "1: Transmit PDMA function Enabled"]
    _1 = 1,
}
impl From<TXPDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXPDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPDMAEN` reader - PDMA Transmit Channel Available"]
pub struct TXPDMAEN_R(crate::FieldReader<bool, TXPDMAEN_A>);
impl TXPDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPDMAEN_A {
        match self.bits {
            false => TXPDMAEN_A::_0,
            true => TXPDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXPDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXPDMAEN_A::_1
    }
}
impl core::ops::Deref for TXPDMAEN_R {
    type Target = crate::FieldReader<bool, TXPDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPDMAEN` writer - PDMA Transmit Channel Available"]
pub struct TXPDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit PDMA function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXPDMAEN_A::_0)
    }
    #[doc = "Transmit PDMA function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXPDMAEN_A::_1)
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
#[doc = "PDMA Receive Channel Available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPDMAEN_A {
    #[doc = "0: Receive PDMA function Disabled"]
    _0 = 0,
    #[doc = "1: Receive PDMA function Enabled"]
    _1 = 1,
}
impl From<RXPDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXPDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPDMAEN` reader - PDMA Receive Channel Available"]
pub struct RXPDMAEN_R(crate::FieldReader<bool, RXPDMAEN_A>);
impl RXPDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXPDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPDMAEN_A {
        match self.bits {
            false => RXPDMAEN_A::_0,
            true => RXPDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXPDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXPDMAEN_A::_1
    }
}
impl core::ops::Deref for RXPDMAEN_R {
    type Target = crate::FieldReader<bool, RXPDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPDMAEN` writer - PDMA Receive Channel Available"]
pub struct RXPDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive PDMA function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXPDMAEN_A::_0)
    }
    #[doc = "Receive PDMA function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXPDMAEN_A::_1)
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
#[doc = "PDMA Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMAEN_A {
    #[doc = "0: PDMA function Disabled"]
    _0 = 0,
    #[doc = "1: PDMA function Enabled"]
    _1 = 1,
}
impl From<PDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMAEN` reader - PDMA Mode Enable Bit"]
pub struct PDMAEN_R(crate::FieldReader<bool, PDMAEN_A>);
impl PDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMAEN_A {
        match self.bits {
            false => PDMAEN_A::_0,
            true => PDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDMAEN_A::_1
    }
}
impl core::ops::Deref for PDMAEN_R {
    type Target = crate::FieldReader<bool, PDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMAEN` writer - PDMA Mode Enable Bit"]
pub struct PDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDMAEN_A::_0)
    }
    #[doc = "PDMA function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDMAEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - PDMA Reset"]
    #[inline(always)]
    pub fn pdmarst(&self) -> PDMARST_R {
        PDMARST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDMA Transmit Channel Available"]
    #[inline(always)]
    pub fn txpdmaen(&self) -> TXPDMAEN_R {
        TXPDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDMA Receive Channel Available"]
    #[inline(always)]
    pub fn rxpdmaen(&self) -> RXPDMAEN_R {
        RXPDMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDMA Mode Enable Bit"]
    #[inline(always)]
    pub fn pdmaen(&self) -> PDMAEN_R {
        PDMAEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Reset"]
    #[inline(always)]
    pub fn pdmarst(&mut self) -> PDMARST_W {
        PDMARST_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Transmit Channel Available"]
    #[inline(always)]
    pub fn txpdmaen(&mut self) -> TXPDMAEN_W {
        TXPDMAEN_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Receive Channel Available"]
    #[inline(always)]
    pub fn rxpdmaen(&mut self) -> RXPDMAEN_W {
        RXPDMAEN_W { w: self }
    }
    #[doc = "Bit 3 - PDMA Mode Enable Bit"]
    #[inline(always)]
    pub fn pdmaen(&mut self) -> PDMAEN_W {
        PDMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI PDMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_pdmactl](index.html) module"]
pub struct UUART_PDMACTL_SPEC;
impl crate::RegisterSpec for UUART_PDMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_pdmactl::R](R) reader structure"]
impl crate::Readable for UUART_PDMACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuart_pdmactl::W](W) writer structure"]
impl crate::Writable for UUART_PDMACTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UUART_PDMACTL to value 0"]
impl crate::Resettable for UUART_PDMACTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
