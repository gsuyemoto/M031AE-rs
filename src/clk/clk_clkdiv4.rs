#[doc = "Register `CLK_CLKDIV4` reader"]
pub struct R(crate::R<CLK_CLKDIV4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKDIV4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CLKDIV4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CLKDIV4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKDIV4` writer"]
pub struct W(crate::W<CLK_CLKDIV4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKDIV4_SPEC>;
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
impl From<crate::W<CLK_CLKDIV4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CLKDIV4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART2DIV` reader - UART2 Clock Divide Number From UART2 Clock Source"]
pub struct UART2DIV_R(crate::FieldReader<u8, u8>);
impl UART2DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART2DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2DIV` writer - UART2 Clock Divide Number From UART2 Clock Source"]
pub struct UART2DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `UART3DIV` reader - UART3 Clock Divide Number From UART3 Clock Source"]
pub struct UART3DIV_R(crate::FieldReader<u8, u8>);
impl UART3DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART3DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART3DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART3DIV` writer - UART3 Clock Divide Number From UART3 Clock Source"]
pub struct UART3DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `UART4DIV` reader - UART4 Clock Divide Number From UART4 Clock Source"]
pub struct UART4DIV_R(crate::FieldReader<u8, u8>);
impl UART4DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART4DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4DIV` writer - UART4 Clock Divide Number From UART4 Clock Source"]
pub struct UART4DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `UART5DIV` reader - UART5 Clock Divide Number From UART5 Clock Source"]
pub struct UART5DIV_R(crate::FieldReader<u8, u8>);
impl UART5DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART5DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART5DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5DIV` writer - UART5 Clock Divide Number From UART5 Clock Source"]
pub struct UART5DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `UART6DIV` reader - UART6 Clock Divide Number From UART6 Clock Source"]
pub struct UART6DIV_R(crate::FieldReader<u8, u8>);
impl UART6DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART6DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART6DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART6DIV` writer - UART6 Clock Divide Number From UART6 Clock Source"]
pub struct UART6DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART6DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `UART7DIV` reader - UART7 Clock Divide Number From UART7 Clock Source"]
pub struct UART7DIV_R(crate::FieldReader<u8, u8>);
impl UART7DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART7DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART7DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART7DIV` writer - UART7 Clock Divide Number From UART7 Clock Source"]
pub struct UART7DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - UART2 Clock Divide Number From UART2 Clock Source"]
    #[inline(always)]
    pub fn uart2div(&self) -> UART2DIV_R {
        UART2DIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - UART3 Clock Divide Number From UART3 Clock Source"]
    #[inline(always)]
    pub fn uart3div(&self) -> UART3DIV_R {
        UART3DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - UART4 Clock Divide Number From UART4 Clock Source"]
    #[inline(always)]
    pub fn uart4div(&self) -> UART4DIV_R {
        UART4DIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - UART5 Clock Divide Number From UART5 Clock Source"]
    #[inline(always)]
    pub fn uart5div(&self) -> UART5DIV_R {
        UART5DIV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - UART6 Clock Divide Number From UART6 Clock Source"]
    #[inline(always)]
    pub fn uart6div(&self) -> UART6DIV_R {
        UART6DIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - UART7 Clock Divide Number From UART7 Clock Source"]
    #[inline(always)]
    pub fn uart7div(&self) -> UART7DIV_R {
        UART7DIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - UART2 Clock Divide Number From UART2 Clock Source"]
    #[inline(always)]
    pub fn uart2div(&mut self) -> UART2DIV_W {
        UART2DIV_W { w: self }
    }
    #[doc = "Bits 4:7 - UART3 Clock Divide Number From UART3 Clock Source"]
    #[inline(always)]
    pub fn uart3div(&mut self) -> UART3DIV_W {
        UART3DIV_W { w: self }
    }
    #[doc = "Bits 8:11 - UART4 Clock Divide Number From UART4 Clock Source"]
    #[inline(always)]
    pub fn uart4div(&mut self) -> UART4DIV_W {
        UART4DIV_W { w: self }
    }
    #[doc = "Bits 12:15 - UART5 Clock Divide Number From UART5 Clock Source"]
    #[inline(always)]
    pub fn uart5div(&mut self) -> UART5DIV_W {
        UART5DIV_W { w: self }
    }
    #[doc = "Bits 16:19 - UART6 Clock Divide Number From UART6 Clock Source"]
    #[inline(always)]
    pub fn uart6div(&mut self) -> UART6DIV_W {
        UART6DIV_W { w: self }
    }
    #[doc = "Bits 20:23 - UART7 Clock Divide Number From UART7 Clock Source"]
    #[inline(always)]
    pub fn uart7div(&mut self) -> UART7DIV_W {
        UART7DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Divider Number Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clkdiv4](index.html) module"]
pub struct CLK_CLKDIV4_SPEC;
impl crate::RegisterSpec for CLK_CLKDIV4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clkdiv4::R](R) reader structure"]
impl crate::Readable for CLK_CLKDIV4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clkdiv4::W](W) writer structure"]
impl crate::Writable for CLK_CLKDIV4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKDIV4 to value 0"]
impl crate::Resettable for CLK_CLKDIV4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
