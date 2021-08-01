#[doc = "Register `PDMA_CHRST` reader"]
pub struct R(crate::R<PDMA_CHRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_CHRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_CHRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_CHRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_CHRST` writer"]
pub struct W(crate::W<PDMA_CHRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_CHRST_SPEC>;
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
impl From<crate::W<PDMA_CHRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_CHRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel n Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CHNRST_A {
    #[doc = "0: corresponding channel n is not reset"]
    _0 = 0,
    #[doc = "1: corresponding channel n is reset"]
    _1 = 1,
}
impl From<CHNRST_A> for u16 {
    #[inline(always)]
    fn from(variant: CHNRST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHnRST` reader - Channel n Reset"]
pub struct CHNRST_R(crate::FieldReader<u16, CHNRST_A>);
impl CHNRST_R {
    pub(crate) fn new(bits: u16) -> Self {
        CHNRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHNRST_A> {
        match self.bits {
            0 => Some(CHNRST_A::_0),
            1 => Some(CHNRST_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHNRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHNRST_A::_1
    }
}
impl core::ops::Deref for CHNRST_R {
    type Target = crate::FieldReader<u16, CHNRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHnRST` writer - Channel n Reset"]
pub struct CHNRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHNRST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "corresponding channel n is not reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHNRST_A::_0)
    }
    #[doc = "corresponding channel n is reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHNRST_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Channel n Reset"]
    #[inline(always)]
    pub fn chn_rst(&self) -> CHNRST_R {
        CHNRST_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Channel n Reset"]
    #[inline(always)]
    pub fn chn_rst(&mut self) -> CHNRST_W {
        CHNRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Channel Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_chrst](index.html) module"]
pub struct PDMA_CHRST_SPEC;
impl crate::RegisterSpec for PDMA_CHRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_chrst::R](R) reader structure"]
impl crate::Readable for PDMA_CHRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_chrst::W](W) writer structure"]
impl crate::Writable for PDMA_CHRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_CHRST to value 0"]
impl crate::Resettable for PDMA_CHRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
