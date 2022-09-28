#[doc = "Register `CTIAPPSET` reader"]
pub struct R(crate::R<CTIAPPSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIAPPSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIAPPSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIAPPSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTIAPPSET` writer"]
pub struct W(crate::W<CTIAPPSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIAPPSET_SPEC>;
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
impl From<crate::W<CTIAPPSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIAPPSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPSET_0` reader - Application trigger event for channel 0."]
pub type APPSET_0_R = crate::BitReader<APPSET_0_A>;
#[doc = "Application trigger event for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_0_A {
    #[doc = "0: Application trigger 0 is inactive."]
    INACTIVE = 0,
    #[doc = "1: Application trigger 0 is active."]
    ACTIVE = 1,
}
impl From<APPSET_0_A> for bool {
    #[inline(always)]
    fn from(variant: APPSET_0_A) -> Self {
        variant as u8 != 0
    }
}
impl APPSET_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APPSET_0_A {
        match self.bits {
            false => APPSET_0_A::INACTIVE,
            true => APPSET_0_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == APPSET_0_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == APPSET_0_A::ACTIVE
    }
}
#[doc = "Application trigger event for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_0_AW {
    #[doc = "1: Generate channel event for channel 0."]
    ACTIVATE = 1,
}
impl From<APPSET_0_AW> for bool {
    #[inline(always)]
    fn from(variant: APPSET_0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_0` writer - Application trigger event for channel 0."]
pub type APPSET_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIAPPSET_SPEC, APPSET_0_AW, O>;
impl<'a, const O: u8> APPSET_0_W<'a, O> {
    #[doc = "Generate channel event for channel 0."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(APPSET_0_AW::ACTIVATE)
    }
}
#[doc = "Field `APPSET_1` reader - Application trigger event for channel 1."]
pub type APPSET_1_R = crate::BitReader<APPSET_1_A>;
#[doc = "Application trigger event for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_1_A {
    #[doc = "0: Application trigger 1 is inactive."]
    INACTIVE = 0,
    #[doc = "1: Application trigger 1 is active."]
    ACTIVE = 1,
}
impl From<APPSET_1_A> for bool {
    #[inline(always)]
    fn from(variant: APPSET_1_A) -> Self {
        variant as u8 != 0
    }
}
impl APPSET_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APPSET_1_A {
        match self.bits {
            false => APPSET_1_A::INACTIVE,
            true => APPSET_1_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == APPSET_1_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == APPSET_1_A::ACTIVE
    }
}
#[doc = "Application trigger event for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_1_AW {
    #[doc = "1: Generate channel event for channel 1."]
    ACTIVATE = 1,
}
impl From<APPSET_1_AW> for bool {
    #[inline(always)]
    fn from(variant: APPSET_1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_1` writer - Application trigger event for channel 1."]
pub type APPSET_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIAPPSET_SPEC, APPSET_1_AW, O>;
impl<'a, const O: u8> APPSET_1_W<'a, O> {
    #[doc = "Generate channel event for channel 1."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(APPSET_1_AW::ACTIVATE)
    }
}
#[doc = "Field `APPSET_2` reader - Application trigger event for channel 2."]
pub type APPSET_2_R = crate::BitReader<APPSET_2_A>;
#[doc = "Application trigger event for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_2_A {
    #[doc = "0: Application trigger 2 is inactive."]
    INACTIVE = 0,
    #[doc = "1: Application trigger 2 is active."]
    ACTIVE = 1,
}
impl From<APPSET_2_A> for bool {
    #[inline(always)]
    fn from(variant: APPSET_2_A) -> Self {
        variant as u8 != 0
    }
}
impl APPSET_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APPSET_2_A {
        match self.bits {
            false => APPSET_2_A::INACTIVE,
            true => APPSET_2_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == APPSET_2_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == APPSET_2_A::ACTIVE
    }
}
#[doc = "Application trigger event for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_2_AW {
    #[doc = "1: Generate channel event for channel 2."]
    ACTIVATE = 1,
}
impl From<APPSET_2_AW> for bool {
    #[inline(always)]
    fn from(variant: APPSET_2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_2` writer - Application trigger event for channel 2."]
pub type APPSET_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIAPPSET_SPEC, APPSET_2_AW, O>;
impl<'a, const O: u8> APPSET_2_W<'a, O> {
    #[doc = "Generate channel event for channel 2."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(APPSET_2_AW::ACTIVATE)
    }
}
#[doc = "Field `APPSET_3` reader - Application trigger event for channel 3."]
pub type APPSET_3_R = crate::BitReader<APPSET_3_A>;
#[doc = "Application trigger event for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_3_A {
    #[doc = "0: Application trigger 3 is inactive."]
    INACTIVE = 0,
    #[doc = "1: Application trigger 3 is active."]
    ACTIVE = 1,
}
impl From<APPSET_3_A> for bool {
    #[inline(always)]
    fn from(variant: APPSET_3_A) -> Self {
        variant as u8 != 0
    }
}
impl APPSET_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APPSET_3_A {
        match self.bits {
            false => APPSET_3_A::INACTIVE,
            true => APPSET_3_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == APPSET_3_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == APPSET_3_A::ACTIVE
    }
}
#[doc = "Application trigger event for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_3_AW {
    #[doc = "1: Generate channel event for channel 3."]
    ACTIVATE = 1,
}
impl From<APPSET_3_AW> for bool {
    #[inline(always)]
    fn from(variant: APPSET_3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_3` writer - Application trigger event for channel 3."]
pub type APPSET_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIAPPSET_SPEC, APPSET_3_AW, O>;
impl<'a, const O: u8> APPSET_3_W<'a, O> {
    #[doc = "Generate channel event for channel 3."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(APPSET_3_AW::ACTIVATE)
    }
}
impl R {
    #[doc = "Bit 0 - Application trigger event for channel 0."]
    #[inline(always)]
    pub fn appset_0(&self) -> APPSET_0_R {
        APPSET_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Application trigger event for channel 1."]
    #[inline(always)]
    pub fn appset_1(&self) -> APPSET_1_R {
        APPSET_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Application trigger event for channel 2."]
    #[inline(always)]
    pub fn appset_2(&self) -> APPSET_2_R {
        APPSET_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Application trigger event for channel 3."]
    #[inline(always)]
    pub fn appset_3(&self) -> APPSET_3_R {
        APPSET_3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Application trigger event for channel 0."]
    #[inline(always)]
    pub fn appset_0(&mut self) -> APPSET_0_W<0> {
        APPSET_0_W::new(self)
    }
    #[doc = "Bit 1 - Application trigger event for channel 1."]
    #[inline(always)]
    pub fn appset_1(&mut self) -> APPSET_1_W<1> {
        APPSET_1_W::new(self)
    }
    #[doc = "Bit 2 - Application trigger event for channel 2."]
    #[inline(always)]
    pub fn appset_2(&mut self) -> APPSET_2_W<2> {
        APPSET_2_W::new(self)
    }
    #[doc = "Bit 3 - Application trigger event for channel 3."]
    #[inline(always)]
    pub fn appset_3(&mut self) -> APPSET_3_W<3> {
        APPSET_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTI Application Trigger Set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctiappset](index.html) module"]
pub struct CTIAPPSET_SPEC;
impl crate::RegisterSpec for CTIAPPSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctiappset::R](R) reader structure"]
impl crate::Readable for CTIAPPSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctiappset::W](W) writer structure"]
impl crate::Writable for CTIAPPSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIAPPSET to value 0"]
impl crate::Resettable for CTIAPPSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
