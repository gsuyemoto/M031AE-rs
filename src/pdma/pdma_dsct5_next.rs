#[doc = "Register `PDMA_DSCT5_NEXT` reader"]
pub struct R(crate::R<PDMA_DSCT5_NEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_DSCT5_NEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_DSCT5_NEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_DSCT5_NEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_DSCT5_NEXT` writer"]
pub struct W(crate::W<PDMA_DSCT5_NEXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_DSCT5_NEXT_SPEC>;
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
impl From<crate::W<PDMA_DSCT5_NEXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_DSCT5_NEXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEXT` reader - PDMA Next Descriptor Table Offset\nThis field indicates the offset of the next descriptor table address in system memory. \nWrite Operation:\nIf the system memory based address is 0x2000_0000 (PDMA_SCATBA), and the next descriptor table is start from 0x2000_0100, then this field must fill in 0x0100.\nRead Operation:\nWhen operating in scatter-gather mode, the last two bits NEXT\\[1:0\\]
will become reserved, and indicate the first next address of system memory.\nNote 1: The descriptor table address must be word boundary.\nNote 2: Before filled transfer task in the descriptor table, user must check if the descriptor table is complete."]
pub struct NEXT_R(crate::FieldReader<u16, u16>);
impl NEXT_R {
    pub(crate) fn new(bits: u16) -> Self {
        NEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEXT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEXT` writer - PDMA Next Descriptor Table Offset\nThis field indicates the offset of the next descriptor table address in system memory. \nWrite Operation:\nIf the system memory based address is 0x2000_0000 (PDMA_SCATBA), and the next descriptor table is start from 0x2000_0100, then this field must fill in 0x0100.\nRead Operation:\nWhen operating in scatter-gather mode, the last two bits NEXT\\[1:0\\]
will become reserved, and indicate the first next address of system memory.\nNote 1: The descriptor table address must be word boundary.\nNote 2: Before filled transfer task in the descriptor table, user must check if the descriptor table is complete."]
pub struct NEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `EXENEXT` reader - PDMA Execution Next Descriptor Table Offset\nThis field indicates the offset of next descriptor table address of current execution descriptor table in system memory. \nNote: write operation is useless in this field."]
pub struct EXENEXT_R(crate::FieldReader<u16, u16>);
impl EXENEXT_R {
    pub(crate) fn new(bits: u16) -> Self {
        EXENEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXENEXT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXENEXT` writer - PDMA Execution Next Descriptor Table Offset\nThis field indicates the offset of next descriptor table address of current execution descriptor table in system memory. \nNote: write operation is useless in this field."]
pub struct EXENEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXENEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PDMA Next Descriptor Table Offset This field indicates the offset of the next descriptor table address in system memory. Write Operation: If the system memory based address is 0x2000_0000 (PDMA_SCATBA), and the next descriptor table is start from 0x2000_0100, then this field must fill in 0x0100. Read Operation: When operating in scatter-gather mode, the last two bits NEXT\\[1:0\\]
will become reserved, and indicate the first next address of system memory. Note 1: The descriptor table address must be word boundary. Note 2: Before filled transfer task in the descriptor table, user must check if the descriptor table is complete."]
    #[inline(always)]
    pub fn next(&self) -> NEXT_R {
        NEXT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PDMA Execution Next Descriptor Table Offset This field indicates the offset of next descriptor table address of current execution descriptor table in system memory. Note: write operation is useless in this field."]
    #[inline(always)]
    pub fn exenext(&self) -> EXENEXT_R {
        EXENEXT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDMA Next Descriptor Table Offset This field indicates the offset of the next descriptor table address in system memory. Write Operation: If the system memory based address is 0x2000_0000 (PDMA_SCATBA), and the next descriptor table is start from 0x2000_0100, then this field must fill in 0x0100. Read Operation: When operating in scatter-gather mode, the last two bits NEXT\\[1:0\\]
will become reserved, and indicate the first next address of system memory. Note 1: The descriptor table address must be word boundary. Note 2: Before filled transfer task in the descriptor table, user must check if the descriptor table is complete."]
    #[inline(always)]
    pub fn next(&mut self) -> NEXT_W {
        NEXT_W { w: self }
    }
    #[doc = "Bits 16:31 - PDMA Execution Next Descriptor Table Offset This field indicates the offset of next descriptor table address of current execution descriptor table in system memory. Note: write operation is useless in this field."]
    #[inline(always)]
    pub fn exenext(&mut self) -> EXENEXT_W {
        EXENEXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_dsct5_next](index.html) module"]
pub struct PDMA_DSCT5_NEXT_SPEC;
impl crate::RegisterSpec for PDMA_DSCT5_NEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_dsct5_next::R](R) reader structure"]
impl crate::Readable for PDMA_DSCT5_NEXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_dsct5_next::W](W) writer structure"]
impl crate::Writable for PDMA_DSCT5_NEXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_DSCT5_NEXT to value 0"]
impl crate::Resettable for PDMA_DSCT5_NEXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
