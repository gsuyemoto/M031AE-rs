#[doc = "Register `CLK_LDOCTL` reader"]
pub struct R(crate::R<CLK_LDOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_LDOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_LDOCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_LDOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_LDOCTL` writer"]
pub struct W(crate::W<CLK_LDOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_LDOCTL_SPEC>;
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
impl From<crate::W<CLK_LDOCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_LDOCTL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ldoctl](index.html) module"]
pub struct CLK_LDOCTL_SPEC;
impl crate::RegisterSpec for CLK_LDOCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ldoctl::R](R) reader structure"]
impl crate::Readable for CLK_LDOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ldoctl::W](W) writer structure"]
impl crate::Writable for CLK_LDOCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_LDOCTL to value 0"]
impl crate::Resettable for CLK_LDOCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
