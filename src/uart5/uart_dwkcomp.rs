#[doc = "Register `UART_DWKCOMP` reader"]
pub struct R(crate::R<UART_DWKCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_DWKCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_DWKCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_DWKCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_DWKCOMP` writer"]
pub struct W(crate::W<UART_DWKCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_DWKCOMP_SPEC>;
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
impl From<crate::W<UART_DWKCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_DWKCOMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCOMP` reader - Start Bit Compensation Value\nThese bits field indicate how many clock cycle selected by UART_CLK do the UART controller can get the 1st bit (start bit) when the device is wake-up from Power-down mode.\nNote: It is valid only when WKDATEN (UART_WKCTL\\[1\\]) is set."]
pub struct STCOMP_R(crate::FieldReader<u16, u16>);
impl STCOMP_R {
    pub(crate) fn new(bits: u16) -> Self {
        STCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STCOMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STCOMP` writer - Start Bit Compensation Value\nThese bits field indicate how many clock cycle selected by UART_CLK do the UART controller can get the 1st bit (start bit) when the device is wake-up from Power-down mode.\nNote: It is valid only when WKDATEN (UART_WKCTL\\[1\\]) is set."]
pub struct STCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> STCOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Start Bit Compensation Value These bits field indicate how many clock cycle selected by UART_CLK do the UART controller can get the 1st bit (start bit) when the device is wake-up from Power-down mode. Note: It is valid only when WKDATEN (UART_WKCTL\\[1\\]) is set."]
    #[inline(always)]
    pub fn stcomp(&self) -> STCOMP_R {
        STCOMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start Bit Compensation Value These bits field indicate how many clock cycle selected by UART_CLK do the UART controller can get the 1st bit (start bit) when the device is wake-up from Power-down mode. Note: It is valid only when WKDATEN (UART_WKCTL\\[1\\]) is set."]
    #[inline(always)]
    pub fn stcomp(&mut self) -> STCOMP_W {
        STCOMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Incoming Data Wake-up Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_dwkcomp](index.html) module"]
pub struct UART_DWKCOMP_SPEC;
impl crate::RegisterSpec for UART_DWKCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_dwkcomp::R](R) reader structure"]
impl crate::Readable for UART_DWKCOMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_dwkcomp::W](W) writer structure"]
impl crate::Writable for UART_DWKCOMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_DWKCOMP to value 0"]
impl crate::Resettable for UART_DWKCOMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
