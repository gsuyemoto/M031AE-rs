#[doc = "Register `UART_BRCOMP` reader"]
pub struct R(crate::R<UART_BRCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_BRCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_BRCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_BRCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_BRCOMP` writer"]
pub struct W(crate::W<UART_BRCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_BRCOMP_SPEC>;
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
impl From<crate::W<UART_BRCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_BRCOMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRCOMP` reader - Baud Rate Compensation Patten\nThese 9-bits are used to define the relative bit is compensated or not. \nBRCOMP\\[7:0\\]
is used to define the compensation of UART_DAT\\[7:0\\]
and BRCOM\\[8\\]
is used to define the parity bit."]
pub struct BRCOMP_R(crate::FieldReader<u16, u16>);
impl BRCOMP_R {
    pub(crate) fn new(bits: u16) -> Self {
        BRCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRCOMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRCOMP` writer - Baud Rate Compensation Patten\nThese 9-bits are used to define the relative bit is compensated or not. \nBRCOMP\\[7:0\\]
is used to define the compensation of UART_DAT\\[7:0\\]
and BRCOM\\[8\\]
is used to define the parity bit."]
pub struct BRCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRCOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Baud Rate Compensation Decrease\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRCOMPDEC_A {
    #[doc = "0: Positive (increase one module clock) compensation for each compensated bit"]
    _0 = 0,
    #[doc = "1: Negative (decrease one module clock) compensation for each compensated bit"]
    _1 = 1,
}
impl From<BRCOMPDEC_A> for bool {
    #[inline(always)]
    fn from(variant: BRCOMPDEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRCOMPDEC` reader - Baud Rate Compensation Decrease"]
pub struct BRCOMPDEC_R(crate::FieldReader<bool, BRCOMPDEC_A>);
impl BRCOMPDEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRCOMPDEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRCOMPDEC_A {
        match self.bits {
            false => BRCOMPDEC_A::_0,
            true => BRCOMPDEC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRCOMPDEC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRCOMPDEC_A::_1
    }
}
impl core::ops::Deref for BRCOMPDEC_R {
    type Target = crate::FieldReader<bool, BRCOMPDEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRCOMPDEC` writer - Baud Rate Compensation Decrease"]
pub struct BRCOMPDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> BRCOMPDEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRCOMPDEC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Positive (increase one module clock) compensation for each compensated bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRCOMPDEC_A::_0)
    }
    #[doc = "Negative (decrease one module clock) compensation for each compensated bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRCOMPDEC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Baud Rate Compensation Patten These 9-bits are used to define the relative bit is compensated or not. BRCOMP\\[7:0\\]
is used to define the compensation of UART_DAT\\[7:0\\]
and BRCOM\\[8\\]
is used to define the parity bit."]
    #[inline(always)]
    pub fn brcomp(&self) -> BRCOMP_R {
        BRCOMP_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - Baud Rate Compensation Decrease"]
    #[inline(always)]
    pub fn brcompdec(&self) -> BRCOMPDEC_R {
        BRCOMPDEC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Baud Rate Compensation Patten These 9-bits are used to define the relative bit is compensated or not. BRCOMP\\[7:0\\]
is used to define the compensation of UART_DAT\\[7:0\\]
and BRCOM\\[8\\]
is used to define the parity bit."]
    #[inline(always)]
    pub fn brcomp(&mut self) -> BRCOMP_W {
        BRCOMP_W { w: self }
    }
    #[doc = "Bit 31 - Baud Rate Compensation Decrease"]
    #[inline(always)]
    pub fn brcompdec(&mut self) -> BRCOMPDEC_W {
        BRCOMPDEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baud Rate Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_brcomp](index.html) module"]
pub struct UART_BRCOMP_SPEC;
impl crate::RegisterSpec for UART_BRCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_brcomp::R](R) reader structure"]
impl crate::Readable for UART_BRCOMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_brcomp::W](W) writer structure"]
impl crate::Writable for UART_BRCOMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_BRCOMP to value 0"]
impl crate::Resettable for UART_BRCOMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
