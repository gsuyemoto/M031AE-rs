#[doc = "Register `UUART_CTLIN0` reader"]
pub struct R(crate::R<UUART_CTLIN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_CTLIN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_CTLIN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_CTLIN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUART_CTLIN0` writer"]
pub struct W(crate::W<UUART_CTLIN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUART_CTLIN0_SPEC>;
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
impl From<crate::W<UUART_CTLIN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUART_CTLIN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input Synchronization Signal Selection\nThis bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCSEL_A {
    #[doc = "0: The un-synchronized signal can be taken as input for the data shift unit"]
    _0 = 0,
    #[doc = "1: The synchronized signal can be taken as input for the data shift unit"]
    _1 = 1,
}
impl From<SYNCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCSEL` reader - Input Synchronization Signal Selection\nThis bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit."]
pub struct SYNCSEL_R(crate::FieldReader<bool, SYNCSEL_A>);
impl SYNCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCSEL_A {
        match self.bits {
            false => SYNCSEL_A::_0,
            true => SYNCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYNCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYNCSEL_A::_1
    }
}
impl core::ops::Deref for SYNCSEL_R {
    type Target = crate::FieldReader<bool, SYNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCSEL` writer - Input Synchronization Signal Selection\nThis bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit."]
pub struct SYNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The un-synchronized signal can be taken as input for the data shift unit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCSEL_A::_0)
    }
    #[doc = "The synchronized signal can be taken as input for the data shift unit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCSEL_A::_1)
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
#[doc = "Input Signal Inverse Selection\nThis bit defines the inverter enable of the input asynchronous signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ININV_A {
    #[doc = "0: The un-synchronized input signal will not be inverted"]
    _0 = 0,
    #[doc = "1: The un-synchronized input signal will be inverted"]
    _1 = 1,
}
impl From<ININV_A> for bool {
    #[inline(always)]
    fn from(variant: ININV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ININV` reader - Input Signal Inverse Selection\nThis bit defines the inverter enable of the input asynchronous signal."]
pub struct ININV_R(crate::FieldReader<bool, ININV_A>);
impl ININV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ININV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ININV_A {
        match self.bits {
            false => ININV_A::_0,
            true => ININV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ININV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ININV_A::_1
    }
}
impl core::ops::Deref for ININV_R {
    type Target = crate::FieldReader<bool, ININV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ININV` writer - Input Signal Inverse Selection\nThis bit defines the inverter enable of the input asynchronous signal."]
pub struct ININV_W<'a> {
    w: &'a mut W,
}
impl<'a> ININV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ININV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The un-synchronized input signal will not be inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ININV_A::_0)
    }
    #[doc = "The un-synchronized input signal will be inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ININV_A::_1)
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
impl R {
    #[doc = "Bit 0 - Input Synchronization Signal Selection This bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit."]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input Signal Inverse Selection This bit defines the inverter enable of the input asynchronous signal."]
    #[inline(always)]
    pub fn ininv(&self) -> ININV_R {
        ININV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Synchronization Signal Selection This bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit."]
    #[inline(always)]
    pub fn syncsel(&mut self) -> SYNCSEL_W {
        SYNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Input Signal Inverse Selection This bit defines the inverter enable of the input asynchronous signal."]
    #[inline(always)]
    pub fn ininv(&mut self) -> ININV_W {
        ININV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Input Control Signal Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_ctlin0](index.html) module"]
pub struct UUART_CTLIN0_SPEC;
impl crate::RegisterSpec for UUART_CTLIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_ctlin0::R](R) reader structure"]
impl crate::Readable for UUART_CTLIN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuart_ctlin0::W](W) writer structure"]
impl crate::Writable for UUART_CTLIN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UUART_CTLIN0 to value 0"]
impl crate::Resettable for UUART_CTLIN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
