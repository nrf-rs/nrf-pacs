#[doc = "Register `WA` reader"]
pub struct R(crate::R<WA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WA` writer"]
pub struct W(crate::W<WA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WA_SPEC>;
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
impl From<crate::W<WA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WA` reader - "]
pub type WA_R = crate::BitReader<bool>;
#[doc = "Field `WA` writer - "]
pub type WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, WA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W<0> {
        WA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster\\[n\\]: Write access to peripheral region n detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wa](index.html) module"]
pub struct WA_SPEC;
impl crate::RegisterSpec for WA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wa::R](R) reader structure"]
impl crate::Readable for WA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wa::W](W) writer structure"]
impl crate::Writable for WA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WA to value 0"]
impl crate::Resettable for WA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
