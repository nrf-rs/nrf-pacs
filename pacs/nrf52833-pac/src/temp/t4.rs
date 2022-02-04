#[doc = "Register `T4` reader"]
pub struct R(crate::R<T4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T4` writer"]
pub struct W(crate::W<T4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T4_SPEC>;
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
impl From<crate::W<T4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T4` reader - End point of 5th piece wise linear function"]
pub struct T4_R(crate::FieldReader<u8, u8>);
impl T4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        T4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T4` writer - End point of 5th piece wise linear function"]
pub struct T4_W<'a> {
    w: &'a mut W,
}
impl<'a> T4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - End point of 5th piece wise linear function"]
    #[inline(always)]
    pub fn t4(&self) -> T4_R {
        T4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 5th piece wise linear function"]
    #[inline(always)]
    pub fn t4(&mut self) -> T4_W {
        T4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End point of 5th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t4](index.html) module"]
pub struct T4_SPEC;
impl crate::RegisterSpec for T4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t4::R](R) reader structure"]
impl crate::Readable for T4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t4::W](W) writer structure"]
impl crate::Writable for T4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T4 to value 0x50"]
impl crate::Resettable for T4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x50
    }
}
