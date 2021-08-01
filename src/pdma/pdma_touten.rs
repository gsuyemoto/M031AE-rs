#[doc = "Register `PDMA_TOUTEN` reader"]
pub struct R(crate::R<PDMA_TOUTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TOUTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_TOUTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_TOUTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_TOUTEN` writer"]
pub struct W(crate::W<PDMA_TOUTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_TOUTEN_SPEC>;
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
impl From<crate::W<PDMA_TOUTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_TOUTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Time-out Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUTEN0_A {
    #[doc = "0: PDMA Channel n time-out function Disabled"]
    _0 = 0,
    #[doc = "1: PDMA Channel n time-out function Enabled"]
    _1 = 1,
}
impl From<TOUTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: TOUTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOUTEN0` reader - PDMA Time-out Enable Bits"]
pub struct TOUTEN0_R(crate::FieldReader<bool, TOUTEN0_A>);
impl TOUTEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTEN0_A {
        match self.bits {
            false => TOUTEN0_A::_0,
            true => TOUTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTEN0_A::_1
    }
}
impl core::ops::Deref for TOUTEN0_R {
    type Target = crate::FieldReader<bool, TOUTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTEN0` writer - PDMA Time-out Enable Bits"]
pub struct TOUTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA Channel n time-out function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTEN0_A::_0)
    }
    #[doc = "PDMA Channel n time-out function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTEN0_A::_1)
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
#[doc = "PDMA Time-out Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUTEN1_A {
    #[doc = "0: PDMA Channel n time-out function Disabled"]
    _0 = 0,
    #[doc = "1: PDMA Channel n time-out function Enabled"]
    _1 = 1,
}
impl From<TOUTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TOUTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOUTEN1` reader - PDMA Time-out Enable Bits"]
pub struct TOUTEN1_R(crate::FieldReader<bool, TOUTEN1_A>);
impl TOUTEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUTEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTEN1_A {
        match self.bits {
            false => TOUTEN1_A::_0,
            true => TOUTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTEN1_A::_1
    }
}
impl core::ops::Deref for TOUTEN1_R {
    type Target = crate::FieldReader<bool, TOUTEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTEN1` writer - PDMA Time-out Enable Bits"]
pub struct TOUTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA Channel n time-out function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTEN1_A::_0)
    }
    #[doc = "PDMA Channel n time-out function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTEN1_A::_1)
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
    #[doc = "Bit 0 - PDMA Time-out Enable Bits"]
    #[inline(always)]
    pub fn touten0(&self) -> TOUTEN0_R {
        TOUTEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDMA Time-out Enable Bits"]
    #[inline(always)]
    pub fn touten1(&self) -> TOUTEN1_R {
        TOUTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Time-out Enable Bits"]
    #[inline(always)]
    pub fn touten0(&mut self) -> TOUTEN0_W {
        TOUTEN0_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Time-out Enable Bits"]
    #[inline(always)]
    pub fn touten1(&mut self) -> TOUTEN1_W {
        TOUTEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Time-out Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_touten](index.html) module"]
pub struct PDMA_TOUTEN_SPEC;
impl crate::RegisterSpec for PDMA_TOUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_touten::R](R) reader structure"]
impl crate::Readable for PDMA_TOUTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_touten::W](W) writer structure"]
impl crate::Writable for PDMA_TOUTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_TOUTEN to value 0"]
impl crate::Resettable for PDMA_TOUTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
