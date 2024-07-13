#[doc = "Register `HOST_CRYPTOKEY_SEL` reader"]
pub struct R(crate::R<HOST_CRYPTOKEY_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_CRYPTOKEY_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_CRYPTOKEY_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_CRYPTOKEY_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_CRYPTOKEY_SEL` writer"]
pub struct W(crate::W<HOST_CRYPTOKEY_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_CRYPTOKEY_SEL_SPEC>;
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
impl From<crate::W<HOST_CRYPTOKEY_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_CRYPTOKEY_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_CRYPTOKEY_SEL` reader - Select the source of the HW key that is used by the AES engine"]
pub type HOST_CRYPTOKEY_SEL_R = crate::FieldReader<u8, HOST_CRYPTOKEY_SEL_A>;
#[doc = "Select the source of the HW key that is used by the AES engine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HOST_CRYPTOKEY_SEL_A {
    #[doc = "0: Use device root key K_DR from CRYPTOCELL AO power domain"]
    K_DR = 0,
    #[doc = "1: Use hard-coded RTL key K_PRTL"]
    K_PRTL = 1,
    #[doc = "2: Use provided session key"]
    SESSION = 2,
}
impl From<HOST_CRYPTOKEY_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HOST_CRYPTOKEY_SEL_A) -> Self {
        variant as _
    }
}
impl HOST_CRYPTOKEY_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HOST_CRYPTOKEY_SEL_A> {
        match self.bits {
            0 => Some(HOST_CRYPTOKEY_SEL_A::K_DR),
            1 => Some(HOST_CRYPTOKEY_SEL_A::K_PRTL),
            2 => Some(HOST_CRYPTOKEY_SEL_A::SESSION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `K_DR`"]
    #[inline(always)]
    pub fn is_k_dr(&self) -> bool {
        *self == HOST_CRYPTOKEY_SEL_A::K_DR
    }
    #[doc = "Checks if the value of the field is `K_PRTL`"]
    #[inline(always)]
    pub fn is_k_prtl(&self) -> bool {
        *self == HOST_CRYPTOKEY_SEL_A::K_PRTL
    }
    #[doc = "Checks if the value of the field is `SESSION`"]
    #[inline(always)]
    pub fn is_session(&self) -> bool {
        *self == HOST_CRYPTOKEY_SEL_A::SESSION
    }
}
#[doc = "Field `HOST_CRYPTOKEY_SEL` writer - Select the source of the HW key that is used by the AES engine"]
pub type HOST_CRYPTOKEY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_CRYPTOKEY_SEL_SPEC, u8, HOST_CRYPTOKEY_SEL_A, 2, O>;
impl<'a, const O: u8> HOST_CRYPTOKEY_SEL_W<'a, O> {
    #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn k_dr(self) -> &'a mut W {
        self.variant(HOST_CRYPTOKEY_SEL_A::K_DR)
    }
    #[doc = "Use hard-coded RTL key K_PRTL"]
    #[inline(always)]
    pub fn k_prtl(self) -> &'a mut W {
        self.variant(HOST_CRYPTOKEY_SEL_A::K_PRTL)
    }
    #[doc = "Use provided session key"]
    #[inline(always)]
    pub fn session(self) -> &'a mut W {
        self.variant(HOST_CRYPTOKEY_SEL_A::SESSION)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select the source of the HW key that is used by the AES engine"]
    #[inline(always)]
    pub fn host_cryptokey_sel(&self) -> HOST_CRYPTOKEY_SEL_R {
        HOST_CRYPTOKEY_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the source of the HW key that is used by the AES engine"]
    #[inline(always)]
    pub fn host_cryptokey_sel(&mut self) -> HOST_CRYPTOKEY_SEL_W<0> {
        HOST_CRYPTOKEY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES hardware key select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_cryptokey_sel](index.html) module"]
pub struct HOST_CRYPTOKEY_SEL_SPEC;
impl crate::RegisterSpec for HOST_CRYPTOKEY_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_cryptokey_sel::R](R) reader structure"]
impl crate::Readable for HOST_CRYPTOKEY_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_cryptokey_sel::W](W) writer structure"]
impl crate::Writable for HOST_CRYPTOKEY_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_CRYPTOKEY_SEL to value 0"]
impl crate::Resettable for HOST_CRYPTOKEY_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
