#[doc = "Register `SYS_GPH_MFPL` reader"]
pub struct R(crate::R<SYS_GPH_MFPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPH_MFPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_GPH_MFPL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_GPH_MFPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPH_MFPL` writer"]
pub struct W(crate::W<SYS_GPH_MFPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPH_MFPL_SPEC>;
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
impl From<crate::W<SYS_GPH_MFPL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_GPH_MFPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PH0MFP` reader - PH.0 Multi-function Pin Selection"]
pub struct PH0MFP_R(crate::FieldReader<u8, u8>);
impl PH0MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH0MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH0MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH0MFP` writer - PH.0 Multi-function Pin Selection"]
pub struct PH0MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH0MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PH1MFP` reader - PH.1 Multi-function Pin Selection"]
pub struct PH1MFP_R(crate::FieldReader<u8, u8>);
impl PH1MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH1MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH1MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH1MFP` writer - PH.1 Multi-function Pin Selection"]
pub struct PH1MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH1MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PH2MFP` reader - PH.2 Multi-function Pin Selection"]
pub struct PH2MFP_R(crate::FieldReader<u8, u8>);
impl PH2MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH2MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH2MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH2MFP` writer - PH.2 Multi-function Pin Selection"]
pub struct PH2MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH2MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PH3MFP` reader - PH.3 Multi-function Pin Selection"]
pub struct PH3MFP_R(crate::FieldReader<u8, u8>);
impl PH3MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH3MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH3MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH3MFP` writer - PH.3 Multi-function Pin Selection"]
pub struct PH3MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH3MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PH4MFP` reader - PH.4 Multi-function Pin Selection"]
pub struct PH4MFP_R(crate::FieldReader<u8, u8>);
impl PH4MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH4MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH4MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH4MFP` writer - PH.4 Multi-function Pin Selection"]
pub struct PH4MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH4MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PH5MFP` reader - PH.5 Multi-function Pin Selection"]
pub struct PH5MFP_R(crate::FieldReader<u8, u8>);
impl PH5MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH5MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH5MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH5MFP` writer - PH.5 Multi-function Pin Selection"]
pub struct PH5MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH5MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PH6MFP` reader - PH.6 Multi-function Pin Selection"]
pub struct PH6MFP_R(crate::FieldReader<u8, u8>);
impl PH6MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH6MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH6MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH6MFP` writer - PH.6 Multi-function Pin Selection"]
pub struct PH6MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH6MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PH7MFP` reader - PH.7 Multi-function Pin Selection"]
pub struct PH7MFP_R(crate::FieldReader<u8, u8>);
impl PH7MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PH7MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PH7MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PH7MFP` writer - PH.7 Multi-function Pin Selection"]
pub struct PH7MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PH7MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PH.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph0mfp(&self) -> PH0MFP_R {
        PH0MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PH.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph1mfp(&self) -> PH1MFP_R {
        PH1MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PH.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph2mfp(&self) -> PH2MFP_R {
        PH2MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PH.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph3mfp(&self) -> PH3MFP_R {
        PH3MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PH.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph4mfp(&self) -> PH4MFP_R {
        PH4MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PH.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph5mfp(&self) -> PH5MFP_R {
        PH5MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PH.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph6mfp(&self) -> PH6MFP_R {
        PH6MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PH.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph7mfp(&self) -> PH7MFP_R {
        PH7MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PH.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph0mfp(&mut self) -> PH0MFP_W {
        PH0MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PH.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph1mfp(&mut self) -> PH1MFP_W {
        PH1MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PH.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph2mfp(&mut self) -> PH2MFP_W {
        PH2MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PH.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph3mfp(&mut self) -> PH3MFP_W {
        PH3MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PH.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph4mfp(&mut self) -> PH4MFP_W {
        PH4MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PH.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph5mfp(&mut self) -> PH5MFP_W {
        PH5MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PH.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph6mfp(&mut self) -> PH6MFP_W {
        PH6MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PH.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn ph7mfp(&mut self) -> PH7MFP_W {
        PH7MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOH Low Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gph_mfpl](index.html) module"]
pub struct SYS_GPH_MFPL_SPEC;
impl crate::RegisterSpec for SYS_GPH_MFPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gph_mfpl::R](R) reader structure"]
impl crate::Readable for SYS_GPH_MFPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gph_mfpl::W](W) writer structure"]
impl crate::Writable for SYS_GPH_MFPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPH_MFPL to value 0"]
impl crate::Resettable for SYS_GPH_MFPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
