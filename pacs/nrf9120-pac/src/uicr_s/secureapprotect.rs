#[doc = "Register `SECUREAPPROTECT` reader"]
pub struct R(crate::R<SECUREAPPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECUREAPPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECUREAPPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECUREAPPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECUREAPPROTECT` writer"]
pub struct W(crate::W<SECUREAPPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECUREAPPROTECT_SPEC>;
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
impl From<crate::W<SECUREAPPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECUREAPPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALL` reader - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
pub type PALL_R = crate::FieldReader<u32, PALL_A>;
#[doc = "Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PALL_A {
    #[doc = "1358582010: HwUnprotected"]
    HW_UNPROTECTED = 1358582010,
    #[doc = "0: Protected"]
    PROTECTED = 0,
}
impl From<PALL_A> for u32 {
    #[inline(always)]
    fn from(variant: PALL_A) -> Self {
        variant as _
    }
}
impl PALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PALL_A> {
        match self.bits {
            1358582010 => Some(PALL_A::HW_UNPROTECTED),
            0 => Some(PALL_A::PROTECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HW_UNPROTECTED`"]
    #[inline(always)]
    pub fn is_hw_unprotected(&self) -> bool {
        *self == PALL_A::HW_UNPROTECTED
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == PALL_A::PROTECTED
    }
}
#[doc = "Field `PALL` writer - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
pub type PALL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECUREAPPROTECT_SPEC, u32, PALL_A, 32, O>;
impl<'a, const O: u8> PALL_W<'a, O> {
    #[doc = "HwUnprotected"]
    #[inline(always)]
    pub fn hw_unprotected(self) -> &'a mut W {
        self.variant(PALL_A::HW_UNPROTECTED)
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn protected(self) -> &'a mut W {
        self.variant(PALL_A::PROTECTED)
    }
}
impl R {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
    #[inline(always)]
    pub fn pall(&self) -> PALL_R {
        PALL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
    #[inline(always)]
    pub fn pall(&mut self) -> PALL_W<0> {
        PALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secureapprotect](index.html) module"]
pub struct SECUREAPPROTECT_SPEC;
impl crate::RegisterSpec for SECUREAPPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secureapprotect::R](R) reader structure"]
impl crate::Readable for SECUREAPPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secureapprotect::W](W) writer structure"]
impl crate::Writable for SECUREAPPROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECUREAPPROTECT to value 0"]
impl crate::Resettable for SECUREAPPROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
