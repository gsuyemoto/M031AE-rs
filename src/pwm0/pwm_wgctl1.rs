#[doc = "Register `PWM_WGCTL1` reader"]
pub struct R(crate::R<PWM_WGCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_WGCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_WGCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_WGCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_WGCTL1` writer"]
pub struct W(crate::W<PWM_WGCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_WGCTL1_SPEC>;
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
impl From<crate::W<PWM_WGCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_WGCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPUCTL0_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare up point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare up point output High"]
    _2 = 2,
    #[doc = "3: PWM compare up point output Toggle"]
    _3 = 3,
}
impl From<CMPUCTL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPUCTL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPUCTL0` reader - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL0_R(crate::FieldReader<u8, CMPUCTL0_A>);
impl CMPUCTL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPUCTL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUCTL0_A {
        match self.bits {
            0 => CMPUCTL0_A::_0,
            1 => CMPUCTL0_A::_1,
            2 => CMPUCTL0_A::_2,
            3 => CMPUCTL0_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUCTL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUCTL0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPUCTL0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPUCTL0_A::_3
    }
}
impl core::ops::Deref for CMPUCTL0_R {
    type Target = crate::FieldReader<u8, CMPUCTL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUCTL0` writer - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUCTL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUCTL0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUCTL0_A::_0)
    }
    #[doc = "PWM compare up point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUCTL0_A::_1)
    }
    #[doc = "PWM compare up point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPUCTL0_A::_2)
    }
    #[doc = "PWM compare up point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPUCTL0_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPUCTL1_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare up point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare up point output High"]
    _2 = 2,
    #[doc = "3: PWM compare up point output Toggle"]
    _3 = 3,
}
impl From<CMPUCTL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPUCTL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPUCTL1` reader - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL1_R(crate::FieldReader<u8, CMPUCTL1_A>);
impl CMPUCTL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPUCTL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUCTL1_A {
        match self.bits {
            0 => CMPUCTL1_A::_0,
            1 => CMPUCTL1_A::_1,
            2 => CMPUCTL1_A::_2,
            3 => CMPUCTL1_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUCTL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUCTL1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPUCTL1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPUCTL1_A::_3
    }
}
impl core::ops::Deref for CMPUCTL1_R {
    type Target = crate::FieldReader<u8, CMPUCTL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUCTL1` writer - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUCTL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUCTL1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUCTL1_A::_0)
    }
    #[doc = "PWM compare up point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUCTL1_A::_1)
    }
    #[doc = "PWM compare up point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPUCTL1_A::_2)
    }
    #[doc = "PWM compare up point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPUCTL1_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPUCTL2_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare up point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare up point output High"]
    _2 = 2,
    #[doc = "3: PWM compare up point output Toggle"]
    _3 = 3,
}
impl From<CMPUCTL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPUCTL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPUCTL2` reader - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL2_R(crate::FieldReader<u8, CMPUCTL2_A>);
impl CMPUCTL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPUCTL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUCTL2_A {
        match self.bits {
            0 => CMPUCTL2_A::_0,
            1 => CMPUCTL2_A::_1,
            2 => CMPUCTL2_A::_2,
            3 => CMPUCTL2_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUCTL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUCTL2_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPUCTL2_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPUCTL2_A::_3
    }
}
impl core::ops::Deref for CMPUCTL2_R {
    type Target = crate::FieldReader<u8, CMPUCTL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUCTL2` writer - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUCTL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUCTL2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUCTL2_A::_0)
    }
    #[doc = "PWM compare up point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUCTL2_A::_1)
    }
    #[doc = "PWM compare up point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPUCTL2_A::_2)
    }
    #[doc = "PWM compare up point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPUCTL2_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPUCTL3_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare up point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare up point output High"]
    _2 = 2,
    #[doc = "3: PWM compare up point output Toggle"]
    _3 = 3,
}
impl From<CMPUCTL3_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPUCTL3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPUCTL3` reader - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL3_R(crate::FieldReader<u8, CMPUCTL3_A>);
impl CMPUCTL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPUCTL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUCTL3_A {
        match self.bits {
            0 => CMPUCTL3_A::_0,
            1 => CMPUCTL3_A::_1,
            2 => CMPUCTL3_A::_2,
            3 => CMPUCTL3_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUCTL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUCTL3_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPUCTL3_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPUCTL3_A::_3
    }
}
impl core::ops::Deref for CMPUCTL3_R {
    type Target = crate::FieldReader<u8, CMPUCTL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUCTL3` writer - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUCTL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUCTL3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUCTL3_A::_0)
    }
    #[doc = "PWM compare up point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUCTL3_A::_1)
    }
    #[doc = "PWM compare up point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPUCTL3_A::_2)
    }
    #[doc = "PWM compare up point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPUCTL3_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPUCTL4_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare up point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare up point output High"]
    _2 = 2,
    #[doc = "3: PWM compare up point output Toggle"]
    _3 = 3,
}
impl From<CMPUCTL4_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPUCTL4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPUCTL4` reader - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL4_R(crate::FieldReader<u8, CMPUCTL4_A>);
impl CMPUCTL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPUCTL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUCTL4_A {
        match self.bits {
            0 => CMPUCTL4_A::_0,
            1 => CMPUCTL4_A::_1,
            2 => CMPUCTL4_A::_2,
            3 => CMPUCTL4_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUCTL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUCTL4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPUCTL4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPUCTL4_A::_3
    }
}
impl core::ops::Deref for CMPUCTL4_R {
    type Target = crate::FieldReader<u8, CMPUCTL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUCTL4` writer - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUCTL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUCTL4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUCTL4_A::_0)
    }
    #[doc = "PWM compare up point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUCTL4_A::_1)
    }
    #[doc = "PWM compare up point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPUCTL4_A::_2)
    }
    #[doc = "PWM compare up point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPUCTL4_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPUCTL5_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare up point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare up point output High"]
    _2 = 2,
    #[doc = "3: PWM compare up point output Toggle"]
    _3 = 3,
}
impl From<CMPUCTL5_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPUCTL5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPUCTL5` reader - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL5_R(crate::FieldReader<u8, CMPUCTL5_A>);
impl CMPUCTL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPUCTL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPUCTL5_A {
        match self.bits {
            0 => CMPUCTL5_A::_0,
            1 => CMPUCTL5_A::_1,
            2 => CMPUCTL5_A::_2,
            3 => CMPUCTL5_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUCTL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUCTL5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPUCTL5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPUCTL5_A::_3
    }
}
impl core::ops::Deref for CMPUCTL5_R {
    type Target = crate::FieldReader<u8, CMPUCTL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUCTL5` writer - PWM Compare Up Point Control\nNote 1: PWM can control output level when PWM counter counts up to CMPDAT.\nNote 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUCTL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUCTL5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUCTL5_A::_0)
    }
    #[doc = "PWM compare up point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUCTL5_A::_1)
    }
    #[doc = "PWM compare up point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPUCTL5_A::_2)
    }
    #[doc = "PWM compare up point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPUCTL5_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPDCTL0_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare down point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare down point output High"]
    _2 = 2,
    #[doc = "3: PWM compare down point output Toggle"]
    _3 = 3,
}
impl From<CMPDCTL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPDCTL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPDCTL0` reader - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL0_R(crate::FieldReader<u8, CMPDCTL0_A>);
impl CMPDCTL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPDCTL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDCTL0_A {
        match self.bits {
            0 => CMPDCTL0_A::_0,
            1 => CMPDCTL0_A::_1,
            2 => CMPDCTL0_A::_2,
            3 => CMPDCTL0_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDCTL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDCTL0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPDCTL0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPDCTL0_A::_3
    }
}
impl core::ops::Deref for CMPDCTL0_R {
    type Target = crate::FieldReader<u8, CMPDCTL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDCTL0` writer - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDCTL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDCTL0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDCTL0_A::_0)
    }
    #[doc = "PWM compare down point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDCTL0_A::_1)
    }
    #[doc = "PWM compare down point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPDCTL0_A::_2)
    }
    #[doc = "PWM compare down point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPDCTL0_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPDCTL1_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare down point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare down point output High"]
    _2 = 2,
    #[doc = "3: PWM compare down point output Toggle"]
    _3 = 3,
}
impl From<CMPDCTL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPDCTL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPDCTL1` reader - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL1_R(crate::FieldReader<u8, CMPDCTL1_A>);
impl CMPDCTL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPDCTL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDCTL1_A {
        match self.bits {
            0 => CMPDCTL1_A::_0,
            1 => CMPDCTL1_A::_1,
            2 => CMPDCTL1_A::_2,
            3 => CMPDCTL1_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDCTL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDCTL1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPDCTL1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPDCTL1_A::_3
    }
}
impl core::ops::Deref for CMPDCTL1_R {
    type Target = crate::FieldReader<u8, CMPDCTL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDCTL1` writer - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDCTL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDCTL1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDCTL1_A::_0)
    }
    #[doc = "PWM compare down point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDCTL1_A::_1)
    }
    #[doc = "PWM compare down point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPDCTL1_A::_2)
    }
    #[doc = "PWM compare down point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPDCTL1_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPDCTL2_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare down point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare down point output High"]
    _2 = 2,
    #[doc = "3: PWM compare down point output Toggle"]
    _3 = 3,
}
impl From<CMPDCTL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPDCTL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPDCTL2` reader - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL2_R(crate::FieldReader<u8, CMPDCTL2_A>);
impl CMPDCTL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPDCTL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDCTL2_A {
        match self.bits {
            0 => CMPDCTL2_A::_0,
            1 => CMPDCTL2_A::_1,
            2 => CMPDCTL2_A::_2,
            3 => CMPDCTL2_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDCTL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDCTL2_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPDCTL2_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPDCTL2_A::_3
    }
}
impl core::ops::Deref for CMPDCTL2_R {
    type Target = crate::FieldReader<u8, CMPDCTL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDCTL2` writer - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDCTL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDCTL2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDCTL2_A::_0)
    }
    #[doc = "PWM compare down point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDCTL2_A::_1)
    }
    #[doc = "PWM compare down point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPDCTL2_A::_2)
    }
    #[doc = "PWM compare down point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPDCTL2_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPDCTL3_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare down point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare down point output High"]
    _2 = 2,
    #[doc = "3: PWM compare down point output Toggle"]
    _3 = 3,
}
impl From<CMPDCTL3_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPDCTL3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPDCTL3` reader - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL3_R(crate::FieldReader<u8, CMPDCTL3_A>);
impl CMPDCTL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPDCTL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDCTL3_A {
        match self.bits {
            0 => CMPDCTL3_A::_0,
            1 => CMPDCTL3_A::_1,
            2 => CMPDCTL3_A::_2,
            3 => CMPDCTL3_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDCTL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDCTL3_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPDCTL3_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPDCTL3_A::_3
    }
}
impl core::ops::Deref for CMPDCTL3_R {
    type Target = crate::FieldReader<u8, CMPDCTL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDCTL3` writer - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDCTL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDCTL3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDCTL3_A::_0)
    }
    #[doc = "PWM compare down point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDCTL3_A::_1)
    }
    #[doc = "PWM compare down point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPDCTL3_A::_2)
    }
    #[doc = "PWM compare down point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPDCTL3_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPDCTL4_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare down point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare down point output High"]
    _2 = 2,
    #[doc = "3: PWM compare down point output Toggle"]
    _3 = 3,
}
impl From<CMPDCTL4_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPDCTL4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPDCTL4` reader - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL4_R(crate::FieldReader<u8, CMPDCTL4_A>);
impl CMPDCTL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPDCTL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDCTL4_A {
        match self.bits {
            0 => CMPDCTL4_A::_0,
            1 => CMPDCTL4_A::_1,
            2 => CMPDCTL4_A::_2,
            3 => CMPDCTL4_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDCTL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDCTL4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPDCTL4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPDCTL4_A::_3
    }
}
impl core::ops::Deref for CMPDCTL4_R {
    type Target = crate::FieldReader<u8, CMPDCTL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDCTL4` writer - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDCTL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDCTL4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDCTL4_A::_0)
    }
    #[doc = "PWM compare down point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDCTL4_A::_1)
    }
    #[doc = "PWM compare down point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPDCTL4_A::_2)
    }
    #[doc = "PWM compare down point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPDCTL4_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPDCTL5_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare down point output Low"]
    _1 = 1,
    #[doc = "2: PWM compare down point output High"]
    _2 = 2,
    #[doc = "3: PWM compare down point output Toggle"]
    _3 = 3,
}
impl From<CMPDCTL5_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPDCTL5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPDCTL5` reader - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL5_R(crate::FieldReader<u8, CMPDCTL5_A>);
impl CMPDCTL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPDCTL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPDCTL5_A {
        match self.bits {
            0 => CMPDCTL5_A::_0,
            1 => CMPDCTL5_A::_1,
            2 => CMPDCTL5_A::_2,
            3 => CMPDCTL5_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDCTL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDCTL5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPDCTL5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPDCTL5_A::_3
    }
}
impl core::ops::Deref for CMPDCTL5_R {
    type Target = crate::FieldReader<u8, CMPDCTL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDCTL5` writer - PWM Compare Down Point Control\nNote 1: PWM can control output level when PWM counter counts down to CMPDAT.\nNote 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDCTL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDCTL5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDCTL5_A::_0)
    }
    #[doc = "PWM compare down point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDCTL5_A::_1)
    }
    #[doc = "PWM compare down point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPDCTL5_A::_2)
    }
    #[doc = "PWM compare down point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPDCTL5_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl0(&self) -> CMPUCTL0_R {
        CMPUCTL0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl1(&self) -> CMPUCTL1_R {
        CMPUCTL1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl2(&self) -> CMPUCTL2_R {
        CMPUCTL2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl3(&self) -> CMPUCTL3_R {
        CMPUCTL3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl4(&self) -> CMPUCTL4_R {
        CMPUCTL4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl5(&self) -> CMPUCTL5_R {
        CMPUCTL5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl0(&self) -> CMPDCTL0_R {
        CMPDCTL0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl1(&self) -> CMPDCTL1_R {
        CMPDCTL1_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl2(&self) -> CMPDCTL2_R {
        CMPDCTL2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl3(&self) -> CMPDCTL3_R {
        CMPDCTL3_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl4(&self) -> CMPDCTL4_R {
        CMPDCTL4_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl5(&self) -> CMPDCTL5_R {
        CMPDCTL5_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl0(&mut self) -> CMPUCTL0_W {
        CMPUCTL0_W { w: self }
    }
    #[doc = "Bits 2:3 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl1(&mut self) -> CMPUCTL1_W {
        CMPUCTL1_W { w: self }
    }
    #[doc = "Bits 4:5 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl2(&mut self) -> CMPUCTL2_W {
        CMPUCTL2_W { w: self }
    }
    #[doc = "Bits 6:7 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl3(&mut self) -> CMPUCTL3_W {
        CMPUCTL3_W { w: self }
    }
    #[doc = "Bits 8:9 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl4(&mut self) -> CMPUCTL4_W {
        CMPUCTL4_W { w: self }
    }
    #[doc = "Bits 10:11 - PWM Compare Up Point Control Note 1: PWM can control output level when PWM counter counts up to CMPDAT. Note 2: In complementary mode, CMPUCTL1, 3, 5 is used as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctl5(&mut self) -> CMPUCTL5_W {
        CMPUCTL5_W { w: self }
    }
    #[doc = "Bits 16:17 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl0(&mut self) -> CMPDCTL0_W {
        CMPDCTL0_W { w: self }
    }
    #[doc = "Bits 18:19 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl1(&mut self) -> CMPDCTL1_W {
        CMPDCTL1_W { w: self }
    }
    #[doc = "Bits 20:21 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl2(&mut self) -> CMPDCTL2_W {
        CMPDCTL2_W { w: self }
    }
    #[doc = "Bits 22:23 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl3(&mut self) -> CMPDCTL3_W {
        CMPDCTL3_W { w: self }
    }
    #[doc = "Bits 24:25 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl4(&mut self) -> CMPDCTL4_W {
        CMPDCTL4_W { w: self }
    }
    #[doc = "Bits 26:27 - PWM Compare Down Point Control Note 1: PWM can control output level when PWM counter counts down to CMPDAT. Note 2: In complementary mode, CMPDCTL1, 3, 5 is used as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctl5(&mut self) -> CMPDCTL5_W {
        CMPDCTL5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Generation Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_wgctl1](index.html) module"]
pub struct PWM_WGCTL1_SPEC;
impl crate::RegisterSpec for PWM_WGCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_wgctl1::R](R) reader structure"]
impl crate::Readable for PWM_WGCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_wgctl1::W](W) writer structure"]
impl crate::Writable for PWM_WGCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_WGCTL1 to value 0"]
impl crate::Resettable for PWM_WGCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
