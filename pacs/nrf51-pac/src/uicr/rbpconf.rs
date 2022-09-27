#[doc = "Register `RBPCONF` reader"]
pub struct R(crate::R<RBPCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBPCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBPCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBPCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBPCONF` writer"]
pub struct W(crate::W<RBPCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBPCONF_SPEC>;
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
impl From<crate::W<RBPCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBPCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR0` reader - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
pub type PR0_R = crate::FieldReader<u8, PR0_A>;
#[doc = "Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PR0_A {
    #[doc = "255: Disabled."]
    DISABLED = 255,
    #[doc = "0: Enabled."]
    ENABLED = 0,
}
impl From<PR0_A> for u8 {
    #[inline(always)]
    fn from(variant: PR0_A) -> Self {
        variant as _
    }
}
impl PR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PR0_A> {
        match self.bits {
            255 => Some(PR0_A::DISABLED),
            0 => Some(PR0_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PR0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PR0_A::ENABLED
    }
}
#[doc = "Field `PR0` writer - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
pub type PR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBPCONF_SPEC, u8, PR0_A, 8, O>;
impl<'a, const O: u8> PR0_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PR0_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PR0_A::ENABLED)
    }
}
#[doc = "Field `PALL` reader - Readback protect all code in the device."]
pub type PALL_R = crate::FieldReader<u8, PALL_A>;
#[doc = "Readback protect all code in the device.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PALL_A {
    #[doc = "255: Disabled."]
    DISABLED = 255,
    #[doc = "0: Enabled."]
    ENABLED = 0,
}
impl From<PALL_A> for u8 {
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
            255 => Some(PALL_A::DISABLED),
            0 => Some(PALL_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PALL_A::ENABLED
    }
}
#[doc = "Field `PALL` writer - Readback protect all code in the device."]
pub type PALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBPCONF_SPEC, u8, PALL_A, 8, O>;
impl<'a, const O: u8> PALL_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PALL_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PALL_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:7 - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Readback protect all code in the device."]
    #[inline(always)]
    pub fn pall(&self) -> PALL_R {
        PALL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W<0> {
        PR0_W::new(self)
    }
    #[doc = "Bits 8:15 - Readback protect all code in the device."]
    #[inline(always)]
    pub fn pall(&mut self) -> PALL_W<8> {
        PALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Readback protection configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbpconf](index.html) module"]
pub struct RBPCONF_SPEC;
impl crate::RegisterSpec for RBPCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbpconf::R](R) reader structure"]
impl crate::Readable for RBPCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbpconf::W](W) writer structure"]
impl crate::Writable for RBPCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RBPCONF to value 0xffff_ffff"]
impl crate::Resettable for RBPCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
