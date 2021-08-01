#[doc = "Register `BPWM_CLKPSC` reader"]
pub struct R(crate::R<BPWM_CLKPSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CLKPSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CLKPSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CLKPSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CLKPSC` writer"]
pub struct W(crate::W<BPWM_CLKPSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CLKPSC_SPEC>;
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
impl From<crate::W<BPWM_CLKPSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CLKPSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKPSC` reader - BPWM Counter Clock Prescale \nThe clock of BPWM counter is decided by clock prescaler. Each BPWM pair share one BPWM counter clock prescaler. The clock of BPWM counter is divided by (CLKPSC+ 1)."]
pub struct CLKPSC_R(crate::FieldReader<u16, u16>);
impl CLKPSC_R {
    pub(crate) fn new(bits: u16) -> Self {
        CLKPSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPSC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKPSC` writer - BPWM Counter Clock Prescale \nThe clock of BPWM counter is decided by clock prescaler. Each BPWM pair share one BPWM counter clock prescaler. The clock of BPWM counter is divided by (CLKPSC+ 1)."]
pub struct CLKPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - BPWM Counter Clock Prescale The clock of BPWM counter is decided by clock prescaler. Each BPWM pair share one BPWM counter clock prescaler. The clock of BPWM counter is divided by (CLKPSC+ 1)."]
    #[inline(always)]
    pub fn clkpsc(&self) -> CLKPSC_R {
        CLKPSC_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - BPWM Counter Clock Prescale The clock of BPWM counter is decided by clock prescaler. Each BPWM pair share one BPWM counter clock prescaler. The clock of BPWM counter is divided by (CLKPSC+ 1)."]
    #[inline(always)]
    pub fn clkpsc(&mut self) -> CLKPSC_W {
        CLKPSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Clock Prescale Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_clkpsc](index.html) module"]
pub struct BPWM_CLKPSC_SPEC;
impl crate::RegisterSpec for BPWM_CLKPSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_clkpsc::R](R) reader structure"]
impl crate::Readable for BPWM_CLKPSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_clkpsc::W](W) writer structure"]
impl crate::Writable for BPWM_CLKPSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CLKPSC to value 0"]
impl crate::Resettable for BPWM_CLKPSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
