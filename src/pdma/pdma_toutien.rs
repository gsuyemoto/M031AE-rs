#[doc = "Register `PDMA_TOUTIEN` reader"]
pub struct R(crate::R<PDMA_TOUTIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TOUTIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_TOUTIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_TOUTIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_TOUTIEN` writer"]
pub struct W(crate::W<PDMA_TOUTIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_TOUTIEN_SPEC>;
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
impl From<crate::W<PDMA_TOUTIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_TOUTIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Time-out Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUTIEN0_A {
    #[doc = "0: PDMA Channel n time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA Channel n time-out interrupt Enabled"]
    _1 = 1,
}
impl From<TOUTIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: TOUTIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOUTIEN0` reader - PDMA Time-out Interrupt Enable Bits"]
pub struct TOUTIEN0_R(crate::FieldReader<bool, TOUTIEN0_A>);
impl TOUTIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUTIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTIEN0_A {
        match self.bits {
            false => TOUTIEN0_A::_0,
            true => TOUTIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTIEN0_A::_1
    }
}
impl core::ops::Deref for TOUTIEN0_R {
    type Target = crate::FieldReader<bool, TOUTIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTIEN0` writer - PDMA Time-out Interrupt Enable Bits"]
pub struct TOUTIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA Channel n time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTIEN0_A::_0)
    }
    #[doc = "PDMA Channel n time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTIEN0_A::_1)
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
#[doc = "PDMA Time-out Interrupt Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUTIEN1_A {
    #[doc = "0: PDMA Channel n time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA Channel n time-out interrupt Enabled"]
    _1 = 1,
}
impl From<TOUTIEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TOUTIEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOUTIEN1` reader - PDMA Time-out Interrupt Enable Bits"]
pub struct TOUTIEN1_R(crate::FieldReader<bool, TOUTIEN1_A>);
impl TOUTIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUTIEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTIEN1_A {
        match self.bits {
            false => TOUTIEN1_A::_0,
            true => TOUTIEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTIEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTIEN1_A::_1
    }
}
impl core::ops::Deref for TOUTIEN1_R {
    type Target = crate::FieldReader<bool, TOUTIEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTIEN1` writer - PDMA Time-out Interrupt Enable Bits"]
pub struct TOUTIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTIEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTIEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA Channel n time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTIEN1_A::_0)
    }
    #[doc = "PDMA Channel n time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTIEN1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDMA Time-out Interrupt Enable Bits"]
    #[inline(always)]
    pub fn toutien0(&self) -> TOUTIEN0_R {
        TOUTIEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDMA Time-out Interrupt Enable Bits"]
    #[inline(always)]
    pub fn toutien1(&self) -> TOUTIEN1_R {
        TOUTIEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Time-out Interrupt Enable Bits"]
    #[inline(always)]
    pub fn toutien0(&mut self) -> TOUTIEN0_W {
        TOUTIEN0_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Time-out Interrupt Enable Bits"]
    #[inline(always)]
    pub fn toutien1(&mut self) -> TOUTIEN1_W {
        TOUTIEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Time-out Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_toutien](index.html) module"]
pub struct PDMA_TOUTIEN_SPEC;
impl crate::RegisterSpec for PDMA_TOUTIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_toutien::R](R) reader structure"]
impl crate::Readable for PDMA_TOUTIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_toutien::W](W) writer structure"]
impl crate::Writable for PDMA_TOUTIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_TOUTIEN to value 0"]
impl crate::Resettable for PDMA_TOUTIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
