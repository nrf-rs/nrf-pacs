#[doc = "Register `KEY0` writer"]
pub struct W(crate::W<KEY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY0_SPEC>;
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
impl From<crate::W<KEY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` writer - Bits 31:0 of XIP AES KEY"]
pub type KEY0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEY0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Bits 31:0 of XIP AES KEY"]
    #[inline(always)]
    pub fn key0(&mut self) -> KEY0_W<0> {
        KEY0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bits 31:0 of XIP AES KEY\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key0](index.html) module"]
pub struct KEY0_SPEC;
impl crate::RegisterSpec for KEY0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [key0::W](W) writer structure"]
impl crate::Writable for KEY0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY0 to value 0"]
impl crate::Resettable for KEY0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
