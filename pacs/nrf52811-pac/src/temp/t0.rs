#[doc = "Register `T0` reader"]
pub struct R(crate::R<T0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0` writer"]
pub struct W(crate::W<T0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0_SPEC>;
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
impl From<crate::W<T0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0` reader - End point of 1st piece wise linear function"]
pub struct T0_R(crate::FieldReader<u8, u8>);
impl T0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        T0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0` writer - End point of 1st piece wise linear function"]
pub struct T0_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - End point of 1st piece wise linear function"]
    #[inline(always)]
    pub fn t0(&self) -> T0_R {
        T0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 1st piece wise linear function"]
    #[inline(always)]
    pub fn t0(&mut self) -> T0_W {
        T0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End point of 1st piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0](index.html) module"]
pub struct T0_SPEC;
impl crate::RegisterSpec for T0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0::R](R) reader structure"]
impl crate::Readable for T0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0::W](W) writer structure"]
impl crate::Writable for T0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0 to value 0xe2"]
impl crate::Resettable for T0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe2
    }
}
