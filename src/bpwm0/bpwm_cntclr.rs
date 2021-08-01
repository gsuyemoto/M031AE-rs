#[doc = "Register `BPWM_CNTCLR` reader"]
pub struct R(crate::R<BPWM_CNTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CNTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CNTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CNTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CNTCLR` writer"]
pub struct W(crate::W<BPWM_CNTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CNTCLR_SPEC>;
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
impl From<crate::W<BPWM_CNTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CNTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear BPWM Counter Control Bit 0\nIt is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTCLR0_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear 16-bit BPWM counter to 0000H"]
    _1 = 1,
}
impl From<CNTCLR0_A> for bool {
    #[inline(always)]
    fn from(variant: CNTCLR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTCLR0` reader - Clear BPWM Counter Control Bit 0\nIt is automatically cleared by hardware."]
pub struct CNTCLR0_R(crate::FieldReader<bool, CNTCLR0_A>);
impl CNTCLR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTCLR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTCLR0_A {
        match self.bits {
            false => CNTCLR0_A::_0,
            true => CNTCLR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTCLR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTCLR0_A::_1
    }
}
impl core::ops::Deref for CNTCLR0_R {
    type Target = crate::FieldReader<bool, CNTCLR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTCLR0` writer - Clear BPWM Counter Control Bit 0\nIt is automatically cleared by hardware."]
pub struct CNTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCLR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTCLR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTCLR0_A::_0)
    }
    #[doc = "Clear 16-bit BPWM counter to 0000H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTCLR0_A::_1)
    }
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
    #[doc = "Bit 0 - Clear BPWM Counter Control Bit 0 It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn cntclr0(&self) -> CNTCLR0_R {
        CNTCLR0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear BPWM Counter Control Bit 0 It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn cntclr0(&mut self) -> CNTCLR0_W {
        CNTCLR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Clear Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_cntclr](index.html) module"]
pub struct BPWM_CNTCLR_SPEC;
impl crate::RegisterSpec for BPWM_CNTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_cntclr::R](R) reader structure"]
impl crate::Readable for BPWM_CNTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_cntclr::W](W) writer structure"]
impl crate::Writable for BPWM_CNTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CNTCLR to value 0"]
impl crate::Resettable for BPWM_CNTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
