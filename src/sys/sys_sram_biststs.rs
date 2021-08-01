#[doc = "Register `SYS_SRAM_BISTSTS` reader"]
pub struct R(crate::R<SYS_SRAM_BISTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BISTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BISTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BISTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "System SRAM BIST Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBISTEF_A {
    #[doc = "0: System SRAM BIST test pass"]
    _0 = 0,
    #[doc = "1: System SRAM BIST test fail"]
    _1 = 1,
}
impl From<SRBISTEF_A> for bool {
    #[inline(always)]
    fn from(variant: SRBISTEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBISTEF` reader - System SRAM BIST Fail Flag"]
pub struct SRBISTEF_R(crate::FieldReader<bool, SRBISTEF_A>);
impl SRBISTEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBISTEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBISTEF_A {
        match self.bits {
            false => SRBISTEF_A::_0,
            true => SRBISTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRBISTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRBISTEF_A::_1
    }
}
impl core::ops::Deref for SRBISTEF_R {
    type Target = crate::FieldReader<bool, SRBISTEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CACHE SRAM BIST Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CR0BISTEF_A {
    #[doc = "0: System CACHE RAM BIST test pass"]
    _0 = 0,
    #[doc = "1: System CACHE RAM BIST test failed"]
    _1 = 1,
}
impl From<CR0BISTEF_A> for bool {
    #[inline(always)]
    fn from(variant: CR0BISTEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CR0BISTEF` reader - CACHE SRAM BIST Fail Flag"]
pub struct CR0BISTEF_R(crate::FieldReader<bool, CR0BISTEF_A>);
impl CR0BISTEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR0BISTEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CR0BISTEF_A {
        match self.bits {
            false => CR0BISTEF_A::_0,
            true => CR0BISTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CR0BISTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CR0BISTEF_A::_1
    }
}
impl core::ops::Deref for CR0BISTEF_R {
    type Target = crate::FieldReader<bool, CR0BISTEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB SRAM BIST Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBBEF_A {
    #[doc = "0: USB SRAM BIST test pass"]
    _0 = 0,
    #[doc = "1: USB SRAM BIST test fail"]
    _1 = 1,
}
impl From<USBBEF_A> for bool {
    #[inline(always)]
    fn from(variant: USBBEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBBEF` reader - USB SRAM BIST Fail Flag"]
pub struct USBBEF_R(crate::FieldReader<bool, USBBEF_A>);
impl USBBEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBBEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBBEF_A {
        match self.bits {
            false => USBBEF_A::_0,
            true => USBBEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBBEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBBEF_A::_1
    }
}
impl core::ops::Deref for USBBEF_R {
    type Target = crate::FieldReader<bool, USBBEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA SRAM BIST Failed Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMABISTF_A {
    #[doc = "0: PDMA SRAM BIST pass"]
    _0 = 0,
    #[doc = "1: PDMA SRAM BIST failed"]
    _1 = 1,
}
impl From<PDMABISTF_A> for bool {
    #[inline(always)]
    fn from(variant: PDMABISTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMABISTF` reader - PDMA SRAM BIST Failed Flag"]
pub struct PDMABISTF_R(crate::FieldReader<bool, PDMABISTF_A>);
impl PDMABISTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMABISTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMABISTF_A {
        match self.bits {
            false => PDMABISTF_A::_0,
            true => PDMABISTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDMABISTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDMABISTF_A::_1
    }
}
impl core::ops::Deref for PDMABISTF_R {
    type Target = crate::FieldReader<bool, PDMABISTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System SRAM BIST Test Finish\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBEND_A {
    #[doc = "0: System SRAM BIST active"]
    _0 = 0,
    #[doc = "1: System SRAM BIST finish"]
    _1 = 1,
}
impl From<SRBEND_A> for bool {
    #[inline(always)]
    fn from(variant: SRBEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBEND` reader - System SRAM BIST Test Finish"]
pub struct SRBEND_R(crate::FieldReader<bool, SRBEND_A>);
impl SRBEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBEND_A {
        match self.bits {
            false => SRBEND_A::_0,
            true => SRBEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRBEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRBEND_A::_1
    }
}
impl core::ops::Deref for SRBEND_R {
    type Target = crate::FieldReader<bool, SRBEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CACHE SRAM BIST Test Finish\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRBEND_A {
    #[doc = "0: System CACHE RAM BIST is active"]
    _0 = 0,
    #[doc = "1: System CACHE RAM BIST test finished"]
    _1 = 1,
}
impl From<CRBEND_A> for bool {
    #[inline(always)]
    fn from(variant: CRBEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRBEND` reader - CACHE SRAM BIST Test Finish"]
pub struct CRBEND_R(crate::FieldReader<bool, CRBEND_A>);
impl CRBEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRBEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRBEND_A {
        match self.bits {
            false => CRBEND_A::_0,
            true => CRBEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRBEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRBEND_A::_1
    }
}
impl core::ops::Deref for CRBEND_R {
    type Target = crate::FieldReader<bool, CRBEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB SRAM BIST Test Finish\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBBEND_A {
    #[doc = "0: USB SRAM BIST is active"]
    _0 = 0,
    #[doc = "1: USB SRAM BIST test finish"]
    _1 = 1,
}
impl From<USBBEND_A> for bool {
    #[inline(always)]
    fn from(variant: USBBEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBBEND` reader - USB SRAM BIST Test Finish"]
pub struct USBBEND_R(crate::FieldReader<bool, USBBEND_A>);
impl USBBEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBBEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBBEND_A {
        match self.bits {
            false => USBBEND_A::_0,
            true => USBBEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBBEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBBEND_A::_1
    }
}
impl core::ops::Deref for USBBEND_R {
    type Target = crate::FieldReader<bool, USBBEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PDMA SRAM BIST Test Finish\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMAEND_A {
    #[doc = "0: PDMA SRAM BIST is active"]
    _0 = 0,
    #[doc = "1: PDMA SRAM BIST test finish"]
    _1 = 1,
}
impl From<PDMAEND_A> for bool {
    #[inline(always)]
    fn from(variant: PDMAEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMAEND` reader - PDMA SRAM BIST Test Finish"]
pub struct PDMAEND_R(crate::FieldReader<bool, PDMAEND_A>);
impl PDMAEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMAEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMAEND_A {
        match self.bits {
            false => PDMAEND_A::_0,
            true => PDMAEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDMAEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDMAEND_A::_1
    }
}
impl core::ops::Deref for PDMAEND_R {
    type Target = crate::FieldReader<bool, PDMAEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - System SRAM BIST Fail Flag"]
    #[inline(always)]
    pub fn srbistef(&self) -> SRBISTEF_R {
        SRBISTEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CACHE SRAM BIST Fail Flag"]
    #[inline(always)]
    pub fn cr0bistef(&self) -> CR0BISTEF_R {
        CR0BISTEF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB SRAM BIST Fail Flag"]
    #[inline(always)]
    pub fn usbbef(&self) -> USBBEF_R {
        USBBEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDMA SRAM BIST Failed Flag"]
    #[inline(always)]
    pub fn pdmabistf(&self) -> PDMABISTF_R {
        PDMABISTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - System SRAM BIST Test Finish"]
    #[inline(always)]
    pub fn srbend(&self) -> SRBEND_R {
        SRBEND_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CACHE SRAM BIST Test Finish"]
    #[inline(always)]
    pub fn crbend(&self) -> CRBEND_R {
        CRBEND_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - USB SRAM BIST Test Finish"]
    #[inline(always)]
    pub fn usbbend(&self) -> USBBEND_R {
        USBBEND_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PDMA SRAM BIST Test Finish"]
    #[inline(always)]
    pub fn pdmaend(&self) -> PDMAEND_R {
        PDMAEND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "System SRAM BIST Test Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_biststs](index.html) module"]
pub struct SYS_SRAM_BISTSTS_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BISTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_biststs::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BISTSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYS_SRAM_BISTSTS to value 0"]
impl crate::Resettable for SYS_SRAM_BISTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
