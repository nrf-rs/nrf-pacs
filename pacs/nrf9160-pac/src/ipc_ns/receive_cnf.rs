#[doc = "Register `RECEIVE_CNF[%s]` reader"]
pub struct R(crate::R<RECEIVE_CNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_CNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_CNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_CNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECEIVE_CNF[%s]` writer"]
pub struct W(crate::W<RECEIVE_CNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_CNF_SPEC>;
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
impl From<crate::W<RECEIVE_CNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_CNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEN0` reader - Enable subscription to channel 0."]
pub type CHEN0_R = crate::BitReader<CHEN0_A>;
#[doc = "Enable subscription to channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN0_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN0_A {
        match self.bits {
            false => CHEN0_A::DISABLE,
            true => CHEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN0_A::ENABLE
    }
}
#[doc = "Field `CHEN0` writer - Enable subscription to channel 0."]
pub type CHEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECEIVE_CNF_SPEC, CHEN0_A, O>;
impl<'a, const O: u8> CHEN0_W<'a, O> {
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN0_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN0_A::ENABLE)
    }
}
#[doc = "Field `CHEN1` reader - Enable subscription to channel 1."]
pub type CHEN1_R = crate::BitReader<CHEN1_A>;
#[doc = "Enable subscription to channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN1_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN1_A {
        match self.bits {
            false => CHEN1_A::DISABLE,
            true => CHEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN1_A::ENABLE
    }
}
#[doc = "Field `CHEN1` writer - Enable subscription to channel 1."]
pub type CHEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECEIVE_CNF_SPEC, CHEN1_A, O>;
impl<'a, const O: u8> CHEN1_W<'a, O> {
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN1_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN1_A::ENABLE)
    }
}
#[doc = "Field `CHEN2` reader - Enable subscription to channel 2."]
pub type CHEN2_R = crate::BitReader<CHEN2_A>;
#[doc = "Enable subscription to channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN2_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN2_A {
        match self.bits {
            false => CHEN2_A::DISABLE,
            true => CHEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN2_A::ENABLE
    }
}
#[doc = "Field `CHEN2` writer - Enable subscription to channel 2."]
pub type CHEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECEIVE_CNF_SPEC, CHEN2_A, O>;
impl<'a, const O: u8> CHEN2_W<'a, O> {
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN2_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN2_A::ENABLE)
    }
}
#[doc = "Field `CHEN3` reader - Enable subscription to channel 3."]
pub type CHEN3_R = crate::BitReader<CHEN3_A>;
#[doc = "Enable subscription to channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN3_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN3_A {
        match self.bits {
            false => CHEN3_A::DISABLE,
            true => CHEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN3_A::ENABLE
    }
}
#[doc = "Field `CHEN3` writer - Enable subscription to channel 3."]
pub type CHEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECEIVE_CNF_SPEC, CHEN3_A, O>;
impl<'a, const O: u8> CHEN3_W<'a, O> {
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN3_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN3_A::ENABLE)
    }
}
#[doc = "Field `CHEN4` reader - Enable subscription to channel 4."]
pub type CHEN4_R = crate::BitReader<CHEN4_A>;
#[doc = "Enable subscription to channel 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN4_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN4_A {
        match self.bits {
            false => CHEN4_A::DISABLE,
            true => CHEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN4_A::ENABLE
    }
}
#[doc = "Field `CHEN4` writer - Enable subscription to channel 4."]
pub type CHEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECEIVE_CNF_SPEC, CHEN4_A, O>;
impl<'a, const O: u8> CHEN4_W<'a, O> {
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN4_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN4_A::ENABLE)
    }
}
#[doc = "Field `CHEN5` reader - Enable subscription to channel 5."]
pub type CHEN5_R = crate::BitReader<CHEN5_A>;
#[doc = "Enable subscription to channel 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN5_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN5_A {
        match self.bits {
            false => CHEN5_A::DISABLE,
            true => CHEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN5_A::ENABLE
    }
}
#[doc = "Field `CHEN5` writer - Enable subscription to channel 5."]
pub type CHEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECEIVE_CNF_SPEC, CHEN5_A, O>;
impl<'a, const O: u8> CHEN5_W<'a, O> {
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN5_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN5_A::ENABLE)
    }
}
#[doc = "Field `CHEN6` reader - Enable subscription to channel 6."]
pub type CHEN6_R = crate::BitReader<CHEN6_A>;
#[doc = "Enable subscription to channel 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN6_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN6_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN6_A {
        match self.bits {
            false => CHEN6_A::DISABLE,
            true => CHEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN6_A::ENABLE
    }
}
#[doc = "Field `CHEN6` writer - Enable subscription to channel 6."]
pub type CHEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECEIVE_CNF_SPEC, CHEN6_A, O>;
impl<'a, const O: u8> CHEN6_W<'a, O> {
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN6_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN6_A::ENABLE)
    }
}
#[doc = "Field `CHEN7` reader - Enable subscription to channel 7."]
pub type CHEN7_R = crate::BitReader<CHEN7_A>;
#[doc = "Enable subscription to channel 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN7_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN7_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN7_A {
        match self.bits {
            false => CHEN7_A::DISABLE,
            true => CHEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN7_A::ENABLE
    }
}
#[doc = "Field `CHEN7` writer - Enable subscription to channel 7."]
pub type CHEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECEIVE_CNF_SPEC, CHEN7_A, O>;
impl<'a, const O: u8> CHEN7_W<'a, O> {
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN7_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN7_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable subscription to channel 0."]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable subscription to channel 1."]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable subscription to channel 2."]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable subscription to channel 3."]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable subscription to channel 4."]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable subscription to channel 5."]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable subscription to channel 6."]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable subscription to channel 7."]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable subscription to channel 0."]
    #[inline(always)]
    pub fn chen0(&mut self) -> CHEN0_W<0> {
        CHEN0_W::new(self)
    }
    #[doc = "Bit 1 - Enable subscription to channel 1."]
    #[inline(always)]
    pub fn chen1(&mut self) -> CHEN1_W<1> {
        CHEN1_W::new(self)
    }
    #[doc = "Bit 2 - Enable subscription to channel 2."]
    #[inline(always)]
    pub fn chen2(&mut self) -> CHEN2_W<2> {
        CHEN2_W::new(self)
    }
    #[doc = "Bit 3 - Enable subscription to channel 3."]
    #[inline(always)]
    pub fn chen3(&mut self) -> CHEN3_W<3> {
        CHEN3_W::new(self)
    }
    #[doc = "Bit 4 - Enable subscription to channel 4."]
    #[inline(always)]
    pub fn chen4(&mut self) -> CHEN4_W<4> {
        CHEN4_W::new(self)
    }
    #[doc = "Bit 5 - Enable subscription to channel 5."]
    #[inline(always)]
    pub fn chen5(&mut self) -> CHEN5_W<5> {
        CHEN5_W::new(self)
    }
    #[doc = "Bit 6 - Enable subscription to channel 6."]
    #[inline(always)]
    pub fn chen6(&mut self) -> CHEN6_W<6> {
        CHEN6_W::new(self)
    }
    #[doc = "Bit 7 - Enable subscription to channel 7."]
    #[inline(always)]
    pub fn chen7(&mut self) -> CHEN7_W<7> {
        CHEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_cnf](index.html) module"]
pub struct RECEIVE_CNF_SPEC;
impl crate::RegisterSpec for RECEIVE_CNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_cnf::R](R) reader structure"]
impl crate::Readable for RECEIVE_CNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_cnf::W](W) writer structure"]
impl crate::Writable for RECEIVE_CNF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RECEIVE_CNF[%s]
to value 0"]
impl crate::Resettable for RECEIVE_CNF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
