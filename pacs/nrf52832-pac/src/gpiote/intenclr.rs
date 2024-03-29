#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN0` reader - Write '1' to Disable interrupt for IN\\[0\\]
event"]
pub type IN0_R = crate::BitReader<IN0_A>;
#[doc = "Write '1' to Disable interrupt for IN\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
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
            false => IN0_A::DISABLED,
            true => IN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN0_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for IN\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<IN0_AW> for bool {
    #[inline(always)]
    fn from(variant: IN0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN0` writer - Write '1' to Disable interrupt for IN\\[0\\]
event"]
pub type IN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, IN0_AW, O>;
impl<'a, const O: u8> IN0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN0_AW::CLEAR)
    }
}
#[doc = "Field `IN1` reader - Write '1' to Disable interrupt for IN\\[1\\]
event"]
pub type IN1_R = crate::BitReader<IN1_A>;
#[doc = "Write '1' to Disable interrupt for IN\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
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
            false => IN1_A::DISABLED,
            true => IN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN1_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for IN\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<IN1_AW> for bool {
    #[inline(always)]
    fn from(variant: IN1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN1` writer - Write '1' to Disable interrupt for IN\\[1\\]
event"]
pub type IN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, IN1_AW, O>;
impl<'a, const O: u8> IN1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN1_AW::CLEAR)
    }
}
#[doc = "Field `IN2` reader - Write '1' to Disable interrupt for IN\\[2\\]
event"]
pub type IN2_R = crate::BitReader<IN2_A>;
#[doc = "Write '1' to Disable interrupt for IN\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
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
            false => IN2_A::DISABLED,
            true => IN2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN2_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for IN\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<IN2_AW> for bool {
    #[inline(always)]
    fn from(variant: IN2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN2` writer - Write '1' to Disable interrupt for IN\\[2\\]
event"]
pub type IN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, IN2_AW, O>;
impl<'a, const O: u8> IN2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN2_AW::CLEAR)
    }
}
#[doc = "Field `IN3` reader - Write '1' to Disable interrupt for IN\\[3\\]
event"]
pub type IN3_R = crate::BitReader<IN3_A>;
#[doc = "Write '1' to Disable interrupt for IN\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
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
            false => IN3_A::DISABLED,
            true => IN3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN3_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for IN\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<IN3_AW> for bool {
    #[inline(always)]
    fn from(variant: IN3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN3` writer - Write '1' to Disable interrupt for IN\\[3\\]
event"]
pub type IN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, IN3_AW, O>;
impl<'a, const O: u8> IN3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN3_AW::CLEAR)
    }
}
#[doc = "Field `IN4` reader - Write '1' to Disable interrupt for IN\\[4\\]
event"]
pub type IN4_R = crate::BitReader<IN4_A>;
#[doc = "Write '1' to Disable interrupt for IN\\[4\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
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
            false => IN4_A::DISABLED,
            true => IN4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN4_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for IN\\[4\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<IN4_AW> for bool {
    #[inline(always)]
    fn from(variant: IN4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN4` writer - Write '1' to Disable interrupt for IN\\[4\\]
event"]
pub type IN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, IN4_AW, O>;
impl<'a, const O: u8> IN4_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN4_AW::CLEAR)
    }
}
#[doc = "Field `IN5` reader - Write '1' to Disable interrupt for IN\\[5\\]
event"]
pub type IN5_R = crate::BitReader<IN5_A>;
#[doc = "Write '1' to Disable interrupt for IN\\[5\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN5_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
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
            false => IN5_A::DISABLED,
            true => IN5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN5_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for IN\\[5\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN5_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<IN5_AW> for bool {
    #[inline(always)]
    fn from(variant: IN5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN5` writer - Write '1' to Disable interrupt for IN\\[5\\]
event"]
pub type IN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, IN5_AW, O>;
impl<'a, const O: u8> IN5_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN5_AW::CLEAR)
    }
}
#[doc = "Field `IN6` reader - Write '1' to Disable interrupt for IN\\[6\\]
event"]
pub type IN6_R = crate::BitReader<IN6_A>;
#[doc = "Write '1' to Disable interrupt for IN\\[6\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN6_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
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
            false => IN6_A::DISABLED,
            true => IN6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN6_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for IN\\[6\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN6_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<IN6_AW> for bool {
    #[inline(always)]
    fn from(variant: IN6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN6` writer - Write '1' to Disable interrupt for IN\\[6\\]
event"]
pub type IN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, IN6_AW, O>;
impl<'a, const O: u8> IN6_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN6_AW::CLEAR)
    }
}
#[doc = "Field `IN7` reader - Write '1' to Disable interrupt for IN\\[7\\]
event"]
pub type IN7_R = crate::BitReader<IN7_A>;
#[doc = "Write '1' to Disable interrupt for IN\\[7\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN7_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
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
            false => IN7_A::DISABLED,
            true => IN7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN7_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for IN\\[7\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN7_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<IN7_AW> for bool {
    #[inline(always)]
    fn from(variant: IN7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN7` writer - Write '1' to Disable interrupt for IN\\[7\\]
event"]
pub type IN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, IN7_AW, O>;
impl<'a, const O: u8> IN7_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN7_AW::CLEAR)
    }
}
#[doc = "Field `PORT` reader - Write '1' to Disable interrupt for PORT event"]
pub type PORT_R = crate::BitReader<PORT_A>;
#[doc = "Write '1' to Disable interrupt for PORT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PORT_A> for bool {
    #[inline(always)]
    fn from(variant: PORT_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_A {
        match self.bits {
            false => PORT_A::DISABLED,
            true => PORT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PORT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PORT_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for PORT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PORT_AW> for bool {
    #[inline(always)]
    fn from(variant: PORT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT` writer - Write '1' to Disable interrupt for PORT event"]
pub type PORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, PORT_AW, O>;
impl<'a, const O: u8> PORT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PORT_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for IN\\[0\\]
event"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for IN\\[1\\]
event"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for IN\\[2\\]
event"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for IN\\[3\\]
event"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for IN\\[4\\]
event"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for IN\\[5\\]
event"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for IN\\[6\\]
event"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for IN\\[7\\]
event"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Write '1' to Disable interrupt for PORT event"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for IN\\[0\\]
event"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W<0> {
        IN0_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for IN\\[1\\]
event"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W<1> {
        IN1_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for IN\\[2\\]
event"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W<2> {
        IN2_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for IN\\[3\\]
event"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W<3> {
        IN3_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for IN\\[4\\]
event"]
    #[inline(always)]
    pub fn in4(&mut self) -> IN4_W<4> {
        IN4_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for IN\\[5\\]
event"]
    #[inline(always)]
    pub fn in5(&mut self) -> IN5_W<5> {
        IN5_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for IN\\[6\\]
event"]
    #[inline(always)]
    pub fn in6(&mut self) -> IN6_W<6> {
        IN6_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for IN\\[7\\]
event"]
    #[inline(always)]
    pub fn in7(&mut self) -> IN7_W<7> {
        IN7_W::new(self)
    }
    #[doc = "Bit 31 - Write '1' to Disable interrupt for PORT event"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W<31> {
        PORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
