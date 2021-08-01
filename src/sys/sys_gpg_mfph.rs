#[doc = "Register `SYS_GPG_MFPH` reader"]
pub struct R(crate::R<SYS_GPG_MFPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPG_MFPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_GPG_MFPH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_GPG_MFPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPG_MFPH` writer"]
pub struct W(crate::W<SYS_GPG_MFPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPG_MFPH_SPEC>;
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
impl From<crate::W<SYS_GPG_MFPH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_GPG_MFPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG8MFP` reader - PG.8 Multi-function Pin Selection"]
pub struct PG8MFP_R(crate::FieldReader<u8, u8>);
impl PG8MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG8MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG8MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG8MFP` writer - PG.8 Multi-function Pin Selection"]
pub struct PG8MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG8MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PG9MFP` reader - PG.9 Multi-function Pin Selection"]
pub struct PG9MFP_R(crate::FieldReader<u8, u8>);
impl PG9MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG9MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG9MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG9MFP` writer - PG.9 Multi-function Pin Selection"]
pub struct PG9MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG9MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PG10MFP` reader - PG.10 Multi-function Pin Selection"]
pub struct PG10MFP_R(crate::FieldReader<u8, u8>);
impl PG10MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG10MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG10MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG10MFP` writer - PG.10 Multi-function Pin Selection"]
pub struct PG10MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG10MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PG11MFP` reader - PG.11 Multi-function Pin Selection"]
pub struct PG11MFP_R(crate::FieldReader<u8, u8>);
impl PG11MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG11MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG11MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG11MFP` writer - PG.11 Multi-function Pin Selection"]
pub struct PG11MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG11MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PG12MFP` reader - PG.12 Multi-function Pin Selection"]
pub struct PG12MFP_R(crate::FieldReader<u8, u8>);
impl PG12MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG12MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG12MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG12MFP` writer - PG.12 Multi-function Pin Selection"]
pub struct PG12MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG12MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PG13MFP` reader - PG.13 Multi-function Pin Selection"]
pub struct PG13MFP_R(crate::FieldReader<u8, u8>);
impl PG13MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG13MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG13MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG13MFP` writer - PG.13 Multi-function Pin Selection"]
pub struct PG13MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG13MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PG14MFP` reader - PG.14 Multi-function Pin Selection"]
pub struct PG14MFP_R(crate::FieldReader<u8, u8>);
impl PG14MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG14MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG14MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG14MFP` writer - PG.14 Multi-function Pin Selection"]
pub struct PG14MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG14MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PG15MFP` reader - PG.15 Multi-function Pin Selection"]
pub struct PG15MFP_R(crate::FieldReader<u8, u8>);
impl PG15MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG15MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG15MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG15MFP` writer - PG.15 Multi-function Pin Selection"]
pub struct PG15MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG15MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PG.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg8mfp(&self) -> PG8MFP_R {
        PG8MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PG.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg9mfp(&self) -> PG9MFP_R {
        PG9MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PG.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg10mfp(&self) -> PG10MFP_R {
        PG10MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PG.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg11mfp(&self) -> PG11MFP_R {
        PG11MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PG.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg12mfp(&self) -> PG12MFP_R {
        PG12MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PG.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg13mfp(&self) -> PG13MFP_R {
        PG13MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PG.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg14mfp(&self) -> PG14MFP_R {
        PG14MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PG.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg15mfp(&self) -> PG15MFP_R {
        PG15MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PG.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg8mfp(&mut self) -> PG8MFP_W {
        PG8MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PG.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg9mfp(&mut self) -> PG9MFP_W {
        PG9MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PG.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg10mfp(&mut self) -> PG10MFP_W {
        PG10MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PG.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg11mfp(&mut self) -> PG11MFP_W {
        PG11MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PG.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg12mfp(&mut self) -> PG12MFP_W {
        PG12MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PG.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg13mfp(&mut self) -> PG13MFP_W {
        PG13MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PG.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg14mfp(&mut self) -> PG14MFP_W {
        PG14MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PG.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg15mfp(&mut self) -> PG15MFP_W {
        PG15MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOG High Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpg_mfph](index.html) module"]
pub struct SYS_GPG_MFPH_SPEC;
impl crate::RegisterSpec for SYS_GPG_MFPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpg_mfph::R](R) reader structure"]
impl crate::Readable for SYS_GPG_MFPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpg_mfph::W](W) writer structure"]
impl crate::Writable for SYS_GPG_MFPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPG_MFPH to value 0"]
impl crate::Resettable for SYS_GPG_MFPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
