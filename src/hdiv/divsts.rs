#[doc = "Register `DIVSTS` reader"]
pub struct R(crate::R<DIVSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Divisor Zero Warning (Read Only)\nNote: The DIV0 flag is used to indicate divide-by-zero situation and updated whenever DIVISOR is written.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV0_A {
    #[doc = "0: The divisor is not 0"]
    _0 = 0,
    #[doc = "1: The divisor is 0"]
    _1 = 1,
}
impl From<DIV0_A> for bool {
    #[inline(always)]
    fn from(variant: DIV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV0` reader - Divisor Zero Warning (Read Only)\nNote: The DIV0 flag is used to indicate divide-by-zero situation and updated whenever DIVISOR is written."]
pub struct DIV0_R(crate::FieldReader<bool, DIV0_A>);
impl DIV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV0_A {
        match self.bits {
            false => DIV0_A::_0,
            true => DIV0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIV0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DIV0_A::_1
    }
}
impl core::ops::Deref for DIV0_R {
    type Target = crate::FieldReader<bool, DIV0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Divisor Zero Warning (Read Only) Note: The DIV0 flag is used to indicate divide-by-zero situation and updated whenever DIVISOR is written."]
    #[inline(always)]
    pub fn div0(&self) -> DIV0_R {
        DIV0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Divider Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divsts](index.html) module"]
pub struct DIVSTS_SPEC;
impl crate::RegisterSpec for DIVSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divsts::R](R) reader structure"]
impl crate::Readable for DIVSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIVSTS to value 0x01"]
impl crate::Resettable for DIVSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
