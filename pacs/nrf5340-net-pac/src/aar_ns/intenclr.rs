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
#[doc = "Field `END` reader - Write '1' to disable interrupt for event END"]
pub type END_R = crate::BitReader<END_A>;
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
impl END_R {
    #[doc = "Get enumerated values variant"]
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
        *self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to disable interrupt for event END"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, END_AW, O>;
impl<'a, const O: u8> END_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(END_AW::CLEAR)
    }
}
#[doc = "Field `RESOLVED` reader - Write '1' to disable interrupt for event RESOLVED"]
pub type RESOLVED_R = crate::BitReader<RESOLVED_A>;
#[doc = "Write '1' to disable interrupt for event RESOLVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLVED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RESOLVED_A> for bool {
    #[inline(always)]
    fn from(variant: RESOLVED_A) -> Self {
        variant as u8 != 0
    }
}
impl RESOLVED_R {
    #[doc = "Get enumerated values variant"]
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
        *self == RESOLVED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESOLVED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RESOLVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLVED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RESOLVED_AW> for bool {
    #[inline(always)]
    fn from(variant: RESOLVED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESOLVED` writer - Write '1' to disable interrupt for event RESOLVED"]
pub type RESOLVED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, RESOLVED_AW, O>;
impl<'a, const O: u8> RESOLVED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RESOLVED_AW::CLEAR)
    }
}
#[doc = "Field `NOTRESOLVED` reader - Write '1' to disable interrupt for event NOTRESOLVED"]
pub type NOTRESOLVED_R = crate::BitReader<NOTRESOLVED_A>;
#[doc = "Write '1' to disable interrupt for event NOTRESOLVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRESOLVED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<NOTRESOLVED_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRESOLVED_A) -> Self {
        variant as u8 != 0
    }
}
impl NOTRESOLVED_R {
    #[doc = "Get enumerated values variant"]
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
        *self == NOTRESOLVED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOTRESOLVED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event NOTRESOLVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRESOLVED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<NOTRESOLVED_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTRESOLVED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTRESOLVED` writer - Write '1' to disable interrupt for event NOTRESOLVED"]
pub type NOTRESOLVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, NOTRESOLVED_AW, O>;
impl<'a, const O: u8> NOTRESOLVED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NOTRESOLVED_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event RESOLVED"]
    #[inline(always)]
    pub fn resolved(&self) -> RESOLVED_R {
        RESOLVED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event NOTRESOLVED"]
    #[inline(always)]
    pub fn notresolved(&self) -> NOTRESOLVED_R {
        NOTRESOLVED_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W<0> {
        END_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event RESOLVED"]
    #[inline(always)]
    pub fn resolved(&mut self) -> RESOLVED_W<1> {
        RESOLVED_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event NOTRESOLVED"]
    #[inline(always)]
    pub fn notresolved(&mut self) -> NOTRESOLVED_W<2> {
        NOTRESOLVED_W::new(self)
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
