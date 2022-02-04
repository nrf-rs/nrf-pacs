#[doc = "Register `SAMPLERATE` reader"]
pub struct R(crate::R<SAMPLERATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLERATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLERATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLERATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLERATE` writer"]
pub struct W(crate::W<SAMPLERATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLERATE_SPEC>;
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
impl From<crate::W<SAMPLERATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLERATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC` reader - Capture and compare value. Sample rate is 16 MHz/CC"]
pub struct CC_R(crate::FieldReader<u16, u16>);
impl CC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC` writer - Capture and compare value. Sample rate is 16 MHz/CC"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Select mode for sample rate control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Rate is controlled from SAMPLE task"]
    TASK = 0,
    #[doc = "1: Rate is controlled from local timer (use CC to control the rate)"]
    TIMERS = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Select mode for sample rate control"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::TASK,
            true => MODE_A::TIMERS,
        }
    }
    #[doc = "Checks if the value of the field is `TASK`"]
    #[inline(always)]
    pub fn is_task(&self) -> bool {
        **self == MODE_A::TASK
    }
    #[doc = "Checks if the value of the field is `TIMERS`"]
    #[inline(always)]
    pub fn is_timers(&self) -> bool {
        **self == MODE_A::TIMERS
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Select mode for sample rate control"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rate is controlled from SAMPLE task"]
    #[inline(always)]
    pub fn task(self) -> &'a mut W {
        self.variant(MODE_A::TASK)
    }
    #[doc = "Rate is controlled from local timer (use CC to control the rate)"]
    #[inline(always)]
    pub fn timers(self) -> &'a mut W {
        self.variant(MODE_A::TIMERS)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Capture and compare value. Sample rate is 16 MHz/CC"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - Select mode for sample rate control"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Capture and compare value. Sample rate is 16 MHz/CC"]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    #[doc = "Bit 12 - Select mode for sample rate control"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls normal or continuous sample rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samplerate](index.html) module"]
pub struct SAMPLERATE_SPEC;
impl crate::RegisterSpec for SAMPLERATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [samplerate::R](R) reader structure"]
impl crate::Readable for SAMPLERATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [samplerate::W](W) writer structure"]
impl crate::Writable for SAMPLERATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPLERATE to value 0"]
impl crate::Resettable for SAMPLERATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
