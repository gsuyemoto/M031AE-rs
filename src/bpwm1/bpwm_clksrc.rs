#[doc = "Register `BPWM_CLKSRC` reader"]
pub struct R(crate::R<BPWM_CLKSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_CLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_CLKSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_CLKSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_CLKSRC` writer"]
pub struct W(crate::W<BPWM_CLKSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_CLKSRC_SPEC>;
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
impl From<crate::W<BPWM_CLKSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_CLKSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM_CH01 External Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECLKSRC0_A {
    #[doc = "0: BPWMx_CLK, x denotes 0 or 1"]
    _0 = 0,
    #[doc = "1: TIMER0 overflow"]
    _1 = 1,
    #[doc = "2: TIMER1 overflow"]
    _2 = 2,
    #[doc = "3: TIMER2 overflow"]
    _3 = 3,
    #[doc = "4: TIMER3 overflow"]
    _4 = 4,
}
impl From<ECLKSRC0_A> for u8 {
    #[inline(always)]
    fn from(variant: ECLKSRC0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ECLKSRC0` reader - BPWM_CH01 External Clock Source Select"]
pub struct ECLKSRC0_R(crate::FieldReader<u8, ECLKSRC0_A>);
impl ECLKSRC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECLKSRC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECLKSRC0_A> {
        match self.bits {
            0 => Some(ECLKSRC0_A::_0),
            1 => Some(ECLKSRC0_A::_1),
            2 => Some(ECLKSRC0_A::_2),
            3 => Some(ECLKSRC0_A::_3),
            4 => Some(ECLKSRC0_A::_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ECLKSRC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ECLKSRC0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ECLKSRC0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ECLKSRC0_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == ECLKSRC0_A::_4
    }
}
impl core::ops::Deref for ECLKSRC0_R {
    type Target = crate::FieldReader<u8, ECLKSRC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECLKSRC0` writer - BPWM_CH01 External Clock Source Select"]
pub struct ECLKSRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ECLKSRC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECLKSRC0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BPWMx_CLK, x denotes 0 or 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_0)
    }
    #[doc = "TIMER0 overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_1)
    }
    #[doc = "TIMER1 overflow"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_2)
    }
    #[doc = "TIMER2 overflow"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_3)
    }
    #[doc = "TIMER3 overflow"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - BPWM_CH01 External Clock Source Select"]
    #[inline(always)]
    pub fn eclksrc0(&self) -> ECLKSRC0_R {
        ECLKSRC0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BPWM_CH01 External Clock Source Select"]
    #[inline(always)]
    pub fn eclksrc0(&mut self) -> ECLKSRC0_W {
        ECLKSRC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Clock Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_clksrc](index.html) module"]
pub struct BPWM_CLKSRC_SPEC;
impl crate::RegisterSpec for BPWM_CLKSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_clksrc::R](R) reader structure"]
impl crate::Readable for BPWM_CLKSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_clksrc::W](W) writer structure"]
impl crate::Writable for BPWM_CLKSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_CLKSRC to value 0"]
impl crate::Resettable for BPWM_CLKSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
