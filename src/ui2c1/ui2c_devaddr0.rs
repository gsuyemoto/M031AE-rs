#[doc = "Register `UI2C_DEVADDR0` reader"]
pub struct R(crate::R<UI2C_DEVADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UI2C_DEVADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UI2C_DEVADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UI2C_DEVADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UI2C_DEVADDR0` writer"]
pub struct W(crate::W<UI2C_DEVADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UI2C_DEVADDR0_SPEC>;
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
impl From<crate::W<UI2C_DEVADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UI2C_DEVADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVADDR` reader - Device Address\nIn I2C protocol, this bit field contains the programmed slave address. If the first received address byte are 1111 0AAXB, the AA bits are compared to the bits DEVADDR\\[9:8\\]
to check for address match, where the X is R/W bit. Then the second address byte is also compared to DEVADDR\\[7:0\\].\nNote 1: The DEVADDR \\[9:7\\]
must be set 3'b000 when I2C operating in 7-bit address mode.\nNote 2: When software set 10'h000, the address can not be used."]
pub struct DEVADDR_R(crate::FieldReader<u16, u16>);
impl DEVADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        DEVADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVADDR` writer - Device Address\nIn I2C protocol, this bit field contains the programmed slave address. If the first received address byte are 1111 0AAXB, the AA bits are compared to the bits DEVADDR\\[9:8\\]
to check for address match, where the X is R/W bit. Then the second address byte is also compared to DEVADDR\\[7:0\\].\nNote 1: The DEVADDR \\[9:7\\]
must be set 3'b000 when I2C operating in 7-bit address mode.\nNote 2: When software set 10'h000, the address can not be used."]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Device Address In I2C protocol, this bit field contains the programmed slave address. If the first received address byte are 1111 0AAXB, the AA bits are compared to the bits DEVADDR\\[9:8\\]
to check for address match, where the X is R/W bit. Then the second address byte is also compared to DEVADDR\\[7:0\\]. Note 1: The DEVADDR \\[9:7\\]
must be set 3'b000 when I2C operating in 7-bit address mode. Note 2: When software set 10'h000, the address can not be used."]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Device Address In I2C protocol, this bit field contains the programmed slave address. If the first received address byte are 1111 0AAXB, the AA bits are compared to the bits DEVADDR\\[9:8\\]
to check for address match, where the X is R/W bit. Then the second address byte is also compared to DEVADDR\\[7:0\\]. Note 1: The DEVADDR \\[9:7\\]
must be set 3'b000 when I2C operating in 7-bit address mode. Note 2: When software set 10'h000, the address can not be used."]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Device Address Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ui2c_devaddr0](index.html) module"]
pub struct UI2C_DEVADDR0_SPEC;
impl crate::RegisterSpec for UI2C_DEVADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ui2c_devaddr0::R](R) reader structure"]
impl crate::Readable for UI2C_DEVADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ui2c_devaddr0::W](W) writer structure"]
impl crate::Writable for UI2C_DEVADDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UI2C_DEVADDR0 to value 0"]
impl crate::Resettable for UI2C_DEVADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
