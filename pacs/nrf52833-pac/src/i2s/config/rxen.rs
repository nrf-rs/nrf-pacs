#[doc = "Register `RXEN` reader"]
pub struct R(crate::R<RXEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXEN` writer"]
pub struct W(crate::W<RXEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXEN_SPEC>;
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
impl From<crate::W<RXEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reception (RX) enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEN_A {
    #[doc = "0: Reception disabled and now data will be written to the RXD.PTR address."]
    DISABLED = 0,
    #[doc = "1: Reception enabled."]
    ENABLED = 1,
}
impl From<RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` reader - Reception (RX) enable."]
pub struct RXEN_R(crate::FieldReader<bool, RXEN_A>);
impl RXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::DISABLED,
            true => RXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXEN_A::ENABLED
    }
}
impl core::ops::Deref for RXEN_R {
    type Target = crate::FieldReader<bool, RXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEN` writer - Reception (RX) enable."]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reception disabled and now data will be written to the RXD.PTR address."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXEN_A::DISABLED)
    }
    #[doc = "Reception enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reception (RX) enable."]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception (RX) enable."]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reception (RX) enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxen](index.html) module"]
pub struct RXEN_SPEC;
impl crate::RegisterSpec for RXEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxen::R](R) reader structure"]
impl crate::Readable for RXEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxen::W](W) writer structure"]
impl crate::Writable for RXEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXEN to value 0"]
impl crate::Resettable for RXEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
