#[doc = "Register `BPWM_CTL1` reader"]
pub struct R(crate::R<BPWM_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CTL1` writer"]
pub struct W(crate::W<BPWM_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CTL1_SPEC>;
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
impl From<crate::W<BPWM_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM Counter Behavior Type 0\nEach bit n controls corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTTYPE0_A {
    #[doc = "0: Up counter type (supports in capture mode)"]
    _0 = 0,
    #[doc = "1: Down count type (supports in capture mode)"]
    _1 = 1,
    #[doc = "2: Up-down counter type"]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<CNTTYPE0_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTTYPE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTTYPE0` reader - BPWM Counter Behavior Type 0\nEach bit n controls corresponding BPWM channel n."]
pub struct CNTTYPE0_R(crate::FieldReader<u8, CNTTYPE0_A>);
impl CNTTYPE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTTYPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTTYPE0_A {
        match self.bits {
            0 => CNTTYPE0_A::_0,
            1 => CNTTYPE0_A::_1,
            2 => CNTTYPE0_A::_2,
            3 => CNTTYPE0_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTTYPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTTYPE0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CNTTYPE0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CNTTYPE0_A::_3
    }
}
impl core::ops::Deref for CNTTYPE0_R {
    type Target = crate::FieldReader<u8, CNTTYPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTTYPE0` writer - BPWM Counter Behavior Type 0\nEach bit n controls corresponding BPWM channel n."]
pub struct CNTTYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTTYPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTTYPE0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Up counter type (supports in capture mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTTYPE0_A::_0)
    }
    #[doc = "Down count type (supports in capture mode)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTTYPE0_A::_1)
    }
    #[doc = "Up-down counter type"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CNTTYPE0_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CNTTYPE0_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BPWM Counter Behavior Type 0 Each bit n controls corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cnttype0(&self) -> CNTTYPE0_R {
        CNTTYPE0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BPWM Counter Behavior Type 0 Each bit n controls corresponding BPWM channel n."]
    #[inline(always)]
    pub fn cnttype0(&mut self) -> CNTTYPE0_W {
        CNTTYPE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_ctl1](index.html) module"]
pub struct BPWM_CTL1_SPEC;
impl crate::RegisterSpec for BPWM_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_ctl1::R](R) reader structure"]
impl crate::Readable for BPWM_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_ctl1::W](W) writer structure"]
impl crate::Writable for BPWM_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CTL1 to value 0"]
impl crate::Resettable for BPWM_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
