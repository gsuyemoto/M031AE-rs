#[doc = "Register `WDT_RSTCNT` writer"]
pub struct W(crate::W<WDT_RSTCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_RSTCNT_SPEC>;
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
impl From<crate::W<WDT_RSTCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_RSTCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTCNT` writer - WDT Reset Counter Register\nWriting 0x00005AA5 to this field will reset the internal 20-bit WDT up counter value to 0.\nNote 1: Performing RSTCNT to reset counter needs 2 * WDT_CLK period to become active."]
pub struct RSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - WDT Reset Counter Register Writing 0x00005AA5 to this field will reset the internal 20-bit WDT up counter value to 0. Note 1: Performing RSTCNT to reset counter needs 2 * WDT_CLK period to become active."]
    #[inline(always)]
    pub fn rstcnt(&mut self) -> RSTCNT_W {
        RSTCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Reset Counter Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_rstcnt](index.html) module"]
pub struct WDT_RSTCNT_SPEC;
impl crate::RegisterSpec for WDT_RSTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wdt_rstcnt::W](W) writer structure"]
impl crate::Writable for WDT_RSTCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_RSTCNT to value 0"]
impl crate::Resettable for WDT_RSTCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
