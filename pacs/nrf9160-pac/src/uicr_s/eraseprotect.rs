#[doc = "Register `ERASEPROTECT` reader"]
pub struct R(crate::R<ERASEPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASEPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASEPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASEPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASEPROTECT` writer"]
pub struct W(crate::W<ERASEPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEPROTECT_SPEC>;
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
impl From<crate::W<ERASEPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PALL_A {
    #[doc = "4294967295: Unprotected"]
    UNPROTECTED = 4294967295,
    #[doc = "0: Protected"]
    PROTECTED = 0,
}
impl From<PALL_A> for u32 {
    #[inline(always)]
    fn from(variant: PALL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PALL` reader - Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
pub struct PALL_R(crate::FieldReader<u32, PALL_A>);
impl PALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PALL_A> {
        match self.bits {
            4294967295 => Some(PALL_A::UNPROTECTED),
            0 => Some(PALL_A::PROTECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNPROTECTED`"]
    #[inline(always)]
    pub fn is_unprotected(&self) -> bool {
        **self == PALL_A::UNPROTECTED
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        **self == PALL_A::PROTECTED
    }
}
impl core::ops::Deref for PALL_R {
    type Target = crate::FieldReader<u32, PALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PALL` writer - Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
pub struct PALL_W<'a> {
    w: &'a mut W,
}
impl<'a> PALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PALL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unprotected"]
    #[inline(always)]
    pub fn unprotected(self) -> &'a mut W {
        self.variant(PALL_A::UNPROTECTED)
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn protected(self) -> &'a mut W {
        self.variant(PALL_A::PROTECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
    #[inline(always)]
    pub fn pall(&self) -> PALL_R {
        PALL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
    #[inline(always)]
    pub fn pall(&mut self) -> PALL_W {
        PALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Erase protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseprotect](index.html) module"]
pub struct ERASEPROTECT_SPEC;
impl crate::RegisterSpec for ERASEPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eraseprotect::R](R) reader structure"]
impl crate::Readable for ERASEPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eraseprotect::W](W) writer structure"]
impl crate::Writable for ERASEPROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASEPROTECT to value 0"]
impl crate::Resettable for ERASEPROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
