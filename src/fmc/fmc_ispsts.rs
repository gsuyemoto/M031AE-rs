#[doc = "Register `FMC_ISPSTS` reader"]
pub struct R(crate::R<FMC_ISPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_ISPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_ISPSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_ISPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_ISPSTS` writer"]
pub struct W(crate::W<FMC_ISPSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_ISPSTS_SPEC>;
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
impl From<crate::W<FMC_ISPSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_ISPSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ISP BUSY (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISPBUSY_A {
    #[doc = "0: ISP operation is finished"]
    _0 = 0,
    #[doc = "1: ISP operation is busy"]
    _1 = 1,
}
impl From<ISPBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: ISPBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISPBUSY` reader - ISP BUSY (Read Only)"]
pub struct ISPBUSY_R(crate::FieldReader<bool, ISPBUSY_A>);
impl ISPBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISPBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISPBUSY_A {
        match self.bits {
            false => ISPBUSY_A::_0,
            true => ISPBUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISPBUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISPBUSY_A::_1
    }
}
impl core::ops::Deref for ISPBUSY_R {
    type Target = crate::FieldReader<bool, ISPBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boot Selection of CONFIG (Read Only)\nThis bit is initiated with the CBS (CONFIG0\\[7:6\\]) after any reset is happened except CPU reset (RSTS_CPU is 1) or system reset (RSTS_SYS) is happened.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBS_A {
    #[doc = "0: LDROM with IAP mode"]
    _0 = 0,
    #[doc = "1: LDROM without IAP mode"]
    _1 = 1,
    #[doc = "2: APROM with IAP mode"]
    _2 = 2,
    #[doc = "3: APROM without IAP mode"]
    _3 = 3,
}
impl From<CBS_A> for u8 {
    #[inline(always)]
    fn from(variant: CBS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CBS` reader - Boot Selection of CONFIG (Read Only)\nThis bit is initiated with the CBS (CONFIG0\\[7:6\\]) after any reset is happened except CPU reset (RSTS_CPU is 1) or system reset (RSTS_SYS) is happened."]
pub struct CBS_R(crate::FieldReader<u8, CBS_A>);
impl CBS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CBS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBS_A {
        match self.bits {
            0 => CBS_A::_0,
            1 => CBS_A::_1,
            2 => CBS_A::_2,
            3 => CBS_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CBS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CBS_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CBS_A::_3
    }
}
impl core::ops::Deref for CBS_R {
    type Target = crate::FieldReader<u8, CBS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Flash Program with Fast Verification Flag (Read Only)\nThis bit is set if data is mismatched at ISP programming verification. This bit is clear by performing ISP Flash erase or ISP read CID operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGFF_A {
    #[doc = "0: Flash Program is successful"]
    _0 = 0,
    #[doc = "1: Flash Program is failed. Program data is different with data in the Flash memory"]
    _1 = 1,
}
impl From<PGFF_A> for bool {
    #[inline(always)]
    fn from(variant: PGFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGFF` reader - Flash Program with Fast Verification Flag (Read Only)\nThis bit is set if data is mismatched at ISP programming verification. This bit is clear by performing ISP Flash erase or ISP read CID operation"]
pub struct PGFF_R(crate::FieldReader<bool, PGFF_A>);
impl PGFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGFF_A {
        match self.bits {
            false => PGFF_A::_0,
            true => PGFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PGFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PGFF_A::_1
    }
}
impl core::ops::Deref for PGFF_R {
    type Target = crate::FieldReader<bool, PGFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPFF` reader - ISP Fail Flag (Write Protect)\nThis bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions:\nAPROM writes to itself if APUEN is set to 0.\nLDROM writes to itself if LDUEN is set to 0.\nCONFIG is erased/programmed if CFGUEN is set to 0.\nSPROM is erased/programmed if SPUEN is set to 0\nSPROM is programmed at SPROM secured mode.\nPage Erase command at LOCK mode with ICE connection\nErase or Program command at brown-out detected\nDestination address is illegal, such as over an available range.\nInvalid ISP commands"]
pub struct ISPFF_R(crate::FieldReader<bool, bool>);
impl ISPFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISPFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISPFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPFF` writer - ISP Fail Flag (Write Protect)\nThis bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions:\nAPROM writes to itself if APUEN is set to 0.\nLDROM writes to itself if LDUEN is set to 0.\nCONFIG is erased/programmed if CFGUEN is set to 0.\nSPROM is erased/programmed if SPUEN is set to 0\nSPROM is programmed at SPROM secured mode.\nPage Erase command at LOCK mode with ICE connection\nErase or Program command at brown-out detected\nDestination address is illegal, such as over an available range.\nInvalid ISP commands"]
pub struct ISPFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPFF_W<'a> {
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
#[doc = "Flash All-one Verification Flag \nThis bit is set by hardware if all of Flash bits are 1, and cleared if Flash bits are not all 1 after 'Run Flash All-One Verification' complete; this bit can also be cleared by writing 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALLONE_A {
    #[doc = "0: Flash bits are not all 1 after'Run Flash All-One Verification' complete"]
    _0 = 0,
    #[doc = "1: All of Flash bits are 1 after'Run Flash All-One Verification' complete"]
    _1 = 1,
}
impl From<ALLONE_A> for bool {
    #[inline(always)]
    fn from(variant: ALLONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLONE` reader - Flash All-one Verification Flag \nThis bit is set by hardware if all of Flash bits are 1, and cleared if Flash bits are not all 1 after 'Run Flash All-One Verification' complete; this bit can also be cleared by writing 1."]
pub struct ALLONE_R(crate::FieldReader<bool, ALLONE_A>);
impl ALLONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALLONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALLONE_A {
        match self.bits {
            false => ALLONE_A::_0,
            true => ALLONE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALLONE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALLONE_A::_1
    }
}
impl core::ops::Deref for ALLONE_R {
    type Target = crate::FieldReader<bool, ALLONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLONE` writer - Flash All-one Verification Flag \nThis bit is set by hardware if all of Flash bits are 1, and cleared if Flash bits are not all 1 after 'Run Flash All-One Verification' complete; this bit can also be cleared by writing 1."]
pub struct ALLONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALLONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash bits are not all 1 after'Run Flash All-One Verification' complete"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALLONE_A::_0)
    }
    #[doc = "All of Flash bits are 1 after'Run Flash All-One Verification' complete"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALLONE_A::_1)
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
#[doc = "ISP Command Finish Interrupt Flag\nNote: Only supported in 256/512 Kbytes Flash.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTFLAG_A {
    #[doc = "0: ISP Not Finished"]
    _0 = 0,
    #[doc = "1: ISP done or ISPFF set"]
    _1 = 1,
}
impl From<INTFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: INTFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTFLAG` reader - ISP Command Finish Interrupt Flag\nNote: Only supported in 256/512 Kbytes Flash."]
pub struct INTFLAG_R(crate::FieldReader<bool, INTFLAG_A>);
impl INTFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTFLAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTFLAG_A {
        match self.bits {
            false => INTFLAG_A::_0,
            true => INTFLAG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTFLAG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTFLAG_A::_1
    }
}
impl core::ops::Deref for INTFLAG_R {
    type Target = crate::FieldReader<bool, INTFLAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTFLAG` writer - ISP Command Finish Interrupt Flag\nNote: Only supported in 256/512 Kbytes Flash."]
pub struct INTFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTFLAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ISP Not Finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTFLAG_A::_0)
    }
    #[doc = "ISP done or ISPFF set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTFLAG_A::_1)
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
#[doc = "Field `VECMAP` reader - Vector Page Mapping Address (Read Only)\nAll access to 0x0000_0000~0x0000_01FF is remapped to the Flash memory or SRAM address {VECMAP\\[20:0\\], 9'h000} ~ {VECMAP\\[20:0\\], 9'h1FF}, except SPROM.\nVECMAP \\[18:12\\]
should be 0."]
pub struct VECMAP_R(crate::FieldReader<u32, u32>);
impl VECMAP_R {
    pub(crate) fn new(bits: u32) -> Self {
        VECMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECMAP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Flash Bank Select Indicator\nThis bit indicates which APROM address model is selected to boot.\nNote: Only supported in 256/512 Kbytes Flash.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBS_A {
    #[doc = "0: Address model OP0 is selected to boot"]
    _0 = 0,
    #[doc = "1: Address model OP1 is selected to boot"]
    _1 = 1,
}
impl From<FBS_A> for bool {
    #[inline(always)]
    fn from(variant: FBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBS` reader - Flash Bank Select Indicator\nThis bit indicates which APROM address model is selected to boot.\nNote: Only supported in 256/512 Kbytes Flash."]
pub struct FBS_R(crate::FieldReader<bool, FBS_A>);
impl FBS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBS_A {
        match self.bits {
            false => FBS_A::_0,
            true => FBS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FBS_A::_1
    }
}
impl core::ops::Deref for FBS_R {
    type Target = crate::FieldReader<bool, FBS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBS` writer - Flash Bank Select Indicator\nThis bit indicates which APROM address model is selected to boot.\nNote: Only supported in 256/512 Kbytes Flash."]
pub struct FBS_W<'a> {
    w: &'a mut W,
}
impl<'a> FBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Address model OP0 is selected to boot"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FBS_A::_0)
    }
    #[doc = "Address model OP1 is selected to boot"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FBS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Security Code Active Flag\nThis bit is set to 1 by hardware when detecting SPROM secured code is active at Flash initialization, or software writes 1 to this bit to make secured code active; this bit is only cleared by SPROM page erase operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCODE_A {
    #[doc = "0: SPROM secured code is inactive"]
    _0 = 0,
    #[doc = "1: SPROM secured code is active"]
    _1 = 1,
}
impl From<SCODE_A> for bool {
    #[inline(always)]
    fn from(variant: SCODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCODE` reader - Security Code Active Flag\nThis bit is set to 1 by hardware when detecting SPROM secured code is active at Flash initialization, or software writes 1 to this bit to make secured code active; this bit is only cleared by SPROM page erase operation."]
pub struct SCODE_R(crate::FieldReader<bool, SCODE_A>);
impl SCODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCODE_A {
        match self.bits {
            false => SCODE_A::_0,
            true => SCODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCODE_A::_1
    }
}
impl core::ops::Deref for SCODE_R {
    type Target = crate::FieldReader<bool, SCODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCODE` writer - Security Code Active Flag\nThis bit is set to 1 by hardware when detecting SPROM secured code is active at Flash initialization, or software writes 1 to this bit to make secured code active; this bit is only cleared by SPROM page erase operation."]
pub struct SCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPROM secured code is inactive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCODE_A::_0)
    }
    #[doc = "SPROM secured code is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCODE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ISP BUSY (Read Only)"]
    #[inline(always)]
    pub fn ispbusy(&self) -> ISPBUSY_R {
        ISPBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Boot Selection of CONFIG (Read Only) This bit is initiated with the CBS (CONFIG0\\[7:6\\]) after any reset is happened except CPU reset (RSTS_CPU is 1) or system reset (RSTS_SYS) is happened."]
    #[inline(always)]
    pub fn cbs(&self) -> CBS_R {
        CBS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Flash Program with Fast Verification Flag (Read Only) This bit is set if data is mismatched at ISP programming verification. This bit is clear by performing ISP Flash erase or ISP read CID operation"]
    #[inline(always)]
    pub fn pgff(&self) -> PGFF_R {
        PGFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ISP Fail Flag (Write Protect) This bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions: APROM writes to itself if APUEN is set to 0. LDROM writes to itself if LDUEN is set to 0. CONFIG is erased/programmed if CFGUEN is set to 0. SPROM is erased/programmed if SPUEN is set to 0 SPROM is programmed at SPROM secured mode. Page Erase command at LOCK mode with ICE connection Erase or Program command at brown-out detected Destination address is illegal, such as over an available range. Invalid ISP commands"]
    #[inline(always)]
    pub fn ispff(&self) -> ISPFF_R {
        ISPFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flash All-one Verification Flag This bit is set by hardware if all of Flash bits are 1, and cleared if Flash bits are not all 1 after 'Run Flash All-One Verification' complete; this bit can also be cleared by writing 1."]
    #[inline(always)]
    pub fn allone(&self) -> ALLONE_R {
        ALLONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ISP Command Finish Interrupt Flag Note: Only supported in 256/512 Kbytes Flash."]
    #[inline(always)]
    pub fn intflag(&self) -> INTFLAG_R {
        INTFLAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:29 - Vector Page Mapping Address (Read Only) All access to 0x0000_0000~0x0000_01FF is remapped to the Flash memory or SRAM address {VECMAP\\[20:0\\], 9'h000} ~ {VECMAP\\[20:0\\], 9'h1FF}, except SPROM. VECMAP \\[18:12\\]
should be 0."]
    #[inline(always)]
    pub fn vecmap(&self) -> VECMAP_R {
        VECMAP_R::new(((self.bits >> 9) & 0x001f_ffff) as u32)
    }
    #[doc = "Bit 30 - Flash Bank Select Indicator This bit indicates which APROM address model is selected to boot. Note: Only supported in 256/512 Kbytes Flash."]
    #[inline(always)]
    pub fn fbs(&self) -> FBS_R {
        FBS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Security Code Active Flag This bit is set to 1 by hardware when detecting SPROM secured code is active at Flash initialization, or software writes 1 to this bit to make secured code active; this bit is only cleared by SPROM page erase operation."]
    #[inline(always)]
    pub fn scode(&self) -> SCODE_R {
        SCODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - ISP Fail Flag (Write Protect) This bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions: APROM writes to itself if APUEN is set to 0. LDROM writes to itself if LDUEN is set to 0. CONFIG is erased/programmed if CFGUEN is set to 0. SPROM is erased/programmed if SPUEN is set to 0 SPROM is programmed at SPROM secured mode. Page Erase command at LOCK mode with ICE connection Erase or Program command at brown-out detected Destination address is illegal, such as over an available range. Invalid ISP commands"]
    #[inline(always)]
    pub fn ispff(&mut self) -> ISPFF_W {
        ISPFF_W { w: self }
    }
    #[doc = "Bit 7 - Flash All-one Verification Flag This bit is set by hardware if all of Flash bits are 1, and cleared if Flash bits are not all 1 after 'Run Flash All-One Verification' complete; this bit can also be cleared by writing 1."]
    #[inline(always)]
    pub fn allone(&mut self) -> ALLONE_W {
        ALLONE_W { w: self }
    }
    #[doc = "Bit 8 - ISP Command Finish Interrupt Flag Note: Only supported in 256/512 Kbytes Flash."]
    #[inline(always)]
    pub fn intflag(&mut self) -> INTFLAG_W {
        INTFLAG_W { w: self }
    }
    #[doc = "Bit 30 - Flash Bank Select Indicator This bit indicates which APROM address model is selected to boot. Note: Only supported in 256/512 Kbytes Flash."]
    #[inline(always)]
    pub fn fbs(&mut self) -> FBS_W {
        FBS_W { w: self }
    }
    #[doc = "Bit 31 - Security Code Active Flag This bit is set to 1 by hardware when detecting SPROM secured code is active at Flash initialization, or software writes 1 to this bit to make secured code active; this bit is only cleared by SPROM page erase operation."]
    #[inline(always)]
    pub fn scode(&mut self) -> SCODE_W {
        SCODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ispsts](index.html) module"]
pub struct FMC_ISPSTS_SPEC;
impl crate::RegisterSpec for FMC_ISPSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_ispsts::R](R) reader structure"]
impl crate::Readable for FMC_ISPSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_ispsts::W](W) writer structure"]
impl crate::Writable for FMC_ISPSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_ISPSTS to value 0"]
impl crate::Resettable for FMC_ISPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
