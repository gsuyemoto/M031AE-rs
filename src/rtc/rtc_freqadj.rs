#[doc = "Register `RTC_FREQADJ` reader"]
pub struct R(crate::R<RTC_FREQADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_FREQADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_FREQADJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_FREQADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_FREQADJ` writer"]
pub struct W(crate::W<RTC_FREQADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_FREQADJ_SPEC>;
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
impl From<crate::W<RTC_FREQADJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_FREQADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACTION` reader - Fraction Part\nNote: Digit in FCR must be expressed as hexadecimal number."]
pub struct FRACTION_R(crate::FieldReader<u8, u8>);
impl FRACTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRACTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRACTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRACTION` writer - Fraction Part\nNote: Digit in FCR must be expressed as hexadecimal number."]
pub struct FRACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Integer Part\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTEGER_A {
    #[doc = "0: Integer part of detected value is 32752"]
    _0 = 0,
    #[doc = "1: Integer part of detected value is 32753"]
    _1 = 1,
    #[doc = "2: Integer part of detected value is 32754"]
    _2 = 2,
    #[doc = "3: Integer part of detected value is 32755"]
    _3 = 3,
    #[doc = "4: Integer part of detected value is 32756"]
    _4 = 4,
    #[doc = "5: Integer part of detected value is 32757"]
    _5 = 5,
    #[doc = "6: Integer part of detected value is 32758"]
    _6 = 6,
    #[doc = "7: Integer part of detected value is 32759"]
    _7 = 7,
    #[doc = "8: Integer part of detected value is 32760"]
    _8 = 8,
    #[doc = "9: Integer part of detected value is 32761"]
    _9 = 9,
    #[doc = "10: Integer part of detected value is 32762"]
    _10 = 10,
    #[doc = "11: Integer part of detected value is 32763"]
    _11 = 11,
    #[doc = "12: Integer part of detected value is 32764"]
    _12 = 12,
    #[doc = "13: Integer part of detected value is 32765"]
    _13 = 13,
    #[doc = "14: Integer part of detected value is 32766"]
    _14 = 14,
    #[doc = "15: Integer part of detected value is 32767"]
    _15 = 15,
    #[doc = "16: Integer part of detected value is 32768"]
    _16 = 16,
    #[doc = "17: Integer part of detected value is 32769"]
    _17 = 17,
    #[doc = "18: Integer part of detected value is 32770"]
    _18 = 18,
    #[doc = "19: Integer part of detected value is 32771"]
    _19 = 19,
    #[doc = "20: Integer part of detected value is 32772"]
    _20 = 20,
    #[doc = "21: Integer part of detected value is 32773"]
    _21 = 21,
    #[doc = "22: Integer part of detected value is 32774"]
    _22 = 22,
    #[doc = "23: Integer part of detected value is 32775"]
    _23 = 23,
    #[doc = "24: Integer part of detected value is 32776"]
    _24 = 24,
    #[doc = "25: Integer part of detected value is 32777"]
    _25 = 25,
    #[doc = "26: Integer part of detected value is 32778"]
    _26 = 26,
    #[doc = "27: Integer part of detected value is 32779"]
    _27 = 27,
    #[doc = "28: Integer part of detected value is 32780"]
    _28 = 28,
    #[doc = "29: Integer part of detected value is 32781"]
    _29 = 29,
    #[doc = "30: Integer part of detected value is 32782"]
    _30 = 30,
    #[doc = "31: Integer part of detected value is 32783"]
    _31 = 31,
}
impl From<INTEGER_A> for u8 {
    #[inline(always)]
    fn from(variant: INTEGER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTEGER` reader - Integer Part"]
pub struct INTEGER_R(crate::FieldReader<u8, INTEGER_A>);
impl INTEGER_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTEGER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEGER_A {
        match self.bits {
            0 => INTEGER_A::_0,
            1 => INTEGER_A::_1,
            2 => INTEGER_A::_2,
            3 => INTEGER_A::_3,
            4 => INTEGER_A::_4,
            5 => INTEGER_A::_5,
            6 => INTEGER_A::_6,
            7 => INTEGER_A::_7,
            8 => INTEGER_A::_8,
            9 => INTEGER_A::_9,
            10 => INTEGER_A::_10,
            11 => INTEGER_A::_11,
            12 => INTEGER_A::_12,
            13 => INTEGER_A::_13,
            14 => INTEGER_A::_14,
            15 => INTEGER_A::_15,
            16 => INTEGER_A::_16,
            17 => INTEGER_A::_17,
            18 => INTEGER_A::_18,
            19 => INTEGER_A::_19,
            20 => INTEGER_A::_20,
            21 => INTEGER_A::_21,
            22 => INTEGER_A::_22,
            23 => INTEGER_A::_23,
            24 => INTEGER_A::_24,
            25 => INTEGER_A::_25,
            26 => INTEGER_A::_26,
            27 => INTEGER_A::_27,
            28 => INTEGER_A::_28,
            29 => INTEGER_A::_29,
            30 => INTEGER_A::_30,
            31 => INTEGER_A::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEGER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEGER_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == INTEGER_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == INTEGER_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == INTEGER_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == INTEGER_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == INTEGER_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == INTEGER_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == INTEGER_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == INTEGER_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == INTEGER_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == INTEGER_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == INTEGER_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == INTEGER_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == INTEGER_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == INTEGER_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == INTEGER_A::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        **self == INTEGER_A::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        **self == INTEGER_A::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline(always)]
    pub fn is_19(&self) -> bool {
        **self == INTEGER_A::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        **self == INTEGER_A::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline(always)]
    pub fn is_21(&self) -> bool {
        **self == INTEGER_A::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline(always)]
    pub fn is_22(&self) -> bool {
        **self == INTEGER_A::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline(always)]
    pub fn is_23(&self) -> bool {
        **self == INTEGER_A::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        **self == INTEGER_A::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        **self == INTEGER_A::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        **self == INTEGER_A::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        **self == INTEGER_A::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        **self == INTEGER_A::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        **self == INTEGER_A::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        **self == INTEGER_A::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        **self == INTEGER_A::_31
    }
}
impl core::ops::Deref for INTEGER_R {
    type Target = crate::FieldReader<u8, INTEGER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEGER` writer - Integer Part"]
pub struct INTEGER_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEGER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEGER_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Integer part of detected value is 32752"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEGER_A::_0)
    }
    #[doc = "Integer part of detected value is 32753"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEGER_A::_1)
    }
    #[doc = "Integer part of detected value is 32754"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(INTEGER_A::_2)
    }
    #[doc = "Integer part of detected value is 32755"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(INTEGER_A::_3)
    }
    #[doc = "Integer part of detected value is 32756"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(INTEGER_A::_4)
    }
    #[doc = "Integer part of detected value is 32757"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(INTEGER_A::_5)
    }
    #[doc = "Integer part of detected value is 32758"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(INTEGER_A::_6)
    }
    #[doc = "Integer part of detected value is 32759"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(INTEGER_A::_7)
    }
    #[doc = "Integer part of detected value is 32760"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(INTEGER_A::_8)
    }
    #[doc = "Integer part of detected value is 32761"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(INTEGER_A::_9)
    }
    #[doc = "Integer part of detected value is 32762"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(INTEGER_A::_10)
    }
    #[doc = "Integer part of detected value is 32763"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(INTEGER_A::_11)
    }
    #[doc = "Integer part of detected value is 32764"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(INTEGER_A::_12)
    }
    #[doc = "Integer part of detected value is 32765"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(INTEGER_A::_13)
    }
    #[doc = "Integer part of detected value is 32766"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(INTEGER_A::_14)
    }
    #[doc = "Integer part of detected value is 32767"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(INTEGER_A::_15)
    }
    #[doc = "Integer part of detected value is 32768"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(INTEGER_A::_16)
    }
    #[doc = "Integer part of detected value is 32769"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut W {
        self.variant(INTEGER_A::_17)
    }
    #[doc = "Integer part of detected value is 32770"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(INTEGER_A::_18)
    }
    #[doc = "Integer part of detected value is 32771"]
    #[inline(always)]
    pub fn _19(self) -> &'a mut W {
        self.variant(INTEGER_A::_19)
    }
    #[doc = "Integer part of detected value is 32772"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(INTEGER_A::_20)
    }
    #[doc = "Integer part of detected value is 32773"]
    #[inline(always)]
    pub fn _21(self) -> &'a mut W {
        self.variant(INTEGER_A::_21)
    }
    #[doc = "Integer part of detected value is 32774"]
    #[inline(always)]
    pub fn _22(self) -> &'a mut W {
        self.variant(INTEGER_A::_22)
    }
    #[doc = "Integer part of detected value is 32775"]
    #[inline(always)]
    pub fn _23(self) -> &'a mut W {
        self.variant(INTEGER_A::_23)
    }
    #[doc = "Integer part of detected value is 32776"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(INTEGER_A::_24)
    }
    #[doc = "Integer part of detected value is 32777"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut W {
        self.variant(INTEGER_A::_25)
    }
    #[doc = "Integer part of detected value is 32778"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut W {
        self.variant(INTEGER_A::_26)
    }
    #[doc = "Integer part of detected value is 32779"]
    #[inline(always)]
    pub fn _27(self) -> &'a mut W {
        self.variant(INTEGER_A::_27)
    }
    #[doc = "Integer part of detected value is 32780"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut W {
        self.variant(INTEGER_A::_28)
    }
    #[doc = "Integer part of detected value is 32781"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut W {
        self.variant(INTEGER_A::_29)
    }
    #[doc = "Integer part of detected value is 32782"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(INTEGER_A::_30)
    }
    #[doc = "Integer part of detected value is 32783"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(INTEGER_A::_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Fraction Part Note: Digit in FCR must be expressed as hexadecimal number."]
    #[inline(always)]
    pub fn fraction(&self) -> FRACTION_R {
        FRACTION_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Integer Part"]
    #[inline(always)]
    pub fn integer(&self) -> INTEGER_R {
        INTEGER_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fraction Part Note: Digit in FCR must be expressed as hexadecimal number."]
    #[inline(always)]
    pub fn fraction(&mut self) -> FRACTION_W {
        FRACTION_W { w: self }
    }
    #[doc = "Bits 8:12 - Integer Part"]
    #[inline(always)]
    pub fn integer(&mut self) -> INTEGER_W {
        INTEGER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Frequency Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_freqadj](index.html) module"]
pub struct RTC_FREQADJ_SPEC;
impl crate::RegisterSpec for RTC_FREQADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_freqadj::R](R) reader structure"]
impl crate::Readable for RTC_FREQADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_freqadj::W](W) writer structure"]
impl crate::Writable for RTC_FREQADJ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_FREQADJ to value 0x1000"]
impl crate::Resettable for RTC_FREQADJ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
