#[doc = "Register `FREQUENCY` reader"]
pub struct R(crate::R<FREQUENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQUENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQUENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQUENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQUENCY` writer"]
pub struct W(crate::W<FREQUENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQUENCY_SPEC>;
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
impl From<crate::W<FREQUENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQUENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQUENCY` reader - Radio channel frequency"]
pub struct FREQUENCY_R(crate::FieldReader<u8, u8>);
impl FREQUENCY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FREQUENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQUENCY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQUENCY` writer - Radio channel frequency"]
pub struct FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Channel map selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAP_A {
    #[doc = "0: Channel map between 2400 MHZ .. 2500 MHz"]
    DEFAULT = 0,
    #[doc = "1: Channel map between 2360 MHZ .. 2460 MHz"]
    LOW = 1,
}
impl From<MAP_A> for bool {
    #[inline(always)]
    fn from(variant: MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAP` reader - Channel map selection."]
pub struct MAP_R(crate::FieldReader<bool, MAP_A>);
impl MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAP_A {
        match self.bits {
            false => MAP_A::DEFAULT,
            true => MAP_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == MAP_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == MAP_A::LOW
    }
}
impl core::ops::Deref for MAP_R {
    type Target = crate::FieldReader<bool, MAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAP` writer - Channel map selection."]
pub struct MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(MAP_A::DEFAULT)
    }
    #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(MAP_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline(always)]
    pub fn frequency(&mut self) -> FREQUENCY_W {
        FREQUENCY_W { w: self }
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline(always)]
    pub fn map(&mut self) -> MAP_W {
        MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frequency](index.html) module"]
pub struct FREQUENCY_SPEC;
impl crate::RegisterSpec for FREQUENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frequency::R](R) reader structure"]
impl crate::Readable for FREQUENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frequency::W](W) writer structure"]
impl crate::Writable for FREQUENCY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQUENCY to value 0x02"]
impl crate::Resettable for FREQUENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
