#[doc = "Register `SELECTKEYSLOT` reader"]
pub struct R(crate::R<SELECTKEYSLOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SELECTKEYSLOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SELECTKEYSLOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SELECTKEYSLOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SELECTKEYSLOT` writer"]
pub struct W(crate::W<SELECTKEYSLOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SELECTKEYSLOT_SPEC>;
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
impl From<crate::W<SELECTKEYSLOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SELECTKEYSLOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\]
and UICR-&gt;KEYSLOT.CONFIG\\[N\\]
corresponds to KMU key slot ID=N+1."]
pub type ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ID` writer - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\]
and UICR-&gt;KEYSLOT.CONFIG\\[N\\]
corresponds to KMU key slot ID=N+1."]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SELECTKEYSLOT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\]
and UICR-&gt;KEYSLOT.CONFIG\\[N\\]
corresponds to KMU key slot ID=N+1."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\]
and UICR-&gt;KEYSLOT.CONFIG\\[N\\]
corresponds to KMU key slot ID=N+1."]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [selectkeyslot](index.html) module"]
pub struct SELECTKEYSLOT_SPEC;
impl crate::RegisterSpec for SELECTKEYSLOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [selectkeyslot::R](R) reader structure"]
impl crate::Readable for SELECTKEYSLOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [selectkeyslot::W](W) writer structure"]
impl crate::Writable for SELECTKEYSLOT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SELECTKEYSLOT to value 0"]
impl crate::Resettable for SELECTKEYSLOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
