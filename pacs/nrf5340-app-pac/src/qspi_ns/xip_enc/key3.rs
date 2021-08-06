#[doc = "Register `KEY3` writer"]
pub struct W(crate::W<KEY3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY3_SPEC>;
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
impl From<crate::W<KEY3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY3` writer - Bits 127:96 of XIP AES KEY"]
pub struct KEY3_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 127:96 of XIP AES KEY"]
    #[inline(always)]
    pub fn key3(&mut self) -> KEY3_W {
        KEY3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bits 127:96 of XIP AES KEY\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key3](index.html) module"]
pub struct KEY3_SPEC;
impl crate::RegisterSpec for KEY3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [key3::W](W) writer structure"]
impl crate::Writable for KEY3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY3 to value 0"]
impl crate::Resettable for KEY3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
