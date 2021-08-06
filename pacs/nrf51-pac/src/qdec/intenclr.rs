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
#[doc = "Disable interrupt on SAMPLERDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<SAMPLERDY_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY` reader - Disable interrupt on SAMPLERDY event."]
pub struct SAMPLERDY_R(crate::FieldReader<bool, SAMPLERDY_A>);
impl SAMPLERDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAMPLERDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLERDY_A {
        match self.bits {
            false => SAMPLERDY_A::DISABLED,
            true => SAMPLERDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SAMPLERDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SAMPLERDY_A::ENABLED
    }
}
impl core::ops::Deref for SAMPLERDY_R {
    type Target = crate::FieldReader<bool, SAMPLERDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on SAMPLERDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<SAMPLERDY_AW> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY` writer - Disable interrupt on SAMPLERDY event."]
pub struct SAMPLERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLERDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLERDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SAMPLERDY_AW::CLEAR)
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
#[doc = "Disable interrupt on REPORTRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<REPORTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY` reader - Disable interrupt on REPORTRDY event."]
pub struct REPORTRDY_R(crate::FieldReader<bool, REPORTRDY_A>);
impl REPORTRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        REPORTRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_A {
        match self.bits {
            false => REPORTRDY_A::DISABLED,
            true => REPORTRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REPORTRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REPORTRDY_A::ENABLED
    }
}
impl core::ops::Deref for REPORTRDY_R {
    type Target = crate::FieldReader<bool, REPORTRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on REPORTRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<REPORTRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY` writer - Disable interrupt on REPORTRDY event."]
pub struct REPORTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> REPORTRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPORTRDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REPORTRDY_AW::CLEAR)
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
#[doc = "Disable interrupt on ACCOF event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCOF_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ACCOF_A> for bool {
    #[inline(always)]
    fn from(variant: ACCOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCOF` reader - Disable interrupt on ACCOF event."]
pub struct ACCOF_R(crate::FieldReader<bool, ACCOF_A>);
impl ACCOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCOF_A {
        match self.bits {
            false => ACCOF_A::DISABLED,
            true => ACCOF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ACCOF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ACCOF_A::ENABLED
    }
}
impl core::ops::Deref for ACCOF_R {
    type Target = crate::FieldReader<bool, ACCOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on ACCOF event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCOF_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<ACCOF_AW> for bool {
    #[inline(always)]
    fn from(variant: ACCOF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCOF` writer - Disable interrupt on ACCOF event."]
pub struct ACCOF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCOF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACCOF_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Disable interrupt on SAMPLERDY event."]
    #[inline(always)]
    pub fn samplerdy(&self) -> SAMPLERDY_R {
        SAMPLERDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on REPORTRDY event."]
    #[inline(always)]
    pub fn reportrdy(&self) -> REPORTRDY_R {
        REPORTRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on ACCOF event."]
    #[inline(always)]
    pub fn accof(&self) -> ACCOF_R {
        ACCOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on SAMPLERDY event."]
    #[inline(always)]
    pub fn samplerdy(&mut self) -> SAMPLERDY_W {
        SAMPLERDY_W { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on REPORTRDY event."]
    #[inline(always)]
    pub fn reportrdy(&mut self) -> REPORTRDY_W {
        REPORTRDY_W { w: self }
    }
    #[doc = "Bit 2 - Disable interrupt on ACCOF event."]
    #[inline(always)]
    pub fn accof(&mut self) -> ACCOF_W {
        ACCOF_W { w: self }
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
