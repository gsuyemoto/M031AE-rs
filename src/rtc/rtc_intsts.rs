#[doc = "Register `RTC_INTSTS` reader"]
pub struct R(crate::R<RTC_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_INTSTS` writer"]
pub struct W(crate::W<RTC_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_INTSTS_SPEC>;
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
impl From<crate::W<RTC_INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC Alarm Interrupt Flag\nNote: Write 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALMIF_A {
    #[doc = "0: Alarm condition is not matched"]
    _0 = 0,
    #[doc = "1: Alarm condition is matched"]
    _1 = 1,
}
impl From<ALMIF_A> for bool {
    #[inline(always)]
    fn from(variant: ALMIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALMIF` reader - RTC Alarm Interrupt Flag\nNote: Write 1 to clear this bit."]
pub struct ALMIF_R(crate::FieldReader<bool, ALMIF_A>);
impl ALMIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALMIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALMIF_A {
        match self.bits {
            false => ALMIF_A::_0,
            true => ALMIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALMIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALMIF_A::_1
    }
}
impl core::ops::Deref for ALMIF_R {
    type Target = crate::FieldReader<bool, ALMIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMIF` writer - RTC Alarm Interrupt Flag\nNote: Write 1 to clear this bit."]
pub struct ALMIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALMIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Alarm condition is not matched"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALMIF_A::_0)
    }
    #[doc = "Alarm condition is matched"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALMIF_A::_1)
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
#[doc = "RTC Time Tick Interrupt Flag\nNote: Write 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKIF_A {
    #[doc = "0: Tick condition did not occur"]
    _0 = 0,
    #[doc = "1: Tick condition occurred"]
    _1 = 1,
}
impl From<TICKIF_A> for bool {
    #[inline(always)]
    fn from(variant: TICKIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKIF` reader - RTC Time Tick Interrupt Flag\nNote: Write 1 to clear this bit."]
pub struct TICKIF_R(crate::FieldReader<bool, TICKIF_A>);
impl TICKIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TICKIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICKIF_A {
        match self.bits {
            false => TICKIF_A::_0,
            true => TICKIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TICKIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TICKIF_A::_1
    }
}
impl core::ops::Deref for TICKIF_R {
    type Target = crate::FieldReader<bool, TICKIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICKIF` writer - RTC Time Tick Interrupt Flag\nNote: Write 1 to clear this bit."]
pub struct TICKIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICKIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tick condition did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TICKIF_A::_0)
    }
    #[doc = "Tick condition occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TICKIF_A::_1)
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
    #[doc = "Bit 0 - RTC Alarm Interrupt Flag Note: Write 1 to clear this bit."]
    #[inline(always)]
    pub fn almif(&self) -> ALMIF_R {
        ALMIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Time Tick Interrupt Flag Note: Write 1 to clear this bit."]
    #[inline(always)]
    pub fn tickif(&self) -> TICKIF_R {
        TICKIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alarm Interrupt Flag Note: Write 1 to clear this bit."]
    #[inline(always)]
    pub fn almif(&mut self) -> ALMIF_W {
        ALMIF_W { w: self }
    }
    #[doc = "Bit 1 - RTC Time Tick Interrupt Flag Note: Write 1 to clear this bit."]
    #[inline(always)]
    pub fn tickif(&mut self) -> TICKIF_W {
        TICKIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_intsts](index.html) module"]
pub struct RTC_INTSTS_SPEC;
impl crate::RegisterSpec for RTC_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_intsts::R](R) reader structure"]
impl crate::Readable for RTC_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_intsts::W](W) writer structure"]
impl crate::Writable for RTC_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_INTSTS to value 0"]
impl crate::Resettable for RTC_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
