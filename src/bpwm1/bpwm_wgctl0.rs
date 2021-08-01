#[doc = "Register `BPWM_WGCTL0` reader"]
pub struct R(crate::R<BPWM_WGCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_WGCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_WGCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_WGCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_WGCTL0` writer"]
pub struct W(crate::W<BPWM_WGCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_WGCTL0_SPEC>;
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
impl From<crate::W<BPWM_WGCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_WGCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZPCTL0_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM zero point output Low"]
    _1 = 1,
    #[doc = "2: BPWM zero point output High"]
    _2 = 2,
    #[doc = "3: BPWM zero point output Toggle"]
    _3 = 3,
}
impl From<ZPCTL0_A> for u8 {
    #[inline(always)]
    fn from(variant: ZPCTL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZPCTL0` reader - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL0_R(crate::FieldReader<u8, ZPCTL0_A>);
impl ZPCTL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZPCTL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZPCTL0_A {
        match self.bits {
            0 => ZPCTL0_A::_0,
            1 => ZPCTL0_A::_1,
            2 => ZPCTL0_A::_2,
            3 => ZPCTL0_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZPCTL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZPCTL0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ZPCTL0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ZPCTL0_A::_3
    }
}
impl core::ops::Deref for ZPCTL0_R {
    type Target = crate::FieldReader<u8, ZPCTL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZPCTL0` writer - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL0_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPCTL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZPCTL0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZPCTL0_A::_0)
    }
    #[doc = "BPWM zero point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZPCTL0_A::_1)
    }
    #[doc = "BPWM zero point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ZPCTL0_A::_2)
    }
    #[doc = "BPWM zero point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ZPCTL0_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZPCTL1_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM zero point output Low"]
    _1 = 1,
    #[doc = "2: BPWM zero point output High"]
    _2 = 2,
    #[doc = "3: BPWM zero point output Toggle"]
    _3 = 3,
}
impl From<ZPCTL1_A> for u8 {
    #[inline(always)]
    fn from(variant: ZPCTL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZPCTL1` reader - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL1_R(crate::FieldReader<u8, ZPCTL1_A>);
impl ZPCTL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZPCTL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZPCTL1_A {
        match self.bits {
            0 => ZPCTL1_A::_0,
            1 => ZPCTL1_A::_1,
            2 => ZPCTL1_A::_2,
            3 => ZPCTL1_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZPCTL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZPCTL1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ZPCTL1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ZPCTL1_A::_3
    }
}
impl core::ops::Deref for ZPCTL1_R {
    type Target = crate::FieldReader<u8, ZPCTL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZPCTL1` writer - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPCTL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZPCTL1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZPCTL1_A::_0)
    }
    #[doc = "BPWM zero point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZPCTL1_A::_1)
    }
    #[doc = "BPWM zero point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ZPCTL1_A::_2)
    }
    #[doc = "BPWM zero point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ZPCTL1_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZPCTL2_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM zero point output Low"]
    _1 = 1,
    #[doc = "2: BPWM zero point output High"]
    _2 = 2,
    #[doc = "3: BPWM zero point output Toggle"]
    _3 = 3,
}
impl From<ZPCTL2_A> for u8 {
    #[inline(always)]
    fn from(variant: ZPCTL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZPCTL2` reader - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL2_R(crate::FieldReader<u8, ZPCTL2_A>);
impl ZPCTL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZPCTL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZPCTL2_A {
        match self.bits {
            0 => ZPCTL2_A::_0,
            1 => ZPCTL2_A::_1,
            2 => ZPCTL2_A::_2,
            3 => ZPCTL2_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZPCTL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZPCTL2_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ZPCTL2_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ZPCTL2_A::_3
    }
}
impl core::ops::Deref for ZPCTL2_R {
    type Target = crate::FieldReader<u8, ZPCTL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZPCTL2` writer - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPCTL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZPCTL2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZPCTL2_A::_0)
    }
    #[doc = "BPWM zero point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZPCTL2_A::_1)
    }
    #[doc = "BPWM zero point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ZPCTL2_A::_2)
    }
    #[doc = "BPWM zero point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ZPCTL2_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZPCTL3_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM zero point output Low"]
    _1 = 1,
    #[doc = "2: BPWM zero point output High"]
    _2 = 2,
    #[doc = "3: BPWM zero point output Toggle"]
    _3 = 3,
}
impl From<ZPCTL3_A> for u8 {
    #[inline(always)]
    fn from(variant: ZPCTL3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZPCTL3` reader - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL3_R(crate::FieldReader<u8, ZPCTL3_A>);
impl ZPCTL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZPCTL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZPCTL3_A {
        match self.bits {
            0 => ZPCTL3_A::_0,
            1 => ZPCTL3_A::_1,
            2 => ZPCTL3_A::_2,
            3 => ZPCTL3_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZPCTL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZPCTL3_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ZPCTL3_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ZPCTL3_A::_3
    }
}
impl core::ops::Deref for ZPCTL3_R {
    type Target = crate::FieldReader<u8, ZPCTL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZPCTL3` writer - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPCTL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZPCTL3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZPCTL3_A::_0)
    }
    #[doc = "BPWM zero point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZPCTL3_A::_1)
    }
    #[doc = "BPWM zero point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ZPCTL3_A::_2)
    }
    #[doc = "BPWM zero point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ZPCTL3_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZPCTL4_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM zero point output Low"]
    _1 = 1,
    #[doc = "2: BPWM zero point output High"]
    _2 = 2,
    #[doc = "3: BPWM zero point output Toggle"]
    _3 = 3,
}
impl From<ZPCTL4_A> for u8 {
    #[inline(always)]
    fn from(variant: ZPCTL4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZPCTL4` reader - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL4_R(crate::FieldReader<u8, ZPCTL4_A>);
impl ZPCTL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZPCTL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZPCTL4_A {
        match self.bits {
            0 => ZPCTL4_A::_0,
            1 => ZPCTL4_A::_1,
            2 => ZPCTL4_A::_2,
            3 => ZPCTL4_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZPCTL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZPCTL4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ZPCTL4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ZPCTL4_A::_3
    }
}
impl core::ops::Deref for ZPCTL4_R {
    type Target = crate::FieldReader<u8, ZPCTL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZPCTL4` writer - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL4_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPCTL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZPCTL4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZPCTL4_A::_0)
    }
    #[doc = "BPWM zero point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZPCTL4_A::_1)
    }
    #[doc = "BPWM zero point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ZPCTL4_A::_2)
    }
    #[doc = "BPWM zero point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ZPCTL4_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZPCTL5_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM zero point output Low"]
    _1 = 1,
    #[doc = "2: BPWM zero point output High"]
    _2 = 2,
    #[doc = "3: BPWM zero point output Toggle"]
    _3 = 3,
}
impl From<ZPCTL5_A> for u8 {
    #[inline(always)]
    fn from(variant: ZPCTL5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZPCTL5` reader - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL5_R(crate::FieldReader<u8, ZPCTL5_A>);
impl ZPCTL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZPCTL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZPCTL5_A {
        match self.bits {
            0 => ZPCTL5_A::_0,
            1 => ZPCTL5_A::_1,
            2 => ZPCTL5_A::_2,
            3 => ZPCTL5_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZPCTL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZPCTL5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ZPCTL5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ZPCTL5_A::_3
    }
}
impl core::ops::Deref for ZPCTL5_R {
    type Target = crate::FieldReader<u8, ZPCTL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZPCTL5` writer - BPWM Zero Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to zero."]
pub struct ZPCTL5_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPCTL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZPCTL5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZPCTL5_A::_0)
    }
    #[doc = "BPWM zero point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZPCTL5_A::_1)
    }
    #[doc = "BPWM zero point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ZPCTL5_A::_2)
    }
    #[doc = "BPWM zero point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ZPCTL5_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRDPCTL0_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM period (center) point output Low"]
    _1 = 1,
    #[doc = "2: BPWM period (center) point output High"]
    _2 = 2,
    #[doc = "3: BPWM period (center) point output Toggle"]
    _3 = 3,
}
impl From<PRDPCTL0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRDPCTL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRDPCTL0` reader - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL0_R(crate::FieldReader<u8, PRDPCTL0_A>);
impl PRDPCTL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRDPCTL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDPCTL0_A {
        match self.bits {
            0 => PRDPCTL0_A::_0,
            1 => PRDPCTL0_A::_1,
            2 => PRDPCTL0_A::_2,
            3 => PRDPCTL0_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRDPCTL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRDPCTL0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == PRDPCTL0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == PRDPCTL0_A::_3
    }
}
impl core::ops::Deref for PRDPCTL0_R {
    type Target = crate::FieldReader<u8, PRDPCTL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDPCTL0` writer - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDPCTL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRDPCTL0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDPCTL0_A::_0)
    }
    #[doc = "BPWM period (center) point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDPCTL0_A::_1)
    }
    #[doc = "BPWM period (center) point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRDPCTL0_A::_2)
    }
    #[doc = "BPWM period (center) point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRDPCTL0_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRDPCTL1_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM period (center) point output Low"]
    _1 = 1,
    #[doc = "2: BPWM period (center) point output High"]
    _2 = 2,
    #[doc = "3: BPWM period (center) point output Toggle"]
    _3 = 3,
}
impl From<PRDPCTL1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRDPCTL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRDPCTL1` reader - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL1_R(crate::FieldReader<u8, PRDPCTL1_A>);
impl PRDPCTL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRDPCTL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDPCTL1_A {
        match self.bits {
            0 => PRDPCTL1_A::_0,
            1 => PRDPCTL1_A::_1,
            2 => PRDPCTL1_A::_2,
            3 => PRDPCTL1_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRDPCTL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRDPCTL1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == PRDPCTL1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == PRDPCTL1_A::_3
    }
}
impl core::ops::Deref for PRDPCTL1_R {
    type Target = crate::FieldReader<u8, PRDPCTL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDPCTL1` writer - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDPCTL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRDPCTL1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDPCTL1_A::_0)
    }
    #[doc = "BPWM period (center) point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDPCTL1_A::_1)
    }
    #[doc = "BPWM period (center) point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRDPCTL1_A::_2)
    }
    #[doc = "BPWM period (center) point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRDPCTL1_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRDPCTL2_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM period (center) point output Low"]
    _1 = 1,
    #[doc = "2: BPWM period (center) point output High"]
    _2 = 2,
    #[doc = "3: BPWM period (center) point output Toggle"]
    _3 = 3,
}
impl From<PRDPCTL2_A> for u8 {
    #[inline(always)]
    fn from(variant: PRDPCTL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRDPCTL2` reader - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL2_R(crate::FieldReader<u8, PRDPCTL2_A>);
impl PRDPCTL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRDPCTL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDPCTL2_A {
        match self.bits {
            0 => PRDPCTL2_A::_0,
            1 => PRDPCTL2_A::_1,
            2 => PRDPCTL2_A::_2,
            3 => PRDPCTL2_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRDPCTL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRDPCTL2_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == PRDPCTL2_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == PRDPCTL2_A::_3
    }
}
impl core::ops::Deref for PRDPCTL2_R {
    type Target = crate::FieldReader<u8, PRDPCTL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDPCTL2` writer - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDPCTL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRDPCTL2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDPCTL2_A::_0)
    }
    #[doc = "BPWM period (center) point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDPCTL2_A::_1)
    }
    #[doc = "BPWM period (center) point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRDPCTL2_A::_2)
    }
    #[doc = "BPWM period (center) point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRDPCTL2_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRDPCTL3_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM period (center) point output Low"]
    _1 = 1,
    #[doc = "2: BPWM period (center) point output High"]
    _2 = 2,
    #[doc = "3: BPWM period (center) point output Toggle"]
    _3 = 3,
}
impl From<PRDPCTL3_A> for u8 {
    #[inline(always)]
    fn from(variant: PRDPCTL3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRDPCTL3` reader - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL3_R(crate::FieldReader<u8, PRDPCTL3_A>);
impl PRDPCTL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRDPCTL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDPCTL3_A {
        match self.bits {
            0 => PRDPCTL3_A::_0,
            1 => PRDPCTL3_A::_1,
            2 => PRDPCTL3_A::_2,
            3 => PRDPCTL3_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRDPCTL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRDPCTL3_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == PRDPCTL3_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == PRDPCTL3_A::_3
    }
}
impl core::ops::Deref for PRDPCTL3_R {
    type Target = crate::FieldReader<u8, PRDPCTL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDPCTL3` writer - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDPCTL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRDPCTL3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDPCTL3_A::_0)
    }
    #[doc = "BPWM period (center) point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDPCTL3_A::_1)
    }
    #[doc = "BPWM period (center) point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRDPCTL3_A::_2)
    }
    #[doc = "BPWM period (center) point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRDPCTL3_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRDPCTL4_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM period (center) point output Low"]
    _1 = 1,
    #[doc = "2: BPWM period (center) point output High"]
    _2 = 2,
    #[doc = "3: BPWM period (center) point output Toggle"]
    _3 = 3,
}
impl From<PRDPCTL4_A> for u8 {
    #[inline(always)]
    fn from(variant: PRDPCTL4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRDPCTL4` reader - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL4_R(crate::FieldReader<u8, PRDPCTL4_A>);
impl PRDPCTL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRDPCTL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDPCTL4_A {
        match self.bits {
            0 => PRDPCTL4_A::_0,
            1 => PRDPCTL4_A::_1,
            2 => PRDPCTL4_A::_2,
            3 => PRDPCTL4_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRDPCTL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRDPCTL4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == PRDPCTL4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == PRDPCTL4_A::_3
    }
}
impl core::ops::Deref for PRDPCTL4_R {
    type Target = crate::FieldReader<u8, PRDPCTL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDPCTL4` writer - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL4_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDPCTL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRDPCTL4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDPCTL4_A::_0)
    }
    #[doc = "BPWM period (center) point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDPCTL4_A::_1)
    }
    #[doc = "BPWM period (center) point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRDPCTL4_A::_2)
    }
    #[doc = "BPWM period (center) point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRDPCTL4_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRDPCTL5_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: BPWM period (center) point output Low"]
    _1 = 1,
    #[doc = "2: BPWM period (center) point output High"]
    _2 = 2,
    #[doc = "3: BPWM period (center) point output Toggle"]
    _3 = 3,
}
impl From<PRDPCTL5_A> for u8 {
    #[inline(always)]
    fn from(variant: PRDPCTL5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRDPCTL5` reader - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL5_R(crate::FieldReader<u8, PRDPCTL5_A>);
impl PRDPCTL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRDPCTL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDPCTL5_A {
        match self.bits {
            0 => PRDPCTL5_A::_0,
            1 => PRDPCTL5_A::_1,
            2 => PRDPCTL5_A::_2,
            3 => PRDPCTL5_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRDPCTL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRDPCTL5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == PRDPCTL5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == PRDPCTL5_A::_3
    }
}
impl core::ops::Deref for PRDPCTL5_R {
    type Target = crate::FieldReader<u8, PRDPCTL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDPCTL5` writer - BPWM Period (Center) Point Control\nEach bit n controls the corresponding BPWM channel n.\nBPWM can control output level when BPWM counter count to (PERIOD+1).\nNote: This bit is center point control when BPWM counter operating in up-down counter type."]
pub struct PRDPCTL5_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDPCTL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRDPCTL5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDPCTL5_A::_0)
    }
    #[doc = "BPWM period (center) point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDPCTL5_A::_1)
    }
    #[doc = "BPWM period (center) point output High"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRDPCTL5_A::_2)
    }
    #[doc = "BPWM period (center) point output Toggle"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRDPCTL5_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl0(&self) -> ZPCTL0_R {
        ZPCTL0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl1(&self) -> ZPCTL1_R {
        ZPCTL1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl2(&self) -> ZPCTL2_R {
        ZPCTL2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl3(&self) -> ZPCTL3_R {
        ZPCTL3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl4(&self) -> ZPCTL4_R {
        ZPCTL4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl5(&self) -> ZPCTL5_R {
        ZPCTL5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl0(&self) -> PRDPCTL0_R {
        PRDPCTL0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl1(&self) -> PRDPCTL1_R {
        PRDPCTL1_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl2(&self) -> PRDPCTL2_R {
        PRDPCTL2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl3(&self) -> PRDPCTL3_R {
        PRDPCTL3_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl4(&self) -> PRDPCTL4_R {
        PRDPCTL4_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl5(&self) -> PRDPCTL5_R {
        PRDPCTL5_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl0(&mut self) -> ZPCTL0_W {
        ZPCTL0_W { w: self }
    }
    #[doc = "Bits 2:3 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl1(&mut self) -> ZPCTL1_W {
        ZPCTL1_W { w: self }
    }
    #[doc = "Bits 4:5 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl2(&mut self) -> ZPCTL2_W {
        ZPCTL2_W { w: self }
    }
    #[doc = "Bits 6:7 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl3(&mut self) -> ZPCTL3_W {
        ZPCTL3_W { w: self }
    }
    #[doc = "Bits 8:9 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl4(&mut self) -> ZPCTL4_W {
        ZPCTL4_W { w: self }
    }
    #[doc = "Bits 10:11 - BPWM Zero Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to zero."]
    #[inline(always)]
    pub fn zpctl5(&mut self) -> ZPCTL5_W {
        ZPCTL5_W { w: self }
    }
    #[doc = "Bits 16:17 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl0(&mut self) -> PRDPCTL0_W {
        PRDPCTL0_W { w: self }
    }
    #[doc = "Bits 18:19 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl1(&mut self) -> PRDPCTL1_W {
        PRDPCTL1_W { w: self }
    }
    #[doc = "Bits 20:21 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl2(&mut self) -> PRDPCTL2_W {
        PRDPCTL2_W { w: self }
    }
    #[doc = "Bits 22:23 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl3(&mut self) -> PRDPCTL3_W {
        PRDPCTL3_W { w: self }
    }
    #[doc = "Bits 24:25 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl4(&mut self) -> PRDPCTL4_W {
        PRDPCTL4_W { w: self }
    }
    #[doc = "Bits 26:27 - BPWM Period (Center) Point Control Each bit n controls the corresponding BPWM channel n. BPWM can control output level when BPWM counter count to (PERIOD+1). Note: This bit is center point control when BPWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctl5(&mut self) -> PRDPCTL5_W {
        PRDPCTL5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Generation Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_wgctl0](index.html) module"]
pub struct BPWM_WGCTL0_SPEC;
impl crate::RegisterSpec for BPWM_WGCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_wgctl0::R](R) reader structure"]
impl crate::Readable for BPWM_WGCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_wgctl0::W](W) writer structure"]
impl crate::Writable for BPWM_WGCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_WGCTL0 to value 0"]
impl crate::Resettable for BPWM_WGCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
