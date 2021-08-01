#[doc = "Register `BPWM_MSK` reader"]
pub struct R(crate::R<BPWM_MSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_MSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_MSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_MSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_MSK` writer"]
pub struct W(crate::W<BPWM_MSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_MSK_SPEC>;
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
impl From<crate::W<BPWM_MSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_MSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKDAT0_A {
    #[doc = "0: Output logic low to BPWMn"]
    _0 = 0,
    #[doc = "1: Output logic high to BPWMn"]
    _1 = 1,
}
impl From<MSKDAT0_A> for bool {
    #[inline(always)]
    fn from(variant: MSKDAT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKDAT0` reader - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT0_R(crate::FieldReader<bool, MSKDAT0_A>);
impl MSKDAT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKDAT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKDAT0_A {
        match self.bits {
            false => MSKDAT0_A::_0,
            true => MSKDAT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKDAT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKDAT0_A::_1
    }
}
impl core::ops::Deref for MSKDAT0_R {
    type Target = crate::FieldReader<bool, MSKDAT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKDAT0` writer - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKDAT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKDAT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output logic low to BPWMn"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKDAT0_A::_0)
    }
    #[doc = "Output logic high to BPWMn"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKDAT0_A::_1)
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
#[doc = "BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKDAT1_A {
    #[doc = "0: Output logic low to BPWMn"]
    _0 = 0,
    #[doc = "1: Output logic high to BPWMn"]
    _1 = 1,
}
impl From<MSKDAT1_A> for bool {
    #[inline(always)]
    fn from(variant: MSKDAT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKDAT1` reader - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT1_R(crate::FieldReader<bool, MSKDAT1_A>);
impl MSKDAT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKDAT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKDAT1_A {
        match self.bits {
            false => MSKDAT1_A::_0,
            true => MSKDAT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKDAT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKDAT1_A::_1
    }
}
impl core::ops::Deref for MSKDAT1_R {
    type Target = crate::FieldReader<bool, MSKDAT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKDAT1` writer - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKDAT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKDAT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output logic low to BPWMn"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKDAT1_A::_0)
    }
    #[doc = "Output logic high to BPWMn"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKDAT1_A::_1)
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
#[doc = "BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKDAT2_A {
    #[doc = "0: Output logic low to BPWMn"]
    _0 = 0,
    #[doc = "1: Output logic high to BPWMn"]
    _1 = 1,
}
impl From<MSKDAT2_A> for bool {
    #[inline(always)]
    fn from(variant: MSKDAT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKDAT2` reader - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT2_R(crate::FieldReader<bool, MSKDAT2_A>);
impl MSKDAT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKDAT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKDAT2_A {
        match self.bits {
            false => MSKDAT2_A::_0,
            true => MSKDAT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKDAT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKDAT2_A::_1
    }
}
impl core::ops::Deref for MSKDAT2_R {
    type Target = crate::FieldReader<bool, MSKDAT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKDAT2` writer - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKDAT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKDAT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output logic low to BPWMn"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKDAT2_A::_0)
    }
    #[doc = "Output logic high to BPWMn"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKDAT2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKDAT3_A {
    #[doc = "0: Output logic low to BPWMn"]
    _0 = 0,
    #[doc = "1: Output logic high to BPWMn"]
    _1 = 1,
}
impl From<MSKDAT3_A> for bool {
    #[inline(always)]
    fn from(variant: MSKDAT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKDAT3` reader - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT3_R(crate::FieldReader<bool, MSKDAT3_A>);
impl MSKDAT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKDAT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKDAT3_A {
        match self.bits {
            false => MSKDAT3_A::_0,
            true => MSKDAT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKDAT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKDAT3_A::_1
    }
}
impl core::ops::Deref for MSKDAT3_R {
    type Target = crate::FieldReader<bool, MSKDAT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKDAT3` writer - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKDAT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKDAT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output logic low to BPWMn"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKDAT3_A::_0)
    }
    #[doc = "Output logic high to BPWMn"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKDAT3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKDAT4_A {
    #[doc = "0: Output logic low to BPWMn"]
    _0 = 0,
    #[doc = "1: Output logic high to BPWMn"]
    _1 = 1,
}
impl From<MSKDAT4_A> for bool {
    #[inline(always)]
    fn from(variant: MSKDAT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKDAT4` reader - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT4_R(crate::FieldReader<bool, MSKDAT4_A>);
impl MSKDAT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKDAT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKDAT4_A {
        match self.bits {
            false => MSKDAT4_A::_0,
            true => MSKDAT4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKDAT4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKDAT4_A::_1
    }
}
impl core::ops::Deref for MSKDAT4_R {
    type Target = crate::FieldReader<bool, MSKDAT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKDAT4` writer - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT4_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKDAT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKDAT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output logic low to BPWMn"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKDAT4_A::_0)
    }
    #[doc = "Output logic high to BPWMn"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKDAT4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSKDAT5_A {
    #[doc = "0: Output logic low to BPWMn"]
    _0 = 0,
    #[doc = "1: Output logic high to BPWMn"]
    _1 = 1,
}
impl From<MSKDAT5_A> for bool {
    #[inline(always)]
    fn from(variant: MSKDAT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSKDAT5` reader - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT5_R(crate::FieldReader<bool, MSKDAT5_A>);
impl MSKDAT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSKDAT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSKDAT5_A {
        match self.bits {
            false => MSKDAT5_A::_0,
            true => MSKDAT5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSKDAT5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSKDAT5_A::_1
    }
}
impl core::ops::Deref for MSKDAT5_R {
    type Target = crate::FieldReader<bool, MSKDAT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSKDAT5` writer - BPWM Mask Data Bit\nThis data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
pub struct MSKDAT5_W<'a> {
    w: &'a mut W,
}
impl<'a> MSKDAT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSKDAT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output logic low to BPWMn"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSKDAT5_A::_0)
    }
    #[doc = "Output logic high to BPWMn"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSKDAT5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat0(&self) -> MSKDAT0_R {
        MSKDAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat1(&self) -> MSKDAT1_R {
        MSKDAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat2(&self) -> MSKDAT2_R {
        MSKDAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat3(&self) -> MSKDAT3_R {
        MSKDAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat4(&self) -> MSKDAT4_R {
        MSKDAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat5(&self) -> MSKDAT5_R {
        MSKDAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat0(&mut self) -> MSKDAT0_W {
        MSKDAT0_W { w: self }
    }
    #[doc = "Bit 1 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat1(&mut self) -> MSKDAT1_W {
        MSKDAT1_W { w: self }
    }
    #[doc = "Bit 2 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat2(&mut self) -> MSKDAT2_W {
        MSKDAT2_W { w: self }
    }
    #[doc = "Bit 3 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat3(&mut self) -> MSKDAT3_W {
        MSKDAT3_W { w: self }
    }
    #[doc = "Bit 4 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat4(&mut self) -> MSKDAT4_W {
        MSKDAT4_W { w: self }
    }
    #[doc = "Bit 5 - BPWM Mask Data Bit This data bit control the state of BPWMn output pin, if corresponding mask function is enabled. Each bit n controls the corresponding BPWM channel n."]
    #[inline(always)]
    pub fn mskdat5(&mut self) -> MSKDAT5_W {
        MSKDAT5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPWM Mask Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_msk](index.html) module"]
pub struct BPWM_MSK_SPEC;
impl crate::RegisterSpec for BPWM_MSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_msk::R](R) reader structure"]
impl crate::Readable for BPWM_MSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_msk::W](W) writer structure"]
impl crate::Writable for BPWM_MSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_MSK to value 0"]
impl crate::Resettable for BPWM_MSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
