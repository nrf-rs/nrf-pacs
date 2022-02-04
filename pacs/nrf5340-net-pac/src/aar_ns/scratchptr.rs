#[doc = "Register `SCRATCHPTR` reader"]
pub struct R(crate::R<SCRATCHPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCRATCHPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCRATCHPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCRATCHPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCRATCHPTR` writer"]
pub struct W(crate::W<SCRATCHPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCRATCHPTR_SPEC>;
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
impl From<crate::W<SCRATCHPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCRATCHPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCHPTR` reader - Pointer to a scratch data area used for temporary storage during resolution. A space of minimum 3 bytes must be reserved."]
pub struct SCRATCHPTR_R(crate::FieldReader<u32, u32>);
impl SCRATCHPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SCRATCHPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCHPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRATCHPTR` writer - Pointer to a scratch data area used for temporary storage during resolution. A space of minimum 3 bytes must be reserved."]
pub struct SCRATCHPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCHPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pointer to a scratch data area used for temporary storage during resolution. A space of minimum 3 bytes must be reserved."]
    #[inline(always)]
    pub fn scratchptr(&self) -> SCRATCHPTR_R {
        SCRATCHPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to a scratch data area used for temporary storage during resolution. A space of minimum 3 bytes must be reserved."]
    #[inline(always)]
    pub fn scratchptr(&mut self) -> SCRATCHPTR_W {
        SCRATCHPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pointer to data area used for temporary storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratchptr](index.html) module"]
pub struct SCRATCHPTR_SPEC;
impl crate::RegisterSpec for SCRATCHPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scratchptr::R](R) reader structure"]
impl crate::Readable for SCRATCHPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scratchptr::W](W) writer structure"]
impl crate::Writable for SCRATCHPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCRATCHPTR to value 0"]
impl crate::Resettable for SCRATCHPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
