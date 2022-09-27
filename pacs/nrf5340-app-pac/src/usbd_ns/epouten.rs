#[doc = "Register `EPOUTEN` reader"]
pub struct R(crate::R<EPOUTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPOUTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPOUTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPOUTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPOUTEN` writer"]
pub struct W(crate::W<EPOUTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPOUTEN_SPEC>;
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
impl From<crate::W<EPOUTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPOUTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT0` reader - Enable OUT endpoint 0"]
pub type OUT0_R = crate::BitReader<OUT0_A>;
#[doc = "Enable OUT endpoint 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT0_A {
    #[doc = "0: Disable endpoint OUT 0 (no response to OUT tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint OUT 0 (response to OUT tokens)"]
    ENABLE = 1,
}
impl From<OUT0_A> for bool {
    #[inline(always)]
    fn from(variant: OUT0_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT0_A {
        match self.bits {
            false => OUT0_A::DISABLE,
            true => OUT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OUT0_A::ENABLE
    }
}
#[doc = "Field `OUT0` writer - Enable OUT endpoint 0"]
pub type OUT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPOUTEN_SPEC, OUT0_A, O>;
impl<'a, const O: u8> OUT0_W<'a, O> {
    #[doc = "Disable endpoint OUT 0 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT0_A::DISABLE)
    }
    #[doc = "Enable endpoint OUT 0 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT0_A::ENABLE)
    }
}
#[doc = "Field `OUT1` reader - Enable OUT endpoint 1"]
pub type OUT1_R = crate::BitReader<OUT1_A>;
#[doc = "Enable OUT endpoint 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT1_A {
    #[doc = "0: Disable endpoint OUT 1 (no response to OUT tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint OUT 1 (response to OUT tokens)"]
    ENABLE = 1,
}
impl From<OUT1_A> for bool {
    #[inline(always)]
    fn from(variant: OUT1_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT1_A {
        match self.bits {
            false => OUT1_A::DISABLE,
            true => OUT1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUT1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OUT1_A::ENABLE
    }
}
#[doc = "Field `OUT1` writer - Enable OUT endpoint 1"]
pub type OUT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPOUTEN_SPEC, OUT1_A, O>;
impl<'a, const O: u8> OUT1_W<'a, O> {
    #[doc = "Disable endpoint OUT 1 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT1_A::DISABLE)
    }
    #[doc = "Enable endpoint OUT 1 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT1_A::ENABLE)
    }
}
#[doc = "Field `OUT2` reader - Enable OUT endpoint 2"]
pub type OUT2_R = crate::BitReader<OUT2_A>;
#[doc = "Enable OUT endpoint 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT2_A {
    #[doc = "0: Disable endpoint OUT 2 (no response to OUT tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint OUT 2 (response to OUT tokens)"]
    ENABLE = 1,
}
impl From<OUT2_A> for bool {
    #[inline(always)]
    fn from(variant: OUT2_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT2_A {
        match self.bits {
            false => OUT2_A::DISABLE,
            true => OUT2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUT2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OUT2_A::ENABLE
    }
}
#[doc = "Field `OUT2` writer - Enable OUT endpoint 2"]
pub type OUT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPOUTEN_SPEC, OUT2_A, O>;
impl<'a, const O: u8> OUT2_W<'a, O> {
    #[doc = "Disable endpoint OUT 2 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT2_A::DISABLE)
    }
    #[doc = "Enable endpoint OUT 2 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT2_A::ENABLE)
    }
}
#[doc = "Field `OUT3` reader - Enable OUT endpoint 3"]
pub type OUT3_R = crate::BitReader<OUT3_A>;
#[doc = "Enable OUT endpoint 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT3_A {
    #[doc = "0: Disable endpoint OUT 3 (no response to OUT tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint OUT 3 (response to OUT tokens)"]
    ENABLE = 1,
}
impl From<OUT3_A> for bool {
    #[inline(always)]
    fn from(variant: OUT3_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT3_A {
        match self.bits {
            false => OUT3_A::DISABLE,
            true => OUT3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUT3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OUT3_A::ENABLE
    }
}
#[doc = "Field `OUT3` writer - Enable OUT endpoint 3"]
pub type OUT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPOUTEN_SPEC, OUT3_A, O>;
impl<'a, const O: u8> OUT3_W<'a, O> {
    #[doc = "Disable endpoint OUT 3 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT3_A::DISABLE)
    }
    #[doc = "Enable endpoint OUT 3 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT3_A::ENABLE)
    }
}
#[doc = "Field `OUT4` reader - Enable OUT endpoint 4"]
pub type OUT4_R = crate::BitReader<OUT4_A>;
#[doc = "Enable OUT endpoint 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT4_A {
    #[doc = "0: Disable endpoint OUT 4 (no response to OUT tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint OUT 4 (response to OUT tokens)"]
    ENABLE = 1,
}
impl From<OUT4_A> for bool {
    #[inline(always)]
    fn from(variant: OUT4_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT4_A {
        match self.bits {
            false => OUT4_A::DISABLE,
            true => OUT4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUT4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OUT4_A::ENABLE
    }
}
#[doc = "Field `OUT4` writer - Enable OUT endpoint 4"]
pub type OUT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPOUTEN_SPEC, OUT4_A, O>;
impl<'a, const O: u8> OUT4_W<'a, O> {
    #[doc = "Disable endpoint OUT 4 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT4_A::DISABLE)
    }
    #[doc = "Enable endpoint OUT 4 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT4_A::ENABLE)
    }
}
#[doc = "Field `OUT5` reader - Enable OUT endpoint 5"]
pub type OUT5_R = crate::BitReader<OUT5_A>;
#[doc = "Enable OUT endpoint 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT5_A {
    #[doc = "0: Disable endpoint OUT 5 (no response to OUT tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint OUT 5 (response to OUT tokens)"]
    ENABLE = 1,
}
impl From<OUT5_A> for bool {
    #[inline(always)]
    fn from(variant: OUT5_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT5_A {
        match self.bits {
            false => OUT5_A::DISABLE,
            true => OUT5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUT5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OUT5_A::ENABLE
    }
}
#[doc = "Field `OUT5` writer - Enable OUT endpoint 5"]
pub type OUT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPOUTEN_SPEC, OUT5_A, O>;
impl<'a, const O: u8> OUT5_W<'a, O> {
    #[doc = "Disable endpoint OUT 5 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT5_A::DISABLE)
    }
    #[doc = "Enable endpoint OUT 5 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT5_A::ENABLE)
    }
}
#[doc = "Field `OUT6` reader - Enable OUT endpoint 6"]
pub type OUT6_R = crate::BitReader<OUT6_A>;
#[doc = "Enable OUT endpoint 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT6_A {
    #[doc = "0: Disable endpoint OUT 6 (no response to OUT tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint OUT 6 (response to OUT tokens)"]
    ENABLE = 1,
}
impl From<OUT6_A> for bool {
    #[inline(always)]
    fn from(variant: OUT6_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT6_A {
        match self.bits {
            false => OUT6_A::DISABLE,
            true => OUT6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUT6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OUT6_A::ENABLE
    }
}
#[doc = "Field `OUT6` writer - Enable OUT endpoint 6"]
pub type OUT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPOUTEN_SPEC, OUT6_A, O>;
impl<'a, const O: u8> OUT6_W<'a, O> {
    #[doc = "Disable endpoint OUT 6 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT6_A::DISABLE)
    }
    #[doc = "Enable endpoint OUT 6 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT6_A::ENABLE)
    }
}
#[doc = "Field `OUT7` reader - Enable OUT endpoint 7"]
pub type OUT7_R = crate::BitReader<OUT7_A>;
#[doc = "Enable OUT endpoint 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT7_A {
    #[doc = "0: Disable endpoint OUT 7 (no response to OUT tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint OUT 7 (response to OUT tokens)"]
    ENABLE = 1,
}
impl From<OUT7_A> for bool {
    #[inline(always)]
    fn from(variant: OUT7_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT7_A {
        match self.bits {
            false => OUT7_A::DISABLE,
            true => OUT7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUT7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OUT7_A::ENABLE
    }
}
#[doc = "Field `OUT7` writer - Enable OUT endpoint 7"]
pub type OUT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPOUTEN_SPEC, OUT7_A, O>;
impl<'a, const O: u8> OUT7_W<'a, O> {
    #[doc = "Disable endpoint OUT 7 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT7_A::DISABLE)
    }
    #[doc = "Enable endpoint OUT 7 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT7_A::ENABLE)
    }
}
#[doc = "Field `ISOOUT` reader - Enable ISO OUT endpoint 8"]
pub type ISOOUT_R = crate::BitReader<ISOOUT_A>;
#[doc = "Enable ISO OUT endpoint 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOOUT_A {
    #[doc = "0: Disable ISO OUT endpoint 8"]
    DISABLE = 0,
    #[doc = "1: Enable ISO OUT endpoint 8"]
    ENABLE = 1,
}
impl From<ISOOUT_A> for bool {
    #[inline(always)]
    fn from(variant: ISOOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl ISOOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOOUT_A {
        match self.bits {
            false => ISOOUT_A::DISABLE,
            true => ISOOUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ISOOUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ISOOUT_A::ENABLE
    }
}
#[doc = "Field `ISOOUT` writer - Enable ISO OUT endpoint 8"]
pub type ISOOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPOUTEN_SPEC, ISOOUT_A, O>;
impl<'a, const O: u8> ISOOUT_W<'a, O> {
    #[doc = "Disable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ISOOUT_A::DISABLE)
    }
    #[doc = "Enable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ISOOUT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable OUT endpoint 0"]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable OUT endpoint 1"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable OUT endpoint 2"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable OUT endpoint 3"]
    #[inline(always)]
    pub fn out3(&self) -> OUT3_R {
        OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable OUT endpoint 4"]
    #[inline(always)]
    pub fn out4(&self) -> OUT4_R {
        OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable OUT endpoint 5"]
    #[inline(always)]
    pub fn out5(&self) -> OUT5_R {
        OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable OUT endpoint 6"]
    #[inline(always)]
    pub fn out6(&self) -> OUT6_R {
        OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable OUT endpoint 7"]
    #[inline(always)]
    pub fn out7(&self) -> OUT7_R {
        OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn isoout(&self) -> ISOOUT_R {
        ISOOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable OUT endpoint 0"]
    #[inline(always)]
    pub fn out0(&mut self) -> OUT0_W<0> {
        OUT0_W::new(self)
    }
    #[doc = "Bit 1 - Enable OUT endpoint 1"]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W<1> {
        OUT1_W::new(self)
    }
    #[doc = "Bit 2 - Enable OUT endpoint 2"]
    #[inline(always)]
    pub fn out2(&mut self) -> OUT2_W<2> {
        OUT2_W::new(self)
    }
    #[doc = "Bit 3 - Enable OUT endpoint 3"]
    #[inline(always)]
    pub fn out3(&mut self) -> OUT3_W<3> {
        OUT3_W::new(self)
    }
    #[doc = "Bit 4 - Enable OUT endpoint 4"]
    #[inline(always)]
    pub fn out4(&mut self) -> OUT4_W<4> {
        OUT4_W::new(self)
    }
    #[doc = "Bit 5 - Enable OUT endpoint 5"]
    #[inline(always)]
    pub fn out5(&mut self) -> OUT5_W<5> {
        OUT5_W::new(self)
    }
    #[doc = "Bit 6 - Enable OUT endpoint 6"]
    #[inline(always)]
    pub fn out6(&mut self) -> OUT6_W<6> {
        OUT6_W::new(self)
    }
    #[doc = "Bit 7 - Enable OUT endpoint 7"]
    #[inline(always)]
    pub fn out7(&mut self) -> OUT7_W<7> {
        OUT7_W::new(self)
    }
    #[doc = "Bit 8 - Enable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn isoout(&mut self) -> ISOOUT_W<8> {
        ISOOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint OUT enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epouten](index.html) module"]
pub struct EPOUTEN_SPEC;
impl crate::RegisterSpec for EPOUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epouten::R](R) reader structure"]
impl crate::Readable for EPOUTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epouten::W](W) writer structure"]
impl crate::Writable for EPOUTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPOUTEN to value 0x01"]
impl crate::Resettable for EPOUTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
