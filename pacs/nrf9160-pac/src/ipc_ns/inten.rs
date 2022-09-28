#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECEIVE0` reader - Enable or disable interrupt for event RECEIVE\\[0\\]"]
pub type RECEIVE0_R = crate::BitReader<RECEIVE0_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE0_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE0_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE0_A {
        match self.bits {
            false => RECEIVE0_A::DISABLED,
            true => RECEIVE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE0_A::ENABLED
    }
}
#[doc = "Field `RECEIVE0` writer - Enable or disable interrupt for event RECEIVE\\[0\\]"]
pub type RECEIVE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE0_A, O>;
impl<'a, const O: u8> RECEIVE0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE0_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE1` reader - Enable or disable interrupt for event RECEIVE\\[1\\]"]
pub type RECEIVE1_R = crate::BitReader<RECEIVE1_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE1_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE1_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE1_A {
        match self.bits {
            false => RECEIVE1_A::DISABLED,
            true => RECEIVE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE1_A::ENABLED
    }
}
#[doc = "Field `RECEIVE1` writer - Enable or disable interrupt for event RECEIVE\\[1\\]"]
pub type RECEIVE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE1_A, O>;
impl<'a, const O: u8> RECEIVE1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE1_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE2` reader - Enable or disable interrupt for event RECEIVE\\[2\\]"]
pub type RECEIVE2_R = crate::BitReader<RECEIVE2_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE2_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE2_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE2_A {
        match self.bits {
            false => RECEIVE2_A::DISABLED,
            true => RECEIVE2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE2_A::ENABLED
    }
}
#[doc = "Field `RECEIVE2` writer - Enable or disable interrupt for event RECEIVE\\[2\\]"]
pub type RECEIVE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE2_A, O>;
impl<'a, const O: u8> RECEIVE2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE2_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE3` reader - Enable or disable interrupt for event RECEIVE\\[3\\]"]
pub type RECEIVE3_R = crate::BitReader<RECEIVE3_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE3_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE3_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE3_A {
        match self.bits {
            false => RECEIVE3_A::DISABLED,
            true => RECEIVE3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE3_A::ENABLED
    }
}
#[doc = "Field `RECEIVE3` writer - Enable or disable interrupt for event RECEIVE\\[3\\]"]
pub type RECEIVE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE3_A, O>;
impl<'a, const O: u8> RECEIVE3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE3_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE4` reader - Enable or disable interrupt for event RECEIVE\\[4\\]"]
pub type RECEIVE4_R = crate::BitReader<RECEIVE4_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE4_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE4_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE4_A {
        match self.bits {
            false => RECEIVE4_A::DISABLED,
            true => RECEIVE4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE4_A::ENABLED
    }
}
#[doc = "Field `RECEIVE4` writer - Enable or disable interrupt for event RECEIVE\\[4\\]"]
pub type RECEIVE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE4_A, O>;
impl<'a, const O: u8> RECEIVE4_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE4_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE5` reader - Enable or disable interrupt for event RECEIVE\\[5\\]"]
pub type RECEIVE5_R = crate::BitReader<RECEIVE5_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE5_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE5_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE5_A {
        match self.bits {
            false => RECEIVE5_A::DISABLED,
            true => RECEIVE5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE5_A::ENABLED
    }
}
#[doc = "Field `RECEIVE5` writer - Enable or disable interrupt for event RECEIVE\\[5\\]"]
pub type RECEIVE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE5_A, O>;
impl<'a, const O: u8> RECEIVE5_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE5_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE6` reader - Enable or disable interrupt for event RECEIVE\\[6\\]"]
pub type RECEIVE6_R = crate::BitReader<RECEIVE6_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE6_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE6_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE6_A {
        match self.bits {
            false => RECEIVE6_A::DISABLED,
            true => RECEIVE6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE6_A::ENABLED
    }
}
#[doc = "Field `RECEIVE6` writer - Enable or disable interrupt for event RECEIVE\\[6\\]"]
pub type RECEIVE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE6_A, O>;
impl<'a, const O: u8> RECEIVE6_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE6_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE7` reader - Enable or disable interrupt for event RECEIVE\\[7\\]"]
pub type RECEIVE7_R = crate::BitReader<RECEIVE7_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE7_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE7_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE7_A {
        match self.bits {
            false => RECEIVE7_A::DISABLED,
            true => RECEIVE7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE7_A::ENABLED
    }
}
#[doc = "Field `RECEIVE7` writer - Enable or disable interrupt for event RECEIVE\\[7\\]"]
pub type RECEIVE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE7_A, O>;
impl<'a, const O: u8> RECEIVE7_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE7_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> RECEIVE0_R {
        RECEIVE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> RECEIVE1_R {
        RECEIVE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> RECEIVE2_R {
        RECEIVE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> RECEIVE3_R {
        RECEIVE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> RECEIVE4_R {
        RECEIVE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> RECEIVE5_R {
        RECEIVE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> RECEIVE6_R {
        RECEIVE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> RECEIVE7_R {
        RECEIVE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&mut self) -> RECEIVE0_W<0> {
        RECEIVE0_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&mut self) -> RECEIVE1_W<1> {
        RECEIVE1_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&mut self) -> RECEIVE2_W<2> {
        RECEIVE2_W::new(self)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&mut self) -> RECEIVE3_W<3> {
        RECEIVE3_W::new(self)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&mut self) -> RECEIVE4_W<4> {
        RECEIVE4_W::new(self)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&mut self) -> RECEIVE5_W<5> {
        RECEIVE5_W::new(self)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&mut self) -> RECEIVE6_W<6> {
        RECEIVE6_W::new(self)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&mut self) -> RECEIVE7_W<7> {
        RECEIVE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
