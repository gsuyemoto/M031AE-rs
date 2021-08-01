#[doc = "Register `FMC_ISPCTL` reader"]
pub struct R(crate::R<FMC_ISPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_ISPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_ISPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_ISPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_ISPCTL` writer"]
pub struct W(crate::W<FMC_ISPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_ISPCTL_SPEC>;
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
impl From<crate::W<FMC_ISPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_ISPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ISP Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISPEN_A {
    #[doc = "1: ISP function Enabled"]
    _1 = 1,
}
impl From<ISPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ISPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISPEN` reader - ISP Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct ISPEN_R(crate::FieldReader<bool, ISPEN_A>);
impl ISPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ISPEN_A> {
        match self.bits {
            true => Some(ISPEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISPEN_A::_1
    }
}
impl core::ops::Deref for ISPEN_R {
    type Target = crate::FieldReader<bool, ISPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPEN` writer - ISP Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct ISPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ISP function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISPEN_A::_1)
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
#[doc = "Boot Selection (Write Protect)\nSet/clear this bit to select next booting from LDROM/APROM, respectively. This bit also functions as chip booting status flag, which can be used to check where chip booted from. This bit is initiated with the inversed value of CBS\\[1\\]
(CONFIG0\\[7\\]) after any reset is happened except CPU reset (RSTS_CPU is 1) or system reset (RSTS_SYS) is happened\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS_A {
    #[doc = "0: Booting from APROM"]
    _0 = 0,
    #[doc = "1: Booting from LDROM"]
    _1 = 1,
}
impl From<BS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS` reader - Boot Selection (Write Protect)\nSet/clear this bit to select next booting from LDROM/APROM, respectively. This bit also functions as chip booting status flag, which can be used to check where chip booted from. This bit is initiated with the inversed value of CBS\\[1\\]
(CONFIG0\\[7\\]) after any reset is happened except CPU reset (RSTS_CPU is 1) or system reset (RSTS_SYS) is happened\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct BS_R(crate::FieldReader<bool, BS_A>);
impl BS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_A {
        match self.bits {
            false => BS_A::_0,
            true => BS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BS_A::_1
    }
}
impl core::ops::Deref for BS_R {
    type Target = crate::FieldReader<bool, BS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BS` writer - Boot Selection (Write Protect)\nSet/clear this bit to select next booting from LDROM/APROM, respectively. This bit also functions as chip booting status flag, which can be used to check where chip booted from. This bit is initiated with the inversed value of CBS\\[1\\]
(CONFIG0\\[7\\]) after any reset is happened except CPU reset (RSTS_CPU is 1) or system reset (RSTS_SYS) is happened\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct BS_W<'a> {
    w: &'a mut W,
}
impl<'a> BS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Booting from APROM"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BS_A::_0)
    }
    #[doc = "Booting from LDROM"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BS_A::_1)
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
#[doc = "SPROM Update Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPUEN_A {
    #[doc = "0: SPROM cannot be updated"]
    _0 = 0,
    #[doc = "1: SPROM can be updated"]
    _1 = 1,
}
impl From<SPUEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPUEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPUEN` reader - SPROM Update Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct SPUEN_R(crate::FieldReader<bool, SPUEN_A>);
impl SPUEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPUEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPUEN_A {
        match self.bits {
            false => SPUEN_A::_0,
            true => SPUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPUEN_A::_1
    }
}
impl core::ops::Deref for SPUEN_R {
    type Target = crate::FieldReader<bool, SPUEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPUEN` writer - SPROM Update Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct SPUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPUEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPUEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPROM cannot be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPUEN_A::_0)
    }
    #[doc = "SPROM can be updated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPUEN_A::_1)
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
#[doc = "APROM Update Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APUEN_A {
    #[doc = "0: APROM cannot be updated when the chip runs in APROM"]
    _0 = 0,
    #[doc = "1: APROM can be updated when the chip runs in APROM"]
    _1 = 1,
}
impl From<APUEN_A> for bool {
    #[inline(always)]
    fn from(variant: APUEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APUEN` reader - APROM Update Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct APUEN_R(crate::FieldReader<bool, APUEN_A>);
impl APUEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        APUEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APUEN_A {
        match self.bits {
            false => APUEN_A::_0,
            true => APUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == APUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == APUEN_A::_1
    }
}
impl core::ops::Deref for APUEN_R {
    type Target = crate::FieldReader<bool, APUEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APUEN` writer - APROM Update Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct APUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APUEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APUEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "APROM cannot be updated when the chip runs in APROM"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(APUEN_A::_0)
    }
    #[doc = "APROM can be updated when the chip runs in APROM"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(APUEN_A::_1)
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
#[doc = "CONFIG Update Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGUEN_A {
    #[doc = "0: CONFIG cannot be updated"]
    _0 = 0,
    #[doc = "1: CONFIG can be updated"]
    _1 = 1,
}
impl From<CFGUEN_A> for bool {
    #[inline(always)]
    fn from(variant: CFGUEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGUEN` reader - CONFIG Update Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct CFGUEN_R(crate::FieldReader<bool, CFGUEN_A>);
impl CFGUEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGUEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGUEN_A {
        match self.bits {
            false => CFGUEN_A::_0,
            true => CFGUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CFGUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CFGUEN_A::_1
    }
}
impl core::ops::Deref for CFGUEN_R {
    type Target = crate::FieldReader<bool, CFGUEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGUEN` writer - CONFIG Update Enable Bit (Write Protect)\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct CFGUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGUEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGUEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CONFIG cannot be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFGUEN_A::_0)
    }
    #[doc = "CONFIG can be updated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFGUEN_A::_1)
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
#[doc = "LDROM Update Enable Bit (Write Protect)\nLDROM update enable bit.\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDUEN_A {
    #[doc = "0: LDROM cannot be updated"]
    _0 = 0,
    #[doc = "1: LDROM can be updated"]
    _1 = 1,
}
impl From<LDUEN_A> for bool {
    #[inline(always)]
    fn from(variant: LDUEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDUEN` reader - LDROM Update Enable Bit (Write Protect)\nLDROM update enable bit.\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct LDUEN_R(crate::FieldReader<bool, LDUEN_A>);
impl LDUEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDUEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDUEN_A {
        match self.bits {
            false => LDUEN_A::_0,
            true => LDUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LDUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LDUEN_A::_1
    }
}
impl core::ops::Deref for LDUEN_R {
    type Target = crate::FieldReader<bool, LDUEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDUEN` writer - LDROM Update Enable Bit (Write Protect)\nLDROM update enable bit.\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct LDUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDUEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDUEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LDROM cannot be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDUEN_A::_0)
    }
    #[doc = "LDROM can be updated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDUEN_A::_1)
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
#[doc = "Field `ISPFF` reader - ISP Fail Flag (Write Protect)\nThis bit is set by hardware when a triggered ISP meets any of the following conditions:\nThis bit needs to be cleared by writing 1 to it.\nAPROM writes to itself if APUEN is set to 0.\nLDROM writes to itself if LDUEN is set to 0.\nCONFIG is erased/programmed if CFGUEN is set to 0.\nSPROM is erased/programmed if SPUEN is set to 0\nSPROM is programmed at SPROM secured mode.\nPage Erase command at LOCK mode with ICE connection\nErase or Program command at brown-out detected\nDestination address is illegal, such as over an available range.\nInvalid ISP commands\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `ISPFF` writer - ISP Fail Flag (Write Protect)\nThis bit is set by hardware when a triggered ISP meets any of the following conditions:\nThis bit needs to be cleared by writing 1 to it.\nAPROM writes to itself if APUEN is set to 0.\nLDROM writes to itself if LDUEN is set to 0.\nCONFIG is erased/programmed if CFGUEN is set to 0.\nSPROM is erased/programmed if SPUEN is set to 0\nSPROM is programmed at SPROM secured mode.\nPage Erase command at LOCK mode with ICE connection\nErase or Program command at brown-out detected\nDestination address is illegal, such as over an available range.\nInvalid ISP commands\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "ISP Interrupt Enabled Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.Before use INT,user needs to clear the INTFLAG(FMC_ISPSTS\\[8\\]) make sure INT happen at correct time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: ISP INT Disabled"]
    _0 = 0,
    #[doc = "1: ISP INT Enabled"]
    _1 = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - ISP Interrupt Enabled Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.Before use INT,user needs to clear the INTFLAG(FMC_ISPSTS\\[8\\]) make sure INT happen at correct time."]
pub struct INTEN_R(crate::FieldReader<bool, INTEN_A>);
impl INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::_0,
            true => INTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN_A::_1
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - ISP Interrupt Enabled Bit (Write Protect)\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.Before use INT,user needs to clear the INTFLAG(FMC_ISPSTS\\[8\\]) make sure INT happen at correct time."]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ISP INT Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN_A::_0)
    }
    #[doc = "ISP INT Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ISP Enable Bit (Write Protect) Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ispen(&self) -> ISPEN_R {
        ISPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Boot Selection (Write Protect) Set/clear this bit to select next booting from LDROM/APROM, respectively. This bit also functions as chip booting status flag, which can be used to check where chip booted from. This bit is initiated with the inversed value of CBS\\[1\\]
(CONFIG0\\[7\\]) after any reset is happened except CPU reset (RSTS_CPU is 1) or system reset (RSTS_SYS) is happened Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SPROM Update Enable Bit (Write Protect) Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn spuen(&self) -> SPUEN_R {
        SPUEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - APROM Update Enable Bit (Write Protect) Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn apuen(&self) -> APUEN_R {
        APUEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CONFIG Update Enable Bit (Write Protect) Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cfguen(&self) -> CFGUEN_R {
        CFGUEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LDROM Update Enable Bit (Write Protect) LDROM update enable bit. Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lduen(&self) -> LDUEN_R {
        LDUEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ISP Fail Flag (Write Protect) This bit is set by hardware when a triggered ISP meets any of the following conditions: This bit needs to be cleared by writing 1 to it. APROM writes to itself if APUEN is set to 0. LDROM writes to itself if LDUEN is set to 0. CONFIG is erased/programmed if CFGUEN is set to 0. SPROM is erased/programmed if SPUEN is set to 0 SPROM is programmed at SPROM secured mode. Page Erase command at LOCK mode with ICE connection Erase or Program command at brown-out detected Destination address is illegal, such as over an available range. Invalid ISP commands Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ispff(&self) -> ISPFF_R {
        ISPFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ISP Interrupt Enabled Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register.Before use INT,user needs to clear the INTFLAG(FMC_ISPSTS\\[8\\]) make sure INT happen at correct time."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISP Enable Bit (Write Protect) Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ispen(&mut self) -> ISPEN_W {
        ISPEN_W { w: self }
    }
    #[doc = "Bit 1 - Boot Selection (Write Protect) Set/clear this bit to select next booting from LDROM/APROM, respectively. This bit also functions as chip booting status flag, which can be used to check where chip booted from. This bit is initiated with the inversed value of CBS\\[1\\]
(CONFIG0\\[7\\]) after any reset is happened except CPU reset (RSTS_CPU is 1) or system reset (RSTS_SYS) is happened Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bs(&mut self) -> BS_W {
        BS_W { w: self }
    }
    #[doc = "Bit 2 - SPROM Update Enable Bit (Write Protect) Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn spuen(&mut self) -> SPUEN_W {
        SPUEN_W { w: self }
    }
    #[doc = "Bit 3 - APROM Update Enable Bit (Write Protect) Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn apuen(&mut self) -> APUEN_W {
        APUEN_W { w: self }
    }
    #[doc = "Bit 4 - CONFIG Update Enable Bit (Write Protect) Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cfguen(&mut self) -> CFGUEN_W {
        CFGUEN_W { w: self }
    }
    #[doc = "Bit 5 - LDROM Update Enable Bit (Write Protect) LDROM update enable bit. Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lduen(&mut self) -> LDUEN_W {
        LDUEN_W { w: self }
    }
    #[doc = "Bit 6 - ISP Fail Flag (Write Protect) This bit is set by hardware when a triggered ISP meets any of the following conditions: This bit needs to be cleared by writing 1 to it. APROM writes to itself if APUEN is set to 0. LDROM writes to itself if LDUEN is set to 0. CONFIG is erased/programmed if CFGUEN is set to 0. SPROM is erased/programmed if SPUEN is set to 0 SPROM is programmed at SPROM secured mode. Page Erase command at LOCK mode with ICE connection Erase or Program command at brown-out detected Destination address is illegal, such as over an available range. Invalid ISP commands Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ispff(&mut self) -> ISPFF_W {
        ISPFF_W { w: self }
    }
    #[doc = "Bit 24 - ISP Interrupt Enabled Bit (Write Protect) Note: This bit is write protected. Refer to the SYS_REGLCTL register.Before use INT,user needs to clear the INTFLAG(FMC_ISPSTS\\[8\\]) make sure INT happen at correct time."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ispctl](index.html) module"]
pub struct FMC_ISPCTL_SPEC;
impl crate::RegisterSpec for FMC_ISPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_ispctl::R](R) reader structure"]
impl crate::Readable for FMC_ISPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_ispctl::W](W) writer structure"]
impl crate::Writable for FMC_ISPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_ISPCTL to value 0"]
impl crate::Resettable for FMC_ISPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
