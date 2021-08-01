#[doc = "Register `SYS_SRAM_BISTCTL` reader"]
pub struct R(crate::R<SYS_SRAM_BISTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BISTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BISTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BISTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BISTCTL` writer"]
pub struct W(crate::W<SYS_SRAM_BISTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BISTCTL_SPEC>;
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
impl From<crate::W<SYS_SRAM_BISTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BISTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM BIST Enable Bit (Write Protect)\nThis bit enables BIST test for SRAM.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBIST_A {
    #[doc = "0: system SRAM BIST Disabled"]
    _0 = 0,
    #[doc = "1: system SRAM BIST Enabled"]
    _1 = 1,
}
impl From<SRBIST_A> for bool {
    #[inline(always)]
    fn from(variant: SRBIST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBIST` reader - SRAM BIST Enable Bit (Write Protect)\nThis bit enables BIST test for SRAM.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct SRBIST_R(crate::FieldReader<bool, SRBIST_A>);
impl SRBIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBIST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBIST_A {
        match self.bits {
            false => SRBIST_A::_0,
            true => SRBIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRBIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRBIST_A::_1
    }
}
impl core::ops::Deref for SRBIST_R {
    type Target = crate::FieldReader<bool, SRBIST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRBIST` writer - SRAM BIST Enable Bit (Write Protect)\nThis bit enables BIST test for SRAM.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct SRBIST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBIST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBIST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "system SRAM BIST Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRBIST_A::_0)
    }
    #[doc = "system SRAM BIST Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRBIST_A::_1)
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
#[doc = "FMC CACHE BIST Enable Bit (Write Protect)\nThis bit enables BIST test for CACHE RAM\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCBIST_A {
    #[doc = "0: System CACHE BIST Disabled"]
    _0 = 0,
    #[doc = "1: System CACHE BIST Enabled"]
    _1 = 1,
}
impl From<FMCBIST_A> for bool {
    #[inline(always)]
    fn from(variant: FMCBIST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCBIST` reader - FMC CACHE BIST Enable Bit (Write Protect)\nThis bit enables BIST test for CACHE RAM\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct FMCBIST_R(crate::FieldReader<bool, FMCBIST_A>);
impl FMCBIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMCBIST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCBIST_A {
        match self.bits {
            false => FMCBIST_A::_0,
            true => FMCBIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FMCBIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FMCBIST_A::_1
    }
}
impl core::ops::Deref for FMCBIST_R {
    type Target = crate::FieldReader<bool, FMCBIST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMCBIST` writer - FMC CACHE BIST Enable Bit (Write Protect)\nThis bit enables BIST test for CACHE RAM\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct FMCBIST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCBIST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCBIST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System CACHE BIST Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FMCBIST_A::_0)
    }
    #[doc = "System CACHE BIST Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FMCBIST_A::_1)
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
#[doc = "USB BIST Enable Bit (Write Protect) \nThis bit enables BIST test for USB RAM\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBBIST_A {
    #[doc = "0: system USB BIST Disabled"]
    _0 = 0,
    #[doc = "1: system USB BIST Enabled"]
    _1 = 1,
}
impl From<USBBIST_A> for bool {
    #[inline(always)]
    fn from(variant: USBBIST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBBIST` reader - USB BIST Enable Bit (Write Protect) \nThis bit enables BIST test for USB RAM\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct USBBIST_R(crate::FieldReader<bool, USBBIST_A>);
impl USBBIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBBIST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBBIST_A {
        match self.bits {
            false => USBBIST_A::_0,
            true => USBBIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBBIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBBIST_A::_1
    }
}
impl core::ops::Deref for USBBIST_R {
    type Target = crate::FieldReader<bool, USBBIST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBBIST` writer - USB BIST Enable Bit (Write Protect) \nThis bit enables BIST test for USB RAM\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct USBBIST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBBIST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBBIST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "system USB BIST Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBBIST_A::_0)
    }
    #[doc = "system USB BIST Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBBIST_A::_1)
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
#[doc = "PDMA BIST Enable Bit (Write Protect)\nThis bit enables BIST test for PDMA RAM\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMABIST_A {
    #[doc = "0: system PDMA BIST Disabled"]
    _0 = 0,
    #[doc = "1: system PDMA BIST Enabled"]
    _1 = 1,
}
impl From<PDMABIST_A> for bool {
    #[inline(always)]
    fn from(variant: PDMABIST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMABIST` reader - PDMA BIST Enable Bit (Write Protect)\nThis bit enables BIST test for PDMA RAM\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDMABIST_R(crate::FieldReader<bool, PDMABIST_A>);
impl PDMABIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMABIST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMABIST_A {
        match self.bits {
            false => PDMABIST_A::_0,
            true => PDMABIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDMABIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDMABIST_A::_1
    }
}
impl core::ops::Deref for PDMABIST_R {
    type Target = crate::FieldReader<bool, PDMABIST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMABIST` writer - PDMA BIST Enable Bit (Write Protect)\nThis bit enables BIST test for PDMA RAM\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDMABIST_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMABIST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMABIST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "system PDMA BIST Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDMABIST_A::_0)
    }
    #[doc = "system PDMA BIST Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDMABIST_A::_1)
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
#[doc = "SRAM Bank0 Section 0 BIST Select (Write Protect)\nThis bit define if the bank0 section0 (0x2000_0000~0x2000_7FFF) of SRAM is selected or not when doing bist test.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: At least one section of SRAM should be selected when doing SRAM bist test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRS0_A {
    #[doc = "0: SRAM bank0 section0 is deselected when doing bist test"]
    _0 = 0,
    #[doc = "1: SRAM bank0 section0 is selected when doing bist test"]
    _1 = 1,
}
impl From<SRS0_A> for bool {
    #[inline(always)]
    fn from(variant: SRS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRS0` reader - SRAM Bank0 Section 0 BIST Select (Write Protect)\nThis bit define if the bank0 section0 (0x2000_0000~0x2000_7FFF) of SRAM is selected or not when doing bist test.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: At least one section of SRAM should be selected when doing SRAM bist test."]
pub struct SRS0_R(crate::FieldReader<bool, SRS0_A>);
impl SRS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRS0_A {
        match self.bits {
            false => SRS0_A::_0,
            true => SRS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRS0_A::_1
    }
}
impl core::ops::Deref for SRS0_R {
    type Target = crate::FieldReader<bool, SRS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRS0` writer - SRAM Bank0 Section 0 BIST Select (Write Protect)\nThis bit define if the bank0 section0 (0x2000_0000~0x2000_7FFF) of SRAM is selected or not when doing bist test.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: At least one section of SRAM should be selected when doing SRAM bist test."]
pub struct SRS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAM bank0 section0 is deselected when doing bist test"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRS0_A::_0)
    }
    #[doc = "SRAM bank0 section0 is selected when doing bist test"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRS0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "SRAM Bank0 Section 1 BIST Select (Write Protect)\nThis bit define if the bank0 section1 (0x2000_8000~0x2000_FFFF) of SRAM is selected or not when doing bist test.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: At least one section of SRAM should be selected when doing SRAM bist test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRS1_A {
    #[doc = "0: SRAM bank0 section1 is deselected when doing bist test"]
    _0 = 0,
    #[doc = "1: SRAM bank0 section1 is selected when doing bist test"]
    _1 = 1,
}
impl From<SRS1_A> for bool {
    #[inline(always)]
    fn from(variant: SRS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRS1` reader - SRAM Bank0 Section 1 BIST Select (Write Protect)\nThis bit define if the bank0 section1 (0x2000_8000~0x2000_FFFF) of SRAM is selected or not when doing bist test.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: At least one section of SRAM should be selected when doing SRAM bist test."]
pub struct SRS1_R(crate::FieldReader<bool, SRS1_A>);
impl SRS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRS1_A {
        match self.bits {
            false => SRS1_A::_0,
            true => SRS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRS1_A::_1
    }
}
impl core::ops::Deref for SRS1_R {
    type Target = crate::FieldReader<bool, SRS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRS1` writer - SRAM Bank0 Section 1 BIST Select (Write Protect)\nThis bit define if the bank0 section1 (0x2000_8000~0x2000_FFFF) of SRAM is selected or not when doing bist test.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: At least one section of SRAM should be selected when doing SRAM bist test."]
pub struct SRS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAM bank0 section1 is deselected when doing bist test"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRS1_A::_0)
    }
    #[doc = "SRAM bank0 section1 is selected when doing bist test"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRS1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "SRAM Bank0 Section 2 BIST Select (Write Protect)\nThis bit define if the bank0 section2 (0x2001_0000~0x2001_7FFF) of SRAM is selected or not when doing bist test.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: At least one section of SRAM should be selected when doing SRAM bist test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRS2_A {
    #[doc = "0: SRAM back0 section2 is deselected when doing bist test"]
    _0 = 0,
    #[doc = "1: SRAM back0 section2 is selected when doing bist test"]
    _1 = 1,
}
impl From<SRS2_A> for bool {
    #[inline(always)]
    fn from(variant: SRS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRS2` reader - SRAM Bank0 Section 2 BIST Select (Write Protect)\nThis bit define if the bank0 section2 (0x2001_0000~0x2001_7FFF) of SRAM is selected or not when doing bist test.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: At least one section of SRAM should be selected when doing SRAM bist test."]
pub struct SRS2_R(crate::FieldReader<bool, SRS2_A>);
impl SRS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRS2_A {
        match self.bits {
            false => SRS2_A::_0,
            true => SRS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRS2_A::_1
    }
}
impl core::ops::Deref for SRS2_R {
    type Target = crate::FieldReader<bool, SRS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRS2` writer - SRAM Bank0 Section 2 BIST Select (Write Protect)\nThis bit define if the bank0 section2 (0x2001_0000~0x2001_7FFF) of SRAM is selected or not when doing bist test.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote: At least one section of SRAM should be selected when doing SRAM bist test."]
pub struct SRS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAM back0 section2 is deselected when doing bist test"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRS2_A::_0)
    }
    #[doc = "SRAM back0 section2 is selected when doing bist test"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRS2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SRAM BIST Enable Bit (Write Protect) This bit enables BIST test for SRAM. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn srbist(&self) -> SRBIST_R {
        SRBIST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - FMC CACHE BIST Enable Bit (Write Protect) This bit enables BIST test for CACHE RAM Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn fmcbist(&self) -> FMCBIST_R {
        FMCBIST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB BIST Enable Bit (Write Protect) This bit enables BIST test for USB RAM Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn usbbist(&self) -> USBBIST_R {
        USBBIST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDMA BIST Enable Bit (Write Protect) This bit enables BIST test for PDMA RAM Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdmabist(&self) -> PDMABIST_R {
        PDMABIST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SRAM Bank0 Section 0 BIST Select (Write Protect) This bit define if the bank0 section0 (0x2000_0000~0x2000_7FFF) of SRAM is selected or not when doing bist test. Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: At least one section of SRAM should be selected when doing SRAM bist test."]
    #[inline(always)]
    pub fn srs0(&self) -> SRS0_R {
        SRS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SRAM Bank0 Section 1 BIST Select (Write Protect) This bit define if the bank0 section1 (0x2000_8000~0x2000_FFFF) of SRAM is selected or not when doing bist test. Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: At least one section of SRAM should be selected when doing SRAM bist test."]
    #[inline(always)]
    pub fn srs1(&self) -> SRS1_R {
        SRS1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SRAM Bank0 Section 2 BIST Select (Write Protect) This bit define if the bank0 section2 (0x2001_0000~0x2001_7FFF) of SRAM is selected or not when doing bist test. Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: At least one section of SRAM should be selected when doing SRAM bist test."]
    #[inline(always)]
    pub fn srs2(&self) -> SRS2_R {
        SRS2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM BIST Enable Bit (Write Protect) This bit enables BIST test for SRAM. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn srbist(&mut self) -> SRBIST_W {
        SRBIST_W { w: self }
    }
    #[doc = "Bit 2 - FMC CACHE BIST Enable Bit (Write Protect) This bit enables BIST test for CACHE RAM Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn fmcbist(&mut self) -> FMCBIST_W {
        FMCBIST_W { w: self }
    }
    #[doc = "Bit 4 - USB BIST Enable Bit (Write Protect) This bit enables BIST test for USB RAM Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn usbbist(&mut self) -> USBBIST_W {
        USBBIST_W { w: self }
    }
    #[doc = "Bit 7 - PDMA BIST Enable Bit (Write Protect) This bit enables BIST test for PDMA RAM Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdmabist(&mut self) -> PDMABIST_W {
        PDMABIST_W { w: self }
    }
    #[doc = "Bit 16 - SRAM Bank0 Section 0 BIST Select (Write Protect) This bit define if the bank0 section0 (0x2000_0000~0x2000_7FFF) of SRAM is selected or not when doing bist test. Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: At least one section of SRAM should be selected when doing SRAM bist test."]
    #[inline(always)]
    pub fn srs0(&mut self) -> SRS0_W {
        SRS0_W { w: self }
    }
    #[doc = "Bit 17 - SRAM Bank0 Section 1 BIST Select (Write Protect) This bit define if the bank0 section1 (0x2000_8000~0x2000_FFFF) of SRAM is selected or not when doing bist test. Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: At least one section of SRAM should be selected when doing SRAM bist test."]
    #[inline(always)]
    pub fn srs1(&mut self) -> SRS1_W {
        SRS1_W { w: self }
    }
    #[doc = "Bit 18 - SRAM Bank0 Section 2 BIST Select (Write Protect) This bit define if the bank0 section2 (0x2001_0000~0x2001_7FFF) of SRAM is selected or not when doing bist test. Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note: At least one section of SRAM should be selected when doing SRAM bist test."]
    #[inline(always)]
    pub fn srs2(&mut self) -> SRS2_W {
        SRS2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System SRAM BIST Test Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_bistctl](index.html) module"]
pub struct SYS_SRAM_BISTCTL_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BISTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_bistctl::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BISTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_bistctl::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BISTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BISTCTL to value 0"]
impl crate::Resettable for SYS_SRAM_BISTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
