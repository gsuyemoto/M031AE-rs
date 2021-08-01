#[doc = "Register `CLK_HXTFSEL` reader"]
pub struct R(crate::R<CLK_HXTFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_HXTFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_HXTFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_HXTFSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_HXTFSEL` writer"]
pub struct W(crate::W<CLK_HXTFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_HXTFSEL_SPEC>;
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
impl From<crate::W<CLK_HXTFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_HXTFSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HXT Filter Select \nNote: This bit should not be changed during HXT running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTFSEL_A {
    #[doc = "0: HXT frequency is greater than12 MHz"]
    _0 = 0,
    #[doc = "1: HXT frequency is less than or equal to 12 MHz"]
    _1 = 1,
}
impl From<HXTFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HXTFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTFSEL` reader - HXT Filter Select \nNote: This bit should not be changed during HXT running."]
pub struct HXTFSEL_R(crate::FieldReader<bool, HXTFSEL_A>);
impl HXTFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTFSEL_A {
        match self.bits {
            false => HXTFSEL_A::_0,
            true => HXTFSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTFSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTFSEL_A::_1
    }
}
impl core::ops::Deref for HXTFSEL_R {
    type Target = crate::FieldReader<bool, HXTFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTFSEL` writer - HXT Filter Select \nNote: This bit should not be changed during HXT running."]
pub struct HXTFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTFSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HXT frequency is greater than12 MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTFSEL_A::_0)
    }
    #[doc = "HXT frequency is less than or equal to 12 MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTFSEL_A::_1)
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
    #[doc = "Bit 0 - HXT Filter Select Note: This bit should not be changed during HXT running."]
    #[inline(always)]
    pub fn hxtfsel(&self) -> HXTFSEL_R {
        HXTFSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HXT Filter Select Note: This bit should not be changed during HXT running."]
    #[inline(always)]
    pub fn hxtfsel(&mut self) -> HXTFSEL_W {
        HXTFSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HXT Filter Select Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_hxtfsel](index.html) module"]
pub struct CLK_HXTFSEL_SPEC;
impl crate::RegisterSpec for CLK_HXTFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_hxtfsel::R](R) reader structure"]
impl crate::Readable for CLK_HXTFSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_hxtfsel::W](W) writer structure"]
impl crate::Writable for CLK_HXTFSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_HXTFSEL to value 0"]
impl crate::Resettable for CLK_HXTFSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
