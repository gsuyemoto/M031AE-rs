#[doc = "Register `ADC_ADCMPR0` reader"]
pub struct R(crate::R<ADC_ADCMPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADCMPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADCMPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADCMPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_ADCMPR0` writer"]
pub struct W(crate::W<ADC_ADCMPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_ADCMPR0_SPEC>;
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
impl From<crate::W<ADC_ADCMPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_ADCMPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Compare Enable Bit\nSet this bit to 1 to enable ADC controller to compare CMPD (ADCMPRx\\[27:16\\]) with specified channel conversion result when converted data is loaded into ADDR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPEN_A {
    #[doc = "0: Compare function Disabled"]
    _0 = 0,
    #[doc = "1: Compare function Enabled"]
    _1 = 1,
}
impl From<CMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEN` reader - Compare Enable Bit\nSet this bit to 1 to enable ADC controller to compare CMPD (ADCMPRx\\[27:16\\]) with specified channel conversion result when converted data is loaded into ADDR register."]
pub struct CMPEN_R(crate::FieldReader<bool, CMPEN_A>);
impl CMPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN_A {
        match self.bits {
            false => CMPEN_A::_0,
            true => CMPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPEN_A::_1
    }
}
impl core::ops::Deref for CMPEN_R {
    type Target = crate::FieldReader<bool, CMPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPEN` writer - Compare Enable Bit\nSet this bit to 1 to enable ADC controller to compare CMPD (ADCMPRx\\[27:16\\]) with specified channel conversion result when converted data is loaded into ADDR register."]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN_A::_0)
    }
    #[doc = "Compare function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN_A::_1)
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
#[doc = "Compare Interrupt Enable Bit\nIf the compare function is enabled and the compare condition matches the setting of CMPCOND and CMPMATCNT, CMPFx bit will be asserted, in the meanwhile, if CMPIE bit is set to 1, a compare interrupt request is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPIE_A {
    #[doc = "0: Compare function interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare function interrupt Enabled"]
    _1 = 1,
}
impl From<CMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPIE` reader - Compare Interrupt Enable Bit\nIf the compare function is enabled and the compare condition matches the setting of CMPCOND and CMPMATCNT, CMPFx bit will be asserted, in the meanwhile, if CMPIE bit is set to 1, a compare interrupt request is generated."]
pub struct CMPIE_R(crate::FieldReader<bool, CMPIE_A>);
impl CMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPIE_A {
        match self.bits {
            false => CMPIE_A::_0,
            true => CMPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPIE_A::_1
    }
}
impl core::ops::Deref for CMPIE_R {
    type Target = crate::FieldReader<bool, CMPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPIE` writer - Compare Interrupt Enable Bit\nIf the compare function is enabled and the compare condition matches the setting of CMPCOND and CMPMATCNT, CMPFx bit will be asserted, in the meanwhile, if CMPIE bit is set to 1, a compare interrupt request is generated."]
pub struct CMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare function interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPIE_A::_0)
    }
    #[doc = "Compare function interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPIE_A::_1)
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
#[doc = "Compare Condition\nNote: When the internal counter reaches to (CMPMATCNT +1), the CMPFx bit will be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPCOND_A {
    #[doc = "0: Set the compare condition as that when a 12-bit A/D conversion result is less than the 12-bit CMPD bits, the internal match counter will increase one"]
    _0 = 0,
    #[doc = "1: Set the compare condition as that when a 12-bit A/D conversion result is greater than or equal to the 12-bit CMPD bits, the internal match counter will increase one"]
    _1 = 1,
}
impl From<CMPCOND_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCOND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCOND` reader - Compare Condition\nNote: When the internal counter reaches to (CMPMATCNT +1), the CMPFx bit will be set."]
pub struct CMPCOND_R(crate::FieldReader<bool, CMPCOND_A>);
impl CMPCOND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPCOND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCOND_A {
        match self.bits {
            false => CMPCOND_A::_0,
            true => CMPCOND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPCOND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPCOND_A::_1
    }
}
impl core::ops::Deref for CMPCOND_R {
    type Target = crate::FieldReader<bool, CMPCOND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPCOND` writer - Compare Condition\nNote: When the internal counter reaches to (CMPMATCNT +1), the CMPFx bit will be set."]
pub struct CMPCOND_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPCOND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPCOND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set the compare condition as that when a 12-bit A/D conversion result is less than the 12-bit CMPD bits, the internal match counter will increase one"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCOND_A::_0)
    }
    #[doc = "Set the compare condition as that when a 12-bit A/D conversion result is greater than or equal to the 12-bit CMPD bits, the internal match counter will increase one"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCOND_A::_1)
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
#[doc = "Compare Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPCH_A {
    #[doc = "0: Channel 0 conversion result is selected to be compared"]
    _0 = 0,
    #[doc = "1: Channel 1 conversion result is selected to be compared"]
    _1 = 1,
    #[doc = "2: Channel 2 conversion result is selected to be compared"]
    _2 = 2,
    #[doc = "3: Channel 3 conversion result is selected to be compared"]
    _3 = 3,
    #[doc = "4: Channel 4 conversion result is selected to be compared"]
    _4 = 4,
    #[doc = "5: Channel 5 conversion result is selected to be compared"]
    _5 = 5,
    #[doc = "6: Channel 6 conversion result is selected to be compared"]
    _6 = 6,
    #[doc = "7: Channel 7 conversion result is selected to be compared"]
    _7 = 7,
    #[doc = "8: Channel 8 conversion result is selected to be compared"]
    _8 = 8,
    #[doc = "9: Channel 9 conversion result is selected to be compared"]
    _9 = 9,
    #[doc = "10: Channel 10 conversion result is selected to be compared"]
    _10 = 10,
    #[doc = "11: Channel 11 conversion result is selected to be compared"]
    _11 = 11,
    #[doc = "12: Channel 12 conversion result is selected to be compared"]
    _12 = 12,
    #[doc = "13: Channel 13 conversion result is selected to be compared"]
    _13 = 13,
    #[doc = "14: Channel 14 conversion result is selected to be compared"]
    _14 = 14,
    #[doc = "15: Channel 15 conversion result is selected to be compared"]
    _15 = 15,
    #[doc = "28: Floating detect channel conversion result is selected to be compared"]
    _28 = 28,
    #[doc = "29: Band-gap voltage conversion result is selected to be compared"]
    _29 = 29,
}
impl From<CMPCH_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPCH` reader - Compare Channel Selection"]
pub struct CMPCH_R(crate::FieldReader<u8, CMPCH_A>);
impl CMPCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPCH_A> {
        match self.bits {
            0 => Some(CMPCH_A::_0),
            1 => Some(CMPCH_A::_1),
            2 => Some(CMPCH_A::_2),
            3 => Some(CMPCH_A::_3),
            4 => Some(CMPCH_A::_4),
            5 => Some(CMPCH_A::_5),
            6 => Some(CMPCH_A::_6),
            7 => Some(CMPCH_A::_7),
            8 => Some(CMPCH_A::_8),
            9 => Some(CMPCH_A::_9),
            10 => Some(CMPCH_A::_10),
            11 => Some(CMPCH_A::_11),
            12 => Some(CMPCH_A::_12),
            13 => Some(CMPCH_A::_13),
            14 => Some(CMPCH_A::_14),
            15 => Some(CMPCH_A::_15),
            28 => Some(CMPCH_A::_28),
            29 => Some(CMPCH_A::_29),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPCH_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPCH_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPCH_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == CMPCH_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == CMPCH_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == CMPCH_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == CMPCH_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == CMPCH_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == CMPCH_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CMPCH_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CMPCH_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == CMPCH_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == CMPCH_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == CMPCH_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == CMPCH_A::_15
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        **self == CMPCH_A::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        **self == CMPCH_A::_29
    }
}
impl core::ops::Deref for CMPCH_R {
    type Target = crate::FieldReader<u8, CMPCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPCH` writer - Compare Channel Selection"]
pub struct CMPCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 0 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCH_A::_0)
    }
    #[doc = "Channel 1 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCH_A::_1)
    }
    #[doc = "Channel 2 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPCH_A::_2)
    }
    #[doc = "Channel 3 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPCH_A::_3)
    }
    #[doc = "Channel 4 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CMPCH_A::_4)
    }
    #[doc = "Channel 5 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(CMPCH_A::_5)
    }
    #[doc = "Channel 6 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(CMPCH_A::_6)
    }
    #[doc = "Channel 7 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(CMPCH_A::_7)
    }
    #[doc = "Channel 8 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CMPCH_A::_8)
    }
    #[doc = "Channel 9 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(CMPCH_A::_9)
    }
    #[doc = "Channel 10 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPCH_A::_10)
    }
    #[doc = "Channel 11 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPCH_A::_11)
    }
    #[doc = "Channel 12 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(CMPCH_A::_12)
    }
    #[doc = "Channel 13 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(CMPCH_A::_13)
    }
    #[doc = "Channel 14 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(CMPCH_A::_14)
    }
    #[doc = "Channel 15 conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(CMPCH_A::_15)
    }
    #[doc = "Floating detect channel conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut W {
        self.variant(CMPCH_A::_28)
    }
    #[doc = "Band-gap voltage conversion result is selected to be compared"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut W {
        self.variant(CMPCH_A::_29)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `CMPMATCNT` reader - Compare Match Count\nWhen the specified A/D channel analog conversion result matches the compare condition defined by CMPCOND bit, the internal match counter will increase 1. When the internal counter reaches the value to (CMPMATCNT +1), the CMPFx bit will be set."]
pub struct CMPMATCNT_R(crate::FieldReader<u8, u8>);
impl CMPMATCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPMATCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPMATCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPMATCNT` writer - Compare Match Count\nWhen the specified A/D channel analog conversion result matches the compare condition defined by CMPCOND bit, the internal match counter will increase 1. When the internal counter reaches the value to (CMPMATCNT +1), the CMPFx bit will be set."]
pub struct CMPMATCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMATCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Compare Window Mode Enable Bit\nNote: This bit is only presented in ADCMPR0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPWEN_A {
    #[doc = "0: Compare Window Mode Disabled"]
    _0 = 0,
    #[doc = "1: Compare Window Mode Enabled"]
    _1 = 1,
}
impl From<CMPWEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMPWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPWEN` reader - Compare Window Mode Enable Bit\nNote: This bit is only presented in ADCMPR0 register."]
pub struct CMPWEN_R(crate::FieldReader<bool, CMPWEN_A>);
impl CMPWEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPWEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPWEN_A {
        match self.bits {
            false => CMPWEN_A::_0,
            true => CMPWEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPWEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPWEN_A::_1
    }
}
impl core::ops::Deref for CMPWEN_R {
    type Target = crate::FieldReader<bool, CMPWEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPWEN` writer - Compare Window Mode Enable Bit\nNote: This bit is only presented in ADCMPR0 register."]
pub struct CMPWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPWEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPWEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare Window Mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPWEN_A::_0)
    }
    #[doc = "Compare Window Mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPWEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CMPD` reader - Comparison Data\nThe 12-bit data is used to compare with conversion result of specified channel.\nNote: CMPD bits should be filled in unsigned format (straight binary format)."]
pub struct CMPD_R(crate::FieldReader<u16, u16>);
impl CMPD_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPD` writer - Comparison Data\nThe 12-bit data is used to compare with conversion result of specified channel.\nNote: CMPD bits should be filled in unsigned format (straight binary format)."]
pub struct CMPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Compare Enable Bit Set this bit to 1 to enable ADC controller to compare CMPD (ADCMPRx\\[27:16\\]) with specified channel conversion result when converted data is loaded into ADDR register."]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Compare Interrupt Enable Bit If the compare function is enabled and the compare condition matches the setting of CMPCOND and CMPMATCNT, CMPFx bit will be asserted, in the meanwhile, if CMPIE bit is set to 1, a compare interrupt request is generated."]
    #[inline(always)]
    pub fn cmpie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Compare Condition Note: When the internal counter reaches to (CMPMATCNT +1), the CMPFx bit will be set."]
    #[inline(always)]
    pub fn cmpcond(&self) -> CMPCOND_R {
        CMPCOND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - Compare Channel Selection"]
    #[inline(always)]
    pub fn cmpch(&self) -> CMPCH_R {
        CMPCH_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Compare Match Count When the specified A/D channel analog conversion result matches the compare condition defined by CMPCOND bit, the internal match counter will increase 1. When the internal counter reaches the value to (CMPMATCNT +1), the CMPFx bit will be set."]
    #[inline(always)]
    pub fn cmpmatcnt(&self) -> CMPMATCNT_R {
        CMPMATCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Compare Window Mode Enable Bit Note: This bit is only presented in ADCMPR0 register."]
    #[inline(always)]
    pub fn cmpwen(&self) -> CMPWEN_R {
        CMPWEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Comparison Data The 12-bit data is used to compare with conversion result of specified channel. Note: CMPD bits should be filled in unsigned format (straight binary format)."]
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Enable Bit Set this bit to 1 to enable ADC controller to compare CMPD (ADCMPRx\\[27:16\\]) with specified channel conversion result when converted data is loaded into ADDR register."]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Bit 1 - Compare Interrupt Enable Bit If the compare function is enabled and the compare condition matches the setting of CMPCOND and CMPMATCNT, CMPFx bit will be asserted, in the meanwhile, if CMPIE bit is set to 1, a compare interrupt request is generated."]
    #[inline(always)]
    pub fn cmpie(&mut self) -> CMPIE_W {
        CMPIE_W { w: self }
    }
    #[doc = "Bit 2 - Compare Condition Note: When the internal counter reaches to (CMPMATCNT +1), the CMPFx bit will be set."]
    #[inline(always)]
    pub fn cmpcond(&mut self) -> CMPCOND_W {
        CMPCOND_W { w: self }
    }
    #[doc = "Bits 3:7 - Compare Channel Selection"]
    #[inline(always)]
    pub fn cmpch(&mut self) -> CMPCH_W {
        CMPCH_W { w: self }
    }
    #[doc = "Bits 8:11 - Compare Match Count When the specified A/D channel analog conversion result matches the compare condition defined by CMPCOND bit, the internal match counter will increase 1. When the internal counter reaches the value to (CMPMATCNT +1), the CMPFx bit will be set."]
    #[inline(always)]
    pub fn cmpmatcnt(&mut self) -> CMPMATCNT_W {
        CMPMATCNT_W { w: self }
    }
    #[doc = "Bit 15 - Compare Window Mode Enable Bit Note: This bit is only presented in ADCMPR0 register."]
    #[inline(always)]
    pub fn cmpwen(&mut self) -> CMPWEN_W {
        CMPWEN_W { w: self }
    }
    #[doc = "Bits 16:27 - Comparison Data The 12-bit data is used to compare with conversion result of specified channel. Note: CMPD bits should be filled in unsigned format (straight binary format)."]
    #[inline(always)]
    pub fn cmpd(&mut self) -> CMPD_W {
        CMPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Compare Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_adcmpr0](index.html) module"]
pub struct ADC_ADCMPR0_SPEC;
impl crate::RegisterSpec for ADC_ADCMPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_adcmpr0::R](R) reader structure"]
impl crate::Readable for ADC_ADCMPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_adcmpr0::W](W) writer structure"]
impl crate::Writable for ADC_ADCMPR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_ADCMPR0 to value 0"]
impl crate::Resettable for ADC_ADCMPR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
