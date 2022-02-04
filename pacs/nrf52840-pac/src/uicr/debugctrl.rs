#[doc = "Register `DEBUGCTRL` reader"]
pub struct R(crate::R<DEBUGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUGCTRL` writer"]
pub struct W(crate::W<DEBUGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUGCTRL_SPEC>;
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
impl From<crate::W<DEBUGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configure CPU non-intrusive debug features\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPUNIDEN_A {
    #[doc = "255: Enable CPU ITM and ETM functionality (default behavior)"]
    ENABLED = 255,
    #[doc = "0: Disable CPU ITM and ETM functionality"]
    DISABLED = 0,
}
impl From<CPUNIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPUNIDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPUNIDEN` reader - Configure CPU non-intrusive debug features"]
pub struct CPUNIDEN_R(crate::FieldReader<u8, CPUNIDEN_A>);
impl CPUNIDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPUNIDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPUNIDEN_A> {
        match self.bits {
            255 => Some(CPUNIDEN_A::ENABLED),
            0 => Some(CPUNIDEN_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CPUNIDEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CPUNIDEN_A::DISABLED
    }
}
impl core::ops::Deref for CPUNIDEN_R {
    type Target = crate::FieldReader<u8, CPUNIDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUNIDEN` writer - Configure CPU non-intrusive debug features"]
pub struct CPUNIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUNIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUNIDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable CPU ITM and ETM functionality (default behavior)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPUNIDEN_A::ENABLED)
    }
    #[doc = "Disable CPU ITM and ETM functionality"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPUNIDEN_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Configure CPU flash patch and breakpoint (FPB) unit behavior\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPUFPBEN_A {
    #[doc = "255: Enable CPU FPB unit (default behavior)"]
    ENABLED = 255,
    #[doc = "0: Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    DISABLED = 0,
}
impl From<CPUFPBEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPUFPBEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPUFPBEN` reader - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
pub struct CPUFPBEN_R(crate::FieldReader<u8, CPUFPBEN_A>);
impl CPUFPBEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPUFPBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPUFPBEN_A> {
        match self.bits {
            255 => Some(CPUFPBEN_A::ENABLED),
            0 => Some(CPUFPBEN_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CPUFPBEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CPUFPBEN_A::DISABLED
    }
}
impl core::ops::Deref for CPUFPBEN_R {
    type Target = crate::FieldReader<u8, CPUFPBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUFPBEN` writer - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
pub struct CPUFPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUFPBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUFPBEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable CPU FPB unit (default behavior)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPUFPBEN_A::ENABLED)
    }
    #[doc = "Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPUFPBEN_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Configure CPU non-intrusive debug features"]
    #[inline(always)]
    pub fn cpuniden(&self) -> CPUNIDEN_R {
        CPUNIDEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline(always)]
    pub fn cpufpben(&self) -> CPUFPBEN_R {
        CPUFPBEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configure CPU non-intrusive debug features"]
    #[inline(always)]
    pub fn cpuniden(&mut self) -> CPUNIDEN_W {
        CPUNIDEN_W { w: self }
    }
    #[doc = "Bits 8:15 - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline(always)]
    pub fn cpufpben(&mut self) -> CPUFPBEN_W {
        CPUFPBEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Processor debug control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debugctrl](index.html) module"]
pub struct DEBUGCTRL_SPEC;
impl crate::RegisterSpec for DEBUGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debugctrl::R](R) reader structure"]
impl crate::Readable for DEBUGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debugctrl::W](W) writer structure"]
impl crate::Writable for DEBUGCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUGCTRL to value 0xffff_ffff"]
impl crate::Resettable for DEBUGCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
