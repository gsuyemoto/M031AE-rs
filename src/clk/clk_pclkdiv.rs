#[doc = "Register `CLK_PCLKDIV` reader"]
pub struct R(crate::R<CLK_PCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PCLKDIV` writer"]
pub struct W(crate::W<CLK_PCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PCLKDIV_SPEC>;
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
impl From<crate::W<CLK_PCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB0DIV` reader - APB0 Clock DIvider\nAPB0 clock can be divided from HCLK\nOthers: Reserved."]
pub struct APB0DIV_R(crate::FieldReader<u8, u8>);
impl APB0DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        APB0DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB0DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB0DIV` writer - APB0 Clock DIvider\nAPB0 clock can be divided from HCLK\nOthers: Reserved."]
pub struct APB0DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB0DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `APB1DIV` reader - APB1 Clock DIvider\nAPB1 clock can be divided from HCLK\nOthers: Reserved."]
pub struct APB1DIV_R(crate::FieldReader<u8, u8>);
impl APB1DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        APB1DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB1DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB1DIV` writer - APB1 Clock DIvider\nAPB1 clock can be divided from HCLK\nOthers: Reserved."]
pub struct APB1DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB1DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - APB0 Clock DIvider APB0 clock can be divided from HCLK Others: Reserved."]
    #[inline(always)]
    pub fn apb0div(&self) -> APB0DIV_R {
        APB0DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - APB1 Clock DIvider APB1 clock can be divided from HCLK Others: Reserved."]
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB0 Clock DIvider APB0 clock can be divided from HCLK Others: Reserved."]
    #[inline(always)]
    pub fn apb0div(&mut self) -> APB0DIV_W {
        APB0DIV_W { w: self }
    }
    #[doc = "Bits 4:6 - APB1 Clock DIvider APB1 clock can be divided from HCLK Others: Reserved."]
    #[inline(always)]
    pub fn apb1div(&mut self) -> APB1DIV_W {
        APB1DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pclkdiv](index.html) module"]
pub struct CLK_PCLKDIV_SPEC;
impl crate::RegisterSpec for CLK_PCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_pclkdiv::R](R) reader structure"]
impl crate::Readable for CLK_PCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pclkdiv::W](W) writer structure"]
impl crate::Writable for CLK_PCLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PCLKDIV to value 0"]
impl crate::Resettable for CLK_PCLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
