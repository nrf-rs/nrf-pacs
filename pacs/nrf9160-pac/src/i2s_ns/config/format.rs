#[doc = "Register `FORMAT` reader"]
pub struct R(crate::R<FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FORMAT` writer"]
pub struct W(crate::W<FORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORMAT_SPEC>;
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
impl From<crate::W<FORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frame format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMAT_A {
    #[doc = "0: Original I2S format."]
    I2S = 0,
    #[doc = "1: Alternate (left- or right-aligned) format."]
    ALIGNED = 1,
}
impl From<FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORMAT` reader - Frame format."]
pub struct FORMAT_R(crate::FieldReader<bool, FORMAT_A>);
impl FORMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            false => FORMAT_A::I2S,
            true => FORMAT_A::ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        **self == FORMAT_A::I2S
    }
    #[doc = "Checks if the value of the field is `ALIGNED`"]
    #[inline(always)]
    pub fn is_aligned(&self) -> bool {
        **self == FORMAT_A::ALIGNED
    }
}
impl core::ops::Deref for FORMAT_R {
    type Target = crate::FieldReader<bool, FORMAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORMAT` writer - Frame format."]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORMAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Original I2S format."]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(FORMAT_A::I2S)
    }
    #[doc = "Alternate (left- or right-aligned) format."]
    #[inline(always)]
    pub fn aligned(self) -> &'a mut W {
        self.variant(FORMAT_A::ALIGNED)
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
    #[doc = "Bit 0 - Frame format."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame format."]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame format.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [format](index.html) module"]
pub struct FORMAT_SPEC;
impl crate::RegisterSpec for FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [format::R](R) reader structure"]
impl crate::Readable for FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [format::W](W) writer structure"]
impl crate::Writable for FORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FORMAT to value 0"]
impl crate::Resettable for FORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
