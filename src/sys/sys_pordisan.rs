#[doc = "Register `SYS_PORDISAN` reader"]
pub struct R(crate::R<SYS_PORDISAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_PORDISAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_PORDISAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_PORDISAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_PORDISAN` writer"]
pub struct W(crate::W<SYS_PORDISAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_PORDISAN_SPEC>;
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
impl From<crate::W<SYS_PORDISAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_PORDISAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POROFFAN` reader - Power-on Reset Enable Bit (Write Protect)\nAfter powered on, User can turn off internal analog POR circuit to save power by writing 0x5AA5 to this field.\nThe analog POR circuit will be active again when this field is set to another value or chip is reset by other reset source, including:\nnRESET, Watchdog, LVR reset, BOD reset, ICE reset command and the software-chip reset function.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct POROFFAN_R(crate::FieldReader<u16, u16>);
impl POROFFAN_R {
    pub(crate) fn new(bits: u16) -> Self {
        POROFFAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POROFFAN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POROFFAN` writer - Power-on Reset Enable Bit (Write Protect)\nAfter powered on, User can turn off internal analog POR circuit to save power by writing 0x5AA5 to this field.\nThe analog POR circuit will be active again when this field is set to another value or chip is reset by other reset source, including:\nnRESET, Watchdog, LVR reset, BOD reset, ICE reset command and the software-chip reset function.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct POROFFAN_W<'a> {
    w: &'a mut W,
}
impl<'a> POROFFAN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Power-on Reset Enable Bit (Write Protect) After powered on, User can turn off internal analog POR circuit to save power by writing 0x5AA5 to this field. The analog POR circuit will be active again when this field is set to another value or chip is reset by other reset source, including: nRESET, Watchdog, LVR reset, BOD reset, ICE reset command and the software-chip reset function. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn poroffan(&self) -> POROFFAN_R {
        POROFFAN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Power-on Reset Enable Bit (Write Protect) After powered on, User can turn off internal analog POR circuit to save power by writing 0x5AA5 to this field. The analog POR circuit will be active again when this field is set to another value or chip is reset by other reset source, including: nRESET, Watchdog, LVR reset, BOD reset, ICE reset command and the software-chip reset function. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn poroffan(&mut self) -> POROFFAN_W {
        POROFFAN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog POR Disable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_pordisan](index.html) module"]
pub struct SYS_PORDISAN_SPEC;
impl crate::RegisterSpec for SYS_PORDISAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_pordisan::R](R) reader structure"]
impl crate::Readable for SYS_PORDISAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_pordisan::W](W) writer structure"]
impl crate::Writable for SYS_PORDISAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_PORDISAN to value 0"]
impl crate::Resettable for SYS_PORDISAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
