#[doc = "Register `VALUE[%s]` reader"]
pub struct R(crate::R<VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VALUE[%s]` writer"]
pub struct W(crate::W<VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VALUE_SPEC>;
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
impl From<crate::W<VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value](index.html) module"]
pub struct VALUE_SPEC;
impl crate::RegisterSpec for VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [value::R](R) reader structure"]
impl crate::Readable for VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [value::W](W) writer structure"]
impl crate::Writable for VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VALUE[%s]
to value 0xffff_ffff"]
impl crate::Resettable for VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
