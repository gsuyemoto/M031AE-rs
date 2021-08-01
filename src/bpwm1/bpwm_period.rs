#[doc = "Register `BPWM_PERIOD` reader"]
pub struct R(crate::R<BPWM_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_PERIOD` writer"]
pub struct W(crate::W<BPWM_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_PERIOD_SPEC>;
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
impl From<crate::W<BPWM_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - BPWM Period Register\nUp-Count mode: In this mode, BPWM counter counts from 0 to PERIOD, and restarts from 0.\nDown-Count mode: In this mode, BPWM counter counts from PERIOD to 0, and restarts from PERIOD."]
pub struct PERIOD_R(crate::FieldReader<u16, u16>);
impl PERIOD_R {
    pub(crate) fn new(bits: u16) -> Self {
        PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIOD` writer - BPWM Period Register\nUp-Count mode: In this mode, BPWM counter counts from 0 to PERIOD, and restarts from 0.\nDown-Count mode: In this mode, BPWM counter counts from PERIOD to 0, and restarts from PERIOD."]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BPWM Period Register Up-Count mode: In this mode, BPWM counter counts from 0 to PERIOD, and restarts from 0. Down-Count mode: In this mode, BPWM counter counts from PERIOD to 0, and restarts from PERIOD."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPWM Period Register Up-Count mode: In this mode, BPWM counter counts from 0 to PERIOD, and restarts from 0. Down-Count mode: In this mode, BPWM counter counts from PERIOD to 0, and restarts from PERIOD."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_period](index.html) module"]
pub struct BPWM_PERIOD_SPEC;
impl crate::RegisterSpec for BPWM_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_period::R](R) reader structure"]
impl crate::Readable for BPWM_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_period::W](W) writer structure"]
impl crate::Writable for BPWM_PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_PERIOD to value 0"]
impl crate::Resettable for BPWM_PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
