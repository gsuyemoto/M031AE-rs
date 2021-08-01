#[doc = "Register `SYS_SRAM_STATUS` reader"]
pub struct R(crate::R<SYS_SRAM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_STATUS` writer"]
pub struct W(crate::W<SYS_SRAM_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_STATUS_SPEC>;
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
impl From<crate::W<SYS_SRAM_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM Parity Check Error Flag\nThis bit indicates the System SRAM parity error occurred. Write 1 to clear this to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERRIF_A {
    #[doc = "0: No System SRAM parity error"]
    _0 = 0,
    #[doc = "1: System SRAM parity error occur"]
    _1 = 1,
}
impl From<PERRIF_A> for bool {
    #[inline(always)]
    fn from(variant: PERRIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIF` reader - SRAM Parity Check Error Flag\nThis bit indicates the System SRAM parity error occurred. Write 1 to clear this to 0."]
pub struct PERRIF_R(crate::FieldReader<bool, PERRIF_A>);
impl PERRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERRIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERRIF_A {
        match self.bits {
            false => PERRIF_A::_0,
            true => PERRIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PERRIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PERRIF_A::_1
    }
}
impl core::ops::Deref for PERRIF_R {
    type Target = crate::FieldReader<bool, PERRIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERRIF` writer - SRAM Parity Check Error Flag\nThis bit indicates the System SRAM parity error occurred. Write 1 to clear this to 0."]
pub struct PERRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERRIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No System SRAM parity error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PERRIF_A::_0)
    }
    #[doc = "System SRAM parity error occur"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PERRIF_A::_1)
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
    #[doc = "Bit 0 - SRAM Parity Check Error Flag This bit indicates the System SRAM parity error occurred. Write 1 to clear this to 0."]
    #[inline(always)]
    pub fn perrif(&self) -> PERRIF_R {
        PERRIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM Parity Check Error Flag This bit indicates the System SRAM parity error occurred. Write 1 to clear this to 0."]
    #[inline(always)]
    pub fn perrif(&mut self) -> PERRIF_W {
        PERRIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System SRAM Parity Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_status](index.html) module"]
pub struct SYS_SRAM_STATUS_SPEC;
impl crate::RegisterSpec for SYS_SRAM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_status::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_status::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_STATUS to value 0"]
impl crate::Resettable for SYS_SRAM_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
