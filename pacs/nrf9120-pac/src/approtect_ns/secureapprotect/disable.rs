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
#[doc = "Field `DISABLE` reader - Software disable SECUREAPPROTECT mechanism"]
pub type DISABLE_R = crate::FieldReader<u8, DISABLE_A>;
#[doc = "Software disable SECUREAPPROTECT mechanism\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DISABLE_A {
    #[doc = "90: Software disable SECUREAPPROTECT mechanism"]
    SW_UNPROTECTED = 90,
}
impl From<DISABLE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_A) -> Self {
        variant as _
    }
}
impl DISABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISABLE_A> {
        match self.bits {
            90 => Some(DISABLE_A::SW_UNPROTECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SW_UNPROTECTED`"]
    #[inline(always)]
    pub fn is_sw_unprotected(&self) -> bool {
        *self == DISABLE_A::SW_UNPROTECTED
    }
}
#[doc = "Field `DISABLE` writer - Software disable SECUREAPPROTECT mechanism"]
pub type DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISABLE_SPEC, u8, DISABLE_A, 8, O>;
impl<'a, const O: u8> DISABLE_W<'a, O> {
    #[doc = "Software disable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub fn sw_unprotected(self) -> &'a mut W {
        self.variant(DISABLE_A::SW_UNPROTECTED)
    }
}
impl R {
    #[doc = "Bits 0:7 - Software disable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software disable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W<0> {
        DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software disable SECUREAPPROTECT mechanism\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disable](index.html) module"]
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
#[doc = "`reset()` method sets DISABLE to value 0x01"]
impl crate::Resettable for DISABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
