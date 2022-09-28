#[doc = "Register `PROTECT` reader"]
pub struct R(crate::R<PROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROTECT` writer"]
pub struct W(crate::W<PROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROTECT_SPEC>;
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
impl From<crate::W<PROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE0` reader - Control access to slave 0 of master EXTCODE\\[n\\]"]
pub type SLAVE0_R = crate::BitReader<SLAVE0_A>;
#[doc = "Control access to slave 0 of master EXTCODE\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE0_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE0_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE0_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE0_A {
        match self.bits {
            false => SLAVE0_A::ALLOWED,
            true => SLAVE0_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE0_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE0_A::BLOCKED
    }
}
#[doc = "Field `SLAVE0` writer - Control access to slave 0 of master EXTCODE\\[n\\]"]
pub type SLAVE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROTECT_SPEC, SLAVE0_A, O>;
impl<'a, const O: u8> SLAVE0_W<'a, O> {
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE0_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE0_A::BLOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTCODE\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&self) -> SLAVE0_R {
        SLAVE0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTCODE\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&mut self) -> SLAVE0_W<0> {
        SLAVE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Control access from master connected to AMLI master port EXTCODE\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [protect](index.html) module"]
pub struct PROTECT_SPEC;
impl crate::RegisterSpec for PROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [protect::R](R) reader structure"]
impl crate::Readable for PROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [protect::W](W) writer structure"]
impl crate::Writable for PROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROTECT to value 0"]
impl crate::Resettable for PROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
