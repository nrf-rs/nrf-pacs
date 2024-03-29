#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `END` reader - Write '1' to Enable interrupt for END event"]
pub type END_R = crate::BitReader<END_A>;
#[doc = "Write '1' to Enable interrupt for END event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for END event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to Enable interrupt for END event"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, END_AW, O>;
impl<'a, const O: u8> END_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(END_AW::SET)
    }
}
#[doc = "Field `ENDRX` reader - Write '1' to Enable interrupt for ENDRX event"]
pub type ENDRX_R = crate::BitReader<ENDRX_A>;
#[doc = "Write '1' to Enable interrupt for ENDRX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_A {
        match self.bits {
            false => ENDRX_A::DISABLED,
            true => ENDRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRX_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for ENDRX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ENDRX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` writer - Write '1' to Enable interrupt for ENDRX event"]
pub type ENDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ENDRX_AW, O>;
impl<'a, const O: u8> ENDRX_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDRX_AW::SET)
    }
}
#[doc = "Field `ACQUIRED` reader - Write '1' to Enable interrupt for ACQUIRED event"]
pub type ACQUIRED_R = crate::BitReader<ACQUIRED_A>;
#[doc = "Write '1' to Enable interrupt for ACQUIRED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACQUIRED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ACQUIRED_A> for bool {
    #[inline(always)]
    fn from(variant: ACQUIRED_A) -> Self {
        variant as u8 != 0
    }
}
impl ACQUIRED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACQUIRED_A {
        match self.bits {
            false => ACQUIRED_A::DISABLED,
            true => ACQUIRED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACQUIRED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACQUIRED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for ACQUIRED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACQUIRED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ACQUIRED_AW> for bool {
    #[inline(always)]
    fn from(variant: ACQUIRED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACQUIRED` writer - Write '1' to Enable interrupt for ACQUIRED event"]
pub type ACQUIRED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ACQUIRED_AW, O>;
impl<'a, const O: u8> ACQUIRED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ACQUIRED_AW::SET)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to Enable interrupt for END event"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to Enable interrupt for ACQUIRED event"]
    #[inline(always)]
    pub fn acquired(&self) -> ACQUIRED_R {
        ACQUIRED_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to Enable interrupt for END event"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W<1> {
        END_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W<4> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 10 - Write '1' to Enable interrupt for ACQUIRED event"]
    #[inline(always)]
    pub fn acquired(&mut self) -> ACQUIRED_W<10> {
        ACQUIRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
