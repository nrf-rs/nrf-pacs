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
#[doc = "Configuration of modulation control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODULATIONCTRL_A {
    #[doc = "0: Invalid, defaults to same behaviour as for Internal"]
    INVALID = 0,
    #[doc = "1: Use internal modulator only"]
    INTERNAL = 1,
    #[doc = "2: Output digital modulation signal to a GPIO pin."]
    MODTOGPIO = 2,
    #[doc = "3: Use internal modulator and output digital modulation signal to a GPIO pin."]
    INTERNALANDMODTOGPIO = 3,
}
impl From<MODULATIONCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: MODULATIONCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODULATIONCTRL` reader - Configuration of modulation control."]
pub struct MODULATIONCTRL_R(crate::FieldReader<u8, MODULATIONCTRL_A>);
impl MODULATIONCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODULATIONCTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODULATIONCTRL_A {
        match self.bits {
            0 => MODULATIONCTRL_A::INVALID,
            1 => MODULATIONCTRL_A::INTERNAL,
            2 => MODULATIONCTRL_A::MODTOGPIO,
            3 => MODULATIONCTRL_A::INTERNALANDMODTOGPIO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        **self == MODULATIONCTRL_A::INVALID
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == MODULATIONCTRL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `MODTOGPIO`"]
    #[inline(always)]
    pub fn is_mod_to_gpio(&self) -> bool {
        **self == MODULATIONCTRL_A::MODTOGPIO
    }
    #[doc = "Checks if the value of the field is `INTERNALANDMODTOGPIO`"]
    #[inline(always)]
    pub fn is_internal_and_mod_to_gpio(&self) -> bool {
        **self == MODULATIONCTRL_A::INTERNALANDMODTOGPIO
    }
}
impl core::ops::Deref for MODULATIONCTRL_R {
    type Target = crate::FieldReader<u8, MODULATIONCTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODULATIONCTRL` writer - Configuration of modulation control."]
pub struct MODULATIONCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> MODULATIONCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODULATIONCTRL_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
        self.variant(MODULATIONCTRL_A::MODTOGPIO)
    }
    #[doc = "Use internal modulator and output digital modulation signal to a GPIO pin."]
    #[inline(always)]
    pub fn internal_and_mod_to_gpio(self) -> &'a mut W {
        self.variant(MODULATIONCTRL_A::INTERNALANDMODTOGPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration of modulation control."]
    #[inline(always)]
    pub fn modulationctrl(&self) -> MODULATIONCTRL_R {
        MODULATIONCTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration of modulation control."]
    #[inline(always)]
    pub fn modulationctrl(&mut self) -> MODULATIONCTRL_W {
        MODULATIONCTRL_W { w: self }
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
