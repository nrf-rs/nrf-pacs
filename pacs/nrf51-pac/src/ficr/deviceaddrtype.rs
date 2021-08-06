#[doc = "Register `DEVICEADDRTYPE` reader"]
pub struct R(crate::R<DEVICEADDRTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEADDRTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEADDRTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEADDRTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVICEADDRTYPE` writer"]
pub struct W(crate::W<DEVICEADDRTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICEADDRTYPE_SPEC>;
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
impl From<crate::W<DEVICEADDRTYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICEADDRTYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Device address type.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVICEADDRTYPE_A {
    #[doc = "0: Public address."]
    PUBLIC = 0,
    #[doc = "1: Random address."]
    RANDOM = 1,
}
impl From<DEVICEADDRTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: DEVICEADDRTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVICEADDRTYPE` reader - Device address type."]
pub struct DEVICEADDRTYPE_R(crate::FieldReader<bool, DEVICEADDRTYPE_A>);
impl DEVICEADDRTYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEVICEADDRTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVICEADDRTYPE_A {
        match self.bits {
            false => DEVICEADDRTYPE_A::PUBLIC,
            true => DEVICEADDRTYPE_A::RANDOM,
        }
    }
    #[doc = "Checks if the value of the field is `PUBLIC`"]
    #[inline(always)]
    pub fn is_public(&self) -> bool {
        **self == DEVICEADDRTYPE_A::PUBLIC
    }
    #[doc = "Checks if the value of the field is `RANDOM`"]
    #[inline(always)]
    pub fn is_random(&self) -> bool {
        **self == DEVICEADDRTYPE_A::RANDOM
    }
}
impl core::ops::Deref for DEVICEADDRTYPE_R {
    type Target = crate::FieldReader<bool, DEVICEADDRTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVICEADDRTYPE` writer - Device address type."]
pub struct DEVICEADDRTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICEADDRTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVICEADDRTYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Public address."]
    #[inline(always)]
    pub fn public(self) -> &'a mut W {
        self.variant(DEVICEADDRTYPE_A::PUBLIC)
    }
    #[doc = "Random address."]
    #[inline(always)]
    pub fn random(self) -> &'a mut W {
        self.variant(DEVICEADDRTYPE_A::RANDOM)
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
    #[doc = "Bit 0 - Device address type."]
    #[inline(always)]
    pub fn deviceaddrtype(&self) -> DEVICEADDRTYPE_R {
        DEVICEADDRTYPE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device address type."]
    #[inline(always)]
    pub fn deviceaddrtype(&mut self) -> DEVICEADDRTYPE_W {
        DEVICEADDRTYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device address type.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceaddrtype](index.html) module"]
pub struct DEVICEADDRTYPE_SPEC;
impl crate::RegisterSpec for DEVICEADDRTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deviceaddrtype::R](R) reader structure"]
impl crate::Readable for DEVICEADDRTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deviceaddrtype::W](W) writer structure"]
impl crate::Writable for DEVICEADDRTYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVICEADDRTYPE to value 0xffff_ffff"]
impl crate::Resettable for DEVICEADDRTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
