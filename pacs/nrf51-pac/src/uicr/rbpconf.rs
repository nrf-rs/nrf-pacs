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
#[doc = "Field `PR0` reader - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
pub struct PR0_R(crate::FieldReader<u8, PR0_A>);
impl PR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PR0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PR0_A::ENABLED
    }
}
impl core::ops::Deref for PR0_R {
    type Target = crate::FieldReader<u8, PR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR0` writer - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
pub struct PR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
#[doc = "Field `PALL` reader - Readback protect all code in the device."]
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
#[doc = "Field `PALL` writer - Readback protect all code in the device."]
pub struct PALL_W<'a> {
    w: &'a mut W,
}
impl<'a> PALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PALL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
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
    pub fn pr0(&mut self) -> PR0_W {
        PR0_W { w: self }
    }
    #[doc = "Bits 8:15 - Readback protect all code in the device."]
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
