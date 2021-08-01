#[doc = "Register `BPWM_CNTEN` reader"]
pub struct R(crate::R<BPWM_CNTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CNTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CNTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CNTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CNTEN` writer"]
pub struct W(crate::W<BPWM_CNTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CNTEN_SPEC>;
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
impl From<crate::W<BPWM_CNTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CNTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM Counter 0 Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEN0_A {
    #[doc = "0: BPWM Counter and clock prescaler stop running"]
    _0 = 0,
    #[doc = "1: BPWM Counter and clock prescaler start running"]
    _1 = 1,
}
impl From<CNTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTEN0` reader - BPWM Counter 0 Enable Bit"]
pub struct CNTEN0_R(crate::FieldReader<bool, CNTEN0_A>);
impl CNTEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN0_A {
        match self.bits {
            false => CNTEN0_A::_0,
            true => CNTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTEN0_A::_1
    }
}
impl core::ops::Deref for CNTEN0_R {
    type Target = crate::FieldReader<bool, CNTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTEN0` writer - BPWM Counter 0 Enable Bit"]
pub struct CNTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BPWM Counter and clock prescaler stop running"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN0_A::_0)
    }
    #[doc = "BPWM Counter and clock prescaler start running"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN0_A::_1)
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
    #[doc = "Bit 0 - BPWM Counter 0 Enable Bit"]
    #[inline(always)]
    pub fn cnten0(&self) -> CNTEN0_R {
        CNTEN0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BPWM Counter 0 Enable Bit"]
    #[inline(always)]
    pub fn cnten0(&mut self) -> CNTEN0_W {
        CNTEN0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Counter Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_cnten](index.html) module"]
pub struct BPWM_CNTEN_SPEC;
impl crate::RegisterSpec for BPWM_CNTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_cnten::R](R) reader structure"]
impl crate::Readable for BPWM_CNTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_cnten::W](W) writer structure"]
impl crate::Writable for BPWM_CNTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CNTEN to value 0"]
impl crate::Resettable for BPWM_CNTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
