#[doc = "Register `SPIx_I2SCLK` reader"]
pub struct R(crate::R<SPIX_I2SCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIX_I2SCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIX_I2SCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIX_I2SCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIx_I2SCLK` writer"]
pub struct W(crate::W<SPIX_I2SCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIX_I2SCLK_SPEC>;
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
impl From<crate::W<SPIX_I2SCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIX_I2SCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCLKDIV` reader - Master Clock Divider\nIf MCLKEN is set to 1, I2S controller will generate master clock for external audio devices. The frequency of master clock, fMCLK, is determined by the following expressions:\nwhere\n is the frequency of I2S peripheral clock source, which is defined in the clock control register CLK_CLKSEL2. In general, the master clock rate is 256 times sampling clock rate."]
pub struct MCLKDIV_R(crate::FieldReader<u8, u8>);
impl MCLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLKDIV` writer - Master Clock Divider\nIf MCLKEN is set to 1, I2S controller will generate master clock for external audio devices. The frequency of master clock, fMCLK, is determined by the following expressions:\nwhere\n is the frequency of I2S peripheral clock source, which is defined in the clock control register CLK_CLKSEL2. In general, the master clock rate is 256 times sampling clock rate."]
pub struct MCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `BCLKDIV` reader - Bit Clock Divider\nThe I2S controller will generate bit clock in Master mode. The clock frequency of bit clock , fBCLK, is determined by the following expression:\n\nwhere \n is the frequency of I2S peripheral clock source, which is defined in the clock control register CLK_CLKSEL2.\nIn I2S Slave mode, this field is used to define the frequency of peripheral clock and it's determined by . \nThe peripheral clock frequency in I2S Slave mode must be equal to or faster than 6 times of input bit clock.\nNote: The time interval must be larger than or equal to 8 peripheral clock cycles between releasing SPI IP software reset and setting this clock divider register."]
pub struct BCLKDIV_R(crate::FieldReader<u16, u16>);
impl BCLKDIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        BCLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCLKDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCLKDIV` writer - Bit Clock Divider\nThe I2S controller will generate bit clock in Master mode. The clock frequency of bit clock , fBCLK, is determined by the following expression:\n\nwhere \n is the frequency of I2S peripheral clock source, which is defined in the clock control register CLK_CLKSEL2.\nIn I2S Slave mode, this field is used to define the frequency of peripheral clock and it's determined by . \nThe peripheral clock frequency in I2S Slave mode must be equal to or faster than 6 times of input bit clock.\nNote: The time interval must be larger than or equal to 8 peripheral clock cycles between releasing SPI IP software reset and setting this clock divider register."]
pub struct BCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 8)) | ((value as u32 & 0x03ff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Master Clock Divider If MCLKEN is set to 1, I2S controller will generate master clock for external audio devices. The frequency of master clock, fMCLK, is determined by the following expressions: where is the frequency of I2S peripheral clock source, which is defined in the clock control register CLK_CLKSEL2. In general, the master clock rate is 256 times sampling clock rate."]
    #[inline(always)]
    pub fn mclkdiv(&self) -> MCLKDIV_R {
        MCLKDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:17 - Bit Clock Divider The I2S controller will generate bit clock in Master mode. The clock frequency of bit clock , fBCLK, is determined by the following expression: where is the frequency of I2S peripheral clock source, which is defined in the clock control register CLK_CLKSEL2. In I2S Slave mode, this field is used to define the frequency of peripheral clock and it's determined by . The peripheral clock frequency in I2S Slave mode must be equal to or faster than 6 times of input bit clock. Note: The time interval must be larger than or equal to 8 peripheral clock cycles between releasing SPI IP software reset and setting this clock divider register."]
    #[inline(always)]
    pub fn bclkdiv(&self) -> BCLKDIV_R {
        BCLKDIV_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Master Clock Divider If MCLKEN is set to 1, I2S controller will generate master clock for external audio devices. The frequency of master clock, fMCLK, is determined by the following expressions: where is the frequency of I2S peripheral clock source, which is defined in the clock control register CLK_CLKSEL2. In general, the master clock rate is 256 times sampling clock rate."]
    #[inline(always)]
    pub fn mclkdiv(&mut self) -> MCLKDIV_W {
        MCLKDIV_W { w: self }
    }
    #[doc = "Bits 8:17 - Bit Clock Divider The I2S controller will generate bit clock in Master mode. The clock frequency of bit clock , fBCLK, is determined by the following expression: where is the frequency of I2S peripheral clock source, which is defined in the clock control register CLK_CLKSEL2. In I2S Slave mode, this field is used to define the frequency of peripheral clock and it's determined by . The peripheral clock frequency in I2S Slave mode must be equal to or faster than 6 times of input bit clock. Note: The time interval must be larger than or equal to 8 peripheral clock cycles between releasing SPI IP software reset and setting this clock divider register."]
    #[inline(always)]
    pub fn bclkdiv(&mut self) -> BCLKDIV_W {
        BCLKDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Clock Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spix_i2sclk](index.html) module"]
pub struct SPIX_I2SCLK_SPEC;
impl crate::RegisterSpec for SPIX_I2SCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spix_i2sclk::R](R) reader structure"]
impl crate::Readable for SPIX_I2SCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spix_i2sclk::W](W) writer structure"]
impl crate::Writable for SPIX_I2SCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIx_I2SCLK to value 0"]
impl crate::Resettable for SPIX_I2SCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
