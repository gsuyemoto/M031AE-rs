#[doc = "Register `PDMA_INTSTS` reader"]
pub struct R(crate::R<PDMA_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_INTSTS` writer"]
pub struct W(crate::W<PDMA_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_INTSTS_SPEC>;
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
impl From<crate::W<PDMA_INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Read/Write Target Abort Interrupt Flag (Read Only)\nThis bit indicates that PDMA has target abort error; Software can read PDMA_ABTSTS register to find which channel has target abort error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF_A {
    #[doc = "0: No AHB bus ERROR response received"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received"]
    _1 = 1,
}
impl From<ABTIF_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF` reader - PDMA Read/Write Target Abort Interrupt Flag (Read Only)\nThis bit indicates that PDMA has target abort error; Software can read PDMA_ABTSTS register to find which channel has target abort error."]
pub struct ABTIF_R(crate::FieldReader<bool, ABTIF_A>);
impl ABTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF_A {
        match self.bits {
            false => ABTIF_A::_0,
            true => ABTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF_A::_1
    }
}
impl core::ops::Deref for ABTIF_R {
    type Target = crate::FieldReader<bool, ABTIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer Done Interrupt Flag (Read Only)\nThis bit indicates that PDMA controller has finished transmission; User can read PDMA_TDSTS register to indicate which channel finished transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF_A {
    #[doc = "0: Not finished yet"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF` reader - Transfer Done Interrupt Flag (Read Only)\nThis bit indicates that PDMA controller has finished transmission; User can read PDMA_TDSTS register to indicate which channel finished transfer."]
pub struct TDIF_R(crate::FieldReader<bool, TDIF_A>);
impl TDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF_A {
        match self.bits {
            false => TDIF_A::_0,
            true => TDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF_A::_1
    }
}
impl core::ops::Deref for TDIF_R {
    type Target = crate::FieldReader<bool, TDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transfer Alignment Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGNF_A {
    #[doc = "0: PDMA channel source address and destination address both follow transfer width setting"]
    _0 = 0,
    #[doc = "1: PDMA channel source address or destination address is not follow transfer width setting"]
    _1 = 1,
}
impl From<ALIGNF_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGNF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGNF` reader - Transfer Alignment Interrupt Flag (Read Only)"]
pub struct ALIGNF_R(crate::FieldReader<bool, ALIGNF_A>);
impl ALIGNF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGNF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGNF_A {
        match self.bits {
            false => ALIGNF_A::_0,
            true => ALIGNF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALIGNF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALIGNF_A::_1
    }
}
impl core::ops::Deref for ALIGNF_R {
    type Target = crate::FieldReader<bool, ALIGNF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Request Time-out Flag for Channel 0\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC0, user can write 1 to clear these bits.\nNote: Please disable time-out function before clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQTOF0_A {
    #[doc = "0: No request time-out"]
    _0 = 0,
    #[doc = "1: Peripheral request time-out"]
    _1 = 1,
}
impl From<REQTOF0_A> for bool {
    #[inline(always)]
    fn from(variant: REQTOF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQTOF0` reader - Request Time-out Flag for Channel 0\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC0, user can write 1 to clear these bits.\nNote: Please disable time-out function before clear this bit."]
pub struct REQTOF0_R(crate::FieldReader<bool, REQTOF0_A>);
impl REQTOF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQTOF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQTOF0_A {
        match self.bits {
            false => REQTOF0_A::_0,
            true => REQTOF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQTOF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQTOF0_A::_1
    }
}
impl core::ops::Deref for REQTOF0_R {
    type Target = crate::FieldReader<bool, REQTOF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQTOF0` writer - Request Time-out Flag for Channel 0\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC0, user can write 1 to clear these bits.\nNote: Please disable time-out function before clear this bit."]
pub struct REQTOF0_W<'a> {
    w: &'a mut W,
}
impl<'a> REQTOF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQTOF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No request time-out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REQTOF0_A::_0)
    }
    #[doc = "Peripheral request time-out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REQTOF0_A::_1)
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
#[doc = "Request Time-out Flag for Channel 1\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC1, user can write 1 to clear these bits.\nNote: Please disable time-out function before clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQTOF1_A {
    #[doc = "0: No request time-out"]
    _0 = 0,
    #[doc = "1: Peripheral request time-out"]
    _1 = 1,
}
impl From<REQTOF1_A> for bool {
    #[inline(always)]
    fn from(variant: REQTOF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQTOF1` reader - Request Time-out Flag for Channel 1\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC1, user can write 1 to clear these bits.\nNote: Please disable time-out function before clear this bit."]
pub struct REQTOF1_R(crate::FieldReader<bool, REQTOF1_A>);
impl REQTOF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQTOF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQTOF1_A {
        match self.bits {
            false => REQTOF1_A::_0,
            true => REQTOF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQTOF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQTOF1_A::_1
    }
}
impl core::ops::Deref for REQTOF1_R {
    type Target = crate::FieldReader<bool, REQTOF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQTOF1` writer - Request Time-out Flag for Channel 1\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC1, user can write 1 to clear these bits.\nNote: Please disable time-out function before clear this bit."]
pub struct REQTOF1_W<'a> {
    w: &'a mut W,
}
impl<'a> REQTOF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQTOF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No request time-out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REQTOF1_A::_0)
    }
    #[doc = "Peripheral request time-out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REQTOF1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDMA Read/Write Target Abort Interrupt Flag (Read Only) This bit indicates that PDMA has target abort error; Software can read PDMA_ABTSTS register to find which channel has target abort error."]
    #[inline(always)]
    pub fn abtif(&self) -> ABTIF_R {
        ABTIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Done Interrupt Flag (Read Only) This bit indicates that PDMA controller has finished transmission; User can read PDMA_TDSTS register to indicate which channel finished transfer."]
    #[inline(always)]
    pub fn tdif(&self) -> TDIF_R {
        TDIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Alignment Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn alignf(&self) -> ALIGNF_R {
        ALIGNF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Request Time-out Flag for Channel 0 This flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC0, user can write 1 to clear these bits. Note: Please disable time-out function before clear this bit."]
    #[inline(always)]
    pub fn reqtof0(&self) -> REQTOF0_R {
        REQTOF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Request Time-out Flag for Channel 1 This flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC1, user can write 1 to clear these bits. Note: Please disable time-out function before clear this bit."]
    #[inline(always)]
    pub fn reqtof1(&self) -> REQTOF1_R {
        REQTOF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Request Time-out Flag for Channel 0 This flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC0, user can write 1 to clear these bits. Note: Please disable time-out function before clear this bit."]
    #[inline(always)]
    pub fn reqtof0(&mut self) -> REQTOF0_W {
        REQTOF0_W { w: self }
    }
    #[doc = "Bit 9 - Request Time-out Flag for Channel 1 This flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOC1, user can write 1 to clear these bits. Note: Please disable time-out function before clear this bit."]
    #[inline(always)]
    pub fn reqtof1(&mut self) -> REQTOF1_W {
        REQTOF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_intsts](index.html) module"]
pub struct PDMA_INTSTS_SPEC;
impl crate::RegisterSpec for PDMA_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_intsts::R](R) reader structure"]
impl crate::Readable for PDMA_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_intsts::W](W) writer structure"]
impl crate::Writable for PDMA_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_INTSTS to value 0"]
impl crate::Resettable for PDMA_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
