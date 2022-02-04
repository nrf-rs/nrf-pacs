#[doc = "Register `T1` reader"]
pub struct R(crate::R<T1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T1` writer"]
pub struct W(crate::W<T1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1_SPEC>;
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
impl From<crate::W<T1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T1` reader - End point of 2nd piece wise linear function"]
pub struct T1_R(crate::FieldReader<u8, u8>);
impl T1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        T1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1` writer - End point of 2nd piece wise linear function"]
pub struct T1_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - End point of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn t1(&self) -> T1_R {
        T1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn t1(&mut self) -> T1_W {
        T1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End point of 2nd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1](index.html) module"]
pub struct T1_SPEC;
impl crate::RegisterSpec for T1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1::R](R) reader structure"]
impl crate::Readable for T1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t1::W](W) writer structure"]
impl crate::Writable for T1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T1 to value 0"]
impl crate::Resettable for T1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
