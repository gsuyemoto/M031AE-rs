#[doc = "Register `ADC_ADDR9` reader"]
pub struct R(crate::R<ADC_ADDR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_ADDR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_ADDR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_ADDR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSLT` reader - A/D Conversion Result (Read Only)\nThis field contains conversion result of ADC."]
pub struct RSLT_R(crate::FieldReader<u16, u16>);
impl RSLT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RSLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSLT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Overrun Flag (Read Only)\nIf converted data in RSLT bits has not been read before new conversion result is loaded to this register, OVERRUN bit is set to 1. It is cleared by hardware after ADDR register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUN_A {
    #[doc = "0: Data in RSLT bits is not overwritten"]
    _0 = 0,
    #[doc = "1: Data in RSLT bits is overwritten"]
    _1 = 1,
}
impl From<OVERRUN_A> for bool {
    #[inline(always)]
    fn from(variant: OVERRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` reader - Overrun Flag (Read Only)\nIf converted data in RSLT bits has not been read before new conversion result is loaded to this register, OVERRUN bit is set to 1. It is cleared by hardware after ADDR register is read."]
pub struct OVERRUN_R(crate::FieldReader<bool, OVERRUN_A>);
impl OVERRUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERRUN_A {
        match self.bits {
            false => OVERRUN_A::_0,
            true => OVERRUN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OVERRUN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OVERRUN_A::_1
    }
}
impl core::ops::Deref for OVERRUN_R {
    type Target = crate::FieldReader<bool, OVERRUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Valid Flag (Read Only)\nThis bit will be set to 1 when the conversion of the corresponding channel is completed. This bit will be cleared to 0 by hardware after ADDR register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_A {
    #[doc = "0: Data in RSLT bits is not valid"]
    _0 = 0,
    #[doc = "1: Data in RSLT bits is valid"]
    _1 = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - Valid Flag (Read Only)\nThis bit will be set to 1 when the conversion of the corresponding channel is completed. This bit will be cleared to 0 by hardware after ADDR register is read."]
pub struct VALID_R(crate::FieldReader<bool, VALID_A>);
impl VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::_0,
            true => VALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VALID_A::_1
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<bool, VALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - A/D Conversion Result (Read Only) This field contains conversion result of ADC."]
    #[inline(always)]
    pub fn rslt(&self) -> RSLT_R {
        RSLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overrun Flag (Read Only) If converted data in RSLT bits has not been read before new conversion result is loaded to this register, OVERRUN bit is set to 1. It is cleared by hardware after ADDR register is read."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Valid Flag (Read Only) This bit will be set to 1 when the conversion of the corresponding channel is completed. This bit will be cleared to 0 by hardware after ADDR register is read."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "ADC Data Register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_addr9](index.html) module"]
pub struct ADC_ADDR9_SPEC;
impl crate::RegisterSpec for ADC_ADDR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_addr9::R](R) reader structure"]
impl crate::Readable for ADC_ADDR9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_ADDR9 to value 0"]
impl crate::Resettable for ADC_ADDR9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
