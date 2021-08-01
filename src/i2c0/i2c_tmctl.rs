#[doc = "Register `I2C_TMCTL` reader"]
pub struct R(crate::R<I2C_TMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TMCTL` writer"]
pub struct W(crate::W<I2C_TMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TMCTL_SPEC>;
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
impl From<crate::W<I2C_TMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCTL` reader - Setup Time Configure Control\nThis field is used to generate a delay timing between SDA falling edge and SCL rising edge in transmission mode.\nNote: Setup time setting should not make SCL output less than three PCLKs."]
pub struct STCTL_R(crate::FieldReader<u16, u16>);
impl STCTL_R {
    pub(crate) fn new(bits: u16) -> Self {
        STCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STCTL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STCTL` writer - Setup Time Configure Control\nThis field is used to generate a delay timing between SDA falling edge and SCL rising edge in transmission mode.\nNote: Setup time setting should not make SCL output less than three PCLKs."]
pub struct STCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> STCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `HTCTL` reader - Hold Time Configure Control \nThis field is used to generate the delay timing between SCL falling edge and SDA rising edge in transmission mode."]
pub struct HTCTL_R(crate::FieldReader<u16, u16>);
impl HTCTL_R {
    pub(crate) fn new(bits: u16) -> Self {
        HTCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTCTL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTCTL` writer - Hold Time Configure Control \nThis field is used to generate the delay timing between SCL falling edge and SDA rising edge in transmission mode."]
pub struct HTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> HTCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Setup Time Configure Control This field is used to generate a delay timing between SDA falling edge and SCL rising edge in transmission mode. Note: Setup time setting should not make SCL output less than three PCLKs."]
    #[inline(always)]
    pub fn stctl(&self) -> STCTL_R {
        STCTL_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Hold Time Configure Control This field is used to generate the delay timing between SCL falling edge and SDA rising edge in transmission mode."]
    #[inline(always)]
    pub fn htctl(&self) -> HTCTL_R {
        HTCTL_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Setup Time Configure Control This field is used to generate a delay timing between SDA falling edge and SCL rising edge in transmission mode. Note: Setup time setting should not make SCL output less than three PCLKs."]
    #[inline(always)]
    pub fn stctl(&mut self) -> STCTL_W {
        STCTL_W { w: self }
    }
    #[doc = "Bits 16:24 - Hold Time Configure Control This field is used to generate the delay timing between SCL falling edge and SDA rising edge in transmission mode."]
    #[inline(always)]
    pub fn htctl(&mut self) -> HTCTL_W {
        HTCTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Timing Configure Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_tmctl](index.html) module"]
pub struct I2C_TMCTL_SPEC;
impl crate::RegisterSpec for I2C_TMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_tmctl::R](R) reader structure"]
impl crate::Readable for I2C_TMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_tmctl::W](W) writer structure"]
impl crate::Writable for I2C_TMCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_TMCTL to value 0"]
impl crate::Resettable for I2C_TMCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
