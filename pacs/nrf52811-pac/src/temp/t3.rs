#[doc = "Register `T3` reader"]
pub struct R(crate::R<T3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T3` writer"]
pub struct W(crate::W<T3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T3_SPEC>;
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
impl From<crate::W<T3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T3` reader - End point of 4th piece wise linear function"]
pub struct T3_R(crate::FieldReader<u8, u8>);
impl T3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        T3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T3` writer - End point of 4th piece wise linear function"]
pub struct T3_W<'a> {
    w: &'a mut W,
}
impl<'a> T3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - End point of 4th piece wise linear function"]
    #[inline(always)]
    pub fn t3(&self) -> T3_R {
        T3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 4th piece wise linear function"]
    #[inline(always)]
    pub fn t3(&mut self) -> T3_W {
        T3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End point of 4th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3](index.html) module"]
pub struct T3_SPEC;
impl crate::RegisterSpec for T3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t3::R](R) reader structure"]
impl crate::Readable for T3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t3::W](W) writer structure"]
impl crate::Writable for T3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T3 to value 0x3c"]
impl crate::Resettable for T3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3c
    }
}
