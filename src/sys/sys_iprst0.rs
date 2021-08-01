#[doc = "Register `SYS_IPRST0` reader"]
pub struct R(crate::R<SYS_IPRST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_IPRST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_IPRST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_IPRST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_IPRST0` writer"]
pub struct W(crate::W<SYS_IPRST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_IPRST0_SPEC>;
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
impl From<crate::W<SYS_IPRST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_IPRST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Chip One-shot Reset (Write Protect)\nSetting this bit will reset the whole chip, including Processor core and all peripherals, and this bit will automatically return to 0 after the 2 clock cycles.\nThe CHIPRST is same as the POR reset, all the chip controllers is reset and the chip setting from Flash are also reload.\nAbout the difference between CHIPRST and SYSRESETREQ(AIRCR\\[2\\]), please refer to section 6.2.2\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote : reset by powr on reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHIPRST_A {
    #[doc = "0: Chip normal operation"]
    _0 = 0,
    #[doc = "1: Chip one-shot reset"]
    _1 = 1,
}
impl From<CHIPRST_A> for bool {
    #[inline(always)]
    fn from(variant: CHIPRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHIPRST` reader - Chip One-shot Reset (Write Protect)\nSetting this bit will reset the whole chip, including Processor core and all peripherals, and this bit will automatically return to 0 after the 2 clock cycles.\nThe CHIPRST is same as the POR reset, all the chip controllers is reset and the chip setting from Flash are also reload.\nAbout the difference between CHIPRST and SYSRESETREQ(AIRCR\\[2\\]), please refer to section 6.2.2\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote : reset by powr on reset"]
pub struct CHIPRST_R(crate::FieldReader<bool, CHIPRST_A>);
impl CHIPRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHIPRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIPRST_A {
        match self.bits {
            false => CHIPRST_A::_0,
            true => CHIPRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHIPRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHIPRST_A::_1
    }
}
impl core::ops::Deref for CHIPRST_R {
    type Target = crate::FieldReader<bool, CHIPRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIPRST` writer - Chip One-shot Reset (Write Protect)\nSetting this bit will reset the whole chip, including Processor core and all peripherals, and this bit will automatically return to 0 after the 2 clock cycles.\nThe CHIPRST is same as the POR reset, all the chip controllers is reset and the chip setting from Flash are also reload.\nAbout the difference between CHIPRST and SYSRESETREQ(AIRCR\\[2\\]), please refer to section 6.2.2\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\nNote : reset by powr on reset"]
pub struct CHIPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIPRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHIPRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHIPRST_A::_0)
    }
    #[doc = "Chip one-shot reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHIPRST_A::_1)
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
#[doc = "Processor Core One-shot Reset (Write Protect)\nSetting this bit will only reset the processor core and Flash Memory Controller(FMC), and this bit will automatically return to 0 after the 2 clock cycles.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPURST_A {
    #[doc = "0: Processor core normal operation"]
    _0 = 0,
    #[doc = "1: Processor core one-shot reset"]
    _1 = 1,
}
impl From<CPURST_A> for bool {
    #[inline(always)]
    fn from(variant: CPURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPURST` reader - Processor Core One-shot Reset (Write Protect)\nSetting this bit will only reset the processor core and Flash Memory Controller(FMC), and this bit will automatically return to 0 after the 2 clock cycles.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct CPURST_R(crate::FieldReader<bool, CPURST_A>);
impl CPURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPURST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPURST_A {
        match self.bits {
            false => CPURST_A::_0,
            true => CPURST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPURST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPURST_A::_1
    }
}
impl core::ops::Deref for CPURST_R {
    type Target = crate::FieldReader<bool, CPURST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPURST` writer - Processor Core One-shot Reset (Write Protect)\nSetting this bit will only reset the processor core and Flash Memory Controller(FMC), and this bit will automatically return to 0 after the 2 clock cycles.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct CPURST_W<'a> {
    w: &'a mut W,
}
impl<'a> CPURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPURST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor core normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPURST_A::_0)
    }
    #[doc = "Processor core one-shot reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPURST_A::_1)
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
#[doc = "PDMA Controller Reset (Write Protect)\nSetting this bit to 1 will generate a reset signal to the PDMA. User needs to set this bit to 0 to release from reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMARST_A {
    #[doc = "0: PDMA controller normal operation"]
    _0 = 0,
    #[doc = "1: PDMA controller reset"]
    _1 = 1,
}
impl From<PDMARST_A> for bool {
    #[inline(always)]
    fn from(variant: PDMARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMARST` reader - PDMA Controller Reset (Write Protect)\nSetting this bit to 1 will generate a reset signal to the PDMA. User needs to set this bit to 0 to release from reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDMARST_R(crate::FieldReader<bool, PDMARST_A>);
impl PDMARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMARST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMARST_A {
        match self.bits {
            false => PDMARST_A::_0,
            true => PDMARST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDMARST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDMARST_A::_1
    }
}
impl core::ops::Deref for PDMARST_R {
    type Target = crate::FieldReader<bool, PDMARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMARST` writer - PDMA Controller Reset (Write Protect)\nSetting this bit to 1 will generate a reset signal to the PDMA. User needs to set this bit to 0 to release from reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDMARST_A::_0)
    }
    #[doc = "PDMA controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDMARST_A::_1)
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
#[doc = "EBI Controller Reset (Write Protect)\nSet this bit to 1 will generate a reset signal to the EBI. User needs to set this bit to 0 to release from the reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBIRST_A {
    #[doc = "0: EBI controller normal operation"]
    _0 = 0,
    #[doc = "1: EBI controller reset"]
    _1 = 1,
}
impl From<EBIRST_A> for bool {
    #[inline(always)]
    fn from(variant: EBIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBIRST` reader - EBI Controller Reset (Write Protect)\nSet this bit to 1 will generate a reset signal to the EBI. User needs to set this bit to 0 to release from the reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct EBIRST_R(crate::FieldReader<bool, EBIRST_A>);
impl EBIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EBIRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBIRST_A {
        match self.bits {
            false => EBIRST_A::_0,
            true => EBIRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EBIRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EBIRST_A::_1
    }
}
impl core::ops::Deref for EBIRST_R {
    type Target = crate::FieldReader<bool, EBIRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBIRST` writer - EBI Controller Reset (Write Protect)\nSet this bit to 1 will generate a reset signal to the EBI. User needs to set this bit to 0 to release from the reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct EBIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EBIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBIRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EBI controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EBIRST_A::_0)
    }
    #[doc = "EBI controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EBIRST_A::_1)
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
#[doc = "HDIV Controller Reset (Write Protect)\nSet this bit to 1 will generate a reset signal to the hardware divider. User need to set this bit to 0 to release from the reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDIV_RST_A {
    #[doc = "0: Hardware divider controller normal operation"]
    _0 = 0,
    #[doc = "1: Hardware divider controller reset"]
    _1 = 1,
}
impl From<HDIV_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HDIV_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDIV_RST` reader - HDIV Controller Reset (Write Protect)\nSet this bit to 1 will generate a reset signal to the hardware divider. User need to set this bit to 0 to release from the reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HDIV_RST_R(crate::FieldReader<bool, HDIV_RST_A>);
impl HDIV_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDIV_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDIV_RST_A {
        match self.bits {
            false => HDIV_RST_A::_0,
            true => HDIV_RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HDIV_RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HDIV_RST_A::_1
    }
}
impl core::ops::Deref for HDIV_RST_R {
    type Target = crate::FieldReader<bool, HDIV_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDIV_RST` writer - HDIV Controller Reset (Write Protect)\nSet this bit to 1 will generate a reset signal to the hardware divider. User need to set this bit to 0 to release from the reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HDIV_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HDIV_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDIV_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware divider controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDIV_RST_A::_0)
    }
    #[doc = "Hardware divider controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDIV_RST_A::_1)
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
#[doc = "CRC Calculation Controller Reset (Write Protect)\nSet this bit to 1 will generate a reset signal to the CRC calculation controller. User needs to set this bit to 0 to release from the reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCRST_A {
    #[doc = "0: CRC calculation controller normal operation"]
    _0 = 0,
    #[doc = "1: CRC calculation controller reset"]
    _1 = 1,
}
impl From<CRCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCRST` reader - CRC Calculation Controller Reset (Write Protect)\nSet this bit to 1 will generate a reset signal to the CRC calculation controller. User needs to set this bit to 0 to release from the reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct CRCRST_R(crate::FieldReader<bool, CRCRST_A>);
impl CRCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCRST_A {
        match self.bits {
            false => CRCRST_A::_0,
            true => CRCRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRCRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRCRST_A::_1
    }
}
impl core::ops::Deref for CRCRST_R {
    type Target = crate::FieldReader<bool, CRCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCRST` writer - CRC Calculation Controller Reset (Write Protect)\nSet this bit to 1 will generate a reset signal to the CRC calculation controller. User needs to set this bit to 0 to release from the reset state.\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct CRCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CRC calculation controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCRST_A::_0)
    }
    #[doc = "CRC calculation controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCRST_A::_1)
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
impl R {
    #[doc = "Bit 0 - Chip One-shot Reset (Write Protect) Setting this bit will reset the whole chip, including Processor core and all peripherals, and this bit will automatically return to 0 after the 2 clock cycles. The CHIPRST is same as the POR reset, all the chip controllers is reset and the chip setting from Flash are also reload. About the difference between CHIPRST and SYSRESETREQ(AIRCR\\[2\\]), please refer to section 6.2.2 Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note : reset by powr on reset"]
    #[inline(always)]
    pub fn chiprst(&self) -> CHIPRST_R {
        CHIPRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Processor Core One-shot Reset (Write Protect) Setting this bit will only reset the processor core and Flash Memory Controller(FMC), and this bit will automatically return to 0 after the 2 clock cycles. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpurst(&self) -> CPURST_R {
        CPURST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDMA Controller Reset (Write Protect) Setting this bit to 1 will generate a reset signal to the PDMA. User needs to set this bit to 0 to release from reset state. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdmarst(&self) -> PDMARST_R {
        PDMARST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EBI Controller Reset (Write Protect) Set this bit to 1 will generate a reset signal to the EBI. User needs to set this bit to 0 to release from the reset state. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ebirst(&self) -> EBIRST_R {
        EBIRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HDIV Controller Reset (Write Protect) Set this bit to 1 will generate a reset signal to the hardware divider. User need to set this bit to 0 to release from the reset state. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hdiv_rst(&self) -> HDIV_RST_R {
        HDIV_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC Calculation Controller Reset (Write Protect) Set this bit to 1 will generate a reset signal to the CRC calculation controller. User needs to set this bit to 0 to release from the reset state. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Chip One-shot Reset (Write Protect) Setting this bit will reset the whole chip, including Processor core and all peripherals, and this bit will automatically return to 0 after the 2 clock cycles. The CHIPRST is same as the POR reset, all the chip controllers is reset and the chip setting from Flash are also reload. About the difference between CHIPRST and SYSRESETREQ(AIRCR\\[2\\]), please refer to section 6.2.2 Note: This bit is write protected. Refer to the SYS_REGLCTL register. Note : reset by powr on reset"]
    #[inline(always)]
    pub fn chiprst(&mut self) -> CHIPRST_W {
        CHIPRST_W { w: self }
    }
    #[doc = "Bit 1 - Processor Core One-shot Reset (Write Protect) Setting this bit will only reset the processor core and Flash Memory Controller(FMC), and this bit will automatically return to 0 after the 2 clock cycles. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpurst(&mut self) -> CPURST_W {
        CPURST_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Controller Reset (Write Protect) Setting this bit to 1 will generate a reset signal to the PDMA. User needs to set this bit to 0 to release from reset state. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdmarst(&mut self) -> PDMARST_W {
        PDMARST_W { w: self }
    }
    #[doc = "Bit 3 - EBI Controller Reset (Write Protect) Set this bit to 1 will generate a reset signal to the EBI. User needs to set this bit to 0 to release from the reset state. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ebirst(&mut self) -> EBIRST_W {
        EBIRST_W { w: self }
    }
    #[doc = "Bit 4 - HDIV Controller Reset (Write Protect) Set this bit to 1 will generate a reset signal to the hardware divider. User need to set this bit to 0 to release from the reset state. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hdiv_rst(&mut self) -> HDIV_RST_W {
        HDIV_RST_W { w: self }
    }
    #[doc = "Bit 7 - CRC Calculation Controller Reset (Write Protect) Set this bit to 1 will generate a reset signal to the CRC calculation controller. User needs to set this bit to 0 to release from the reset state. Note: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W {
        CRCRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_iprst0](index.html) module"]
pub struct SYS_IPRST0_SPEC;
impl crate::RegisterSpec for SYS_IPRST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_iprst0::R](R) reader structure"]
impl crate::Readable for SYS_IPRST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_iprst0::W](W) writer structure"]
impl crate::Writable for SYS_IPRST0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_IPRST0 to value 0"]
impl crate::Resettable for SYS_IPRST0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
