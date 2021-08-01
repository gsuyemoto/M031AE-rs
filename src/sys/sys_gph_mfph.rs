#[doc = "Register `SYS_GPH_MFPH` reader"]
pub struct R(crate::R<SYS_GPH_MFPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPH_MFPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_GPH_MFPH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_GPH_MFPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPH_MFPH` writer"]
pub struct W(crate::W<SYS_GPH_MFPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPH_MFPH_SPEC>;
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
impl From<crate::W<SYS_GPH_MFPH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_GPH_MFPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PH8MFP` reader - PH.8 Multi-function Pin Selection"]
pub struct PH8MFP_R(crate::FieldReader<u8, u8>);
impl PH8MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH8MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH8MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH8MFP` writer - PH.8 Multi-function Pin Selection"]
pub struct PH8MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH8MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PH9MFP` reader - PH.9 Multi-function Pin Selection"]
pub struct PH9MFP_R(crate::FieldReader<u8, u8>);
impl PH9MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH9MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH9MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH9MFP` writer - PH.9 Multi-function Pin Selection"]
pub struct PH9MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH9MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PH10MFP` reader - PH.10 Multi-function Pin Selection"]
pub struct PH10MFP_R(crate::FieldReader<u8, u8>);
impl PH10MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH10MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH10MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH10MFP` writer - PH.10 Multi-function Pin Selection"]
pub struct PH10MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH10MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PH11MFP` reader - PH.11 Multi-function Pin Selection"]
pub struct PH11MFP_R(crate::FieldReader<u8, u8>);
impl PH11MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH11MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH11MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH11MFP` writer - PH.11 Multi-function Pin Selection"]
pub struct PH11MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH11MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PH12MFP` reader - PH.12 Multi-function Pin Selection"]
pub struct PH12MFP_R(crate::FieldReader<u8, u8>);
impl PH12MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH12MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH12MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH12MFP` writer - PH.12 Multi-function Pin Selection"]
pub struct PH12MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH12MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PH13MFP` reader - PH.13 Multi-function Pin Selection"]
pub struct PH13MFP_R(crate::FieldReader<u8, u8>);
impl PH13MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH13MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH13MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH13MFP` writer - PH.13 Multi-function Pin Selection"]
pub struct PH13MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH13MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PH14MFP` reader - PH.14 Multi-function Pin Selection"]
pub struct PH14MFP_R(crate::FieldReader<u8, u8>);
impl PH14MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH14MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH14MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH14MFP` writer - PH.14 Multi-function Pin Selection"]
pub struct PH14MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH14MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PH15MFP` reader - PH.15 Multi-function Pin Selection"]
pub struct PH15MFP_R(crate::FieldReader<u8, u8>);
impl PH15MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH15MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH15MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH15MFP` writer - PH.15 Multi-function Pin Selection"]
pub struct PH15MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH15MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PH.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph8mfp(&self) -> PH8MFP_R {
        PH8MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PH.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph9mfp(&self) -> PH9MFP_R {
        PH9MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PH.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph10mfp(&self) -> PH10MFP_R {
        PH10MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PH.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph11mfp(&self) -> PH11MFP_R {
        PH11MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PH.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph12mfp(&self) -> PH12MFP_R {
        PH12MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PH.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph13mfp(&self) -> PH13MFP_R {
        PH13MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PH.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph14mfp(&self) -> PH14MFP_R {
        PH14MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PH.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph15mfp(&self) -> PH15MFP_R {
        PH15MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PH.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph8mfp(&mut self) -> PH8MFP_W {
        PH8MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PH.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph9mfp(&mut self) -> PH9MFP_W {
        PH9MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PH.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph10mfp(&mut self) -> PH10MFP_W {
        PH10MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PH.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph11mfp(&mut self) -> PH11MFP_W {
        PH11MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PH.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph12mfp(&mut self) -> PH12MFP_W {
        PH12MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PH.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph13mfp(&mut self) -> PH13MFP_W {
        PH13MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PH.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph14mfp(&mut self) -> PH14MFP_W {
        PH14MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PH.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph15mfp(&mut self) -> PH15MFP_W {
        PH15MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOH High Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gph_mfph](index.html) module"]
pub struct SYS_GPH_MFPH_SPEC;
impl crate::RegisterSpec for SYS_GPH_MFPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gph_mfph::R](R) reader structure"]
impl crate::Readable for SYS_GPH_MFPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gph_mfph::W](W) writer structure"]
impl crate::Writable for SYS_GPH_MFPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPH_MFPH to value 0"]
impl crate::Resettable for SYS_GPH_MFPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
