#[doc = "Register `REFRESH` reader"]
pub struct R(crate::R<REFRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFRESH` writer"]
pub struct W(crate::W<REFRESH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFRESH_SPEC>;
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
impl From<crate::W<REFRESH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFRESH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CNT_A {
    #[doc = "0: Update every PWM period"]
    CONTINUOUS = 0,
}
impl From<CNT_A> for u32 {
    #[inline(always)]
    fn from(variant: CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNT` reader - Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
pub struct CNT_R(crate::FieldReader<u32, CNT_A>);
impl CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNT_A> {
        match self.bits {
            0 => Some(CNT_A::CONTINUOUS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == CNT_A::CONTINUOUS
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u32, CNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Update every PWM period"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CNT_A::CONTINUOUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refresh](index.html) module"]
pub struct REFRESH_SPEC;
impl crate::RegisterSpec for REFRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refresh::R](R) reader structure"]
impl crate::Readable for REFRESH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refresh::W](W) writer structure"]
impl crate::Writable for REFRESH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFRESH to value 0x01"]
impl crate::Resettable for REFRESH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
