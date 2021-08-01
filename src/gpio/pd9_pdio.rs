#[doc = "Register `PD9_PDIO` reader"]
pub struct R(crate::R<PD9_PDIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD9_PDIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD9_PDIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD9_PDIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD9_PDIO` writer"]
pub struct W(crate::W<PD9_PDIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD9_PDIO_SPEC>;
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
impl From<crate::W<PD9_PDIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD9_PDIO_SPEC>) -> Self {
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
#[doc = "GPIO PD.n Pin Data Input/Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd9_pdio](index.html) module"]
pub struct PD9_PDIO_SPEC;
impl crate::RegisterSpec for PD9_PDIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd9_pdio::R](R) reader structure"]
impl crate::Readable for PD9_PDIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd9_pdio::W](W) writer structure"]
impl crate::Writable for PD9_PDIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD9_PDIO to value 0"]
impl crate::Resettable for PD9_PDIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
