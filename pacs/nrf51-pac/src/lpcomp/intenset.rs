#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY` reader - Enable interrupt on READY event."]
pub type READY_R = crate::BitReader<READY_A>;
#[doc = "Enable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::DISABLED,
            true => READY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_A::ENABLED
    }
}
#[doc = "Enable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<READY_AW> for bool {
    #[inline(always)]
    fn from(variant: READY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Enable interrupt on READY event."]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, READY_AW, O>;
impl<'a, const O: u8> READY_W<'a, O> {
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(READY_AW::SET)
    }
}
#[doc = "Field `DOWN` reader - Enable interrupt on DOWN event."]
pub type DOWN_R = crate::BitReader<DOWN_A>;
#[doc = "Enable interrupt on DOWN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWN_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DOWN_A) -> Self {
        variant as u8 != 0
    }
}
impl DOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOWN_A {
        match self.bits {
            false => DOWN_A::DISABLED,
            true => DOWN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOWN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOWN_A::ENABLED
    }
}
#[doc = "Enable interrupt on DOWN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWN_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<DOWN_AW> for bool {
    #[inline(always)]
    fn from(variant: DOWN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN` writer - Enable interrupt on DOWN event."]
pub type DOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, DOWN_AW, O>;
impl<'a, const O: u8> DOWN_W<'a, O> {
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DOWN_AW::SET)
    }
}
#[doc = "Field `UP` reader - Enable interrupt on UP event."]
pub type UP_R = crate::BitReader<UP_A>;
#[doc = "Enable interrupt on UP event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<UP_A> for bool {
    #[inline(always)]
    fn from(variant: UP_A) -> Self {
        variant as u8 != 0
    }
}
impl UP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_A {
        match self.bits {
            false => UP_A::DISABLED,
            true => UP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UP_A::ENABLED
    }
}
#[doc = "Enable interrupt on UP event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<UP_AW> for bool {
    #[inline(always)]
    fn from(variant: UP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP` writer - Enable interrupt on UP event."]
pub type UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, UP_AW, O>;
impl<'a, const O: u8> UP_W<'a, O> {
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(UP_AW::SET)
    }
}
#[doc = "Field `CROSS` reader - Enable interrupt on CROSS event."]
pub type CROSS_R = crate::BitReader<CROSS_A>;
#[doc = "Enable interrupt on CROSS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROSS_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<CROSS_A> for bool {
    #[inline(always)]
    fn from(variant: CROSS_A) -> Self {
        variant as u8 != 0
    }
}
impl CROSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CROSS_A {
        match self.bits {
            false => CROSS_A::DISABLED,
            true => CROSS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CROSS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CROSS_A::ENABLED
    }
}
#[doc = "Enable interrupt on CROSS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROSS_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<CROSS_AW> for bool {
    #[inline(always)]
    fn from(variant: CROSS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROSS` writer - Enable interrupt on CROSS event."]
pub type CROSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CROSS_AW, O>;
impl<'a, const O: u8> CROSS_W<'a, O> {
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CROSS_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt on DOWN event."]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable interrupt on UP event."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable interrupt on CROSS event."]
    #[inline(always)]
    pub fn cross(&self) -> CROSS_R {
        CROSS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W<0> {
        READY_W::new(self)
    }
    #[doc = "Bit 1 - Enable interrupt on DOWN event."]
    #[inline(always)]
    pub fn down(&mut self) -> DOWN_W<1> {
        DOWN_W::new(self)
    }
    #[doc = "Bit 2 - Enable interrupt on UP event."]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W<2> {
        UP_W::new(self)
    }
    #[doc = "Bit 3 - Enable interrupt on CROSS event."]
    #[inline(always)]
    pub fn cross(&mut self) -> CROSS_W<3> {
        CROSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable set register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
