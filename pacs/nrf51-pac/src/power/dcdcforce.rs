#[doc = "Register `DCDCFORCE` reader"]
pub struct R(crate::R<DCDCFORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCFORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCFORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCFORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCFORCE` writer"]
pub struct W(crate::W<DCDCFORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCFORCE_SPEC>;
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
impl From<crate::W<DCDCFORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCFORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCEOFF` reader - DCDC power-up force off."]
pub type FORCEOFF_R = crate::BitReader<FORCEOFF_A>;
#[doc = "DCDC power-up force off.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFF_A {
    #[doc = "0: No force."]
    NO_FORCE = 0,
    #[doc = "1: Force."]
    FORCE = 1,
}
impl From<FORCEOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCEOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEOFF_A {
        match self.bits {
            false => FORCEOFF_A::NO_FORCE,
            true => FORCEOFF_A::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FORCE`"]
    #[inline(always)]
    pub fn is_no_force(&self) -> bool {
        *self == FORCEOFF_A::NO_FORCE
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == FORCEOFF_A::FORCE
    }
}
#[doc = "Field `FORCEOFF` writer - DCDC power-up force off."]
pub type FORCEOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCFORCE_SPEC, FORCEOFF_A, O>;
impl<'a, const O: u8> FORCEOFF_W<'a, O> {
    #[doc = "No force."]
    #[inline(always)]
    pub fn no_force(self) -> &'a mut W {
        self.variant(FORCEOFF_A::NO_FORCE)
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEOFF_A::FORCE)
    }
}
#[doc = "Field `FORCEON` reader - DCDC power-up force on."]
pub type FORCEON_R = crate::BitReader<FORCEON_A>;
#[doc = "DCDC power-up force on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEON_A {
    #[doc = "0: No force."]
    NO_FORCE = 0,
    #[doc = "1: Force."]
    FORCE = 1,
}
impl From<FORCEON_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEON_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEON_A {
        match self.bits {
            false => FORCEON_A::NO_FORCE,
            true => FORCEON_A::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FORCE`"]
    #[inline(always)]
    pub fn is_no_force(&self) -> bool {
        *self == FORCEON_A::NO_FORCE
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == FORCEON_A::FORCE
    }
}
#[doc = "Field `FORCEON` writer - DCDC power-up force on."]
pub type FORCEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCFORCE_SPEC, FORCEON_A, O>;
impl<'a, const O: u8> FORCEON_W<'a, O> {
    #[doc = "No force."]
    #[inline(always)]
    pub fn no_force(self) -> &'a mut W {
        self.variant(FORCEON_A::NO_FORCE)
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEON_A::FORCE)
    }
}
impl R {
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline(always)]
    pub fn forceoff(&self) -> FORCEOFF_R {
        FORCEOFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline(always)]
    pub fn forceon(&self) -> FORCEON_R {
        FORCEON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline(always)]
    pub fn forceoff(&mut self) -> FORCEOFF_W<0> {
        FORCEOFF_W::new(self)
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline(always)]
    pub fn forceon(&mut self) -> FORCEON_W<1> {
        FORCEON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC power-up force register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcforce](index.html) module"]
pub struct DCDCFORCE_SPEC;
impl crate::RegisterSpec for DCDCFORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdcforce::R](R) reader structure"]
impl crate::Readable for DCDCFORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdcforce::W](W) writer structure"]
impl crate::Writable for DCDCFORCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDCFORCE to value 0"]
impl crate::Resettable for DCDCFORCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
