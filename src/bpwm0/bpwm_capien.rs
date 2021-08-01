#[doc = "Register `BPWM_CAPIEN` reader"]
pub struct R(crate::R<BPWM_CAPIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CAPIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CAPIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CAPIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CAPIEN` writer"]
pub struct W(crate::W<BPWM_CAPIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CAPIEN_SPEC>;
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
impl From<crate::W<BPWM_CAPIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CAPIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM Capture Rising Latch Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPRIENN_A {
    #[doc = "0: Capture rising edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture rising edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPRIENN_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPRIENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPRIENn` reader - BPWM Capture Rising Latch Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPRIENN_R(crate::FieldReader<u8, CAPRIENN_A>);
impl CAPRIENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPRIENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPRIENN_A> {
        match self.bits {
            0 => Some(CAPRIENN_A::_0),
            1 => Some(CAPRIENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPRIENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPRIENN_A::_1
    }
}
impl core::ops::Deref for CAPRIENN_R {
    type Target = crate::FieldReader<u8, CAPRIENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPRIENn` writer - BPWM Capture Rising Latch Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPRIENN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPRIENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPRIENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Capture rising edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPRIENN_A::_0)
    }
    #[doc = "Capture rising edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPRIENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "BPWM Capture Falling Latch Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPFIENN_A {
    #[doc = "0: Capture falling edge latch interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Capture falling edge latch interrupt Enabled"]
    _1 = 1,
}
impl From<CAPFIENN_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPFIENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPFIENn` reader - BPWM Capture Falling Latch Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPFIENN_R(crate::FieldReader<u8, CAPFIENN_A>);
impl CAPFIENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPFIENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPFIENN_A> {
        match self.bits {
            0 => Some(CAPFIENN_A::_0),
            1 => Some(CAPFIENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFIENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFIENN_A::_1
    }
}
impl core::ops::Deref for CAPFIENN_R {
    type Target = crate::FieldReader<u8, CAPFIENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFIENn` writer - BPWM Capture Falling Latch Interrupt Enable Bits\nEach bit n controls the corresponding BPWM channel n."]
pub struct CAPFIENN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFIENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFIENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Capture falling edge latch interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFIENN_A::_0)
    }
    #[doc = "Capture falling edge latch interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFIENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - BPWM Capture Rising Latch Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn caprienn(&self) -> CAPRIENN_R {
        CAPRIENN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - BPWM Capture Falling Latch Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capfienn(&self) -> CAPFIENN_R {
        CAPFIENN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - BPWM Capture Rising Latch Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn caprienn(&mut self) -> CAPRIENN_W {
        CAPRIENN_W { w: self }
    }
    #[doc = "Bits 8:13 - BPWM Capture Falling Latch Interrupt Enable Bits Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn capfienn(&mut self) -> CAPFIENN_W {
        CAPFIENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Capture Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_capien](index.html) module"]
pub struct BPWM_CAPIEN_SPEC;
impl crate::RegisterSpec for BPWM_CAPIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_capien::R](R) reader structure"]
impl crate::Readable for BPWM_CAPIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_capien::W](W) writer structure"]
impl crate::Writable for BPWM_CAPIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CAPIEN to value 0"]
impl crate::Resettable for BPWM_CAPIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
