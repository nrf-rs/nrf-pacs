#[doc = "Register `ERASEPCR0` reader"]
pub struct R(crate::R<ERASEPCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASEPCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASEPCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASEPCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASEPCR0` writer"]
pub struct W(crate::W<ERASEPCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEPCR0_SPEC>;
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
impl From<crate::W<ERASEPCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEPCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEPCR0` reader - Register for starting erase of a page in Code area. Equivalent to ERASEPAGE."]
pub type ERASEPCR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ERASEPCR0` writer - Register for starting erase of a page in Code area. Equivalent to ERASEPAGE."]
pub type ERASEPCR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ERASEPCR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Register for starting erase of a page in Code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub fn erasepcr0(&self) -> ERASEPCR0_R {
        ERASEPCR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register for starting erase of a page in Code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub fn erasepcr0(&mut self) -> ERASEPCR0_W<0> {
        ERASEPCR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepcr0](index.html) module"]
pub struct ERASEPCR0_SPEC;
impl crate::RegisterSpec for ERASEPCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erasepcr0::R](R) reader structure"]
impl crate::Readable for ERASEPCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erasepcr0::W](W) writer structure"]
impl crate::Writable for ERASEPCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASEPCR0 to value 0"]
impl crate::Resettable for ERASEPCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
