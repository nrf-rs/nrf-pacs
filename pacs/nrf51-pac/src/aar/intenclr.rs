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
#[doc = "Disable interrupt on ENDKSGEN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - Disable interrupt on ENDKSGEN event."]
pub struct END_R(crate::FieldReader<bool, END_A>);
impl END_R {
    pub(crate) fn new(bits: bool) -> Self {
        END_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_A {
        match self.bits {
            false => END_A::DISABLED,
            true => END_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == END_A::ENABLED
    }
}
impl core::ops::Deref for END_R {
    type Target = crate::FieldReader<bool, END_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on ENDKSGEN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Disable interrupt on ENDKSGEN event."]
pub struct END_W<'a> {
    w: &'a mut W,
}
impl<'a> END_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(END_AW::CLEAR)
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
#[doc = "Disable interrupt on RESOLVED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLVED_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<RESOLVED_A> for bool {
    #[inline(always)]
    fn from(variant: RESOLVED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESOLVED` reader - Disable interrupt on RESOLVED event."]
pub struct RESOLVED_R(crate::FieldReader<bool, RESOLVED_A>);
impl RESOLVED_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESOLVED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESOLVED_A {
        match self.bits {
            false => RESOLVED_A::DISABLED,
            true => RESOLVED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RESOLVED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RESOLVED_A::ENABLED
    }
}
impl core::ops::Deref for RESOLVED_R {
    type Target = crate::FieldReader<bool, RESOLVED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on RESOLVED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLVED_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<RESOLVED_AW> for bool {
    #[inline(always)]
    fn from(variant: RESOLVED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESOLVED` writer - Disable interrupt on RESOLVED event."]
pub struct RESOLVED_W<'a> {
    w: &'a mut W,
}
impl<'a> RESOLVED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESOLVED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RESOLVED_AW::CLEAR)
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
#[doc = "Disable interrupt on NOTRESOLVED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRESOLVED_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<NOTRESOLVED_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRESOLVED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTRESOLVED` reader - Disable interrupt on NOTRESOLVED event."]
pub struct NOTRESOLVED_R(crate::FieldReader<bool, NOTRESOLVED_A>);
impl NOTRESOLVED_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOTRESOLVED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTRESOLVED_A {
        match self.bits {
            false => NOTRESOLVED_A::DISABLED,
            true => NOTRESOLVED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NOTRESOLVED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NOTRESOLVED_A::ENABLED
    }
}
impl core::ops::Deref for NOTRESOLVED_R {
    type Target = crate::FieldReader<bool, NOTRESOLVED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on NOTRESOLVED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRESOLVED_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<NOTRESOLVED_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTRESOLVED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTRESOLVED` writer - Disable interrupt on NOTRESOLVED event."]
pub struct NOTRESOLVED_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTRESOLVED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOTRESOLVED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NOTRESOLVED_AW::CLEAR)
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
    #[doc = "Bit 0 - Disable interrupt on ENDKSGEN event."]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on RESOLVED event."]
    #[inline(always)]
    pub fn resolved(&self) -> RESOLVED_R {
        RESOLVED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on NOTRESOLVED event."]
    #[inline(always)]
    pub fn notresolved(&self) -> NOTRESOLVED_R {
        NOTRESOLVED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on ENDKSGEN event."]
    #[inline(always)]
    pub fn end(&mut self) -> END_W {
        END_W { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on RESOLVED event."]
    #[inline(always)]
    pub fn resolved(&mut self) -> RESOLVED_W {
        RESOLVED_W { w: self }
    }
    #[doc = "Bit 2 - Disable interrupt on NOTRESOLVED event."]
    #[inline(always)]
    pub fn notresolved(&mut self) -> NOTRESOLVED_W {
        NOTRESOLVED_W { w: self }
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
