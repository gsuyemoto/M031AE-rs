#[doc = "Register `USPI_WKSTS` reader"]
pub struct R(crate::R<USPI_WKSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USPI_WKSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USPI_WKSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USPI_WKSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USPI_WKSTS` writer"]
pub struct W(crate::W<USPI_WKSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USPI_WKSTS_SPEC>;
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
impl From<crate::W<USPI_WKSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USPI_WKSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKF` reader - Wake-up Flag\nWhen chip is woken up from Power-down mode, this bit is set to 1. Software can write 1 to clear this bit."]
pub struct WKF_R(crate::FieldReader<bool, bool>);
impl WKF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKF` writer - Wake-up Flag\nWhen chip is woken up from Power-down mode, this bit is set to 1. Software can write 1 to clear this bit."]
pub struct WKF_W<'a> {
    w: &'a mut W,
}
impl<'a> WKF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Flag When chip is woken up from Power-down mode, this bit is set to 1. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn wkf(&self) -> WKF_R {
        WKF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Flag When chip is woken up from Power-down mode, this bit is set to 1. Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn wkf(&mut self) -> WKF_W {
        WKF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Wake-up Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uspi_wksts](index.html) module"]
pub struct USPI_WKSTS_SPEC;
impl crate::RegisterSpec for USPI_WKSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uspi_wksts::R](R) reader structure"]
impl crate::Readable for USPI_WKSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uspi_wksts::W](W) writer structure"]
impl crate::Writable for USPI_WKSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USPI_WKSTS to value 0"]
impl crate::Resettable for USPI_WKSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
