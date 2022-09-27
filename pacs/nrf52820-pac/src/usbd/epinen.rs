#[doc = "Register `EPINEN` reader"]
pub struct R(crate::R<EPINEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPINEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPINEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPINEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPINEN` writer"]
pub struct W(crate::W<EPINEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPINEN_SPEC>;
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
impl From<crate::W<EPINEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPINEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN0` reader - Enable IN endpoint 0"]
pub type IN0_R = crate::BitReader<IN0_A>;
#[doc = "Enable IN endpoint 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_A {
    #[doc = "0: Disable endpoint IN 0 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 0 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN0_A> for bool {
    #[inline(always)]
    fn from(variant: IN0_A) -> Self {
        variant as u8 != 0
    }
}
impl IN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN0_A {
        match self.bits {
            false => IN0_A::DISABLE,
            true => IN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN0_A::ENABLE
    }
}
#[doc = "Field `IN0` writer - Enable IN endpoint 0"]
pub type IN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPINEN_SPEC, IN0_A, O>;
impl<'a, const O: u8> IN0_W<'a, O> {
    #[doc = "Disable endpoint IN 0 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN0_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 0 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN0_A::ENABLE)
    }
}
#[doc = "Field `IN1` reader - Enable IN endpoint 1"]
pub type IN1_R = crate::BitReader<IN1_A>;
#[doc = "Enable IN endpoint 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_A {
    #[doc = "0: Disable endpoint IN 1 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 1 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN1_A> for bool {
    #[inline(always)]
    fn from(variant: IN1_A) -> Self {
        variant as u8 != 0
    }
}
impl IN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN1_A {
        match self.bits {
            false => IN1_A::DISABLE,
            true => IN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN1_A::ENABLE
    }
}
#[doc = "Field `IN1` writer - Enable IN endpoint 1"]
pub type IN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPINEN_SPEC, IN1_A, O>;
impl<'a, const O: u8> IN1_W<'a, O> {
    #[doc = "Disable endpoint IN 1 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN1_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 1 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN1_A::ENABLE)
    }
}
#[doc = "Field `IN2` reader - Enable IN endpoint 2"]
pub type IN2_R = crate::BitReader<IN2_A>;
#[doc = "Enable IN endpoint 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_A {
    #[doc = "0: Disable endpoint IN 2 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 2 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN2_A> for bool {
    #[inline(always)]
    fn from(variant: IN2_A) -> Self {
        variant as u8 != 0
    }
}
impl IN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN2_A {
        match self.bits {
            false => IN2_A::DISABLE,
            true => IN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN2_A::ENABLE
    }
}
#[doc = "Field `IN2` writer - Enable IN endpoint 2"]
pub type IN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPINEN_SPEC, IN2_A, O>;
impl<'a, const O: u8> IN2_W<'a, O> {
    #[doc = "Disable endpoint IN 2 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN2_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 2 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN2_A::ENABLE)
    }
}
#[doc = "Field `IN3` reader - Enable IN endpoint 3"]
pub type IN3_R = crate::BitReader<IN3_A>;
#[doc = "Enable IN endpoint 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_A {
    #[doc = "0: Disable endpoint IN 3 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 3 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN3_A> for bool {
    #[inline(always)]
    fn from(variant: IN3_A) -> Self {
        variant as u8 != 0
    }
}
impl IN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN3_A {
        match self.bits {
            false => IN3_A::DISABLE,
            true => IN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN3_A::ENABLE
    }
}
#[doc = "Field `IN3` writer - Enable IN endpoint 3"]
pub type IN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPINEN_SPEC, IN3_A, O>;
impl<'a, const O: u8> IN3_W<'a, O> {
    #[doc = "Disable endpoint IN 3 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN3_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 3 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN3_A::ENABLE)
    }
}
#[doc = "Field `IN4` reader - Enable IN endpoint 4"]
pub type IN4_R = crate::BitReader<IN4_A>;
#[doc = "Enable IN endpoint 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4_A {
    #[doc = "0: Disable endpoint IN 4 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 4 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN4_A> for bool {
    #[inline(always)]
    fn from(variant: IN4_A) -> Self {
        variant as u8 != 0
    }
}
impl IN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN4_A {
        match self.bits {
            false => IN4_A::DISABLE,
            true => IN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN4_A::ENABLE
    }
}
#[doc = "Field `IN4` writer - Enable IN endpoint 4"]
pub type IN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPINEN_SPEC, IN4_A, O>;
impl<'a, const O: u8> IN4_W<'a, O> {
    #[doc = "Disable endpoint IN 4 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN4_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 4 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN4_A::ENABLE)
    }
}
#[doc = "Field `IN5` reader - Enable IN endpoint 5"]
pub type IN5_R = crate::BitReader<IN5_A>;
#[doc = "Enable IN endpoint 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN5_A {
    #[doc = "0: Disable endpoint IN 5 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 5 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN5_A> for bool {
    #[inline(always)]
    fn from(variant: IN5_A) -> Self {
        variant as u8 != 0
    }
}
impl IN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN5_A {
        match self.bits {
            false => IN5_A::DISABLE,
            true => IN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN5_A::ENABLE
    }
}
#[doc = "Field `IN5` writer - Enable IN endpoint 5"]
pub type IN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPINEN_SPEC, IN5_A, O>;
impl<'a, const O: u8> IN5_W<'a, O> {
    #[doc = "Disable endpoint IN 5 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN5_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 5 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN5_A::ENABLE)
    }
}
#[doc = "Field `IN6` reader - Enable IN endpoint 6"]
pub type IN6_R = crate::BitReader<IN6_A>;
#[doc = "Enable IN endpoint 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN6_A {
    #[doc = "0: Disable endpoint IN 6 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 6 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN6_A> for bool {
    #[inline(always)]
    fn from(variant: IN6_A) -> Self {
        variant as u8 != 0
    }
}
impl IN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN6_A {
        match self.bits {
            false => IN6_A::DISABLE,
            true => IN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN6_A::ENABLE
    }
}
#[doc = "Field `IN6` writer - Enable IN endpoint 6"]
pub type IN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPINEN_SPEC, IN6_A, O>;
impl<'a, const O: u8> IN6_W<'a, O> {
    #[doc = "Disable endpoint IN 6 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN6_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 6 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN6_A::ENABLE)
    }
}
#[doc = "Field `IN7` reader - Enable IN endpoint 7"]
pub type IN7_R = crate::BitReader<IN7_A>;
#[doc = "Enable IN endpoint 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN7_A {
    #[doc = "0: Disable endpoint IN 7 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 7 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN7_A> for bool {
    #[inline(always)]
    fn from(variant: IN7_A) -> Self {
        variant as u8 != 0
    }
}
impl IN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN7_A {
        match self.bits {
            false => IN7_A::DISABLE,
            true => IN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN7_A::ENABLE
    }
}
#[doc = "Field `IN7` writer - Enable IN endpoint 7"]
pub type IN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPINEN_SPEC, IN7_A, O>;
impl<'a, const O: u8> IN7_W<'a, O> {
    #[doc = "Disable endpoint IN 7 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN7_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 7 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN7_A::ENABLE)
    }
}
#[doc = "Field `ISOIN` reader - Enable ISO IN endpoint"]
pub type ISOIN_R = crate::BitReader<ISOIN_A>;
#[doc = "Enable ISO IN endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOIN_A {
    #[doc = "0: Disable ISO IN endpoint 8"]
    DISABLE = 0,
    #[doc = "1: Enable ISO IN endpoint 8"]
    ENABLE = 1,
}
impl From<ISOIN_A> for bool {
    #[inline(always)]
    fn from(variant: ISOIN_A) -> Self {
        variant as u8 != 0
    }
}
impl ISOIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOIN_A {
        match self.bits {
            false => ISOIN_A::DISABLE,
            true => ISOIN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ISOIN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ISOIN_A::ENABLE
    }
}
#[doc = "Field `ISOIN` writer - Enable ISO IN endpoint"]
pub type ISOIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPINEN_SPEC, ISOIN_A, O>;
impl<'a, const O: u8> ISOIN_W<'a, O> {
    #[doc = "Disable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ISOIN_A::DISABLE)
    }
    #[doc = "Enable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ISOIN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline(always)]
    pub fn isoin(&self) -> ISOIN_R {
        ISOIN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W<0> {
        IN0_W::new(self)
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W<1> {
        IN1_W::new(self)
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W<2> {
        IN2_W::new(self)
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W<3> {
        IN3_W::new(self)
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline(always)]
    pub fn in4(&mut self) -> IN4_W<4> {
        IN4_W::new(self)
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline(always)]
    pub fn in5(&mut self) -> IN5_W<5> {
        IN5_W::new(self)
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline(always)]
    pub fn in6(&mut self) -> IN6_W<6> {
        IN6_W::new(self)
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline(always)]
    pub fn in7(&mut self) -> IN7_W<7> {
        IN7_W::new(self)
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline(always)]
    pub fn isoin(&mut self) -> ISOIN_W<8> {
        ISOIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint IN enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinen](index.html) module"]
pub struct EPINEN_SPEC;
impl crate::RegisterSpec for EPINEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epinen::R](R) reader structure"]
impl crate::Readable for EPINEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epinen::W](W) writer structure"]
impl crate::Writable for EPINEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPINEN to value 0x01"]
impl crate::Resettable for EPINEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
