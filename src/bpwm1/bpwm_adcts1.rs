#[doc = "Register `BPWM_ADCTS1` reader"]
pub struct R(crate::R<BPWM_ADCTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_ADCTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_ADCTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_ADCTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_ADCTS1` writer"]
pub struct W(crate::W<BPWM_ADCTS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_ADCTS1_SPEC>;
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
impl From<crate::W<BPWM_ADCTS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_ADCTS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM_CH4 Trigger ADC Source Select\nOthers reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL4_A {
    #[doc = "0: BPWM_CH4 zero point"]
    _0 = 0,
    #[doc = "1: BPWM_CH4 period point"]
    _1 = 1,
    #[doc = "2: BPWM_CH4 zero or period point"]
    _2 = 2,
    #[doc = "3: BPWM_CH4 up-count CMPDAT point"]
    _3 = 3,
    #[doc = "4: BPWM_CH4 down-count CMPDAT point"]
    _4 = 4,
    #[doc = "5: Reserved."]
    _5 = 5,
    #[doc = "6: Reserved."]
    _6 = 6,
    #[doc = "7: Reserved."]
    _7 = 7,
    #[doc = "8: BPWM_CH5 up-count CMPDAT point"]
    _8 = 8,
    #[doc = "9: BPWM_CH5 down-count CMPDAT point"]
    _9 = 9,
}
impl From<TRGSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL4` reader - BPWM_CH4 Trigger ADC Source Select\nOthers reserved"]
pub struct TRGSEL4_R(crate::FieldReader<u8, TRGSEL4_A>);
impl TRGSEL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGSEL4_A> {
        match self.bits {
            0 => Some(TRGSEL4_A::_0),
            1 => Some(TRGSEL4_A::_1),
            2 => Some(TRGSEL4_A::_2),
            3 => Some(TRGSEL4_A::_3),
            4 => Some(TRGSEL4_A::_4),
            5 => Some(TRGSEL4_A::_5),
            6 => Some(TRGSEL4_A::_6),
            7 => Some(TRGSEL4_A::_7),
            8 => Some(TRGSEL4_A::_8),
            9 => Some(TRGSEL4_A::_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSEL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSEL4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGSEL4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGSEL4_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TRGSEL4_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TRGSEL4_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TRGSEL4_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TRGSEL4_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == TRGSEL4_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == TRGSEL4_A::_9
    }
}
impl core::ops::Deref for TRGSEL4_R {
    type Target = crate::FieldReader<u8, TRGSEL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL4` writer - BPWM_CH4 Trigger ADC Source Select\nOthers reserved"]
pub struct TRGSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BPWM_CH4 zero point"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_0)
    }
    #[doc = "BPWM_CH4 period point"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_1)
    }
    #[doc = "BPWM_CH4 zero or period point"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_2)
    }
    #[doc = "BPWM_CH4 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_3)
    }
    #[doc = "BPWM_CH4 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_4)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_5)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_6)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_7)
    }
    #[doc = "BPWM_CH5 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_8)
    }
    #[doc = "BPWM_CH5 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TRGEN4` reader - BPWM_CH4 Trigger ADC Enable Bit"]
pub struct TRGEN4_R(crate::FieldReader<bool, bool>);
impl TRGEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN4` writer - BPWM_CH4 Trigger ADC Enable Bit"]
pub struct TRGEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN4_W<'a> {
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
#[doc = "BPWM_CH5 Trigger ADC Source Select\nOthers reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL5_A {
    #[doc = "0: BPWM_CH4 zero point"]
    _0 = 0,
    #[doc = "1: BPWM_CH4 period point"]
    _1 = 1,
    #[doc = "2: BPWM_CH4 zero or period point"]
    _2 = 2,
    #[doc = "3: BPWM_CH4 up-count CMPDAT point"]
    _3 = 3,
    #[doc = "4: BPWM_CH4 down-count CMPDAT point"]
    _4 = 4,
    #[doc = "5: Reserved."]
    _5 = 5,
    #[doc = "6: Reserved."]
    _6 = 6,
    #[doc = "7: Reserved."]
    _7 = 7,
    #[doc = "8: BPWM_CH5 up-count CMPDAT point"]
    _8 = 8,
    #[doc = "9: BPWM_CH5 down-count CMPDAT point"]
    _9 = 9,
}
impl From<TRGSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL5` reader - BPWM_CH5 Trigger ADC Source Select\nOthers reserved"]
pub struct TRGSEL5_R(crate::FieldReader<u8, TRGSEL5_A>);
impl TRGSEL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGSEL5_A> {
        match self.bits {
            0 => Some(TRGSEL5_A::_0),
            1 => Some(TRGSEL5_A::_1),
            2 => Some(TRGSEL5_A::_2),
            3 => Some(TRGSEL5_A::_3),
            4 => Some(TRGSEL5_A::_4),
            5 => Some(TRGSEL5_A::_5),
            6 => Some(TRGSEL5_A::_6),
            7 => Some(TRGSEL5_A::_7),
            8 => Some(TRGSEL5_A::_8),
            9 => Some(TRGSEL5_A::_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSEL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSEL5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGSEL5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGSEL5_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TRGSEL5_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TRGSEL5_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TRGSEL5_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TRGSEL5_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == TRGSEL5_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == TRGSEL5_A::_9
    }
}
impl core::ops::Deref for TRGSEL5_R {
    type Target = crate::FieldReader<u8, TRGSEL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL5` writer - BPWM_CH5 Trigger ADC Source Select\nOthers reserved"]
pub struct TRGSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BPWM_CH4 zero point"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_0)
    }
    #[doc = "BPWM_CH4 period point"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_1)
    }
    #[doc = "BPWM_CH4 zero or period point"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_2)
    }
    #[doc = "BPWM_CH4 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_3)
    }
    #[doc = "BPWM_CH4 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_4)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_5)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_6)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_7)
    }
    #[doc = "BPWM_CH5 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_8)
    }
    #[doc = "BPWM_CH5 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TRGEN5` reader - BPWM_CH5 Trigger ADC Enable Bit"]
pub struct TRGEN5_R(crate::FieldReader<bool, bool>);
impl TRGEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN5` writer - BPWM_CH5 Trigger ADC Enable Bit"]
pub struct TRGEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - BPWM_CH4 Trigger ADC Source Select Others reserved"]
    #[inline(always)]
    pub fn trgsel4(&self) -> TRGSEL4_R {
        TRGSEL4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - BPWM_CH4 Trigger ADC Enable Bit"]
    #[inline(always)]
    pub fn trgen4(&self) -> TRGEN4_R {
        TRGEN4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - BPWM_CH5 Trigger ADC Source Select Others reserved"]
    #[inline(always)]
    pub fn trgsel5(&self) -> TRGSEL5_R {
        TRGSEL5_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - BPWM_CH5 Trigger ADC Enable Bit"]
    #[inline(always)]
    pub fn trgen5(&self) -> TRGEN5_R {
        TRGEN5_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - BPWM_CH4 Trigger ADC Source Select Others reserved"]
    #[inline(always)]
    pub fn trgsel4(&mut self) -> TRGSEL4_W {
        TRGSEL4_W { w: self }
    }
    #[doc = "Bit 7 - BPWM_CH4 Trigger ADC Enable Bit"]
    #[inline(always)]
    pub fn trgen4(&mut self) -> TRGEN4_W {
        TRGEN4_W { w: self }
    }
    #[doc = "Bits 8:11 - BPWM_CH5 Trigger ADC Source Select Others reserved"]
    #[inline(always)]
    pub fn trgsel5(&mut self) -> TRGSEL5_W {
        TRGSEL5_W { w: self }
    }
    #[doc = "Bit 15 - BPWM_CH5 Trigger ADC Enable Bit"]
    #[inline(always)]
    pub fn trgen5(&mut self) -> TRGEN5_W {
        TRGEN5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Trigger ADC Source Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_adcts1](index.html) module"]
pub struct BPWM_ADCTS1_SPEC;
impl crate::RegisterSpec for BPWM_ADCTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_adcts1::R](R) reader structure"]
impl crate::Readable for BPWM_ADCTS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_adcts1::W](W) writer structure"]
impl crate::Writable for BPWM_ADCTS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_ADCTS1 to value 0"]
impl crate::Resettable for BPWM_ADCTS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
