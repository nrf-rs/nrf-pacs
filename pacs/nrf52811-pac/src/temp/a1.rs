#[doc = "Register `A1` reader"]
pub struct R(crate::R<A1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A1` writer"]
pub struct W(crate::W<A1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A1_SPEC>;
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
impl From<crate::W<A1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A1` reader - Slope of 2nd piece wise linear function"]
pub struct A1_R(crate::FieldReader<u16, u16>);
impl A1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        A1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A1` writer - Slope of 2nd piece wise linear function"]
pub struct A1_W<'a> {
    w: &'a mut W,
}
impl<'a> A1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Slope of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn a1(&self) -> A1_R {
        A1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn a1(&mut self) -> A1_W {
        A1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope of 2nd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a1](index.html) module"]
pub struct A1_SPEC;
impl crate::RegisterSpec for A1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a1::R](R) reader structure"]
impl crate::Readable for A1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a1::W](W) writer structure"]
impl crate::Writable for A1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets A1 to value 0x0348"]
impl crate::Resettable for A1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0348
    }
}
