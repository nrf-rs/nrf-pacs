#[doc = "Register `POFCON` reader"]
pub struct R(crate::R<POFCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POFCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POFCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POFCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POFCON` writer"]
pub struct W(crate::W<POFCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POFCON_SPEC>;
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
impl From<crate::W<POFCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POFCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POF` reader - Power failure comparator enable."]
pub type POF_R = crate::BitReader<POF_A>;
#[doc = "Power failure comparator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POF_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<POF_A> for bool {
    #[inline(always)]
    fn from(variant: POF_A) -> Self {
        variant as u8 != 0
    }
}
impl POF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POF_A {
        match self.bits {
            false => POF_A::DISABLED,
            true => POF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POF_A::ENABLED
    }
}
#[doc = "Field `POF` writer - Power failure comparator enable."]
pub type POF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POFCON_SPEC, POF_A, O>;
impl<'a, const O: u8> POF_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POF_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POF_A::ENABLED)
    }
}
#[doc = "Field `THRESHOLD` reader - Set threshold level."]
pub type THRESHOLD_R = crate::FieldReader<u8, THRESHOLD_A>;
#[doc = "Set threshold level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THRESHOLD_A {
    #[doc = "0: Set threshold to 2.1Volts."]
    V21 = 0,
    #[doc = "1: Set threshold to 2.3Volts."]
    V23 = 1,
    #[doc = "2: Set threshold to 2.5Volts."]
    V25 = 2,
    #[doc = "3: Set threshold to 2.7Volts."]
    V27 = 3,
}
impl From<THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLD_A) -> Self {
        variant as _
    }
}
impl THRESHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRESHOLD_A {
        match self.bits {
            0 => THRESHOLD_A::V21,
            1 => THRESHOLD_A::V23,
            2 => THRESHOLD_A::V25,
            3 => THRESHOLD_A::V27,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V21`"]
    #[inline(always)]
    pub fn is_v21(&self) -> bool {
        *self == THRESHOLD_A::V21
    }
    #[doc = "Checks if the value of the field is `V23`"]
    #[inline(always)]
    pub fn is_v23(&self) -> bool {
        *self == THRESHOLD_A::V23
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        *self == THRESHOLD_A::V25
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        *self == THRESHOLD_A::V27
    }
}
#[doc = "Field `THRESHOLD` writer - Set threshold level."]
pub type THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, POFCON_SPEC, u8, THRESHOLD_A, 2, O>;
impl<'a, const O: u8> THRESHOLD_W<'a, O> {
    #[doc = "Set threshold to 2.1Volts."]
    #[inline(always)]
    pub fn v21(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V21)
    }
    #[doc = "Set threshold to 2.3Volts."]
    #[inline(always)]
    pub fn v23(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V23)
    }
    #[doc = "Set threshold to 2.5Volts."]
    #[inline(always)]
    pub fn v25(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V25)
    }
    #[doc = "Set threshold to 2.7Volts."]
    #[inline(always)]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V27)
    }
}
impl R {
    #[doc = "Bit 0 - Power failure comparator enable."]
    #[inline(always)]
    pub fn pof(&self) -> POF_R {
        POF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Set threshold level."]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power failure comparator enable."]
    #[inline(always)]
    pub fn pof(&mut self) -> POF_W<0> {
        POF_W::new(self)
    }
    #[doc = "Bits 1:2 - Set threshold level."]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W<1> {
        THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power failure configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pofcon](index.html) module"]
pub struct POFCON_SPEC;
impl crate::RegisterSpec for POFCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pofcon::R](R) reader structure"]
impl crate::Readable for POFCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pofcon::W](W) writer structure"]
impl crate::Writable for POFCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POFCON to value 0"]
impl crate::Resettable for POFCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
