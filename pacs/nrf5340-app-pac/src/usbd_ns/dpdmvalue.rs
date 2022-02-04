#[doc = "Register `DPDMVALUE` reader"]
pub struct R(crate::R<DPDMVALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPDMVALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPDMVALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPDMVALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPDMVALUE` writer"]
pub struct W(crate::W<DPDMVALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPDMVALUE_SPEC>;
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
impl From<crate::W<DPDMVALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPDMVALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "1: D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    RESUME = 1,
    #[doc = "2: D+ forced high, D- forced low (J state)"]
    J = 2,
    #[doc = "4: D+ forced low, D- forced high (K state)"]
    K = 4,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATE` reader - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
pub struct STATE_R(crate::FieldReader<u8, STATE_A>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            1 => Some(STATE_A::RESUME),
            2 => Some(STATE_A::J),
            4 => Some(STATE_A::K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        **self == STATE_A::RESUME
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        **self == STATE_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        **self == STATE_A::K
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE` writer - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(STATE_A::RESUME)
    }
    #[doc = "D+ forced high, D- forced low (J state)"]
    #[inline(always)]
    pub fn j(self) -> &'a mut W {
        self.variant(STATE_A::J)
    }
    #[doc = "D+ forced low, D- forced high (K state)"]
    #[inline(always)]
    pub fn k(self) -> &'a mut W {
        self.variant(STATE_A::K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpdmvalue](index.html) module"]
pub struct DPDMVALUE_SPEC;
impl crate::RegisterSpec for DPDMVALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpdmvalue::R](R) reader structure"]
impl crate::Readable for DPDMVALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpdmvalue::W](W) writer structure"]
impl crate::Writable for DPDMVALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPDMVALUE to value 0"]
impl crate::Resettable for DPDMVALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
