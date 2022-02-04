#[doc = "Register `B0` reader"]
pub struct R(crate::R<B0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B0` writer"]
pub struct W(crate::W<B0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B0_SPEC>;
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
impl From<crate::W<B0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B0` reader - y-intercept of 1st piece wise linear function"]
pub struct B0_R(crate::FieldReader<u16, u16>);
impl B0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        B0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B0` writer - y-intercept of 1st piece wise linear function"]
pub struct B0_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    pub fn b0(&self) -> B0_R {
        B0_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    pub fn b0(&mut self) -> B0_W {
        B0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "y-intercept of 1st piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0](index.html) module"]
pub struct B0_SPEC;
impl crate::RegisterSpec for B0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b0::R](R) reader structure"]
impl crate::Readable for B0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b0::W](W) writer structure"]
impl crate::Writable for B0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets B0 to value 0x3fef"]
impl crate::Resettable for B0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fef
    }
}
