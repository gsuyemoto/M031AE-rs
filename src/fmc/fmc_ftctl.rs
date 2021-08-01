#[doc = "Register `FMC_FTCTL` reader"]
pub struct R(crate::R<FMC_FTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_FTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_FTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_FTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_FTCTL` writer"]
pub struct W(crate::W<FMC_FTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_FTCTL_SPEC>;
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
impl From<crate::W<FMC_FTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_FTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frequency Optimization Mode (Write Protect)\nThe M031/M032 series support adjustable Flash access timing to optimize the Flash access cycles in different system working frequency.\nFor 16/32/64/128 Kbytes Flash:\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FOM_A {
    #[doc = "0: Frequency is less than or equal to 48 MHz"]
    _0 = 0,
    #[doc = "1: Frequency is less than or equal to 24 MHz.\\nFrequency is less than or equal to 12 MHz"]
    _1 = 1,
    #[doc = "2: Frequency is less than or equal to 36 MHz"]
    _2 = 2,
    #[doc = "3: Frequency is less than or equal to 60 MHz"]
    _3 = 3,
}
impl From<FOM_A> for u8 {
    #[inline(always)]
    fn from(variant: FOM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FOM` reader - Frequency Optimization Mode (Write Protect)\nThe M031/M032 series support adjustable Flash access timing to optimize the Flash access cycles in different system working frequency.\nFor 16/32/64/128 Kbytes Flash:\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct FOM_R(crate::FieldReader<u8, FOM_A>);
impl FOM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FOM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FOM_A> {
        match self.bits {
            0 => Some(FOM_A::_0),
            1 => Some(FOM_A::_1),
            2 => Some(FOM_A::_2),
            3 => Some(FOM_A::_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FOM_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == FOM_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == FOM_A::_3
    }
}
impl core::ops::Deref for FOM_R {
    type Target = crate::FieldReader<u8, FOM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOM` writer - Frequency Optimization Mode (Write Protect)\nThe M031/M032 series support adjustable Flash access timing to optimize the Flash access cycles in different system working frequency.\nFor 16/32/64/128 Kbytes Flash:\nNote: This bit is write-protected. Refer to the SYS_REGLCTL register."]
pub struct FOM_W<'a> {
    w: &'a mut W,
}
impl<'a> FOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Frequency is less than or equal to 48 MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FOM_A::_0)
    }
    #[doc = "Frequency is less than or equal to 24 MHz.\nFrequency is less than or equal to 12 MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FOM_A::_1)
    }
    #[doc = "Frequency is less than or equal to 36 MHz"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FOM_A::_2)
    }
    #[doc = "Frequency is less than or equal to 60 MHz"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(FOM_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Flash Branch Buffer Disable Control (Write Protect)\nNote 1: This bit is write-protected. Refer to the SYS_REGLCTL register.\nNote 2: Only suppoted in 16/32/64/128 Kbytes Flash.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBOFF_A {
    #[doc = "0: Flash Branch Buffer function Enabled (default)"]
    _0 = 0,
    #[doc = "1: Flash Branch Buffer function Disabled"]
    _1 = 1,
}
impl From<BBOFF_A> for bool {
    #[inline(always)]
    fn from(variant: BBOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BBOFF` reader - Flash Branch Buffer Disable Control (Write Protect)\nNote 1: This bit is write-protected. Refer to the SYS_REGLCTL register.\nNote 2: Only suppoted in 16/32/64/128 Kbytes Flash."]
pub struct BBOFF_R(crate::FieldReader<bool, BBOFF_A>);
impl BBOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BBOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BBOFF_A {
        match self.bits {
            false => BBOFF_A::_0,
            true => BBOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BBOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BBOFF_A::_1
    }
}
impl core::ops::Deref for BBOFF_R {
    type Target = crate::FieldReader<bool, BBOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBOFF` writer - Flash Branch Buffer Disable Control (Write Protect)\nNote 1: This bit is write-protected. Refer to the SYS_REGLCTL register.\nNote 2: Only suppoted in 16/32/64/128 Kbytes Flash."]
pub struct BBOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BBOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BBOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash Branch Buffer function Enabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BBOFF_A::_0)
    }
    #[doc = "Flash Branch Buffer function Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BBOFF_A::_1)
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
#[doc = "Flash Cache Invalidation (Write Protect)\nNote 1: Write 1 to start cache invalidation. The value will be changed to 0 once the process is finished.\nNote 2: This bit is write-protected. Refer to the SYS_REGLCTL register.\nNote 3: Only supported in 256/512 Kbytes Flash.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEINV_A {
    #[doc = "0: Flash Cache Invalidation finished (default)"]
    _0 = 0,
    #[doc = "1: Flash Cache Invalidation"]
    _1 = 1,
}
impl From<CACHEINV_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEINV` reader - Flash Cache Invalidation (Write Protect)\nNote 1: Write 1 to start cache invalidation. The value will be changed to 0 once the process is finished.\nNote 2: This bit is write-protected. Refer to the SYS_REGLCTL register.\nNote 3: Only supported in 256/512 Kbytes Flash."]
pub struct CACHEINV_R(crate::FieldReader<bool, CACHEINV_A>);
impl CACHEINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHEINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEINV_A {
        match self.bits {
            false => CACHEINV_A::_0,
            true => CACHEINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CACHEINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CACHEINV_A::_1
    }
}
impl core::ops::Deref for CACHEINV_R {
    type Target = crate::FieldReader<bool, CACHEINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEINV` writer - Flash Cache Invalidation (Write Protect)\nNote 1: Write 1 to start cache invalidation. The value will be changed to 0 once the process is finished.\nNote 2: This bit is write-protected. Refer to the SYS_REGLCTL register.\nNote 3: Only supported in 256/512 Kbytes Flash."]
pub struct CACHEINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash Cache Invalidation finished (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CACHEINV_A::_0)
    }
    #[doc = "Flash Cache Invalidation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CACHEINV_A::_1)
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
    #[doc = "Bits 4:6 - Frequency Optimization Mode (Write Protect) The M031/M032 series support adjustable Flash access timing to optimize the Flash access cycles in different system working frequency. For 16/32/64/128 Kbytes Flash: Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn fom(&self) -> FOM_R {
        FOM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Flash Branch Buffer Disable Control (Write Protect) Note 1: This bit is write-protected. Refer to the SYS_REGLCTL register. Note 2: Only suppoted in 16/32/64/128 Kbytes Flash."]
    #[inline(always)]
    pub fn bboff(&self) -> BBOFF_R {
        BBOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flash Cache Invalidation (Write Protect) Note 1: Write 1 to start cache invalidation. The value will be changed to 0 once the process is finished. Note 2: This bit is write-protected. Refer to the SYS_REGLCTL register. Note 3: Only supported in 256/512 Kbytes Flash."]
    #[inline(always)]
    pub fn cacheinv(&self) -> CACHEINV_R {
        CACHEINV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Frequency Optimization Mode (Write Protect) The M031/M032 series support adjustable Flash access timing to optimize the Flash access cycles in different system working frequency. For 16/32/64/128 Kbytes Flash: Note: This bit is write-protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn fom(&mut self) -> FOM_W {
        FOM_W { w: self }
    }
    #[doc = "Bit 7 - Flash Branch Buffer Disable Control (Write Protect) Note 1: This bit is write-protected. Refer to the SYS_REGLCTL register. Note 2: Only suppoted in 16/32/64/128 Kbytes Flash."]
    #[inline(always)]
    pub fn bboff(&mut self) -> BBOFF_W {
        BBOFF_W { w: self }
    }
    #[doc = "Bit 9 - Flash Cache Invalidation (Write Protect) Note 1: Write 1 to start cache invalidation. The value will be changed to 0 once the process is finished. Note 2: This bit is write-protected. Refer to the SYS_REGLCTL register. Note 3: Only supported in 256/512 Kbytes Flash."]
    #[inline(always)]
    pub fn cacheinv(&mut self) -> CACHEINV_W {
        CACHEINV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Access Time Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ftctl](index.html) module"]
pub struct FMC_FTCTL_SPEC;
impl crate::RegisterSpec for FMC_FTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_ftctl::R](R) reader structure"]
impl crate::Readable for FMC_FTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_ftctl::W](W) writer structure"]
impl crate::Writable for FMC_FTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_FTCTL to value 0"]
impl crate::Resettable for FMC_FTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
