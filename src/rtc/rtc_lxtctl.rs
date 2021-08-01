#[doc = "Register `RTC_LXTCTL` reader"]
pub struct R(crate::R<RTC_LXTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_LXTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_LXTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_LXTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_LXTCTL` writer"]
pub struct W(crate::W<RTC_LXTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_LXTCTL_SPEC>;
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
impl From<crate::W<RTC_LXTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_LXTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock 32K Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC32_S_A {
    #[doc = "0: Clock source from LXT32K"]
    _0 = 0,
    #[doc = "1: Clock source from LIRC38K"]
    _1 = 1,
}
impl From<OSC32_S_A> for bool {
    #[inline(always)]
    fn from(variant: OSC32_S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSC32_S` reader - Clock 32K Source Selection"]
pub struct OSC32_S_R(crate::FieldReader<bool, OSC32_S_A>);
impl OSC32_S_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSC32_S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC32_S_A {
        match self.bits {
            false => OSC32_S_A::_0,
            true => OSC32_S_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OSC32_S_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OSC32_S_A::_1
    }
}
impl core::ops::Deref for OSC32_S_R {
    type Target = crate::FieldReader<bool, OSC32_S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC32_S` writer - Clock 32K Source Selection"]
pub struct OSC32_S_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC32_S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock source from LXT32K"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSC32_S_A::_0)
    }
    #[doc = "Clock source from LIRC38K"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSC32_S_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Clock 32K Source Selection"]
    #[inline(always)]
    pub fn osc32_s(&self) -> OSC32_S_R {
        OSC32_S_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Clock 32K Source Selection"]
    #[inline(always)]
    pub fn osc32_s(&mut self) -> OSC32_S_W {
        OSC32_S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC 32K.768 KHz LXT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_lxtctl](index.html) module"]
pub struct RTC_LXTCTL_SPEC;
impl crate::RegisterSpec for RTC_LXTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_lxtctl::R](R) reader structure"]
impl crate::Readable for RTC_LXTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_lxtctl::W](W) writer structure"]
impl crate::Writable for RTC_LXTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_LXTCTL to value 0"]
impl crate::Resettable for RTC_LXTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
