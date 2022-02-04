#[doc = "Register `MAXCNT` reader"]
pub struct R(crate::R<MAXCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXCNT` writer"]
pub struct W(crate::W<MAXCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXCNT_SPEC>;
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
impl From<crate::W<MAXCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFFSIZE` reader - Length of DMA RAM allocation in number of samples"]
pub struct BUFFSIZE_R(crate::FieldReader<u16, u16>);
impl BUFFSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BUFFSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFFSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFFSIZE` writer - Length of DMA RAM allocation in number of samples"]
pub struct BUFFSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub fn buffsize(&self) -> BUFFSIZE_R {
        BUFFSIZE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub fn buffsize(&mut self) -> BUFFSIZE_W {
        BUFFSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of samples to allocate memory for in EasyDMA mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxcnt](index.html) module"]
pub struct MAXCNT_SPEC;
impl crate::RegisterSpec for MAXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxcnt::R](R) reader structure"]
impl crate::Readable for MAXCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxcnt::W](W) writer structure"]
impl crate::Writable for MAXCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXCNT to value 0"]
impl crate::Resettable for MAXCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
