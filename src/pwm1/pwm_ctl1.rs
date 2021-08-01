#[doc = "Register `PWM_CTL1` reader"]
pub struct R(crate::R<PWM_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CTL1` writer"]
pub struct W(crate::W<PWM_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CTL1_SPEC>;
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
impl From<crate::W<PWM_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Counter Behavior Type 0\nThe two bits control channel1 and channel0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTTYPE0_A {
    #[doc = "0: Up counter type (supported in capture mode)"]
    _0 = 0,
    #[doc = "1: Down count type (supported in capture mode)"]
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
#[doc = "Field `CNTTYPE0` reader - PWM Counter Behavior Type 0\nThe two bits control channel1 and channel0"]
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
#[doc = "Field `CNTTYPE0` writer - PWM Counter Behavior Type 0\nThe two bits control channel1 and channel0"]
pub struct CNTTYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTTYPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTTYPE0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Up counter type (supported in capture mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTTYPE0_A::_0)
    }
    #[doc = "Down count type (supported in capture mode)"]
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
#[doc = "PWM Counter Behavior Type 2\nThe two bits control channel3 and channel2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTTYPE2_A {
    #[doc = "0: Up counter type (supported in capture mode)"]
    _0 = 0,
    #[doc = "1: Down count type (supported in capture mode)"]
    _1 = 1,
    #[doc = "2: Up-down counter type"]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<CNTTYPE2_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTTYPE2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTTYPE2` reader - PWM Counter Behavior Type 2\nThe two bits control channel3 and channel2"]
pub struct CNTTYPE2_R(crate::FieldReader<u8, CNTTYPE2_A>);
impl CNTTYPE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTTYPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTTYPE2_A {
        match self.bits {
            0 => CNTTYPE2_A::_0,
            1 => CNTTYPE2_A::_1,
            2 => CNTTYPE2_A::_2,
            3 => CNTTYPE2_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTTYPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTTYPE2_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CNTTYPE2_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CNTTYPE2_A::_3
    }
}
impl core::ops::Deref for CNTTYPE2_R {
    type Target = crate::FieldReader<u8, CNTTYPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTTYPE2` writer - PWM Counter Behavior Type 2\nThe two bits control channel3 and channel2"]
pub struct CNTTYPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTTYPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTTYPE2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Up counter type (supported in capture mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTTYPE2_A::_0)
    }
    #[doc = "Down count type (supported in capture mode)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTTYPE2_A::_1)
    }
    #[doc = "Up-down counter type"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CNTTYPE2_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CNTTYPE2_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "PWM Counter Behavior Type 4\nThe two bits control channel5 and channel4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTTYPE4_A {
    #[doc = "0: Up counter type (supported in capture mode)"]
    _0 = 0,
    #[doc = "1: Down count type (supported in capture mode)"]
    _1 = 1,
    #[doc = "2: Up-down counter type"]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<CNTTYPE4_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTTYPE4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTTYPE4` reader - PWM Counter Behavior Type 4\nThe two bits control channel5 and channel4"]
pub struct CNTTYPE4_R(crate::FieldReader<u8, CNTTYPE4_A>);
impl CNTTYPE4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTTYPE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTTYPE4_A {
        match self.bits {
            0 => CNTTYPE4_A::_0,
            1 => CNTTYPE4_A::_1,
            2 => CNTTYPE4_A::_2,
            3 => CNTTYPE4_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTTYPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTTYPE4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CNTTYPE4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CNTTYPE4_A::_3
    }
}
impl core::ops::Deref for CNTTYPE4_R {
    type Target = crate::FieldReader<u8, CNTTYPE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTTYPE4` writer - PWM Counter Behavior Type 4\nThe two bits control channel5 and channel4"]
pub struct CNTTYPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTTYPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTTYPE4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Up counter type (supported in capture mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTTYPE4_A::_0)
    }
    #[doc = "Down count type (supported in capture mode)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTTYPE4_A::_1)
    }
    #[doc = "Up-down counter type"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CNTTYPE4_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CNTTYPE4_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "PWM Output Mode\nEach bit n controls the output mode of corresponding PWM channel n.\nNote: When operating in group function, these bits must all set to the same mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTMODEN_A {
    #[doc = "0: PWM independent mode"]
    _0 = 0,
    #[doc = "1: PWM complementary mode"]
    _1 = 1,
}
impl From<OUTMODEN_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMODEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUTMODEn` reader - PWM Output Mode\nEach bit n controls the output mode of corresponding PWM channel n.\nNote: When operating in group function, these bits must all set to the same mode."]
pub struct OUTMODEN_R(crate::FieldReader<u8, OUTMODEN_A>);
impl OUTMODEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUTMODEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUTMODEN_A> {
        match self.bits {
            0 => Some(OUTMODEN_A::_0),
            1 => Some(OUTMODEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OUTMODEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OUTMODEN_A::_1
    }
}
impl core::ops::Deref for OUTMODEN_R {
    type Target = crate::FieldReader<u8, OUTMODEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTMODEn` writer - PWM Output Mode\nEach bit n controls the output mode of corresponding PWM channel n.\nNote: When operating in group function, these bits must all set to the same mode."]
pub struct OUTMODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTMODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTMODEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM independent mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUTMODEN_A::_0)
    }
    #[doc = "PWM complementary mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUTMODEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PWM Counter Behavior Type 0 The two bits control channel1 and channel0"]
    #[inline(always)]
    pub fn cnttype0(&self) -> CNTTYPE0_R {
        CNTTYPE0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PWM Counter Behavior Type 2 The two bits control channel3 and channel2"]
    #[inline(always)]
    pub fn cnttype2(&self) -> CNTTYPE2_R {
        CNTTYPE2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PWM Counter Behavior Type 4 The two bits control channel5 and channel4"]
    #[inline(always)]
    pub fn cnttype4(&self) -> CNTTYPE4_R {
        CNTTYPE4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - PWM Output Mode Each bit n controls the output mode of corresponding PWM channel n. Note: When operating in group function, these bits must all set to the same mode."]
    #[inline(always)]
    pub fn outmoden(&self) -> OUTMODEN_R {
        OUTMODEN_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM Counter Behavior Type 0 The two bits control channel1 and channel0"]
    #[inline(always)]
    pub fn cnttype0(&mut self) -> CNTTYPE0_W {
        CNTTYPE0_W { w: self }
    }
    #[doc = "Bits 4:5 - PWM Counter Behavior Type 2 The two bits control channel3 and channel2"]
    #[inline(always)]
    pub fn cnttype2(&mut self) -> CNTTYPE2_W {
        CNTTYPE2_W { w: self }
    }
    #[doc = "Bits 8:9 - PWM Counter Behavior Type 4 The two bits control channel5 and channel4"]
    #[inline(always)]
    pub fn cnttype4(&mut self) -> CNTTYPE4_W {
        CNTTYPE4_W { w: self }
    }
    #[doc = "Bits 24:26 - PWM Output Mode Each bit n controls the output mode of corresponding PWM channel n. Note: When operating in group function, these bits must all set to the same mode."]
    #[inline(always)]
    pub fn outmoden(&mut self) -> OUTMODEN_W {
        OUTMODEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ctl1](index.html) module"]
pub struct PWM_CTL1_SPEC;
impl crate::RegisterSpec for PWM_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ctl1::R](R) reader structure"]
impl crate::Readable for PWM_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ctl1::W](W) writer structure"]
impl crate::Writable for PWM_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CTL1 to value 0"]
impl crate::Resettable for PWM_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
