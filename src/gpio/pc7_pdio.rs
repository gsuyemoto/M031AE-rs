#[doc = "Register `PC7_PDIO` reader"]
pub struct R(crate::R<PC7_PDIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC7_PDIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC7_PDIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC7_PDIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC7_PDIO` writer"]
pub struct W(crate::W<PC7_PDIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC7_PDIO_SPEC>;
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
impl From<crate::W<PC7_PDIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC7_PDIO_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PC.n Pin Data Input/Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc7_pdio](index.html) module"]
pub struct PC7_PDIO_SPEC;
impl crate::RegisterSpec for PC7_PDIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc7_pdio::R](R) reader structure"]
impl crate::Readable for PC7_PDIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc7_pdio::W](W) writer structure"]
impl crate::Writable for PC7_PDIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC7_PDIO to value 0"]
impl crate::Resettable for PC7_PDIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
