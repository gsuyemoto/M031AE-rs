#[doc = "Register `PDMA_PAUSE` writer"]
pub struct W(crate::W<PDMA_PAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_PAUSE_SPEC>;
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
impl From<crate::W<PDMA_PAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_PAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Channel n Transfer Pause Control (Write Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSE0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Pause PDMA channel n transfer"]
    _1 = 1,
}
impl From<PAUSE0_AW> for bool {
    #[inline(always)]
    fn from(variant: PAUSE0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE0` writer - PDMA Channel n Transfer Pause Control (Write Only)"]
pub struct PAUSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSE0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUSE0_AW::_0)
    }
    #[doc = "Pause PDMA channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUSE0_AW::_1)
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
#[doc = "PDMA Channel n Transfer Pause Control (Write Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSE1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Pause PDMA channel n transfer"]
    _1 = 1,
}
impl From<PAUSE1_AW> for bool {
    #[inline(always)]
    fn from(variant: PAUSE1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE1` writer - PDMA Channel n Transfer Pause Control (Write Only)"]
pub struct PAUSE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSE1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUSE1_AW::_0)
    }
    #[doc = "Pause PDMA channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUSE1_AW::_1)
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
#[doc = "PDMA Channel n Transfer Pause Control (Write Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSE2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Pause PDMA channel n transfer"]
    _1 = 1,
}
impl From<PAUSE2_AW> for bool {
    #[inline(always)]
    fn from(variant: PAUSE2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE2` writer - PDMA Channel n Transfer Pause Control (Write Only)"]
pub struct PAUSE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSE2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUSE2_AW::_0)
    }
    #[doc = "Pause PDMA channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUSE2_AW::_1)
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
#[doc = "PDMA Channel n Transfer Pause Control (Write Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSE3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Pause PDMA channel n transfer"]
    _1 = 1,
}
impl From<PAUSE3_AW> for bool {
    #[inline(always)]
    fn from(variant: PAUSE3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE3` writer - PDMA Channel n Transfer Pause Control (Write Only)"]
pub struct PAUSE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSE3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUSE3_AW::_0)
    }
    #[doc = "Pause PDMA channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUSE3_AW::_1)
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
#[doc = "PDMA Channel n Transfer Pause Control (Write Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSE4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Pause PDMA channel n transfer"]
    _1 = 1,
}
impl From<PAUSE4_AW> for bool {
    #[inline(always)]
    fn from(variant: PAUSE4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE4` writer - PDMA Channel n Transfer Pause Control (Write Only)"]
pub struct PAUSE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSE4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUSE4_AW::_0)
    }
    #[doc = "Pause PDMA channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUSE4_AW::_1)
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
#[doc = "PDMA Channel n Transfer Pause Control (Write Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSE5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Pause PDMA channel n transfer"]
    _1 = 1,
}
impl From<PAUSE5_AW> for bool {
    #[inline(always)]
    fn from(variant: PAUSE5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE5` writer - PDMA Channel n Transfer Pause Control (Write Only)"]
pub struct PAUSE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSE5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUSE5_AW::_0)
    }
    #[doc = "Pause PDMA channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUSE5_AW::_1)
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
#[doc = "PDMA Channel n Transfer Pause Control (Write Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSE6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Pause PDMA channel n transfer"]
    _1 = 1,
}
impl From<PAUSE6_AW> for bool {
    #[inline(always)]
    fn from(variant: PAUSE6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE6` writer - PDMA Channel n Transfer Pause Control (Write Only)"]
pub struct PAUSE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSE6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUSE6_AW::_0)
    }
    #[doc = "Pause PDMA channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUSE6_AW::_1)
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
#[doc = "PDMA Channel n Transfer Pause Control (Write Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSE7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Pause PDMA channel n transfer"]
    _1 = 1,
}
impl From<PAUSE7_AW> for bool {
    #[inline(always)]
    fn from(variant: PAUSE7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE7` writer - PDMA Channel n Transfer Pause Control (Write Only)"]
pub struct PAUSE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSE7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUSE7_AW::_0)
    }
    #[doc = "Pause PDMA channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUSE7_AW::_1)
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
#[doc = "PDMA Channel n Transfer Pause Control (Write Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSE8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Pause PDMA channel n transfer"]
    _1 = 1,
}
impl From<PAUSE8_AW> for bool {
    #[inline(always)]
    fn from(variant: PAUSE8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSE8` writer - PDMA Channel n Transfer Pause Control (Write Only)"]
pub struct PAUSE8_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSE8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUSE8_AW::_0)
    }
    #[doc = "Pause PDMA channel n transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUSE8_AW::_1)
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
    #[doc = "Bit 0 - PDMA Channel n Transfer Pause Control (Write Only)"]
    #[inline(always)]
    pub fn pause0(&mut self) -> PAUSE0_W {
        PAUSE0_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Channel n Transfer Pause Control (Write Only)"]
    #[inline(always)]
    pub fn pause1(&mut self) -> PAUSE1_W {
        PAUSE1_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Channel n Transfer Pause Control (Write Only)"]
    #[inline(always)]
    pub fn pause2(&mut self) -> PAUSE2_W {
        PAUSE2_W { w: self }
    }
    #[doc = "Bit 3 - PDMA Channel n Transfer Pause Control (Write Only)"]
    #[inline(always)]
    pub fn pause3(&mut self) -> PAUSE3_W {
        PAUSE3_W { w: self }
    }
    #[doc = "Bit 4 - PDMA Channel n Transfer Pause Control (Write Only)"]
    #[inline(always)]
    pub fn pause4(&mut self) -> PAUSE4_W {
        PAUSE4_W { w: self }
    }
    #[doc = "Bit 5 - PDMA Channel n Transfer Pause Control (Write Only)"]
    #[inline(always)]
    pub fn pause5(&mut self) -> PAUSE5_W {
        PAUSE5_W { w: self }
    }
    #[doc = "Bit 6 - PDMA Channel n Transfer Pause Control (Write Only)"]
    #[inline(always)]
    pub fn pause6(&mut self) -> PAUSE6_W {
        PAUSE6_W { w: self }
    }
    #[doc = "Bit 7 - PDMA Channel n Transfer Pause Control (Write Only)"]
    #[inline(always)]
    pub fn pause7(&mut self) -> PAUSE7_W {
        PAUSE7_W { w: self }
    }
    #[doc = "Bit 8 - PDMA Channel n Transfer Pause Control (Write Only)"]
    #[inline(always)]
    pub fn pause8(&mut self) -> PAUSE8_W {
        PAUSE8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Transfer Pause Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_pause](index.html) module"]
pub struct PDMA_PAUSE_SPEC;
impl crate::RegisterSpec for PDMA_PAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdma_pause::W](W) writer structure"]
impl crate::Writable for PDMA_PAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_PAUSE to value 0"]
impl crate::Resettable for PDMA_PAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
