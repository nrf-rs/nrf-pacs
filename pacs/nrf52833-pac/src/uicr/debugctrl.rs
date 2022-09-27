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
#[doc = "Field `CPUNIDEN` reader - Configure CPU non-intrusive debug features"]
pub type CPUNIDEN_R = crate::FieldReader<u8, CPUNIDEN_A>;
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
impl CPUNIDEN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CPUNIDEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPUNIDEN_A::DISABLED
    }
}
#[doc = "Field `CPUNIDEN` writer - Configure CPU non-intrusive debug features"]
pub type CPUNIDEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUGCTRL_SPEC, u8, CPUNIDEN_A, 8, O>;
impl<'a, const O: u8> CPUNIDEN_W<'a, O> {
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
}
#[doc = "Field `CPUFPBEN` reader - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
pub type CPUFPBEN_R = crate::FieldReader<u8, CPUFPBEN_A>;
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
impl CPUFPBEN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CPUFPBEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPUFPBEN_A::DISABLED
    }
}
#[doc = "Field `CPUFPBEN` writer - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
pub type CPUFPBEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUGCTRL_SPEC, u8, CPUFPBEN_A, 8, O>;
impl<'a, const O: u8> CPUFPBEN_W<'a, O> {
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
    pub fn cpuniden(&mut self) -> CPUNIDEN_W<0> {
        CPUNIDEN_W::new(self)
    }
    #[doc = "Bits 8:15 - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline(always)]
    pub fn cpufpben(&mut self) -> CPUFPBEN_W<8> {
        CPUFPBEN_W::new(self)
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
