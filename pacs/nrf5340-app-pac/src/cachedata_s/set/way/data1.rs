#[doc = "Register `DATA1` reader"]
pub struct R(crate::R<DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA1` writer"]
pub struct W(crate::W<DATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA1_SPEC>;
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
impl From<crate::W<DATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Data` reader - Data"]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data` writer - Data"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Cache data bits \\[63:32\\]
of SET\\[n\\], WAY\\[o\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data1](index.html) module"]
pub struct DATA1_SPEC;
impl crate::RegisterSpec for DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data1::R](R) reader structure"]
impl crate::Readable for DATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data1::W](W) writer structure"]
impl crate::Writable for DATA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA1 to value 0"]
impl crate::Resettable for DATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
