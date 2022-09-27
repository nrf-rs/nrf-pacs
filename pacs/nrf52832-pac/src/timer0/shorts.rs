#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARE0_CLEAR` reader - Shortcut between COMPARE\\[0\\]
event and CLEAR task"]
pub type COMPARE0_CLEAR_R = crate::BitReader<COMPARE0_CLEAR_A>;
#[doc = "Shortcut between COMPARE\\[0\\]
event and CLEAR task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE0_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE0_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_CLEAR_A {
        match self.bits {
            false => COMPARE0_CLEAR_A::DISABLED,
            true => COMPARE0_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE0_CLEAR` writer - Shortcut between COMPARE\\[0\\]
event and CLEAR task"]
pub type COMPARE0_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE0_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE0_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE1_CLEAR` reader - Shortcut between COMPARE\\[1\\]
event and CLEAR task"]
pub type COMPARE1_CLEAR_R = crate::BitReader<COMPARE1_CLEAR_A>;
#[doc = "Shortcut between COMPARE\\[1\\]
event and CLEAR task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE1_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE1_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_CLEAR_A {
        match self.bits {
            false => COMPARE1_CLEAR_A::DISABLED,
            true => COMPARE1_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE1_CLEAR` writer - Shortcut between COMPARE\\[1\\]
event and CLEAR task"]
pub type COMPARE1_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE1_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE1_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE2_CLEAR` reader - Shortcut between COMPARE\\[2\\]
event and CLEAR task"]
pub type COMPARE2_CLEAR_R = crate::BitReader<COMPARE2_CLEAR_A>;
#[doc = "Shortcut between COMPARE\\[2\\]
event and CLEAR task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE2_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE2_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_CLEAR_A {
        match self.bits {
            false => COMPARE2_CLEAR_A::DISABLED,
            true => COMPARE2_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE2_CLEAR` writer - Shortcut between COMPARE\\[2\\]
event and CLEAR task"]
pub type COMPARE2_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE2_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE2_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE3_CLEAR` reader - Shortcut between COMPARE\\[3\\]
event and CLEAR task"]
pub type COMPARE3_CLEAR_R = crate::BitReader<COMPARE3_CLEAR_A>;
#[doc = "Shortcut between COMPARE\\[3\\]
event and CLEAR task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE3_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE3_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_CLEAR_A {
        match self.bits {
            false => COMPARE3_CLEAR_A::DISABLED,
            true => COMPARE3_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE3_CLEAR` writer - Shortcut between COMPARE\\[3\\]
event and CLEAR task"]
pub type COMPARE3_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE3_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE3_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE0_STOP` reader - Shortcut between COMPARE\\[0\\]
event and STOP task"]
pub type COMPARE0_STOP_R = crate::BitReader<COMPARE0_STOP_A>;
#[doc = "Shortcut between COMPARE\\[0\\]
event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE0_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE0_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_STOP_A {
        match self.bits {
            false => COMPARE0_STOP_A::DISABLED,
            true => COMPARE0_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE0_STOP` writer - Shortcut between COMPARE\\[0\\]
event and STOP task"]
pub type COMPARE0_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE0_STOP_A, O>;
impl<'a, const O: u8> COMPARE0_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOP_A::ENABLED)
    }
}
#[doc = "Field `COMPARE1_STOP` reader - Shortcut between COMPARE\\[1\\]
event and STOP task"]
pub type COMPARE1_STOP_R = crate::BitReader<COMPARE1_STOP_A>;
#[doc = "Shortcut between COMPARE\\[1\\]
event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_STOP_A {
        match self.bits {
            false => COMPARE1_STOP_A::DISABLED,
            true => COMPARE1_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE1_STOP` writer - Shortcut between COMPARE\\[1\\]
event and STOP task"]
pub type COMPARE1_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE1_STOP_A, O>;
impl<'a, const O: u8> COMPARE1_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOP_A::ENABLED)
    }
}
#[doc = "Field `COMPARE2_STOP` reader - Shortcut between COMPARE\\[2\\]
event and STOP task"]
pub type COMPARE2_STOP_R = crate::BitReader<COMPARE2_STOP_A>;
#[doc = "Shortcut between COMPARE\\[2\\]
event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_STOP_A {
        match self.bits {
            false => COMPARE2_STOP_A::DISABLED,
            true => COMPARE2_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE2_STOP` writer - Shortcut between COMPARE\\[2\\]
event and STOP task"]
pub type COMPARE2_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE2_STOP_A, O>;
impl<'a, const O: u8> COMPARE2_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOP_A::ENABLED)
    }
}
#[doc = "Field `COMPARE3_STOP` reader - Shortcut between COMPARE\\[3\\]
event and STOP task"]
pub type COMPARE3_STOP_R = crate::BitReader<COMPARE3_STOP_A>;
#[doc = "Shortcut between COMPARE\\[3\\]
event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE3_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_STOP_A {
        match self.bits {
            false => COMPARE3_STOP_A::DISABLED,
            true => COMPARE3_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE3_STOP` writer - Shortcut between COMPARE\\[3\\]
event and STOP task"]
pub type COMPARE3_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE3_STOP_A, O>;
impl<'a, const O: u8> COMPARE3_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOP_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between COMPARE\\[0\\]
event and CLEAR task"]
    #[inline(always)]
    pub fn compare0_clear(&self) -> COMPARE0_CLEAR_R {
        COMPARE0_CLEAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between COMPARE\\[1\\]
event and CLEAR task"]
    #[inline(always)]
    pub fn compare1_clear(&self) -> COMPARE1_CLEAR_R {
        COMPARE1_CLEAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between COMPARE\\[2\\]
event and CLEAR task"]
    #[inline(always)]
    pub fn compare2_clear(&self) -> COMPARE2_CLEAR_R {
        COMPARE2_CLEAR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between COMPARE\\[3\\]
event and CLEAR task"]
    #[inline(always)]
    pub fn compare3_clear(&self) -> COMPARE3_CLEAR_R {
        COMPARE3_CLEAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Shortcut between COMPARE\\[0\\]
event and STOP task"]
    #[inline(always)]
    pub fn compare0_stop(&self) -> COMPARE0_STOP_R {
        COMPARE0_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Shortcut between COMPARE\\[1\\]
event and STOP task"]
    #[inline(always)]
    pub fn compare1_stop(&self) -> COMPARE1_STOP_R {
        COMPARE1_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Shortcut between COMPARE\\[2\\]
event and STOP task"]
    #[inline(always)]
    pub fn compare2_stop(&self) -> COMPARE2_STOP_R {
        COMPARE2_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Shortcut between COMPARE\\[3\\]
event and STOP task"]
    #[inline(always)]
    pub fn compare3_stop(&self) -> COMPARE3_STOP_R {
        COMPARE3_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between COMPARE\\[0\\]
event and CLEAR task"]
    #[inline(always)]
    pub fn compare0_clear(&mut self) -> COMPARE0_CLEAR_W<0> {
        COMPARE0_CLEAR_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between COMPARE\\[1\\]
event and CLEAR task"]
    #[inline(always)]
    pub fn compare1_clear(&mut self) -> COMPARE1_CLEAR_W<1> {
        COMPARE1_CLEAR_W::new(self)
    }
    #[doc = "Bit 2 - Shortcut between COMPARE\\[2\\]
event and CLEAR task"]
    #[inline(always)]
    pub fn compare2_clear(&mut self) -> COMPARE2_CLEAR_W<2> {
        COMPARE2_CLEAR_W::new(self)
    }
    #[doc = "Bit 3 - Shortcut between COMPARE\\[3\\]
event and CLEAR task"]
    #[inline(always)]
    pub fn compare3_clear(&mut self) -> COMPARE3_CLEAR_W<3> {
        COMPARE3_CLEAR_W::new(self)
    }
    #[doc = "Bit 8 - Shortcut between COMPARE\\[0\\]
event and STOP task"]
    #[inline(always)]
    pub fn compare0_stop(&mut self) -> COMPARE0_STOP_W<8> {
        COMPARE0_STOP_W::new(self)
    }
    #[doc = "Bit 9 - Shortcut between COMPARE\\[1\\]
event and STOP task"]
    #[inline(always)]
    pub fn compare1_stop(&mut self) -> COMPARE1_STOP_W<9> {
        COMPARE1_STOP_W::new(self)
    }
    #[doc = "Bit 10 - Shortcut between COMPARE\\[2\\]
event and STOP task"]
    #[inline(always)]
    pub fn compare2_stop(&mut self) -> COMPARE2_STOP_W<10> {
        COMPARE2_STOP_W::new(self)
    }
    #[doc = "Bit 11 - Shortcut between COMPARE\\[3\\]
event and STOP task"]
    #[inline(always)]
    pub fn compare3_stop(&mut self) -> COMPARE3_STOP_W<11> {
        COMPARE3_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcut register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
