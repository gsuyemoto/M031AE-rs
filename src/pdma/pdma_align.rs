#[doc = "Register `PDMA_ALIGN` reader"]
pub struct R(crate::R<PDMA_ALIGN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_ALIGN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_ALIGN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_ALIGN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_ALIGN` writer"]
pub struct W(crate::W<PDMA_ALIGN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_ALIGN_SPEC>;
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
impl From<crate::W<PDMA_ALIGN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_ALIGN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN0_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGN0_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN0` reader - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN0_R(crate::FieldReader<bool, ALIGN0_A>);
impl ALIGN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN0_A {
        match self.bits {
            false => ALIGN0_A::_0,
            true => ALIGN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGN0_A::_1
    }
}
impl core::ops::Deref for ALIGN0_R {
    type Target = crate::FieldReader<bool, ALIGN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN0` writer - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel source address and destination address both follow transfer width setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN0_A::_0)
    }
    #[doc = "PDMA channel source address or destination address is not follow transfer width setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN0_A::_1)
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
#[doc = "Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN1_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGN1_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN1` reader - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN1_R(crate::FieldReader<bool, ALIGN1_A>);
impl ALIGN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN1_A {
        match self.bits {
            false => ALIGN1_A::_0,
            true => ALIGN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGN1_A::_1
    }
}
impl core::ops::Deref for ALIGN1_R {
    type Target = crate::FieldReader<bool, ALIGN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN1` writer - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel source address and destination address both follow transfer width setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN1_A::_0)
    }
    #[doc = "PDMA channel source address or destination address is not follow transfer width setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN1_A::_1)
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
#[doc = "Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN2_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGN2_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN2` reader - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN2_R(crate::FieldReader<bool, ALIGN2_A>);
impl ALIGN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN2_A {
        match self.bits {
            false => ALIGN2_A::_0,
            true => ALIGN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGN2_A::_1
    }
}
impl core::ops::Deref for ALIGN2_R {
    type Target = crate::FieldReader<bool, ALIGN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN2` writer - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel source address and destination address both follow transfer width setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN2_A::_0)
    }
    #[doc = "PDMA channel source address or destination address is not follow transfer width setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN2_A::_1)
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
#[doc = "Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN3_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGN3_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN3` reader - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN3_R(crate::FieldReader<bool, ALIGN3_A>);
impl ALIGN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN3_A {
        match self.bits {
            false => ALIGN3_A::_0,
            true => ALIGN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGN3_A::_1
    }
}
impl core::ops::Deref for ALIGN3_R {
    type Target = crate::FieldReader<bool, ALIGN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN3` writer - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel source address and destination address both follow transfer width setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN3_A::_0)
    }
    #[doc = "PDMA channel source address or destination address is not follow transfer width setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN3_A::_1)
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
#[doc = "Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN4_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGN4_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN4` reader - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN4_R(crate::FieldReader<bool, ALIGN4_A>);
impl ALIGN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN4_A {
        match self.bits {
            false => ALIGN4_A::_0,
            true => ALIGN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGN4_A::_1
    }
}
impl core::ops::Deref for ALIGN4_R {
    type Target = crate::FieldReader<bool, ALIGN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN4` writer - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN4_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel source address and destination address both follow transfer width setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN4_A::_0)
    }
    #[doc = "PDMA channel source address or destination address is not follow transfer width setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN4_A::_1)
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
#[doc = "Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN5_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGN5_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN5` reader - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN5_R(crate::FieldReader<bool, ALIGN5_A>);
impl ALIGN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN5_A {
        match self.bits {
            false => ALIGN5_A::_0,
            true => ALIGN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGN5_A::_1
    }
}
impl core::ops::Deref for ALIGN5_R {
    type Target = crate::FieldReader<bool, ALIGN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN5` writer - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN5_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel source address and destination address both follow transfer width setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN5_A::_0)
    }
    #[doc = "PDMA channel source address or destination address is not follow transfer width setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN5_A::_1)
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
#[doc = "Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN6_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGN6_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN6` reader - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN6_R(crate::FieldReader<bool, ALIGN6_A>);
impl ALIGN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN6_A {
        match self.bits {
            false => ALIGN6_A::_0,
            true => ALIGN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGN6_A::_1
    }
}
impl core::ops::Deref for ALIGN6_R {
    type Target = crate::FieldReader<bool, ALIGN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN6` writer - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN6_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel source address and destination address both follow transfer width setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN6_A::_0)
    }
    #[doc = "PDMA channel source address or destination address is not follow transfer width setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN7_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGN7_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN7` reader - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN7_R(crate::FieldReader<bool, ALIGN7_A>);
impl ALIGN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN7_A {
        match self.bits {
            false => ALIGN7_A::_0,
            true => ALIGN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGN7_A::_1
    }
}
impl core::ops::Deref for ALIGN7_R {
    type Target = crate::FieldReader<bool, ALIGN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN7` writer - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN7_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel source address and destination address both follow transfer width setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN7_A::_0)
    }
    #[doc = "PDMA channel source address or destination address is not follow transfer width setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN7_A::_1)
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
#[doc = "Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN8_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGN8_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN8` reader - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN8_R(crate::FieldReader<bool, ALIGN8_A>);
impl ALIGN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN8_A {
        match self.bits {
            false => ALIGN8_A::_0,
            true => ALIGN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGN8_A::_1
    }
}
impl core::ops::Deref for ALIGN8_R {
    type Target = crate::FieldReader<bool, ALIGN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN8` writer - Transfer Alignment Flag\nThis bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
pub struct ALIGN8_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA channel source address and destination address both follow transfer width setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN8_A::_0)
    }
    #[doc = "PDMA channel source address or destination address is not follow transfer width setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN8_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align0(&self) -> ALIGN0_R {
        ALIGN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align1(&self) -> ALIGN1_R {
        ALIGN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align2(&self) -> ALIGN2_R {
        ALIGN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align3(&self) -> ALIGN3_R {
        ALIGN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align4(&self) -> ALIGN4_R {
        ALIGN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align5(&self) -> ALIGN5_R {
        ALIGN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align6(&self) -> ALIGN6_R {
        ALIGN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align7(&self) -> ALIGN7_R {
        ALIGN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align8(&self) -> ALIGN8_R {
        ALIGN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align0(&mut self) -> ALIGN0_W {
        ALIGN0_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align1(&mut self) -> ALIGN1_W {
        ALIGN1_W { w: self }
    }
    #[doc = "Bit 2 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align2(&mut self) -> ALIGN2_W {
        ALIGN2_W { w: self }
    }
    #[doc = "Bit 3 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align3(&mut self) -> ALIGN3_W {
        ALIGN3_W { w: self }
    }
    #[doc = "Bit 4 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align4(&mut self) -> ALIGN4_W {
        ALIGN4_W { w: self }
    }
    #[doc = "Bit 5 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align5(&mut self) -> ALIGN5_W {
        ALIGN5_W { w: self }
    }
    #[doc = "Bit 6 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align6(&mut self) -> ALIGN6_W {
        ALIGN6_W { w: self }
    }
    #[doc = "Bit 7 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align7(&mut self) -> ALIGN7_W {
        ALIGN7_W { w: self }
    }
    #[doc = "Bit 8 - Transfer Alignment Flag This bit indicates whether source and destination address both follow transfer width setting, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn align8(&mut self) -> ALIGN8_W {
        ALIGN8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Transfer Alignment Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_align](index.html) module"]
pub struct PDMA_ALIGN_SPEC;
impl crate::RegisterSpec for PDMA_ALIGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_align::R](R) reader structure"]
impl crate::Readable for PDMA_ALIGN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_align::W](W) writer structure"]
impl crate::Writable for PDMA_ALIGN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_ALIGN to value 0"]
impl crate::Resettable for PDMA_ALIGN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
