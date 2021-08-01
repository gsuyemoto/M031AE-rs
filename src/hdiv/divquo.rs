#[doc = "Register `DIVQUO` reader"]
pub struct R(crate::R<DIVQUO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVQUO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVQUO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVQUO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVQUO` writer"]
pub struct W(crate::W<DIVQUO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVQUO_SPEC>;
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
impl From<crate::W<DIVQUO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVQUO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUOTIENT` reader - Quotient Result\nThis register holds the quotient result of divider after calculation is complete."]
pub struct QUOTIENT_R(crate::FieldReader<u32, u32>);
impl QUOTIENT_R {
    pub(crate) fn new(bits: u32) -> Self {
        QUOTIENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUOTIENT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUOTIENT` writer - Quotient Result\nThis register holds the quotient result of divider after calculation is complete."]
pub struct QUOTIENT_W<'a> {
    w: &'a mut W,
}
impl<'a> QUOTIENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Quotient Result This register holds the quotient result of divider after calculation is complete."]
    #[inline(always)]
    pub fn quotient(&self) -> QUOTIENT_R {
        QUOTIENT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quotient Result This register holds the quotient result of divider after calculation is complete."]
    #[inline(always)]
    pub fn quotient(&mut self) -> QUOTIENT_W {
        QUOTIENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quotient Result Resister\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divquo](index.html) module"]
pub struct DIVQUO_SPEC;
impl crate::RegisterSpec for DIVQUO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divquo::R](R) reader structure"]
impl crate::Readable for DIVQUO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divquo::W](W) writer structure"]
impl crate::Writable for DIVQUO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIVQUO to value 0"]
impl crate::Resettable for DIVQUO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
