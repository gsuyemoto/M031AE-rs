#[doc = "Register `QSPIx_CLKDIV` reader"]
pub struct R(crate::R<QSPIX_CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPIX_CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPIX_CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPIX_CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPIx_CLKDIV` writer"]
pub struct W(crate::W<QSPIX_CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPIX_CLKDIV_SPEC>;
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
impl From<crate::W<QSPIX_CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPIX_CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVIDER` reader - Clock Divider\nThe value in this field is the frequency divider for generating the peripheral clock, fspi_eclk, and the QSPI bus clock of QSPI Master. The frequency is obtained according to the following equation.\n\nwhere \n is the peripheral clock source, which is defined in the clock control register, CLK_CLKSEL2.\nNote: The time interval must be larger than or equal 8 peripheral clock cycles between releasing QSPI IP software reset and setting this clock divider register."]
pub struct DIVIDER_R(crate::FieldReader<u16, u16>);
impl DIVIDER_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVIDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVIDER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVIDER` writer - Clock Divider\nThe value in this field is the frequency divider for generating the peripheral clock, fspi_eclk, and the QSPI bus clock of QSPI Master. The frequency is obtained according to the following equation.\n\nwhere \n is the peripheral clock source, which is defined in the clock control register, CLK_CLKSEL2.\nNote: The time interval must be larger than or equal 8 peripheral clock cycles between releasing QSPI IP software reset and setting this clock divider register."]
pub struct DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Clock Divider The value in this field is the frequency divider for generating the peripheral clock, fspi_eclk, and the QSPI bus clock of QSPI Master. The frequency is obtained according to the following equation. where is the peripheral clock source, which is defined in the clock control register, CLK_CLKSEL2. Note: The time interval must be larger than or equal 8 peripheral clock cycles between releasing QSPI IP software reset and setting this clock divider register."]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock Divider The value in this field is the frequency divider for generating the peripheral clock, fspi_eclk, and the QSPI bus clock of QSPI Master. The frequency is obtained according to the following equation. where is the peripheral clock source, which is defined in the clock control register, CLK_CLKSEL2. Note: The time interval must be larger than or equal 8 peripheral clock cycles between releasing QSPI IP software reset and setting this clock divider register."]
    #[inline(always)]
    pub fn divider(&mut self) -> DIVIDER_W {
        DIVIDER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspix_clkdiv](index.html) module"]
pub struct QSPIX_CLKDIV_SPEC;
impl crate::RegisterSpec for QSPIX_CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspix_clkdiv::R](R) reader structure"]
impl crate::Readable for QSPIX_CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspix_clkdiv::W](W) writer structure"]
impl crate::Writable for QSPIX_CLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPIx_CLKDIV to value 0"]
impl crate::Resettable for QSPIX_CLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
