#[doc = "Register `DIVREM` reader"]
pub struct R(crate::R<DIVREM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVREM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVREM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVREM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVREM` writer"]
pub struct W(crate::W<DIVREM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVREM_SPEC>;
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
impl From<crate::W<DIVREM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVREM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REMAINDER15_0` reader - Remainder Result\nThis register holds the remainder result of divider after calculation is complete."]
pub struct REMAINDER15_0_R(crate::FieldReader<u16, u16>);
impl REMAINDER15_0_R {
    pub(crate) fn new(bits: u16) -> Self {
        REMAINDER15_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMAINDER15_0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMAINDER15_0` writer - Remainder Result\nThis register holds the remainder result of divider after calculation is complete."]
pub struct REMAINDER15_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REMAINDER15_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `REMAINDER31_16` reader - Sign Extension of REMAINDER\\[15:0\\]\nThe remainder of hardware divider is 16-bit sign integer (REMAINDER\\[15:0\\]) with sign extension (REMAINDER\\[31:16\\]) to 32-bit integer."]
pub struct REMAINDER31_16_R(crate::FieldReader<u16, u16>);
impl REMAINDER31_16_R {
    pub(crate) fn new(bits: u16) -> Self {
        REMAINDER31_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMAINDER31_16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMAINDER31_16` writer - Sign Extension of REMAINDER\\[15:0\\]\nThe remainder of hardware divider is 16-bit sign integer (REMAINDER\\[15:0\\]) with sign extension (REMAINDER\\[31:16\\]) to 32-bit integer."]
pub struct REMAINDER31_16_W<'a> {
    w: &'a mut W,
}
impl<'a> REMAINDER31_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Remainder Result This register holds the remainder result of divider after calculation is complete."]
    #[inline(always)]
    pub fn remainder15_0(&self) -> REMAINDER15_0_R {
        REMAINDER15_0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Sign Extension of REMAINDER\\[15:0\\]
The remainder of hardware divider is 16-bit sign integer (REMAINDER\\[15:0\\]) with sign extension (REMAINDER\\[31:16\\]) to 32-bit integer."]
    #[inline(always)]
    pub fn remainder31_16(&self) -> REMAINDER31_16_R {
        REMAINDER31_16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Remainder Result This register holds the remainder result of divider after calculation is complete."]
    #[inline(always)]
    pub fn remainder15_0(&mut self) -> REMAINDER15_0_W {
        REMAINDER15_0_W { w: self }
    }
    #[doc = "Bits 16:31 - Sign Extension of REMAINDER\\[15:0\\]
The remainder of hardware divider is 16-bit sign integer (REMAINDER\\[15:0\\]) with sign extension (REMAINDER\\[31:16\\]) to 32-bit integer."]
    #[inline(always)]
    pub fn remainder31_16(&mut self) -> REMAINDER31_16_W {
        REMAINDER31_16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remainder Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divrem](index.html) module"]
pub struct DIVREM_SPEC;
impl crate::RegisterSpec for DIVREM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divrem::R](R) reader structure"]
impl crate::Readable for DIVREM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divrem::W](W) writer structure"]
impl crate::Writable for DIVREM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIVREM to value 0"]
impl crate::Resettable for DIVREM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
