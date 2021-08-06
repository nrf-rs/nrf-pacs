#[doc = "Register `DISABLE` reader"]
pub struct R(crate::R<DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISABLE` writer"]
pub struct W(crate::W<DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISABLE_SPEC>;
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
impl From<crate::W<DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software disable APPROTECT mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DISABLE_A {
    #[doc = "90: Software disable APPROTECT mechanism"]
    SWDISABLE = 90,
}
impl From<DISABLE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DISABLE` reader - Software disable APPROTECT mechanism"]
pub struct DISABLE_R(crate::FieldReader<u8, DISABLE_A>);
impl DISABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DISABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISABLE_A> {
        match self.bits {
            90 => Some(DISABLE_A::SWDISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SWDISABLE`"]
    #[inline(always)]
    pub fn is_sw_disable(&self) -> bool {
        **self == DISABLE_A::SWDISABLE
    }
}
impl core::ops::Deref for DISABLE_R {
    type Target = crate::FieldReader<u8, DISABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE` writer - Software disable APPROTECT mechanism"]
pub struct DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn sw_disable(self) -> &'a mut W {
        self.variant(DISABLE_A::SWDISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W {
        DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software disable APPROTECT mechanism\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disable](index.html) module"]
pub struct DISABLE_SPEC;
impl crate::RegisterSpec for DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [disable::R](R) reader structure"]
impl crate::Readable for DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [disable::W](W) writer structure"]
impl crate::Writable for DISABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DISABLE to value 0"]
impl crate::Resettable for DISABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
