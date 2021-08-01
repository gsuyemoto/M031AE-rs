#[doc = "Register `PDMA_PRICLR` writer"]
pub struct W(crate::W<PDMA_PRICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_PRICLR_SPEC>;
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
impl From<crate::W<PDMA_PRICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_PRICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRICLR0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: FPRICLR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRICLR0` writer - PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority."]
pub struct FPRICLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLR0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLR0_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLR0_AW::_1)
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
#[doc = "PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRICLR1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLR1_AW> for bool {
    #[inline(always)]
    fn from(variant: FPRICLR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRICLR1` writer - PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority."]
pub struct FPRICLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLR1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLR1_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLR1_AW::_1)
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
#[doc = "PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRICLR2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLR2_AW> for bool {
    #[inline(always)]
    fn from(variant: FPRICLR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRICLR2` writer - PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority."]
pub struct FPRICLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLR2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLR2_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLR2_AW::_1)
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
#[doc = "PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRICLR3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLR3_AW> for bool {
    #[inline(always)]
    fn from(variant: FPRICLR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRICLR3` writer - PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority."]
pub struct FPRICLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLR3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLR3_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLR3_AW::_1)
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
#[doc = "PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRICLR4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLR4_AW> for bool {
    #[inline(always)]
    fn from(variant: FPRICLR4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRICLR4` writer - PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority."]
pub struct FPRICLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLR4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLR4_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLR4_AW::_1)
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
#[doc = "PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRICLR5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLR5_AW> for bool {
    #[inline(always)]
    fn from(variant: FPRICLR5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRICLR5` writer - PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority."]
pub struct FPRICLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLR5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLR5_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLR5_AW::_1)
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
#[doc = "PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRICLR6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLR6_AW> for bool {
    #[inline(always)]
    fn from(variant: FPRICLR6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRICLR6` writer - PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority."]
pub struct FPRICLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLR6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLR6_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLR6_AW::_1)
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
#[doc = "PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRICLR7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLR7_AW> for bool {
    #[inline(always)]
    fn from(variant: FPRICLR7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRICLR7` writer - PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority."]
pub struct FPRICLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLR7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLR7_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLR7_AW::_1)
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
#[doc = "PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRICLR8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLR8_AW> for bool {
    #[inline(always)]
    fn from(variant: FPRICLR8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRICLR8` writer - PDMA Fixed Priority Clear Bits (Write Only)\nSet this bit to 1 to clear fixed priority level.\nNote: User can read PDMA_PRISET register to know the channel priority."]
pub struct FPRICLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLR8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLR8_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLR8_AW::_1)
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
    #[doc = "Bit 0 - PDMA Fixed Priority Clear Bits (Write Only) Set this bit to 1 to clear fixed priority level. Note: User can read PDMA_PRISET register to know the channel priority."]
    #[inline(always)]
    pub fn fpriclr0(&mut self) -> FPRICLR0_W {
        FPRICLR0_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Fixed Priority Clear Bits (Write Only) Set this bit to 1 to clear fixed priority level. Note: User can read PDMA_PRISET register to know the channel priority."]
    #[inline(always)]
    pub fn fpriclr1(&mut self) -> FPRICLR1_W {
        FPRICLR1_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Fixed Priority Clear Bits (Write Only) Set this bit to 1 to clear fixed priority level. Note: User can read PDMA_PRISET register to know the channel priority."]
    #[inline(always)]
    pub fn fpriclr2(&mut self) -> FPRICLR2_W {
        FPRICLR2_W { w: self }
    }
    #[doc = "Bit 3 - PDMA Fixed Priority Clear Bits (Write Only) Set this bit to 1 to clear fixed priority level. Note: User can read PDMA_PRISET register to know the channel priority."]
    #[inline(always)]
    pub fn fpriclr3(&mut self) -> FPRICLR3_W {
        FPRICLR3_W { w: self }
    }
    #[doc = "Bit 4 - PDMA Fixed Priority Clear Bits (Write Only) Set this bit to 1 to clear fixed priority level. Note: User can read PDMA_PRISET register to know the channel priority."]
    #[inline(always)]
    pub fn fpriclr4(&mut self) -> FPRICLR4_W {
        FPRICLR4_W { w: self }
    }
    #[doc = "Bit 5 - PDMA Fixed Priority Clear Bits (Write Only) Set this bit to 1 to clear fixed priority level. Note: User can read PDMA_PRISET register to know the channel priority."]
    #[inline(always)]
    pub fn fpriclr5(&mut self) -> FPRICLR5_W {
        FPRICLR5_W { w: self }
    }
    #[doc = "Bit 6 - PDMA Fixed Priority Clear Bits (Write Only) Set this bit to 1 to clear fixed priority level. Note: User can read PDMA_PRISET register to know the channel priority."]
    #[inline(always)]
    pub fn fpriclr6(&mut self) -> FPRICLR6_W {
        FPRICLR6_W { w: self }
    }
    #[doc = "Bit 7 - PDMA Fixed Priority Clear Bits (Write Only) Set this bit to 1 to clear fixed priority level. Note: User can read PDMA_PRISET register to know the channel priority."]
    #[inline(always)]
    pub fn fpriclr7(&mut self) -> FPRICLR7_W {
        FPRICLR7_W { w: self }
    }
    #[doc = "Bit 8 - PDMA Fixed Priority Clear Bits (Write Only) Set this bit to 1 to clear fixed priority level. Note: User can read PDMA_PRISET register to know the channel priority."]
    #[inline(always)]
    pub fn fpriclr8(&mut self) -> FPRICLR8_W {
        FPRICLR8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Fixed Priority Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_priclr](index.html) module"]
pub struct PDMA_PRICLR_SPEC;
impl crate::RegisterSpec for PDMA_PRICLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdma_priclr::W](W) writer structure"]
impl crate::Writable for PDMA_PRICLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_PRICLR to value 0"]
impl crate::Resettable for PDMA_PRICLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
