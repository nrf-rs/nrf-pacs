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
#[doc = "Field `RAMACCERR` reader - Write '1' to disable interrupt for event RAMACCERR"]
pub type RAMACCERR_R = crate::BitReader<RAMACCERR_A>;
#[doc = "Write '1' to disable interrupt for event RAMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMACCERR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RAMACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: RAMACCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMACCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMACCERR_A {
        match self.bits {
            false => RAMACCERR_A::DISABLED,
            true => RAMACCERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAMACCERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAMACCERR_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RAMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMACCERR_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RAMACCERR_AW> for bool {
    #[inline(always)]
    fn from(variant: RAMACCERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMACCERR` writer - Write '1' to disable interrupt for event RAMACCERR"]
pub type RAMACCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, RAMACCERR_AW, O>;
impl<'a, const O: u8> RAMACCERR_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RAMACCERR_AW::CLEAR)
    }
}
#[doc = "Field `FLASHACCERR` reader - Write '1' to disable interrupt for event FLASHACCERR"]
pub type FLASHACCERR_R = crate::BitReader<FLASHACCERR_A>;
#[doc = "Write '1' to disable interrupt for event FLASHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHACCERR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<FLASHACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHACCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHACCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHACCERR_A {
        match self.bits {
            false => FLASHACCERR_A::DISABLED,
            true => FLASHACCERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHACCERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHACCERR_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event FLASHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHACCERR_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<FLASHACCERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FLASHACCERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHACCERR` writer - Write '1' to disable interrupt for event FLASHACCERR"]
pub type FLASHACCERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, FLASHACCERR_AW, O>;
impl<'a, const O: u8> FLASHACCERR_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLASHACCERR_AW::CLEAR)
    }
}
#[doc = "Field `PERIPHACCERR` reader - Write '1' to disable interrupt for event PERIPHACCERR"]
pub type PERIPHACCERR_R = crate::BitReader<PERIPHACCERR_A>;
#[doc = "Write '1' to disable interrupt for event PERIPHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHACCERR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PERIPHACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPHACCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PERIPHACCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPHACCERR_A {
        match self.bits {
            false => PERIPHACCERR_A::DISABLED,
            true => PERIPHACCERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PERIPHACCERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PERIPHACCERR_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event PERIPHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHACCERR_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PERIPHACCERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PERIPHACCERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIPHACCERR` writer - Write '1' to disable interrupt for event PERIPHACCERR"]
pub type PERIPHACCERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, PERIPHACCERR_AW, O>;
impl<'a, const O: u8> PERIPHACCERR_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PERIPHACCERR_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event RAMACCERR"]
    #[inline(always)]
    pub fn ramaccerr(&self) -> RAMACCERR_R {
        RAMACCERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event FLASHACCERR"]
    #[inline(always)]
    pub fn flashaccerr(&self) -> FLASHACCERR_R {
        FLASHACCERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn periphaccerr(&self) -> PERIPHACCERR_R {
        PERIPHACCERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event RAMACCERR"]
    #[inline(always)]
    pub fn ramaccerr(&mut self) -> RAMACCERR_W<0> {
        RAMACCERR_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event FLASHACCERR"]
    #[inline(always)]
    pub fn flashaccerr(&mut self) -> FLASHACCERR_W<1> {
        FLASHACCERR_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn periphaccerr(&mut self) -> PERIPHACCERR_W<2> {
        PERIPHACCERR_W::new(self)
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
