#[doc = "Register `PDMA_TOUTPSC` reader"]
pub struct R(crate::R<PDMA_TOUTPSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TOUTPSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_TOUTPSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_TOUTPSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_TOUTPSC` writer"]
pub struct W(crate::W<PDMA_TOUTPSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_TOUTPSC_SPEC>;
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
impl From<crate::W<PDMA_TOUTPSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_TOUTPSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Channel 0 Time-out Clock Source Prescaler Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOUTPSC0_A {
    #[doc = "0: PDMA channel 0 time-out clock source is HCLK/28"]
    _0 = 0,
    #[doc = "1: PDMA channel 0 time-out clock source is HCLK/29"]
    _1 = 1,
    #[doc = "2: PDMA channel 0 time-out clock source is HCLK/210"]
    _2 = 2,
    #[doc = "3: PDMA channel 0 time-out clock source is HCLK/211"]
    _3 = 3,
    #[doc = "4: PDMA channel 0 time-out clock source is HCLK/212"]
    _4 = 4,
    #[doc = "5: PDMA channel 0 time-out clock source is HCLK/213"]
    _5 = 5,
    #[doc = "6: PDMA channel 0 time-out clock source is HCLK/214"]
    _6 = 6,
    #[doc = "7: PDMA channel 0 time-out clock source is HCLK/215"]
    _7 = 7,
}
impl From<TOUTPSC0_A> for u8 {
    #[inline(always)]
    fn from(variant: TOUTPSC0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TOUTPSC0` reader - PDMA Channel 0 Time-out Clock Source Prescaler Bits"]
pub struct TOUTPSC0_R(crate::FieldReader<u8, TOUTPSC0_A>);
impl TOUTPSC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOUTPSC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTPSC0_A {
        match self.bits {
            0 => TOUTPSC0_A::_0,
            1 => TOUTPSC0_A::_1,
            2 => TOUTPSC0_A::_2,
            3 => TOUTPSC0_A::_3,
            4 => TOUTPSC0_A::_4,
            5 => TOUTPSC0_A::_5,
            6 => TOUTPSC0_A::_6,
            7 => TOUTPSC0_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTPSC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTPSC0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TOUTPSC0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TOUTPSC0_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TOUTPSC0_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TOUTPSC0_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TOUTPSC0_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TOUTPSC0_A::_7
    }
}
impl core::ops::Deref for TOUTPSC0_R {
    type Target = crate::FieldReader<u8, TOUTPSC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTPSC0` writer - PDMA Channel 0 Time-out Clock Source Prescaler Bits"]
pub struct TOUTPSC0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTPSC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTPSC0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PDMA channel 0 time-out clock source is HCLK/28"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTPSC0_A::_0)
    }
    #[doc = "PDMA channel 0 time-out clock source is HCLK/29"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTPSC0_A::_1)
    }
    #[doc = "PDMA channel 0 time-out clock source is HCLK/210"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TOUTPSC0_A::_2)
    }
    #[doc = "PDMA channel 0 time-out clock source is HCLK/211"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TOUTPSC0_A::_3)
    }
    #[doc = "PDMA channel 0 time-out clock source is HCLK/212"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TOUTPSC0_A::_4)
    }
    #[doc = "PDMA channel 0 time-out clock source is HCLK/213"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TOUTPSC0_A::_5)
    }
    #[doc = "PDMA channel 0 time-out clock source is HCLK/214"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TOUTPSC0_A::_6)
    }
    #[doc = "PDMA channel 0 time-out clock source is HCLK/215"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TOUTPSC0_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "PDMA Channel 1 Time-out Clock Source Prescaler Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOUTPSC1_A {
    #[doc = "0: PDMA channel 1 time-out clock source is HCLK/28"]
    _0 = 0,
    #[doc = "1: PDMA channel 1 time-out clock source is HCLK/29"]
    _1 = 1,
    #[doc = "2: PDMA channel 1 time-out clock source is HCLK/210"]
    _2 = 2,
    #[doc = "3: PDMA channel 1 time-out clock source is HCLK/211"]
    _3 = 3,
    #[doc = "4: PDMA channel 1 time-out clock source is HCLK/212"]
    _4 = 4,
    #[doc = "5: PDMA channel 1 time-out clock source is HCLK/213"]
    _5 = 5,
    #[doc = "6: PDMA channel 1 time-out clock source is HCLK/214"]
    _6 = 6,
    #[doc = "7: PDMA channel 1 time-out clock source is HCLK/215"]
    _7 = 7,
}
impl From<TOUTPSC1_A> for u8 {
    #[inline(always)]
    fn from(variant: TOUTPSC1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TOUTPSC1` reader - PDMA Channel 1 Time-out Clock Source Prescaler Bits"]
pub struct TOUTPSC1_R(crate::FieldReader<u8, TOUTPSC1_A>);
impl TOUTPSC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOUTPSC1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTPSC1_A {
        match self.bits {
            0 => TOUTPSC1_A::_0,
            1 => TOUTPSC1_A::_1,
            2 => TOUTPSC1_A::_2,
            3 => TOUTPSC1_A::_3,
            4 => TOUTPSC1_A::_4,
            5 => TOUTPSC1_A::_5,
            6 => TOUTPSC1_A::_6,
            7 => TOUTPSC1_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTPSC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTPSC1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TOUTPSC1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TOUTPSC1_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TOUTPSC1_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TOUTPSC1_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TOUTPSC1_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TOUTPSC1_A::_7
    }
}
impl core::ops::Deref for TOUTPSC1_R {
    type Target = crate::FieldReader<u8, TOUTPSC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTPSC1` writer - PDMA Channel 1 Time-out Clock Source Prescaler Bits"]
pub struct TOUTPSC1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTPSC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTPSC1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PDMA channel 1 time-out clock source is HCLK/28"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTPSC1_A::_0)
    }
    #[doc = "PDMA channel 1 time-out clock source is HCLK/29"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTPSC1_A::_1)
    }
    #[doc = "PDMA channel 1 time-out clock source is HCLK/210"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TOUTPSC1_A::_2)
    }
    #[doc = "PDMA channel 1 time-out clock source is HCLK/211"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TOUTPSC1_A::_3)
    }
    #[doc = "PDMA channel 1 time-out clock source is HCLK/212"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TOUTPSC1_A::_4)
    }
    #[doc = "PDMA channel 1 time-out clock source is HCLK/213"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TOUTPSC1_A::_5)
    }
    #[doc = "PDMA channel 1 time-out clock source is HCLK/214"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TOUTPSC1_A::_6)
    }
    #[doc = "PDMA channel 1 time-out clock source is HCLK/215"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TOUTPSC1_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PDMA Channel 0 Time-out Clock Source Prescaler Bits"]
    #[inline(always)]
    pub fn toutpsc0(&self) -> TOUTPSC0_R {
        TOUTPSC0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - PDMA Channel 1 Time-out Clock Source Prescaler Bits"]
    #[inline(always)]
    pub fn toutpsc1(&self) -> TOUTPSC1_R {
        TOUTPSC1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PDMA Channel 0 Time-out Clock Source Prescaler Bits"]
    #[inline(always)]
    pub fn toutpsc0(&mut self) -> TOUTPSC0_W {
        TOUTPSC0_W { w: self }
    }
    #[doc = "Bits 4:6 - PDMA Channel 1 Time-out Clock Source Prescaler Bits"]
    #[inline(always)]
    pub fn toutpsc1(&mut self) -> TOUTPSC1_W {
        TOUTPSC1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Time-out Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_toutpsc](index.html) module"]
pub struct PDMA_TOUTPSC_SPEC;
impl crate::RegisterSpec for PDMA_TOUTPSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_toutpsc::R](R) reader structure"]
impl crate::Readable for PDMA_TOUTPSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_toutpsc::W](W) writer structure"]
impl crate::Writable for PDMA_TOUTPSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_TOUTPSC to value 0"]
impl crate::Resettable for PDMA_TOUTPSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
