#[doc = "Register `PWM_INTSTS0` reader"]
pub struct R(crate::R<PWM_INTSTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_INTSTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_INTSTS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_INTSTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_INTSTS0` writer"]
pub struct W(crate::W<PWM_INTSTS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_INTSTS0_SPEC>;
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
impl From<crate::W<PWM_INTSTS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_INTSTS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZIF0` reader - PWM Zero Point Interrupt Flag 0\nThis bit is set by hardware when PWM_CH0 counter reaches 0.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct ZIF0_R(crate::FieldReader<bool, bool>);
impl ZIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZIF0` writer - PWM Zero Point Interrupt Flag 0\nThis bit is set by hardware when PWM_CH0 counter reaches 0.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct ZIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ZIF0_W<'a> {
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
#[doc = "Field `ZIF2` reader - PWM Zero Point Interrupt Flag 2\nThis bit is set by hardware when PWM_CH2 counter reaches 0.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct ZIF2_R(crate::FieldReader<bool, bool>);
impl ZIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZIF2` writer - PWM Zero Point Interrupt Flag 2\nThis bit is set by hardware when PWM_CH2 counter reaches 0.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct ZIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> ZIF2_W<'a> {
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
#[doc = "Field `ZIF4` reader - PWM Zero Point Interrupt Flag 4\nThis bit is set by hardware when PWM_CH4 counter reaches 0.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct ZIF4_R(crate::FieldReader<bool, bool>);
impl ZIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZIF4` writer - PWM Zero Point Interrupt Flag 4\nThis bit is set by hardware when PWM_CH4 counter reaches 0.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct ZIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> ZIF4_W<'a> {
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
#[doc = "Field `PIF0` reader - PWM Period Point Interrupt Flag 0\nThis bit is set by hardware when PWM_CH0 counter reaches PWM_PERIOD0.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct PIF0_R(crate::FieldReader<bool, bool>);
impl PIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF0` writer - PWM Period Point Interrupt Flag 0\nThis bit is set by hardware when PWM_CH0 counter reaches PWM_PERIOD0.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct PIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF0_W<'a> {
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
#[doc = "Field `PIF2` reader - PWM Period Point Interrupt Flag 2\nThis bit is set by hardware when PWM_CH2 counter reaches PWM_PERIOD2.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct PIF2_R(crate::FieldReader<bool, bool>);
impl PIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF2` writer - PWM Period Point Interrupt Flag 2\nThis bit is set by hardware when PWM_CH2 counter reaches PWM_PERIOD2.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct PIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PIF4` reader - PWM Period Point Interrupt Flag 4\nThis bit is set by hardware when PWM_CH4 counter reaches PWM_PERIOD4.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct PIF4_R(crate::FieldReader<bool, bool>);
impl PIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF4` writer - PWM Period Point Interrupt Flag 4\nThis bit is set by hardware when PWM_CH4 counter reaches PWM_PERIOD4.\nNote: This bit can be cleared to 0 by software writing 1."]
pub struct PIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CMPUIFn` reader - PWM Compare Up Count Interrupt Flag\nFlag is set by hardware when PWM counter up count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPUIF1, 3, 5 is used as another CMPUIF for channel 0, 2, 4."]
pub struct CMPUIFN_R(crate::FieldReader<u8, u8>);
impl CMPUIFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPUIFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPUIFN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIFn` writer - PWM Compare Up Count Interrupt Flag\nFlag is set by hardware when PWM counter up count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPUIF1, 3, 5 is used as another CMPUIF for channel 0, 2, 4."]
pub struct CMPUIFN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `CMPDIF0` reader - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF0_R(crate::FieldReader<bool, bool>);
impl CMPDIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIF0` writer - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIF0_W<'a> {
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
#[doc = "Field `CMPDIF1` reader - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF1_R(crate::FieldReader<bool, bool>);
impl CMPDIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIF1` writer - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIF1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CMPDIF2` reader - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF2_R(crate::FieldReader<bool, bool>);
impl CMPDIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIF2` writer - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIF2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CMPDIF3` reader - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF3_R(crate::FieldReader<bool, bool>);
impl CMPDIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIF3` writer - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIF3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CMPDIF4` reader - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF4_R(crate::FieldReader<bool, bool>);
impl CMPDIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIF4` writer - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIF4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CMPDIF5` reader - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF5_R(crate::FieldReader<bool, bool>);
impl CMPDIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPDIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIF5` writer - PWM Compare Down Count Interrupt Flag\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIF5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWM Zero Point Interrupt Flag 0 This bit is set by hardware when PWM_CH0 counter reaches 0. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn zif0(&self) -> ZIF0_R {
        ZIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Zero Point Interrupt Flag 2 This bit is set by hardware when PWM_CH2 counter reaches 0. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn zif2(&self) -> ZIF2_R {
        ZIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Zero Point Interrupt Flag 4 This bit is set by hardware when PWM_CH4 counter reaches 0. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn zif4(&self) -> ZIF4_R {
        ZIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PWM Period Point Interrupt Flag 0 This bit is set by hardware when PWM_CH0 counter reaches PWM_PERIOD0. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM Period Point Interrupt Flag 2 This bit is set by hardware when PWM_CH2 counter reaches PWM_PERIOD2. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PWM Period Point Interrupt Flag 4 This bit is set by hardware when PWM_CH4 counter reaches PWM_PERIOD4. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - PWM Compare Up Count Interrupt Flag Flag is set by hardware when PWM counter up count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPUIF1, 3, 5 is used as another CMPUIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuifn(&self) -> CMPUIFN_R {
        CMPUIFN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif0(&self) -> CMPDIF0_R {
        CMPDIF0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif1(&self) -> CMPDIF1_R {
        CMPDIF1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif2(&self) -> CMPDIF2_R {
        CMPDIF2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif3(&self) -> CMPDIF3_R {
        CMPDIF3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif4(&self) -> CMPDIF4_R {
        CMPDIF4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif5(&self) -> CMPDIF5_R {
        CMPDIF5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Zero Point Interrupt Flag 0 This bit is set by hardware when PWM_CH0 counter reaches 0. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn zif0(&mut self) -> ZIF0_W {
        ZIF0_W { w: self }
    }
    #[doc = "Bit 2 - PWM Zero Point Interrupt Flag 2 This bit is set by hardware when PWM_CH2 counter reaches 0. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn zif2(&mut self) -> ZIF2_W {
        ZIF2_W { w: self }
    }
    #[doc = "Bit 4 - PWM Zero Point Interrupt Flag 4 This bit is set by hardware when PWM_CH4 counter reaches 0. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn zif4(&mut self) -> ZIF4_W {
        ZIF4_W { w: self }
    }
    #[doc = "Bit 8 - PWM Period Point Interrupt Flag 0 This bit is set by hardware when PWM_CH0 counter reaches PWM_PERIOD0. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn pif0(&mut self) -> PIF0_W {
        PIF0_W { w: self }
    }
    #[doc = "Bit 10 - PWM Period Point Interrupt Flag 2 This bit is set by hardware when PWM_CH2 counter reaches PWM_PERIOD2. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn pif2(&mut self) -> PIF2_W {
        PIF2_W { w: self }
    }
    #[doc = "Bit 12 - PWM Period Point Interrupt Flag 4 This bit is set by hardware when PWM_CH4 counter reaches PWM_PERIOD4. Note: This bit can be cleared to 0 by software writing 1."]
    #[inline(always)]
    pub fn pif4(&mut self) -> PIF4_W {
        PIF4_W { w: self }
    }
    #[doc = "Bits 16:21 - PWM Compare Up Count Interrupt Flag Flag is set by hardware when PWM counter up count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPUIF1, 3, 5 is used as another CMPUIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuifn(&mut self) -> CMPUIFN_W {
        CMPUIFN_W { w: self }
    }
    #[doc = "Bit 24 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif0(&mut self) -> CMPDIF0_W {
        CMPDIF0_W { w: self }
    }
    #[doc = "Bit 25 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif1(&mut self) -> CMPDIF1_W {
        CMPDIF1_W { w: self }
    }
    #[doc = "Bit 26 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif2(&mut self) -> CMPDIF2_W {
        CMPDIF2_W { w: self }
    }
    #[doc = "Bit 27 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif3(&mut self) -> CMPDIF3_W {
        CMPDIF3_W { w: self }
    }
    #[doc = "Bit 28 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif4(&mut self) -> CMPDIF4_W {
        CMPDIF4_W { w: self }
    }
    #[doc = "Bit 29 - PWM Compare Down Count Interrupt Flag Flag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Note: In complementary mode, CMPDIF1, 3, 5 is used as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdif5(&mut self) -> CMPDIF5_W {
        CMPDIF5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Flag Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_intsts0](index.html) module"]
pub struct PWM_INTSTS0_SPEC;
impl crate::RegisterSpec for PWM_INTSTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_intsts0::R](R) reader structure"]
impl crate::Readable for PWM_INTSTS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_intsts0::W](W) writer structure"]
impl crate::Writable for PWM_INTSTS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_INTSTS0 to value 0"]
impl crate::Resettable for PWM_INTSTS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
