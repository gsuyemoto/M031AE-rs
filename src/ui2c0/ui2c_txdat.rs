#[doc = "Register `UI2C_TXDAT` writer"]
pub struct W(crate::W<UI2C_TXDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UI2C_TXDAT_SPEC>;
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
impl From<crate::W<UI2C_TXDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UI2C_TXDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDAT` writer - Transmit Data\nSoftware can use this bit field to write 16-bit transmit data for transmission."]
pub struct TXDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data Software can use this bit field to write 16-bit transmit data for transmission."]
    #[inline(always)]
    pub fn txdat(&mut self) -> TXDAT_W {
        TXDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI Transmit Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ui2c_txdat](index.html) module"]
pub struct UI2C_TXDAT_SPEC;
impl crate::RegisterSpec for UI2C_TXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ui2c_txdat::W](W) writer structure"]
impl crate::Writable for UI2C_TXDAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UI2C_TXDAT to value 0"]
impl crate::Resettable for UI2C_TXDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
