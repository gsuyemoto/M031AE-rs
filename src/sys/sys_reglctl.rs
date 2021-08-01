#[doc = "Register `SYS_REGLCTL` reader"]
pub struct R(crate::R<SYS_REGLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_REGLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_REGLCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_REGLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_REGLCTL` writer"]
pub struct W(crate::W<SYS_REGLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_REGLCTL_SPEC>;
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
impl From<crate::W<SYS_REGLCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_REGLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Register Lock Control Code (Write Only)\nSome registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write.\nREGLCTL\\[0\\]\nRegister Lock Control Disable Index (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REGLCTL_A {
    #[doc = "0: Write-protection Enabled for writing protected registers. Any write to the protected register is ignored"]
    _0 = 0,
    #[doc = "1: Write-protection Disabled for writing protected registers"]
    _1 = 1,
}
impl From<REGLCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: REGLCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REGLCTL` reader - Register Lock Control Code (Write Only)\nSome registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write.\nREGLCTL\\[0\\]\nRegister Lock Control Disable Index (Read Only)"]
pub struct REGLCTL_R(crate::FieldReader<u8, REGLCTL_A>);
impl REGLCTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGLCTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REGLCTL_A> {
        match self.bits {
            0 => Some(REGLCTL_A::_0),
            1 => Some(REGLCTL_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REGLCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REGLCTL_A::_1
    }
}
impl core::ops::Deref for REGLCTL_R {
    type Target = crate::FieldReader<u8, REGLCTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGLCTL` writer - Register Lock Control Code (Write Only)\nSome registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write.\nREGLCTL\\[0\\]\nRegister Lock Control Disable Index (Read Only)"]
pub struct REGLCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> REGLCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGLCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Write-protection Enabled for writing protected registers. Any write to the protected register is ignored"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REGLCTL_A::_0)
    }
    #[doc = "Write-protection Disabled for writing protected registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REGLCTL_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Register Lock Control Code (Write Only) Some registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write. REGLCTL\\[0\\]
Register Lock Control Disable Index (Read Only)"]
    #[inline(always)]
    pub fn reglctl(&self) -> REGLCTL_R {
        REGLCTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Register Lock Control Code (Write Only) Some registers have write-protection function. Writing these registers have to disable the protected function by writing the sequence value '59h', '16h', '88h' to this field. After this sequence is completed, the REGLCTL bit will be set to 1 and write-protection registers can be normal write. REGLCTL\\[0\\]
Register Lock Control Disable Index (Read Only)"]
    #[inline(always)]
    pub fn reglctl(&mut self) -> REGLCTL_W {
        REGLCTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register Lock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reglctl](index.html) module"]
pub struct SYS_REGLCTL_SPEC;
impl crate::RegisterSpec for SYS_REGLCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_reglctl::R](R) reader structure"]
impl crate::Readable for SYS_REGLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_reglctl::W](W) writer structure"]
impl crate::Writable for SYS_REGLCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_REGLCTL to value 0"]
impl crate::Resettable for SYS_REGLCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
