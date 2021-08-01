#[doc = "Register `SYS_GPF_MFPH` reader"]
pub struct R(crate::R<SYS_GPF_MFPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPF_MFPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_GPF_MFPH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_GPF_MFPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPF_MFPH` writer"]
pub struct W(crate::W<SYS_GPF_MFPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPF_MFPH_SPEC>;
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
impl From<crate::W<SYS_GPF_MFPH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_GPF_MFPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF8MFP` reader - PF.8 Multi-function Pin Selection"]
pub struct PF8MFP_R(crate::FieldReader<u8, u8>);
impl PF8MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF8MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF8MFP` writer - PF.8 Multi-function Pin Selection"]
pub struct PF8MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PF9MFP` reader - PF.9 Multi-function Pin Selection"]
pub struct PF9MFP_R(crate::FieldReader<u8, u8>);
impl PF9MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF9MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF9MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF9MFP` writer - PF.9 Multi-function Pin Selection"]
pub struct PF9MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF9MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PF10MFP` reader - PF.10 Multi-function Pin Selection"]
pub struct PF10MFP_R(crate::FieldReader<u8, u8>);
impl PF10MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF10MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF10MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF10MFP` writer - PF.10 Multi-function Pin Selection"]
pub struct PF10MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF10MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PF11MFP` reader - PF.11 Multi-function Pin Selection"]
pub struct PF11MFP_R(crate::FieldReader<u8, u8>);
impl PF11MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF11MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF11MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF11MFP` writer - PF.11 Multi-function Pin Selection"]
pub struct PF11MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF11MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PF12MFP` reader - PF.12 Multi-function Pin Selection"]
pub struct PF12MFP_R(crate::FieldReader<u8, u8>);
impl PF12MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF12MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF12MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF12MFP` writer - PF.12 Multi-function Pin Selection"]
pub struct PF12MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF12MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PF13MFP` reader - PF.13 Multi-function Pin Selection"]
pub struct PF13MFP_R(crate::FieldReader<u8, u8>);
impl PF13MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF13MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF13MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF13MFP` writer - PF.13 Multi-function Pin Selection"]
pub struct PF13MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF13MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PF14MFP` reader - PF.14 Multi-function Pin Selection"]
pub struct PF14MFP_R(crate::FieldReader<u8, u8>);
impl PF14MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF14MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF14MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF14MFP` writer - PF.14 Multi-function Pin Selection"]
pub struct PF14MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF14MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PF15MFP` reader - PF.15 Multi-function Pin Selection"]
pub struct PF15MFP_R(crate::FieldReader<u8, u8>);
impl PF15MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF15MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF15MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF15MFP` writer - PF.15 Multi-function Pin Selection"]
pub struct PF15MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF15MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PF.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf8mfp(&self) -> PF8MFP_R {
        PF8MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PF.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf9mfp(&self) -> PF9MFP_R {
        PF9MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PF.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf10mfp(&self) -> PF10MFP_R {
        PF10MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PF.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf11mfp(&self) -> PF11MFP_R {
        PF11MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PF.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf12mfp(&self) -> PF12MFP_R {
        PF12MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PF.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf13mfp(&self) -> PF13MFP_R {
        PF13MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PF.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf14mfp(&self) -> PF14MFP_R {
        PF14MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PF.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf15mfp(&self) -> PF15MFP_R {
        PF15MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PF.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf8mfp(&mut self) -> PF8MFP_W {
        PF8MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PF.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf9mfp(&mut self) -> PF9MFP_W {
        PF9MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PF.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf10mfp(&mut self) -> PF10MFP_W {
        PF10MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PF.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf11mfp(&mut self) -> PF11MFP_W {
        PF11MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PF.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf12mfp(&mut self) -> PF12MFP_W {
        PF12MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PF.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf13mfp(&mut self) -> PF13MFP_W {
        PF13MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PF.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf14mfp(&mut self) -> PF14MFP_W {
        PF14MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PF.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf15mfp(&mut self) -> PF15MFP_W {
        PF15MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOF High Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpf_mfph](index.html) module"]
pub struct SYS_GPF_MFPH_SPEC;
impl crate::RegisterSpec for SYS_GPF_MFPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpf_mfph::R](R) reader structure"]
impl crate::Readable for SYS_GPF_MFPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpf_mfph::W](W) writer structure"]
impl crate::Writable for SYS_GPF_MFPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPF_MFPH to value 0"]
impl crate::Resettable for SYS_GPF_MFPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
