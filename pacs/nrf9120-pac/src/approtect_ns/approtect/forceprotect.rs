#[doc = "Register `FORCEPROTECT` reader"]
pub struct R(crate::R<FORCEPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FORCEPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FORCEPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FORCEPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FORCEPROTECT` writer"]
pub struct W(crate::W<FORCEPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCEPROTECT_SPEC>;
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
impl From<crate::W<FORCEPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCEPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCEPROTECT` reader - Write 0x1 to force enable APPROTECT mechanism"]
pub type FORCEPROTECT_R = crate::BitReader<FORCEPROTECT_A>;
#[doc = "Write 0x1 to force enable APPROTECT mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEPROTECT_A {
    #[doc = "1: Software force enable APPROTECT mechanism"]
    FORCE = 1,
}
impl From<FORCEPROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEPROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCEPROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORCEPROTECT_A> {
        match self.bits {
            true => Some(FORCEPROTECT_A::FORCE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == FORCEPROTECT_A::FORCE
    }
}
#[doc = "Field `FORCEPROTECT` writer - Write 0x1 to force enable APPROTECT mechanism"]
pub type FORCEPROTECT_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, FORCEPROTECT_SPEC, FORCEPROTECT_A, O>;
impl<'a, const O: u8> FORCEPROTECT_W<'a, O> {
    #[doc = "Software force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEPROTECT_A::FORCE)
    }
}
impl R {
    #[doc = "Bit 9 - Write 0x1 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&self) -> FORCEPROTECT_R {
        FORCEPROTECT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Write 0x1 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&mut self) -> FORCEPROTECT_W<9> {
        FORCEPROTECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software force APPROTECT mechanism\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [forceprotect](index.html) module"]
pub struct FORCEPROTECT_SPEC;
impl crate::RegisterSpec for FORCEPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [forceprotect::R](R) reader structure"]
impl crate::Readable for FORCEPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [forceprotect::W](W) writer structure"]
impl crate::Writable for FORCEPROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FORCEPROTECT to value 0x01"]
impl crate::Resettable for FORCEPROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
