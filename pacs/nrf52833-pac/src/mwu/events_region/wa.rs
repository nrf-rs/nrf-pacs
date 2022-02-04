#[doc = "Register `WA` reader"]
pub struct R(crate::R<WA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WA` writer"]
pub struct W(crate::W<WA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WA_SPEC>;
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
impl From<crate::W<WA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write access to region n detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WA_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<WA_A> for bool {
    #[inline(always)]
    fn from(variant: WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WA` reader - Write access to region n detected"]
pub struct WA_R(crate::FieldReader<bool, WA_A>);
impl WA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WA_A {
        match self.bits {
            false => WA_A::NOTGENERATED,
            true => WA_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        **self == WA_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        **self == WA_A::GENERATED
    }
}
impl core::ops::Deref for WA_R {
    type Target = crate::FieldReader<bool, WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WA` writer - Write access to region n detected"]
pub struct WA_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(WA_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(WA_A::GENERATED)
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
    #[doc = "Bit 0 - Write access to region n detected"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write access to region n detected"]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W {
        WA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Write access to region n detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wa](index.html) module"]
pub struct WA_SPEC;
impl crate::RegisterSpec for WA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wa::R](R) reader structure"]
impl crate::Readable for WA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wa::W](W) writer structure"]
impl crate::Writable for WA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WA to value 0"]
impl crate::Resettable for WA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
