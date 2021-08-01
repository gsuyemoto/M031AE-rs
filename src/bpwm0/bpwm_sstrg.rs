#[doc = "Register `BPWM_SSTRG` writer"]
pub struct W(crate::W<BPWM_SSTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_SSTRG_SPEC>;
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
impl From<crate::W<BPWM_SSTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_SSTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTSEN` writer - BPWM Counter Synchronous Start Enable Bit(Write Only)\nBPMW counter synchronous enable function is used to make PWM or BPWM channels start counting at the same time.\nWriting this bit to 1 will also set the counter enable bit if correlated BPWM channel counter synchronous start function is enabled."]
pub struct CNTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSEN_W<'a> {
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
impl W {
    #[doc = "Bit 0 - BPWM Counter Synchronous Start Enable Bit(Write Only) BPMW counter synchronous enable function is used to make PWM or BPWM channels start counting at the same time. Writing this bit to 1 will also set the counter enable bit if correlated BPWM channel counter synchronous start function is enabled."]
    #[inline(always)]
    pub fn cntsen(&mut self) -> CNTSEN_W {
        CNTSEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Synchronous Start Trigger Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_sstrg](index.html) module"]
pub struct BPWM_SSTRG_SPEC;
impl crate::RegisterSpec for BPWM_SSTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bpwm_sstrg::W](W) writer structure"]
impl crate::Writable for BPWM_SSTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_SSTRG to value 0"]
impl crate::Resettable for BPWM_SSTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
