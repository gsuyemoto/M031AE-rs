#[doc = "Register `PDMA_SWREQ` writer"]
pub struct W(crate::W<PDMA_SWREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_SWREQ_SPEC>;
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
impl From<crate::W<PDMA_SWREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_SWREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQ0_AW> for bool {
    #[inline(always)]
    fn from(variant: SWREQ0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ0` writer - PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
pub struct SWREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQ0_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQ0_AW::_1)
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
#[doc = "PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQ1_AW> for bool {
    #[inline(always)]
    fn from(variant: SWREQ1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ1` writer - PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
pub struct SWREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQ1_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQ1_AW::_1)
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
#[doc = "PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQ2_AW> for bool {
    #[inline(always)]
    fn from(variant: SWREQ2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ2` writer - PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
pub struct SWREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQ2_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQ2_AW::_1)
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
#[doc = "PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQ3_AW> for bool {
    #[inline(always)]
    fn from(variant: SWREQ3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ3` writer - PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
pub struct SWREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQ3_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQ3_AW::_1)
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
#[doc = "PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQ4_AW> for bool {
    #[inline(always)]
    fn from(variant: SWREQ4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ4` writer - PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
pub struct SWREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQ4_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQ4_AW::_1)
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
#[doc = "PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQ5_AW> for bool {
    #[inline(always)]
    fn from(variant: SWREQ5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ5` writer - PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
pub struct SWREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQ5_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQ5_AW::_1)
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
#[doc = "PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQ6_AW> for bool {
    #[inline(always)]
    fn from(variant: SWREQ6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ6` writer - PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
pub struct SWREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQ6_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQ6_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQ7_AW> for bool {
    #[inline(always)]
    fn from(variant: SWREQ7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ7` writer - PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
pub struct SWREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQ7_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQ7_AW::_1)
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
#[doc = "PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQ8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQ8_AW> for bool {
    #[inline(always)]
    fn from(variant: SWREQ8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ8` writer - PDMA Software Request (Write Only)\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\nNote 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\nNote 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
pub struct SWREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQ8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQ8_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQ8_AW::_1)
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
impl W {
    #[doc = "Bit 0 - PDMA Software Request (Write Only) Set this bit to 1 to generate a software request to PDMA \\[n\\]. Note 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request. Note 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
    #[inline(always)]
    pub fn swreq0(&mut self) -> SWREQ0_W {
        SWREQ0_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Software Request (Write Only) Set this bit to 1 to generate a software request to PDMA \\[n\\]. Note 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request. Note 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
    #[inline(always)]
    pub fn swreq1(&mut self) -> SWREQ1_W {
        SWREQ1_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Software Request (Write Only) Set this bit to 1 to generate a software request to PDMA \\[n\\]. Note 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request. Note 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
    #[inline(always)]
    pub fn swreq2(&mut self) -> SWREQ2_W {
        SWREQ2_W { w: self }
    }
    #[doc = "Bit 3 - PDMA Software Request (Write Only) Set this bit to 1 to generate a software request to PDMA \\[n\\]. Note 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request. Note 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
    #[inline(always)]
    pub fn swreq3(&mut self) -> SWREQ3_W {
        SWREQ3_W { w: self }
    }
    #[doc = "Bit 4 - PDMA Software Request (Write Only) Set this bit to 1 to generate a software request to PDMA \\[n\\]. Note 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request. Note 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
    #[inline(always)]
    pub fn swreq4(&mut self) -> SWREQ4_W {
        SWREQ4_W { w: self }
    }
    #[doc = "Bit 5 - PDMA Software Request (Write Only) Set this bit to 1 to generate a software request to PDMA \\[n\\]. Note 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request. Note 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
    #[inline(always)]
    pub fn swreq5(&mut self) -> SWREQ5_W {
        SWREQ5_W { w: self }
    }
    #[doc = "Bit 6 - PDMA Software Request (Write Only) Set this bit to 1 to generate a software request to PDMA \\[n\\]. Note 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request. Note 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
    #[inline(always)]
    pub fn swreq6(&mut self) -> SWREQ6_W {
        SWREQ6_W { w: self }
    }
    #[doc = "Bit 7 - PDMA Software Request (Write Only) Set this bit to 1 to generate a software request to PDMA \\[n\\]. Note 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request. Note 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
    #[inline(always)]
    pub fn swreq7(&mut self) -> SWREQ7_W {
        SWREQ7_W { w: self }
    }
    #[doc = "Bit 8 - PDMA Software Request (Write Only) Set this bit to 1 to generate a software request to PDMA \\[n\\]. Note 1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request. Note 2: If user does not enable corresponding PDMA channel, the software request will be ignored."]
    #[inline(always)]
    pub fn swreq8(&mut self) -> SWREQ8_W {
        SWREQ8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Software Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_swreq](index.html) module"]
pub struct PDMA_SWREQ_SPEC;
impl crate::RegisterSpec for PDMA_SWREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdma_swreq::W](W) writer structure"]
impl crate::Writable for PDMA_SWREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_SWREQ to value 0"]
impl crate::Resettable for PDMA_SWREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
