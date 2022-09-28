#[doc = "Register `CTIGATE` reader"]
pub struct R(crate::R<CTIGATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIGATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIGATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIGATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTIGATE` writer"]
pub struct W(crate::W<CTIGATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIGATE_SPEC>;
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
impl From<crate::W<CTIGATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIGATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIGATEEN_0` reader - Enable ctichout0."]
pub type CTIGATEEN_0_R = crate::BitReader<CTIGATEEN_0_A>;
#[doc = "Enable ctichout0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_0_A {
    #[doc = "1: Enable ctichout channel 0 propagation."]
    ENABLED = 1,
    #[doc = "0: Disable ctichout channel 0 propagation."]
    DISABLED = 0,
}
impl From<CTIGATEEN_0_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_0_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIGATEEN_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_0_A {
        match self.bits {
            true => CTIGATEEN_0_A::ENABLED,
            false => CTIGATEEN_0_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTIGATEEN_0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTIGATEEN_0_A::DISABLED
    }
}
#[doc = "Field `CTIGATEEN_0` writer - Enable ctichout0."]
pub type CTIGATEEN_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIGATE_SPEC, CTIGATEEN_0_A, O>;
impl<'a, const O: u8> CTIGATEEN_0_W<'a, O> {
    #[doc = "Enable ctichout channel 0 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_0_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 0 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_0_A::DISABLED)
    }
}
#[doc = "Field `CTIGATEEN_1` reader - Enable ctichout1."]
pub type CTIGATEEN_1_R = crate::BitReader<CTIGATEEN_1_A>;
#[doc = "Enable ctichout1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_1_A {
    #[doc = "1: Enable ctichout channel 1 propagation."]
    ENABLED = 1,
    #[doc = "0: Disable ctichout channel 1 propagation."]
    DISABLED = 0,
}
impl From<CTIGATEEN_1_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIGATEEN_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_1_A {
        match self.bits {
            true => CTIGATEEN_1_A::ENABLED,
            false => CTIGATEEN_1_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTIGATEEN_1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTIGATEEN_1_A::DISABLED
    }
}
#[doc = "Field `CTIGATEEN_1` writer - Enable ctichout1."]
pub type CTIGATEEN_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIGATE_SPEC, CTIGATEEN_1_A, O>;
impl<'a, const O: u8> CTIGATEEN_1_W<'a, O> {
    #[doc = "Enable ctichout channel 1 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_1_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 1 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_1_A::DISABLED)
    }
}
#[doc = "Field `CTIGATEEN_2` reader - Enable ctichout2."]
pub type CTIGATEEN_2_R = crate::BitReader<CTIGATEEN_2_A>;
#[doc = "Enable ctichout2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_2_A {
    #[doc = "1: Enable ctichout channel 2 propagation."]
    ENABLED = 1,
    #[doc = "0: Disable ctichout channel 2 propagation."]
    DISABLED = 0,
}
impl From<CTIGATEEN_2_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_2_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIGATEEN_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_2_A {
        match self.bits {
            true => CTIGATEEN_2_A::ENABLED,
            false => CTIGATEEN_2_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTIGATEEN_2_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTIGATEEN_2_A::DISABLED
    }
}
#[doc = "Field `CTIGATEEN_2` writer - Enable ctichout2."]
pub type CTIGATEEN_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIGATE_SPEC, CTIGATEEN_2_A, O>;
impl<'a, const O: u8> CTIGATEEN_2_W<'a, O> {
    #[doc = "Enable ctichout channel 2 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_2_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 2 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_2_A::DISABLED)
    }
}
#[doc = "Field `CTIGATEEN_3` reader - Enable ctichout3."]
pub type CTIGATEEN_3_R = crate::BitReader<CTIGATEEN_3_A>;
#[doc = "Enable ctichout3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_3_A {
    #[doc = "1: Enable ctichout channel 3 propagation."]
    ENABLED = 1,
    #[doc = "0: Disable ctichout channel 3 propagation."]
    DISABLED = 0,
}
impl From<CTIGATEEN_3_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_3_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIGATEEN_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_3_A {
        match self.bits {
            true => CTIGATEEN_3_A::ENABLED,
            false => CTIGATEEN_3_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTIGATEEN_3_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTIGATEEN_3_A::DISABLED
    }
}
#[doc = "Field `CTIGATEEN_3` writer - Enable ctichout3."]
pub type CTIGATEEN_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIGATE_SPEC, CTIGATEEN_3_A, O>;
impl<'a, const O: u8> CTIGATEEN_3_W<'a, O> {
    #[doc = "Enable ctichout channel 3 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_3_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 3 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_3_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable ctichout0."]
    #[inline(always)]
    pub fn ctigateen_0(&self) -> CTIGATEEN_0_R {
        CTIGATEEN_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ctichout1."]
    #[inline(always)]
    pub fn ctigateen_1(&self) -> CTIGATEEN_1_R {
        CTIGATEEN_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable ctichout2."]
    #[inline(always)]
    pub fn ctigateen_2(&self) -> CTIGATEEN_2_R {
        CTIGATEEN_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable ctichout3."]
    #[inline(always)]
    pub fn ctigateen_3(&self) -> CTIGATEEN_3_R {
        CTIGATEEN_3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ctichout0."]
    #[inline(always)]
    pub fn ctigateen_0(&mut self) -> CTIGATEEN_0_W<0> {
        CTIGATEEN_0_W::new(self)
    }
    #[doc = "Bit 1 - Enable ctichout1."]
    #[inline(always)]
    pub fn ctigateen_1(&mut self) -> CTIGATEEN_1_W<1> {
        CTIGATEEN_1_W::new(self)
    }
    #[doc = "Bit 2 - Enable ctichout2."]
    #[inline(always)]
    pub fn ctigateen_2(&mut self) -> CTIGATEEN_2_W<2> {
        CTIGATEEN_2_W::new(self)
    }
    #[doc = "Bit 3 - Enable ctichout3."]
    #[inline(always)]
    pub fn ctigateen_3(&mut self) -> CTIGATEEN_3_W<3> {
        CTIGATEEN_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable CTI Channel Gate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctigate](index.html) module"]
pub struct CTIGATE_SPEC;
impl crate::RegisterSpec for CTIGATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctigate::R](R) reader structure"]
impl crate::Readable for CTIGATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctigate::W](W) writer structure"]
impl crate::Writable for CTIGATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIGATE to value 0x0f"]
impl crate::Resettable for CTIGATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
