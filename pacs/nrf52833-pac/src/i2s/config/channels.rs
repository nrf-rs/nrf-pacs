#[doc = "Register `CHANNELS` reader"]
pub struct R(crate::R<CHANNELS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNELS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNELS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNELS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNELS` writer"]
pub struct W(crate::W<CHANNELS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNELS_SPEC>;
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
impl From<crate::W<CHANNELS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNELS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable channels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHANNELS_A {
    #[doc = "0: Stereo."]
    STEREO = 0,
    #[doc = "1: Left only."]
    LEFT = 1,
    #[doc = "2: Right only."]
    RIGHT = 2,
}
impl From<CHANNELS_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNELS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHANNELS` reader - Enable channels."]
pub struct CHANNELS_R(crate::FieldReader<u8, CHANNELS_A>);
impl CHANNELS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHANNELS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHANNELS_A> {
        match self.bits {
            0 => Some(CHANNELS_A::STEREO),
            1 => Some(CHANNELS_A::LEFT),
            2 => Some(CHANNELS_A::RIGHT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        **self == CHANNELS_A::STEREO
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        **self == CHANNELS_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        **self == CHANNELS_A::RIGHT
    }
}
impl core::ops::Deref for CHANNELS_R {
    type Target = crate::FieldReader<u8, CHANNELS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNELS` writer - Enable channels."]
pub struct CHANNELS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNELS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNELS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stereo."]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(CHANNELS_A::STEREO)
    }
    #[doc = "Left only."]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(CHANNELS_A::LEFT)
    }
    #[doc = "Right only."]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(CHANNELS_A::RIGHT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable channels."]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable channels."]
    #[inline(always)]
    pub fn channels(&mut self) -> CHANNELS_W {
        CHANNELS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channels](index.html) module"]
pub struct CHANNELS_SPEC;
impl crate::RegisterSpec for CHANNELS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channels::R](R) reader structure"]
impl crate::Readable for CHANNELS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channels::W](W) writer structure"]
impl crate::Writable for CHANNELS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHANNELS to value 0"]
impl crate::Resettable for CHANNELS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
