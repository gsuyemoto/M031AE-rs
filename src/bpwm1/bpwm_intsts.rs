#[doc = "Register `BPWM_INTSTS` reader"]
pub struct R(crate::R<BPWM_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPWM_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPWM_INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPWM_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPWM_INTSTS` writer"]
pub struct W(crate::W<BPWM_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPWM_INTSTS_SPEC>;
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
impl From<crate::W<BPWM_INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPWM_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZIF0` reader - BPWM Zero Point Interrupt Flag 0\nThis bit is set by hardware when BPWM_CH0 counter reaches 0, software can write 1 to clear this bit to 0."]
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
#[doc = "Field `ZIF0` writer - BPWM Zero Point Interrupt Flag 0\nThis bit is set by hardware when BPWM_CH0 counter reaches 0, software can write 1 to clear this bit to 0."]
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
#[doc = "Field `PIF0` reader - BPWM Period Point Interrupt Flag 0\nThis bit is set by hardware when BPWM_CH0 counter reaches BPWM_PERIOD0, software can write 1 to clear this bit to 0."]
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
#[doc = "Field `PIF0` writer - BPWM Period Point Interrupt Flag 0\nThis bit is set by hardware when BPWM_CH0 counter reaches BPWM_PERIOD0, software can write 1 to clear this bit to 0."]
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
#[doc = "Field `CMPUIF0` reader - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF0_R(crate::FieldReader<bool, bool>);
impl CMPUIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPUIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIF0` writer - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIF0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CMPUIF1` reader - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF1_R(crate::FieldReader<bool, bool>);
impl CMPUIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPUIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIF1` writer - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIF1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CMPUIF2` reader - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF2_R(crate::FieldReader<bool, bool>);
impl CMPUIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPUIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIF2` writer - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIF2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CMPUIF3` reader - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF3_R(crate::FieldReader<bool, bool>);
impl CMPUIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPUIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIF3` writer - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIF3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CMPUIF4` reader - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF4_R(crate::FieldReader<bool, bool>);
impl CMPUIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPUIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIF4` writer - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIF4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CMPUIF5` reader - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF5_R(crate::FieldReader<bool, bool>);
impl CMPUIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPUIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPUIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIF5` writer - BPWM Compare Up Count Interrupt Flag\nFlag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n.\nNote: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
pub struct CMPUIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIF5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CMPDIF0` reader - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF0` writer - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF1` reader - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF1` writer - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF2` reader - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF2` writer - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF3` reader - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF3` writer - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF4` reader - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF4` writer - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF5` reader - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "Field `CMPDIF5` writer - BPWM Compare Down Count Interrupt Flag\nEach bit n controls the corresponding BPWM channel n.\nFlag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it.\nNote: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
    #[doc = "Bit 0 - BPWM Zero Point Interrupt Flag 0 This bit is set by hardware when BPWM_CH0 counter reaches 0, software can write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn zif0(&self) -> ZIF0_R {
        ZIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - BPWM Period Point Interrupt Flag 0 This bit is set by hardware when BPWM_CH0 counter reaches BPWM_PERIOD0, software can write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif0(&self) -> CMPUIF0_R {
        CMPUIF0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif1(&self) -> CMPUIF1_R {
        CMPUIF1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif2(&self) -> CMPUIF2_R {
        CMPUIF2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif3(&self) -> CMPUIF3_R {
        CMPUIF3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif4(&self) -> CMPUIF4_R {
        CMPUIF4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif5(&self) -> CMPUIF5_R {
        CMPUIF5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif0(&self) -> CMPDIF0_R {
        CMPDIF0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif1(&self) -> CMPDIF1_R {
        CMPDIF1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif2(&self) -> CMPDIF2_R {
        CMPDIF2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif3(&self) -> CMPDIF3_R {
        CMPDIF3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif4(&self) -> CMPDIF4_R {
        CMPDIF4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif5(&self) -> CMPDIF5_R {
        CMPDIF5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BPWM Zero Point Interrupt Flag 0 This bit is set by hardware when BPWM_CH0 counter reaches 0, software can write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn zif0(&mut self) -> ZIF0_W {
        ZIF0_W { w: self }
    }
    #[doc = "Bit 8 - BPWM Period Point Interrupt Flag 0 This bit is set by hardware when BPWM_CH0 counter reaches BPWM_PERIOD0, software can write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn pif0(&mut self) -> PIF0_W {
        PIF0_W { w: self }
    }
    #[doc = "Bit 16 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif0(&mut self) -> CMPUIF0_W {
        CMPUIF0_W { w: self }
    }
    #[doc = "Bit 17 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif1(&mut self) -> CMPUIF1_W {
        CMPUIF1_W { w: self }
    }
    #[doc = "Bit 18 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif2(&mut self) -> CMPUIF2_W {
        CMPUIF2_W { w: self }
    }
    #[doc = "Bit 19 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif3(&mut self) -> CMPUIF3_W {
        CMPUIF3_W { w: self }
    }
    #[doc = "Bit 20 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif4(&mut self) -> CMPUIF4_W {
        CMPUIF4_W { w: self }
    }
    #[doc = "Bit 21 - BPWM Compare Up Count Interrupt Flag Flag is set by hardware when BPWM counter up count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding BPWM channel n. Note: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection."]
    #[inline(always)]
    pub fn cmpuif5(&mut self) -> CMPUIF5_W {
        CMPUIF5_W { w: self }
    }
    #[doc = "Bit 24 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif0(&mut self) -> CMPDIF0_W {
        CMPDIF0_W { w: self }
    }
    #[doc = "Bit 25 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif1(&mut self) -> CMPDIF1_W {
        CMPDIF1_W { w: self }
    }
    #[doc = "Bit 26 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif2(&mut self) -> CMPDIF2_W {
        CMPDIF2_W { w: self }
    }
    #[doc = "Bit 27 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif3(&mut self) -> CMPDIF3_W {
        CMPDIF3_W { w: self }
    }
    #[doc = "Bit 28 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
    #[inline(always)]
    pub fn cmpdif4(&mut self) -> CMPDIF4_W {
        CMPDIF4_W { w: self }
    }
    #[doc = "Bit 29 - BPWM Compare Down Count Interrupt Flag Each bit n controls the corresponding BPWM channel n. Flag is set by hardware when BPWM counter down count and reaches BPWM_CMPDATn, software can clear this bit by writing 1 to it. Note: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection."]
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
#[doc = "BPWM Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwm_intsts](index.html) module"]
pub struct BPWM_INTSTS_SPEC;
impl crate::RegisterSpec for BPWM_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpwm_intsts::R](R) reader structure"]
impl crate::Readable for BPWM_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpwm_intsts::W](W) writer structure"]
impl crate::Writable for BPWM_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPWM_INTSTS to value 0"]
impl crate::Resettable for BPWM_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
