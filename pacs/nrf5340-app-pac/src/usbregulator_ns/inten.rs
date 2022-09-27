#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBDETECTED` reader - Enable or disable interrupt for event USBDETECTED"]
pub type USBDETECTED_R = crate::BitReader<USBDETECTED_A>;
#[doc = "Enable or disable interrupt for event USBDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDETECTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USBDETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: USBDETECTED_A) -> Self {
        variant as u8 != 0
    }
}
impl USBDETECTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBDETECTED_A {
        match self.bits {
            false => USBDETECTED_A::DISABLED,
            true => USBDETECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBDETECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBDETECTED_A::ENABLED
    }
}
#[doc = "Field `USBDETECTED` writer - Enable or disable interrupt for event USBDETECTED"]
pub type USBDETECTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, USBDETECTED_A, O>;
impl<'a, const O: u8> USBDETECTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBDETECTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBDETECTED_A::ENABLED)
    }
}
#[doc = "Field `USBREMOVED` reader - Enable or disable interrupt for event USBREMOVED"]
pub type USBREMOVED_R = crate::BitReader<USBREMOVED_A>;
#[doc = "Enable or disable interrupt for event USBREMOVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBREMOVED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USBREMOVED_A> for bool {
    #[inline(always)]
    fn from(variant: USBREMOVED_A) -> Self {
        variant as u8 != 0
    }
}
impl USBREMOVED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBREMOVED_A {
        match self.bits {
            false => USBREMOVED_A::DISABLED,
            true => USBREMOVED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBREMOVED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBREMOVED_A::ENABLED
    }
}
#[doc = "Field `USBREMOVED` writer - Enable or disable interrupt for event USBREMOVED"]
pub type USBREMOVED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, USBREMOVED_A, O>;
impl<'a, const O: u8> USBREMOVED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBREMOVED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBREMOVED_A::ENABLED)
    }
}
#[doc = "Field `USBPWRRDY` reader - Enable or disable interrupt for event USBPWRRDY"]
pub type USBPWRRDY_R = crate::BitReader<USBPWRRDY_A>;
#[doc = "Enable or disable interrupt for event USBPWRRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPWRRDY_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USBPWRRDY_A> for bool {
    #[inline(always)]
    fn from(variant: USBPWRRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl USBPWRRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPWRRDY_A {
        match self.bits {
            false => USBPWRRDY_A::DISABLED,
            true => USBPWRRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBPWRRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBPWRRDY_A::ENABLED
    }
}
#[doc = "Field `USBPWRRDY` writer - Enable or disable interrupt for event USBPWRRDY"]
pub type USBPWRRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, USBPWRRDY_A, O>;
impl<'a, const O: u8> USBPWRRDY_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBPWRRDY_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBPWRRDY_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&self) -> USBDETECTED_R {
        USBDETECTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&self) -> USBREMOVED_R {
        USBREMOVED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn usbpwrrdy(&self) -> USBPWRRDY_R {
        USBPWRRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&mut self) -> USBDETECTED_W<0> {
        USBDETECTED_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&mut self) -> USBREMOVED_W<1> {
        USBREMOVED_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn usbpwrrdy(&mut self) -> USBPWRRDY_W<2> {
        USBPWRRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
