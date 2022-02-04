#[doc = "Register `DEF` reader"]
pub struct R(crate::R<DEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEF` writer"]
pub struct W(crate::W<DEF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEF_SPEC>;
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
impl From<crate::W<DEF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEF` reader - Default character. Character clocked out in case of an ignored transaction."]
pub struct DEF_R(crate::FieldReader<u8, u8>);
impl DEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEF` writer - Default character. Character clocked out in case of an ignored transaction."]
pub struct DEF_W<'a> {
    w: &'a mut W,
}
impl<'a> DEF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub fn def(&self) -> DEF_R {
        DEF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub fn def(&mut self) -> DEF_W {
        DEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Default character. Character clocked out in case of an ignored transaction.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [def](index.html) module"]
pub struct DEF_SPEC;
impl crate::RegisterSpec for DEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [def::R](R) reader structure"]
impl crate::Readable for DEF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [def::W](W) writer structure"]
impl crate::Writable for DEF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEF to value 0"]
impl crate::Resettable for DEF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
