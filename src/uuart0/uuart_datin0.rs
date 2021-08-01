#[doc = "Register `UUART_DATIN0` reader"]
pub struct R(crate::R<UUART_DATIN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUART_DATIN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUART_DATIN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUART_DATIN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUART_DATIN0` writer"]
pub struct W(crate::W<UUART_DATIN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUART_DATIN0_SPEC>;
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
impl From<crate::W<UUART_DATIN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUART_DATIN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input Signal Synchronization Selection\nThis bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit.\n\nValue on reset: 0"]
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
#[doc = "Field `SYNCSEL` reader - Input Signal Synchronization Selection\nThis bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit."]
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
#[doc = "Field `SYNCSEL` writer - Input Signal Synchronization Selection\nThis bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit."]
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
#[doc = "Input Signal Edge Detection Mode\nThis bit field selects which edge actives the trigger event of input data signal.\nNote: In UART function mode, it is suggested to set this bit field as 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGEDET_A {
    #[doc = "0: The trigger event activation is disabled"]
    _0 = 0,
    #[doc = "1: A rising edge activates the trigger event of input data signal"]
    _1 = 1,
    #[doc = "2: A falling edge activates the trigger event of input data signal"]
    _2 = 2,
    #[doc = "3: Both edges activate the trigger event of input data signal"]
    _3 = 3,
}
impl From<EDGEDET_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGEDET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EDGEDET` reader - Input Signal Edge Detection Mode\nThis bit field selects which edge actives the trigger event of input data signal.\nNote: In UART function mode, it is suggested to set this bit field as 10."]
pub struct EDGEDET_R(crate::FieldReader<u8, EDGEDET_A>);
impl EDGEDET_R {
    pub(crate) fn new(bits: u8) -> Self {
        EDGEDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGEDET_A {
        match self.bits {
            0 => EDGEDET_A::_0,
            1 => EDGEDET_A::_1,
            2 => EDGEDET_A::_2,
            3 => EDGEDET_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EDGEDET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EDGEDET_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EDGEDET_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == EDGEDET_A::_3
    }
}
impl core::ops::Deref for EDGEDET_R {
    type Target = crate::FieldReader<u8, EDGEDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGEDET` writer - Input Signal Edge Detection Mode\nThis bit field selects which edge actives the trigger event of input data signal.\nNote: In UART function mode, it is suggested to set this bit field as 10."]
pub struct EDGEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGEDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGEDET_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The trigger event activation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDGEDET_A::_0)
    }
    #[doc = "A rising edge activates the trigger event of input data signal"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDGEDET_A::_1)
    }
    #[doc = "A falling edge activates the trigger event of input data signal"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(EDGEDET_A::_2)
    }
    #[doc = "Both edges activate the trigger event of input data signal"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(EDGEDET_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Input Signal Synchronization Selection This bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit."]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input Signal Inverse Selection This bit defines the inverter enable of the input asynchronous signal."]
    #[inline(always)]
    pub fn ininv(&self) -> ININV_R {
        ININV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Input Signal Edge Detection Mode This bit field selects which edge actives the trigger event of input data signal. Note: In UART function mode, it is suggested to set this bit field as 10."]
    #[inline(always)]
    pub fn edgedet(&self) -> EDGEDET_R {
        EDGEDET_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input Signal Synchronization Selection This bit selects if the un-synchronized input signal (with optionally inverted) or the synchronized (and optionally filtered) signal can be used as input for the data shift unit."]
    #[inline(always)]
    pub fn syncsel(&mut self) -> SYNCSEL_W {
        SYNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Input Signal Inverse Selection This bit defines the inverter enable of the input asynchronous signal."]
    #[inline(always)]
    pub fn ininv(&mut self) -> ININV_W {
        ININV_W { w: self }
    }
    #[doc = "Bits 3:4 - Input Signal Edge Detection Mode This bit field selects which edge actives the trigger event of input data signal. Note: In UART function mode, it is suggested to set this bit field as 10."]
    #[inline(always)]
    pub fn edgedet(&mut self) -> EDGEDET_W {
        EDGEDET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Input Data Signal Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuart_datin0](index.html) module"]
pub struct UUART_DATIN0_SPEC;
impl crate::RegisterSpec for UUART_DATIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuart_datin0::R](R) reader structure"]
impl crate::Readable for UUART_DATIN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuart_datin0::W](W) writer structure"]
impl crate::Writable for UUART_DATIN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UUART_DATIN0 to value 0"]
impl crate::Resettable for UUART_DATIN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
