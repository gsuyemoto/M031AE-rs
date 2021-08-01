#[doc = "Register `ACMP_CALSR` reader"]
pub struct R(crate::R<ACMP_CALSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMP_CALSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMP_CALSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMP_CALSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Comparator0 Calibration Done Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE0_A {
    #[doc = "0: Calibrating"]
    _0 = 0,
    #[doc = "1: Calibration Done"]
    _1 = 1,
}
impl From<DONE0_A> for bool {
    #[inline(always)]
    fn from(variant: DONE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE0` reader - Comparator0 Calibration Done Status"]
pub struct DONE0_R(crate::FieldReader<bool, DONE0_A>);
impl DONE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE0_A {
        match self.bits {
            false => DONE0_A::_0,
            true => DONE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DONE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DONE0_A::_1
    }
}
impl core::ops::Deref for DONE0_R {
    type Target = crate::FieldReader<bool, DONE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator0 Calibration Result Status for NMOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALNS0_A {
    #[doc = "0: Pass"]
    _0 = 0,
    #[doc = "1: Fail"]
    _1 = 1,
}
impl From<CALNS0_A> for bool {
    #[inline(always)]
    fn from(variant: CALNS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALNS0` reader - Comparator0 Calibration Result Status for NMOS"]
pub struct CALNS0_R(crate::FieldReader<bool, CALNS0_A>);
impl CALNS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALNS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALNS0_A {
        match self.bits {
            false => CALNS0_A::_0,
            true => CALNS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALNS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CALNS0_A::_1
    }
}
impl core::ops::Deref for CALNS0_R {
    type Target = crate::FieldReader<bool, CALNS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator0 Calibration Result Status for PMOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALPS0_A {
    #[doc = "0: Pass"]
    _0 = 0,
    #[doc = "1: Fail"]
    _1 = 1,
}
impl From<CALPS0_A> for bool {
    #[inline(always)]
    fn from(variant: CALPS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALPS0` reader - Comparator0 Calibration Result Status for PMOS"]
pub struct CALPS0_R(crate::FieldReader<bool, CALPS0_A>);
impl CALPS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALPS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALPS0_A {
        match self.bits {
            false => CALPS0_A::_0,
            true => CALPS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALPS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CALPS0_A::_1
    }
}
impl core::ops::Deref for CALPS0_R {
    type Target = crate::FieldReader<bool, CALPS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator1 Calibration Done Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE1_A {
    #[doc = "0: Calibrating"]
    _0 = 0,
    #[doc = "1: Calibration Done"]
    _1 = 1,
}
impl From<DONE1_A> for bool {
    #[inline(always)]
    fn from(variant: DONE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE1` reader - Comparator1 Calibration Done Status"]
pub struct DONE1_R(crate::FieldReader<bool, DONE1_A>);
impl DONE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE1_A {
        match self.bits {
            false => DONE1_A::_0,
            true => DONE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DONE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DONE1_A::_1
    }
}
impl core::ops::Deref for DONE1_R {
    type Target = crate::FieldReader<bool, DONE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator1 Calibration Result Status for NMOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALNS1_A {
    #[doc = "0: Pass"]
    _0 = 0,
    #[doc = "1: Fail"]
    _1 = 1,
}
impl From<CALNS1_A> for bool {
    #[inline(always)]
    fn from(variant: CALNS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALNS1` reader - Comparator1 Calibration Result Status for NMOS"]
pub struct CALNS1_R(crate::FieldReader<bool, CALNS1_A>);
impl CALNS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALNS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALNS1_A {
        match self.bits {
            false => CALNS1_A::_0,
            true => CALNS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALNS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CALNS1_A::_1
    }
}
impl core::ops::Deref for CALNS1_R {
    type Target = crate::FieldReader<bool, CALNS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator1 Calibration Result Status for PMOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALPS1_A {
    #[doc = "0: Pass"]
    _0 = 0,
    #[doc = "1: Fail"]
    _1 = 1,
}
impl From<CALPS1_A> for bool {
    #[inline(always)]
    fn from(variant: CALPS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALPS1` reader - Comparator1 Calibration Result Status for PMOS"]
pub struct CALPS1_R(crate::FieldReader<bool, CALPS1_A>);
impl CALPS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALPS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALPS1_A {
        match self.bits {
            false => CALPS1_A::_0,
            true => CALPS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CALPS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CALPS1_A::_1
    }
}
impl core::ops::Deref for CALPS1_R {
    type Target = crate::FieldReader<bool, CALPS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Comparator0 Calibration Done Status"]
    #[inline(always)]
    pub fn done0(&self) -> DONE0_R {
        DONE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator0 Calibration Result Status for NMOS"]
    #[inline(always)]
    pub fn calns0(&self) -> CALNS0_R {
        CALNS0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator0 Calibration Result Status for PMOS"]
    #[inline(always)]
    pub fn calps0(&self) -> CALPS0_R {
        CALPS0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator1 Calibration Done Status"]
    #[inline(always)]
    pub fn done1(&self) -> DONE1_R {
        DONE1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator1 Calibration Result Status for NMOS"]
    #[inline(always)]
    pub fn calns1(&self) -> CALNS1_R {
        CALNS1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comparator1 Calibration Result Status for PMOS"]
    #[inline(always)]
    pub fn calps1(&self) -> CALPS1_R {
        CALPS1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Analog Comparator Calibration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmp_calsr](index.html) module"]
pub struct ACMP_CALSR_SPEC;
impl crate::RegisterSpec for ACMP_CALSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmp_calsr::R](R) reader structure"]
impl crate::Readable for ACMP_CALSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACMP_CALSR to value 0"]
impl crate::Resettable for ACMP_CALSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
