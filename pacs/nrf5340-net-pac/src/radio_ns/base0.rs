#[doc = "Register `BASE0` reader"]
pub struct R(crate::R<BASE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASE0` writer"]
pub struct W(crate::W<BASE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASE0_SPEC>;
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
impl From<crate::W<BASE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE0` reader - Base address 0"]
pub struct BASE0_R(crate::FieldReader<u32, u32>);
impl BASE0_R {
    pub(crate) fn new(bits: u32) -> Self {
        BASE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASE0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BASE0` writer - Base address 0"]
pub struct BASE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address 0"]
    #[inline(always)]
    pub fn base0(&self) -> BASE0_R {
        BASE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address 0"]
    #[inline(always)]
    pub fn base0(&mut self) -> BASE0_W {
        BASE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base0](index.html) module"]
pub struct BASE0_SPEC;
impl crate::RegisterSpec for BASE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base0::R](R) reader structure"]
impl crate::Readable for BASE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [base0::W](W) writer structure"]
impl crate::Writable for BASE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BASE0 to value 0"]
impl crate::Resettable for BASE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
