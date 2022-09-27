#[doc = "Register `MODULATIONCTRL` reader"]
pub struct R(crate::R<MODULATIONCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULATIONCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULATIONCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULATIONCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODULATIONCTRL` writer"]
pub struct W(crate::W<MODULATIONCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODULATIONCTRL_SPEC>;
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
impl From<crate::W<MODULATIONCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODULATIONCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODULATIONCTRL` reader - Configuration of modulation control."]
pub type MODULATIONCTRL_R = crate::FieldReader<u8, MODULATIONCTRL_A>;
#[doc = "Configuration of modulation control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODULATIONCTRL_A {
    #[doc = "0: Invalid, defaults to same behaviour as for Internal"]
    INVALID = 0,
    #[doc = "1: Use internal modulator only"]
    INTERNAL = 1,
    #[doc = "2: Output digital modulation signal to a GPIO pin."]
    MOD_TO_GPIO = 2,
    #[doc = "3: Use internal modulator and output digital modulation signal to a GPIO pin."]
    INTERNAL_AND_MOD_TO_GPIO = 3,
}
impl From<MODULATIONCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: MODULATIONCTRL_A) -> Self {
        variant as _
    }
}
impl MODULATIONCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODULATIONCTRL_A {
        match self.bits {
            0 => MODULATIONCTRL_A::INVALID,
            1 => MODULATIONCTRL_A::INTERNAL,
            2 => MODULATIONCTRL_A::MOD_TO_GPIO,
            3 => MODULATIONCTRL_A::INTERNAL_AND_MOD_TO_GPIO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == MODULATIONCTRL_A::INVALID
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == MODULATIONCTRL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `MOD_TO_GPIO`"]
    #[inline(always)]
    pub fn is_mod_to_gpio(&self) -> bool {
        *self == MODULATIONCTRL_A::MOD_TO_GPIO
    }
    #[doc = "Checks if the value of the field is `INTERNAL_AND_MOD_TO_GPIO`"]
    #[inline(always)]
    pub fn is_internal_and_mod_to_gpio(&self) -> bool {
        *self == MODULATIONCTRL_A::INTERNAL_AND_MOD_TO_GPIO
    }
}
#[doc = "Field `MODULATIONCTRL` writer - Configuration of modulation control."]
pub type MODULATIONCTRL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MODULATIONCTRL_SPEC, u8, MODULATIONCTRL_A, 2, O>;
impl<'a, const O: u8> MODULATIONCTRL_W<'a, O> {
    #[doc = "Invalid, defaults to same behaviour as for Internal"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(MODULATIONCTRL_A::INVALID)
    }
    #[doc = "Use internal modulator only"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(MODULATIONCTRL_A::INTERNAL)
    }
    #[doc = "Output digital modulation signal to a GPIO pin."]
    #[inline(always)]
    pub fn mod_to_gpio(self) -> &'a mut W {
        self.variant(MODULATIONCTRL_A::MOD_TO_GPIO)
    }
    #[doc = "Use internal modulator and output digital modulation signal to a GPIO pin."]
    #[inline(always)]
    pub fn internal_and_mod_to_gpio(self) -> &'a mut W {
        self.variant(MODULATIONCTRL_A::INTERNAL_AND_MOD_TO_GPIO)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration of modulation control."]
    #[inline(always)]
    pub fn modulationctrl(&self) -> MODULATIONCTRL_R {
        MODULATIONCTRL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration of modulation control."]
    #[inline(always)]
    pub fn modulationctrl(&mut self) -> MODULATIONCTRL_W<0> {
        MODULATIONCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enables the modulation output to a GPIO pin which can be connected to a second external antenna.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modulationctrl](index.html) module"]
pub struct MODULATIONCTRL_SPEC;
impl crate::RegisterSpec for MODULATIONCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modulationctrl::R](R) reader structure"]
impl crate::Readable for MODULATIONCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modulationctrl::W](W) writer structure"]
impl crate::Writable for MODULATIONCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODULATIONCTRL to value 0x01"]
impl crate::Resettable for MODULATIONCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
