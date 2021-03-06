#[doc = "Register `PWM_CLKPSC2_3` reader"]
pub struct R(crate::R<PWM_CLKPSC2_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CLKPSC2_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CLKPSC2_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CLKPSC2_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CLKPSC2_3` writer"]
pub struct W(crate::W<PWM_CLKPSC2_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CLKPSC2_3_SPEC>;
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
impl From<crate::W<PWM_CLKPSC2_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CLKPSC2_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKPSC` reader - PWM Counter Clock Prescale \nThe clock of PWM counter is decided by clock prescaler. Each PWM pair share one PWM counter clock prescaler. The clock of PWM counter is divided by (CLKPSC+ 1)."]
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
#[doc = "Field `CLKPSC` writer - PWM Counter Clock Prescale \nThe clock of PWM counter is decided by clock prescaler. Each PWM pair share one PWM counter clock prescaler. The clock of PWM counter is divided by (CLKPSC+ 1)."]
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
    #[doc = "Bits 0:11 - PWM Counter Clock Prescale The clock of PWM counter is decided by clock prescaler. Each PWM pair share one PWM counter clock prescaler. The clock of PWM counter is divided by (CLKPSC+ 1)."]
    #[inline(always)]
    pub fn clkpsc(&self) -> CLKPSC_R {
        CLKPSC_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - PWM Counter Clock Prescale The clock of PWM counter is decided by clock prescaler. Each PWM pair share one PWM counter clock prescaler. The clock of PWM counter is divided by (CLKPSC+ 1)."]
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
#[doc = "PWM Clock Prescale Register 2/3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_clkpsc2_3](index.html) module"]
pub struct PWM_CLKPSC2_3_SPEC;
impl crate::RegisterSpec for PWM_CLKPSC2_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_clkpsc2_3::R](R) reader structure"]
impl crate::Readable for PWM_CLKPSC2_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_clkpsc2_3::W](W) writer structure"]
impl crate::Writable for PWM_CLKPSC2_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CLKPSC2_3 to value 0"]
impl crate::Resettable for PWM_CLKPSC2_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
