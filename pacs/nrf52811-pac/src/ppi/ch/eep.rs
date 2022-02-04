#[doc = "Register `EEP` reader"]
pub struct R(crate::R<EEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEP` writer"]
pub struct W(crate::W<EEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEP_SPEC>;
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
impl From<crate::W<EEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEP` reader - Pointer to event register. Accepts only addresses to registers from the Event group."]
pub struct EEP_R(crate::FieldReader<u32, u32>);
impl EEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEP` writer - Pointer to event register. Accepts only addresses to registers from the Event group."]
pub struct EEP_W<'a> {
    w: &'a mut W,
}
impl<'a> EEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pointer to event register. Accepts only addresses to registers from the Event group."]
    #[inline(always)]
    pub fn eep(&self) -> EEP_R {
        EEP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to event register. Accepts only addresses to registers from the Event group."]
    #[inline(always)]
    pub fn eep(&mut self) -> EEP_W {
        EEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Channel n event end-point\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep](index.html) module"]
pub struct EEP_SPEC;
impl crate::RegisterSpec for EEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eep::R](R) reader structure"]
impl crate::Readable for EEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eep::W](W) writer structure"]
impl crate::Writable for EEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEP to value 0"]
impl crate::Resettable for EEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
