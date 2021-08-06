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
#[doc = "Disable interrupt on IN\\[0\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<IN0_A> for bool {
    #[inline(always)]
    fn from(variant: IN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN0` reader - Disable interrupt on IN\\[0\\]
event."]
pub struct IN0_R(crate::FieldReader<bool, IN0_A>);
impl IN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == IN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IN0_A::ENABLED
    }
}
impl core::ops::Deref for IN0_R {
    type Target = crate::FieldReader<bool, IN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on IN\\[0\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<IN0_AW> for bool {
    #[inline(always)]
    fn from(variant: IN0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN0` writer - Disable interrupt on IN\\[0\\]
event."]
pub struct IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN0_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Disable interrupt on IN\\[1\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<IN1_A> for bool {
    #[inline(always)]
    fn from(variant: IN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN1` reader - Disable interrupt on IN\\[1\\]
event."]
pub struct IN1_R(crate::FieldReader<bool, IN1_A>);
impl IN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == IN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IN1_A::ENABLED
    }
}
impl core::ops::Deref for IN1_R {
    type Target = crate::FieldReader<bool, IN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on IN\\[1\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<IN1_AW> for bool {
    #[inline(always)]
    fn from(variant: IN1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN1` writer - Disable interrupt on IN\\[1\\]
event."]
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN1_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Disable interrupt on IN\\[2\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<IN2_A> for bool {
    #[inline(always)]
    fn from(variant: IN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN2` reader - Disable interrupt on IN\\[2\\]
event."]
pub struct IN2_R(crate::FieldReader<bool, IN2_A>);
impl IN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == IN2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IN2_A::ENABLED
    }
}
impl core::ops::Deref for IN2_R {
    type Target = crate::FieldReader<bool, IN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on IN\\[2\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<IN2_AW> for bool {
    #[inline(always)]
    fn from(variant: IN2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN2` writer - Disable interrupt on IN\\[2\\]
event."]
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN2_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Disable interrupt on IN\\[3\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<IN3_A> for bool {
    #[inline(always)]
    fn from(variant: IN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN3` reader - Disable interrupt on IN\\[3\\]
event."]
pub struct IN3_R(crate::FieldReader<bool, IN3_A>);
impl IN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == IN3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IN3_A::ENABLED
    }
}
impl core::ops::Deref for IN3_R {
    type Target = crate::FieldReader<bool, IN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on IN\\[3\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<IN3_AW> for bool {
    #[inline(always)]
    fn from(variant: IN3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN3` writer - Disable interrupt on IN\\[3\\]
event."]
pub struct IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IN3_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Disable interrupt on PORT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<PORT_A> for bool {
    #[inline(always)]
    fn from(variant: PORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT` reader - Disable interrupt on PORT event."]
pub struct PORT_R(crate::FieldReader<bool, PORT_A>);
impl PORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PORT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PORT_A::ENABLED
    }
}
impl core::ops::Deref for PORT_R {
    type Target = crate::FieldReader<bool, PORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on PORT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<PORT_AW> for bool {
    #[inline(always)]
    fn from(variant: PORT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT` writer - Disable interrupt on PORT event."]
pub struct PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PORT_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Disable interrupt on IN\\[0\\]
event."]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on IN\\[1\\]
event."]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on IN\\[2\\]
event."]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable interrupt on IN\\[3\\]
event."]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Disable interrupt on PORT event."]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on IN\\[0\\]
event."]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W {
        IN0_W { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on IN\\[1\\]
event."]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W {
        IN1_W { w: self }
    }
    #[doc = "Bit 2 - Disable interrupt on IN\\[2\\]
event."]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W {
        IN2_W { w: self }
    }
    #[doc = "Bit 3 - Disable interrupt on IN\\[3\\]
event."]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W {
        IN3_W { w: self }
    }
    #[doc = "Bit 31 - Disable interrupt on PORT event."]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable clear register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
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
