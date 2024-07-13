#[doc = "Register `APPNVMCPOFGUARD` reader"]
pub struct R(crate::R<APPNVMCPOFGUARD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APPNVMCPOFGUARD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APPNVMCPOFGUARD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APPNVMCPOFGUARD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APPNVMCPOFGUARD` writer"]
pub struct W(crate::W<APPNVMCPOFGUARD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APPNVMCPOFGUARD_SPEC>;
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
impl From<crate::W<APPNVMCPOFGUARD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APPNVMCPOFGUARD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NVMCPOFGUARDEN` reader - Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
pub type NVMCPOFGUARDEN_R = crate::BitReader<NVMCPOFGUARDEN_A>;
#[doc = "Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVMCPOFGUARDEN_A {
    #[doc = "0: NVM WRITE and NVM ERASE are not blocked in POFWARN condition"]
    DISABLED = 0,
    #[doc = "1: NVM WRITE and NVM ERASE are blocked in POFWARN condition"]
    ENABLED = 1,
}
impl From<NVMCPOFGUARDEN_A> for bool {
    #[inline(always)]
    fn from(variant: NVMCPOFGUARDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NVMCPOFGUARDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NVMCPOFGUARDEN_A {
        match self.bits {
            false => NVMCPOFGUARDEN_A::DISABLED,
            true => NVMCPOFGUARDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NVMCPOFGUARDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NVMCPOFGUARDEN_A::ENABLED
    }
}
#[doc = "Field `NVMCPOFGUARDEN` writer - Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
pub type NVMCPOFGUARDEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APPNVMCPOFGUARD_SPEC, NVMCPOFGUARDEN_A, O>;
impl<'a, const O: u8> NVMCPOFGUARDEN_W<'a, O> {
    #[doc = "NVM WRITE and NVM ERASE are not blocked in POFWARN condition"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NVMCPOFGUARDEN_A::DISABLED)
    }
    #[doc = "NVM WRITE and NVM ERASE are blocked in POFWARN condition"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NVMCPOFGUARDEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
    #[inline(always)]
    pub fn nvmcpofguarden(&self) -> NVMCPOFGUARDEN_R {
        NVMCPOFGUARDEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
    #[inline(always)]
    pub fn nvmcpofguarden(&mut self) -> NVMCPOFGUARDEN_W<0> {
        NVMCPOFGUARDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable blocking NVM WRITE and aborting NVM ERASE for Application NVM in POFWARN condition .\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [appnvmcpofguard](index.html) module"]
pub struct APPNVMCPOFGUARD_SPEC;
impl crate::RegisterSpec for APPNVMCPOFGUARD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [appnvmcpofguard::R](R) reader structure"]
impl crate::Readable for APPNVMCPOFGUARD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [appnvmcpofguard::W](W) writer structure"]
impl crate::Writable for APPNVMCPOFGUARD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APPNVMCPOFGUARD to value 0xffff_ffff"]
impl crate::Resettable for APPNVMCPOFGUARD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
