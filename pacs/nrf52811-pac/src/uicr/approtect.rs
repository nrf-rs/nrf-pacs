#[doc = "Register `APPROTECT` reader"]
pub struct R(crate::R<APPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APPROTECT` writer"]
pub struct W(crate::W<APPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APPROTECT_SPEC>;
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
impl From<crate::W<APPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable or disable access port protection.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PALL_A {
    #[doc = "255: Disable"]
    DISABLED = 255,
    #[doc = "0: Enable"]
    ENABLED = 0,
}
impl From<PALL_A> for u8 {
    #[inline(always)]
    fn from(variant: PALL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PALL` reader - Enable or disable access port protection."]
pub struct PALL_R(crate::FieldReader<u8, PALL_A>);
impl PALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PALL_A> {
        match self.bits {
            255 => Some(PALL_A::DISABLED),
            0 => Some(PALL_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PALL_A::ENABLED
    }
}
impl core::ops::Deref for PALL_R {
    type Target = crate::FieldReader<u8, PALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PALL` writer - Enable or disable access port protection."]
pub struct PALL_W<'a> {
    w: &'a mut W,
}
impl<'a> PALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PALL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PALL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PALL_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Enable or disable access port protection."]
    #[inline(always)]
    pub fn pall(&self) -> PALL_R {
        PALL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable or disable access port protection."]
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
#[doc = "Access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [approtect](index.html) module"]
pub struct APPROTECT_SPEC;
impl crate::RegisterSpec for APPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [approtect::R](R) reader structure"]
impl crate::Readable for APPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [approtect::W](W) writer structure"]
impl crate::Writable for APPROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APPROTECT to value 0xffff_ffff"]
impl crate::Resettable for APPROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
