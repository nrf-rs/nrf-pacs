#[doc = "Register `DISABLE` reader"]
pub struct R(crate::R<DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISABLE` writer"]
pub struct W(crate::W<DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISABLE_SPEC>;
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
impl From<crate::W<DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - The ERASEALL sequence is initiated if the value of the KEY fields are non-zero and the KEY fields match on both the CPU and debugger sides."]
pub type KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY` writer - The ERASEALL sequence is initiated if the value of the KEY fields are non-zero and the KEY fields match on both the CPU and debugger sides."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DISABLE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The ERASEALL sequence is initiated if the value of the KEY fields are non-zero and the KEY fields match on both the CPU and debugger sides."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ERASEALL sequence is initiated if the value of the KEY fields are non-zero and the KEY fields match on both the CPU and debugger sides."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register disables the ERASEPROTECT register and performs an ERASEALL operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disable](index.html) module"]
pub struct DISABLE_SPEC;
impl crate::RegisterSpec for DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [disable::R](R) reader structure"]
impl crate::Readable for DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [disable::W](W) writer structure"]
impl crate::Writable for DISABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DISABLE to value 0"]
impl crate::Resettable for DISABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
