#[doc = "Register `HOST_IOT_KPRTL_LOCK` reader"]
pub struct R(crate::R<HOST_IOT_KPRTL_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_IOT_KPRTL_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_IOT_KPRTL_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_IOT_KPRTL_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_IOT_KPRTL_LOCK` writer"]
pub struct W(crate::W<HOST_IOT_KPRTL_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_IOT_KPRTL_LOCK_SPEC>;
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
impl From<crate::W<HOST_IOT_KPRTL_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_IOT_KPRTL_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_IOT_KPRTL_LOCK` reader - This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub type HOST_IOT_KPRTL_LOCK_R = crate::BitReader<HOST_IOT_KPRTL_LOCK_A>;
#[doc = "This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_IOT_KPRTL_LOCK_A {
    #[doc = "0: K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    DISABLED = 0,
    #[doc = "1: K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    ENABLED = 1,
}
impl From<HOST_IOT_KPRTL_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_IOT_KPRTL_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl HOST_IOT_KPRTL_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_IOT_KPRTL_LOCK_A {
        match self.bits {
            false => HOST_IOT_KPRTL_LOCK_A::DISABLED,
            true => HOST_IOT_KPRTL_LOCK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HOST_IOT_KPRTL_LOCK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HOST_IOT_KPRTL_LOCK_A::ENABLED
    }
}
#[doc = "Field `HOST_IOT_KPRTL_LOCK` writer - This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub type HOST_IOT_KPRTL_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HOST_IOT_KPRTL_LOCK_SPEC, HOST_IOT_KPRTL_LOCK_A, O>;
impl<'a, const O: u8> HOST_IOT_KPRTL_LOCK_W<'a, O> {
    #[doc = "K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HOST_IOT_KPRTL_LOCK_A::DISABLED)
    }
    #[doc = "K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HOST_IOT_KPRTL_LOCK_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kprtl_lock(&self) -> HOST_IOT_KPRTL_LOCK_R {
        HOST_IOT_KPRTL_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kprtl_lock(&mut self) -> HOST_IOT_KPRTL_LOCK_W<0> {
        HOST_IOT_KPRTL_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_iot_kprtl_lock](index.html) module"]
pub struct HOST_IOT_KPRTL_LOCK_SPEC;
impl crate::RegisterSpec for HOST_IOT_KPRTL_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_iot_kprtl_lock::R](R) reader structure"]
impl crate::Readable for HOST_IOT_KPRTL_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_iot_kprtl_lock::W](W) writer structure"]
impl crate::Writable for HOST_IOT_KPRTL_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_IOT_KPRTL_LOCK to value 0"]
impl crate::Resettable for HOST_IOT_KPRTL_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
