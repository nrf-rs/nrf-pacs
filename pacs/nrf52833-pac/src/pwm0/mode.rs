#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects up mode or up-and-down mode for the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDOWN_A {
    #[doc = "0: Up counter, edge-aligned PWM duty cycle"]
    UP = 0,
    #[doc = "1: Up and down counter, center-aligned PWM duty cycle"]
    UPANDDOWN = 1,
}
impl From<UPDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: UPDOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDOWN` reader - Selects up mode or up-and-down mode for the counter"]
pub struct UPDOWN_R(crate::FieldReader<bool, UPDOWN_A>);
impl UPDOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPDOWN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDOWN_A {
        match self.bits {
            false => UPDOWN_A::UP,
            true => UPDOWN_A::UPANDDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        **self == UPDOWN_A::UP
    }
    #[doc = "Checks if the value of the field is `UPANDDOWN`"]
    #[inline(always)]
    pub fn is_up_and_down(&self) -> bool {
        **self == UPDOWN_A::UPANDDOWN
    }
}
impl core::ops::Deref for UPDOWN_R {
    type Target = crate::FieldReader<bool, UPDOWN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDOWN` writer - Selects up mode or up-and-down mode for the counter"]
pub struct UPDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDOWN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Up counter, edge-aligned PWM duty cycle"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(UPDOWN_A::UP)
    }
    #[doc = "Up and down counter, center-aligned PWM duty cycle"]
    #[inline(always)]
    pub fn up_and_down(self) -> &'a mut W {
        self.variant(UPDOWN_A::UPANDDOWN)
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
    #[doc = "Bit 0 - Selects up mode or up-and-down mode for the counter"]
    #[inline(always)]
    pub fn updown(&self) -> UPDOWN_R {
        UPDOWN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects up mode or up-and-down mode for the counter"]
    #[inline(always)]
    pub fn updown(&mut self) -> UPDOWN_W {
        UPDOWN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects operating mode of the wave counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
