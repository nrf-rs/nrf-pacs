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
#[doc = "Write 0x0 to force enable APPROTECT mechanism\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORCEPROTECT_A {
    #[doc = "0: Software force enable APPROTECT mechanism"]
    FORCE = 0,
}
impl From<FORCEPROTECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORCEPROTECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FORCEPROTECT` reader - Write 0x0 to force enable APPROTECT mechanism"]
pub struct FORCEPROTECT_R(crate::FieldReader<u8, FORCEPROTECT_A>);
impl FORCEPROTECT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FORCEPROTECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORCEPROTECT_A> {
        match self.bits {
            0 => Some(FORCEPROTECT_A::FORCE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        **self == FORCEPROTECT_A::FORCE
    }
}
impl core::ops::Deref for FORCEPROTECT_R {
    type Target = crate::FieldReader<u8, FORCEPROTECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEPROTECT` writer - Write 0x0 to force enable APPROTECT mechanism"]
pub struct FORCEPROTECT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEPROTECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEPROTECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEPROTECT_A::FORCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&self) -> FORCEPROTECT_R {
        FORCEPROTECT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&mut self) -> FORCEPROTECT_W {
        FORCEPROTECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software force enable APPROTECT mechanism until next reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [forceprotect](index.html) module"]
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
#[doc = "`reset()` method sets FORCEPROTECT to value 0xffff_ffff"]
impl crate::Resettable for FORCEPROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
