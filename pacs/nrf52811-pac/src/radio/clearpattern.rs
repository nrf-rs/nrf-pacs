#[doc = "Register `CLEARPATTERN` reader"]
pub struct R(crate::R<CLEARPATTERN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLEARPATTERN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLEARPATTERN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLEARPATTERN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLEARPATTERN` writer"]
pub struct W(crate::W<CLEARPATTERN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEARPATTERN_SPEC>;
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
impl From<crate::W<CLEARPATTERN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEARPATTERN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEARPATTERN` reader - Clears GPIO pattern array for antenna control"]
pub type CLEARPATTERN_R = crate::BitReader<CLEARPATTERN_A>;
#[doc = "Clears GPIO pattern array for antenna control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEARPATTERN_A {
    #[doc = "1: Clear the GPIO pattern"]
    CLEAR = 1,
}
impl From<CLEARPATTERN_A> for bool {
    #[inline(always)]
    fn from(variant: CLEARPATTERN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLEARPATTERN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLEARPATTERN_A> {
        match self.bits {
            true => Some(CLEARPATTERN_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLEARPATTERN_A::CLEAR
    }
}
#[doc = "Field `CLEARPATTERN` writer - Clears GPIO pattern array for antenna control"]
pub type CLEARPATTERN_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, CLEARPATTERN_SPEC, CLEARPATTERN_A, O>;
impl<'a, const O: u8> CLEARPATTERN_W<'a, O> {
    #[doc = "Clear the GPIO pattern"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEARPATTERN_A::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub fn clearpattern(&self) -> CLEARPATTERN_R {
        CLEARPATTERN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub fn clearpattern(&mut self) -> CLEARPATTERN_W<0> {
        CLEARPATTERN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear the GPIO pattern array for antenna control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearpattern](index.html) module"]
pub struct CLEARPATTERN_SPEC;
impl crate::RegisterSpec for CLEARPATTERN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clearpattern::R](R) reader structure"]
impl crate::Readable for CLEARPATTERN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clearpattern::W](W) writer structure"]
impl crate::Writable for CLEARPATTERN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLEARPATTERN to value 0"]
impl crate::Resettable for CLEARPATTERN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
