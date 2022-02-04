#[doc = "Register `B3` reader"]
pub struct R(crate::R<B3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B3` writer"]
pub struct W(crate::W<B3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B3_SPEC>;
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
impl From<crate::W<B3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B3` reader - y-intercept of 4th piece wise linear function"]
pub struct B3_R(crate::FieldReader<u16, u16>);
impl B3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        B3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B3` writer - y-intercept of 4th piece wise linear function"]
pub struct B3_W<'a> {
    w: &'a mut W,
}
impl<'a> B3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - y-intercept of 4th piece wise linear function"]
    #[inline(always)]
    pub fn b3(&self) -> B3_R {
        B3_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 4th piece wise linear function"]
    #[inline(always)]
    pub fn b3(&mut self) -> B3_W {
        B3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "y-intercept of 4th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b3](index.html) module"]
pub struct B3_SPEC;
impl crate::RegisterSpec for B3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b3::R](R) reader structure"]
impl crate::Readable for B3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b3::W](W) writer structure"]
impl crate::Writable for B3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets B3 to value 0x12"]
impl crate::Resettable for B3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x12
    }
}
