#[doc = "Register `FMC_ISPCMD` reader"]
pub struct R(crate::R<FMC_ISPCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_ISPCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_ISPCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_ISPCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_ISPCMD` writer"]
pub struct W(crate::W<FMC_ISPCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_ISPCMD_SPEC>;
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
impl From<crate::W<FMC_ISPCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_ISPCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ISP CMD\nISP command table is shown below:\nThe other commands are invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: Flash Read"]
    _0 = 0,
    #[doc = "4: Read Unique ID"]
    _4 = 4,
    #[doc = "8: Read All One"]
    _8 = 8,
    #[doc = "11: Read Company ID"]
    _11 = 11,
    #[doc = "12: Read Device ID"]
    _12 = 12,
    #[doc = "13: Read CRC32 Checksum"]
    _13 = 13,
    #[doc = "33: Flash 32-bit Program"]
    _33 = 33,
    #[doc = "34: Flash Page Erase"]
    _34 = 34,
    #[doc = "35: Flash APROM Bank Erase"]
    _35 = 35,
    #[doc = "39: Flash Multi-Word Program"]
    _39 = 39,
    #[doc = "40: Run All One"]
    _40 = 40,
    #[doc = "44: APROM Address Operation Model Selection"]
    _44 = 44,
    #[doc = "45: Run CRC32 Checksum Calculation"]
    _45 = 45,
    #[doc = "46: Vector Remap"]
    _46 = 46,
    #[doc = "97: Flash 64-bit Program"]
    _97 = 97,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` reader - ISP CMD\nISP command table is shown below:\nThe other commands are invalid."]
pub struct CMD_R(crate::FieldReader<u8, CMD_A>);
impl CMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::_0),
            4 => Some(CMD_A::_4),
            8 => Some(CMD_A::_8),
            11 => Some(CMD_A::_11),
            12 => Some(CMD_A::_12),
            13 => Some(CMD_A::_13),
            33 => Some(CMD_A::_33),
            34 => Some(CMD_A::_34),
            35 => Some(CMD_A::_35),
            39 => Some(CMD_A::_39),
            40 => Some(CMD_A::_40),
            44 => Some(CMD_A::_44),
            45 => Some(CMD_A::_45),
            46 => Some(CMD_A::_46),
            97 => Some(CMD_A::_97),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == CMD_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == CMD_A::_8
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CMD_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == CMD_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == CMD_A::_13
    }
    #[doc = "Checks if the value of the field is `_33`"]
    #[inline(always)]
    pub fn is_33(&self) -> bool {
        **self == CMD_A::_33
    }
    #[doc = "Checks if the value of the field is `_34`"]
    #[inline(always)]
    pub fn is_34(&self) -> bool {
        **self == CMD_A::_34
    }
    #[doc = "Checks if the value of the field is `_35`"]
    #[inline(always)]
    pub fn is_35(&self) -> bool {
        **self == CMD_A::_35
    }
    #[doc = "Checks if the value of the field is `_39`"]
    #[inline(always)]
    pub fn is_39(&self) -> bool {
        **self == CMD_A::_39
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        **self == CMD_A::_40
    }
    #[doc = "Checks if the value of the field is `_44`"]
    #[inline(always)]
    pub fn is_44(&self) -> bool {
        **self == CMD_A::_44
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline(always)]
    pub fn is_45(&self) -> bool {
        **self == CMD_A::_45
    }
    #[doc = "Checks if the value of the field is `_46`"]
    #[inline(always)]
    pub fn is_46(&self) -> bool {
        **self == CMD_A::_46
    }
    #[doc = "Checks if the value of the field is `_97`"]
    #[inline(always)]
    pub fn is_97(&self) -> bool {
        **self == CMD_A::_97
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - ISP CMD\nISP command table is shown below:\nThe other commands are invalid."]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash Read"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMD_A::_0)
    }
    #[doc = "Read Unique ID"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CMD_A::_4)
    }
    #[doc = "Read All One"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CMD_A::_8)
    }
    #[doc = "Read Company ID"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMD_A::_11)
    }
    #[doc = "Read Device ID"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(CMD_A::_12)
    }
    #[doc = "Read CRC32 Checksum"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(CMD_A::_13)
    }
    #[doc = "Flash 32-bit Program"]
    #[inline(always)]
    pub fn _33(self) -> &'a mut W {
        self.variant(CMD_A::_33)
    }
    #[doc = "Flash Page Erase"]
    #[inline(always)]
    pub fn _34(self) -> &'a mut W {
        self.variant(CMD_A::_34)
    }
    #[doc = "Flash APROM Bank Erase"]
    #[inline(always)]
    pub fn _35(self) -> &'a mut W {
        self.variant(CMD_A::_35)
    }
    #[doc = "Flash Multi-Word Program"]
    #[inline(always)]
    pub fn _39(self) -> &'a mut W {
        self.variant(CMD_A::_39)
    }
    #[doc = "Run All One"]
    #[inline(always)]
    pub fn _40(self) -> &'a mut W {
        self.variant(CMD_A::_40)
    }
    #[doc = "APROM Address Operation Model Selection"]
    #[inline(always)]
    pub fn _44(self) -> &'a mut W {
        self.variant(CMD_A::_44)
    }
    #[doc = "Run CRC32 Checksum Calculation"]
    #[inline(always)]
    pub fn _45(self) -> &'a mut W {
        self.variant(CMD_A::_45)
    }
    #[doc = "Vector Remap"]
    #[inline(always)]
    pub fn _46(self) -> &'a mut W {
        self.variant(CMD_A::_46)
    }
    #[doc = "Flash 64-bit Program"]
    #[inline(always)]
    pub fn _97(self) -> &'a mut W {
        self.variant(CMD_A::_97)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - ISP CMD ISP command table is shown below: The other commands are invalid."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ISP CMD ISP command table is shown below: The other commands are invalid."]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISP Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ispcmd](index.html) module"]
pub struct FMC_ISPCMD_SPEC;
impl crate::RegisterSpec for FMC_ISPCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_ispcmd::R](R) reader structure"]
impl crate::Readable for FMC_ISPCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_ispcmd::W](W) writer structure"]
impl crate::Writable for FMC_ISPCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_ISPCMD to value 0"]
impl crate::Resettable for FMC_ISPCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
