#[doc = "Register `SYS_GPG_MFPL` reader"]
pub struct R(crate::R<SYS_GPG_MFPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPG_MFPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_GPG_MFPL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_GPG_MFPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPG_MFPL` writer"]
pub struct W(crate::W<SYS_GPG_MFPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPG_MFPL_SPEC>;
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
impl From<crate::W<SYS_GPG_MFPL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_GPG_MFPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG0MFP` reader - PG.0 Multi-function Pin Selection"]
pub struct PG0MFP_R(crate::FieldReader<u8, u8>);
impl PG0MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG0MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG0MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG0MFP` writer - PG.0 Multi-function Pin Selection"]
pub struct PG0MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG0MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PG1MFP` reader - PG.1 Multi-function Pin Selection"]
pub struct PG1MFP_R(crate::FieldReader<u8, u8>);
impl PG1MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG1MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG1MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG1MFP` writer - PG.1 Multi-function Pin Selection"]
pub struct PG1MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG1MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PG2MFP` reader - PG.2 Multi-function Pin Selection"]
pub struct PG2MFP_R(crate::FieldReader<u8, u8>);
impl PG2MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG2MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG2MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG2MFP` writer - PG.2 Multi-function Pin Selection"]
pub struct PG2MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG2MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PG3MFP` reader - PG.3 Multi-function Pin Selection"]
pub struct PG3MFP_R(crate::FieldReader<u8, u8>);
impl PG3MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG3MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG3MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG3MFP` writer - PG.3 Multi-function Pin Selection"]
pub struct PG3MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG3MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PG4MFP` reader - PG.4 Multi-function Pin Selection"]
pub struct PG4MFP_R(crate::FieldReader<u8, u8>);
impl PG4MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG4MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG4MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG4MFP` writer - PG.4 Multi-function Pin Selection"]
pub struct PG4MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG4MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PG5MFP` reader - PG.5 Multi-function Pin Selection"]
pub struct PG5MFP_R(crate::FieldReader<u8, u8>);
impl PG5MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG5MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG5MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG5MFP` writer - PG.5 Multi-function Pin Selection"]
pub struct PG5MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG5MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PG6MFP` reader - PG.6 Multi-function Pin Selection"]
pub struct PG6MFP_R(crate::FieldReader<u8, u8>);
impl PG6MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG6MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG6MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG6MFP` writer - PG.6 Multi-function Pin Selection"]
pub struct PG6MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG6MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PG7MFP` reader - PG.7 Multi-function Pin Selection"]
pub struct PG7MFP_R(crate::FieldReader<u8, u8>);
impl PG7MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG7MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG7MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG7MFP` writer - PG.7 Multi-function Pin Selection"]
pub struct PG7MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PG7MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PG.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg0mfp(&self) -> PG0MFP_R {
        PG0MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PG.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg1mfp(&self) -> PG1MFP_R {
        PG1MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PG.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg2mfp(&self) -> PG2MFP_R {
        PG2MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PG.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg3mfp(&self) -> PG3MFP_R {
        PG3MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PG.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg4mfp(&self) -> PG4MFP_R {
        PG4MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PG.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg5mfp(&self) -> PG5MFP_R {
        PG5MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PG.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg6mfp(&self) -> PG6MFP_R {
        PG6MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PG.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg7mfp(&self) -> PG7MFP_R {
        PG7MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PG.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg0mfp(&mut self) -> PG0MFP_W {
        PG0MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PG.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg1mfp(&mut self) -> PG1MFP_W {
        PG1MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PG.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg2mfp(&mut self) -> PG2MFP_W {
        PG2MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PG.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg3mfp(&mut self) -> PG3MFP_W {
        PG3MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PG.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg4mfp(&mut self) -> PG4MFP_W {
        PG4MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PG.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg5mfp(&mut self) -> PG5MFP_W {
        PG5MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PG.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg6mfp(&mut self) -> PG6MFP_W {
        PG6MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PG.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pg7mfp(&mut self) -> PG7MFP_W {
        PG7MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOG Low Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpg_mfpl](index.html) module"]
pub struct SYS_GPG_MFPL_SPEC;
impl crate::RegisterSpec for SYS_GPG_MFPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpg_mfpl::R](R) reader structure"]
impl crate::Readable for SYS_GPG_MFPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpg_mfpl::W](W) writer structure"]
impl crate::Writable for SYS_GPG_MFPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPG_MFPL to value 0"]
impl crate::Resettable for SYS_GPG_MFPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
