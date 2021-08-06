#[doc = "Register `WAY[%s]` reader"]
pub struct R(crate::R<WAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAY[%s]` writer"]
pub struct W(crate::W<WAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAY_SPEC>;
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
impl From<crate::W<WAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAG` reader - Cache tag."]
pub struct TAG_R(crate::FieldReader<u32, u32>);
impl TAG_R {
    pub(crate) fn new(bits: u32) -> Self {
        TAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAG` writer - Cache tag."]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
#[doc = "Valid bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
    #[doc = "0: Invalid cache line"]
    INVALID = 0,
    #[doc = "1: Valid cache line"]
    VALID = 1,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V` reader - Valid bit"]
pub struct V_R(crate::FieldReader<bool, V_A>);
impl V_R {
    pub(crate) fn new(bits: bool) -> Self {
        V_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::INVALID,
            true => V_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        **self == V_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == V_A::VALID
    }
}
impl core::ops::Deref for V_R {
    type Target = crate::FieldReader<bool, V_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Most recently used way.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRU_A {
    #[doc = "0: Way0 was most recently used"]
    WAY0 = 0,
    #[doc = "1: Way1 was most recently used"]
    WAY1 = 1,
}
impl From<MRU_A> for bool {
    #[inline(always)]
    fn from(variant: MRU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRU` reader - Most recently used way."]
pub struct MRU_R(crate::FieldReader<bool, MRU_A>);
impl MRU_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRU_A {
        match self.bits {
            false => MRU_A::WAY0,
            true => MRU_A::WAY1,
        }
    }
    #[doc = "Checks if the value of the field is `WAY0`"]
    #[inline(always)]
    pub fn is_way0(&self) -> bool {
        **self == MRU_A::WAY0
    }
    #[doc = "Checks if the value of the field is `WAY1`"]
    #[inline(always)]
    pub fn is_way1(&self) -> bool {
        **self == MRU_A::WAY1
    }
}
impl core::ops::Deref for MRU_R {
    type Target = crate::FieldReader<bool, MRU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:16 - Cache tag."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 30 - Valid bit"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Most recently used way."]
    #[inline(always)]
    pub fn mru(&self) -> MRU_R {
        MRU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Cache tag."]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Cache information for SET\\[n\\], WAY\\[o\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [way](index.html) module"]
pub struct WAY_SPEC;
impl crate::RegisterSpec for WAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [way::R](R) reader structure"]
impl crate::Readable for WAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [way::W](W) writer structure"]
impl crate::Writable for WAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAY[%s]
to value 0"]
impl crate::Resettable for WAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
