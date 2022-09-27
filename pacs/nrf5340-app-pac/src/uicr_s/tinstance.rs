#[doc = "Register `TINSTANCE` reader"]
pub struct R(crate::R<TINSTANCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TINSTANCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TINSTANCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TINSTANCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TINSTANCE` writer"]
pub struct W(crate::W<TINSTANCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TINSTANCE_SPEC>;
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
impl From<crate::W<TINSTANCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TINSTANCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TINSTANCE` reader - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
pub type TINSTANCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TINSTANCE` writer - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
pub type TINSTANCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TINSTANCE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 28:31 - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
    #[inline(always)]
    pub fn tinstance(&self) -> TINSTANCE_R {
        TINSTANCE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
    #[inline(always)]
    pub fn tinstance(&mut self) -> TINSTANCE_W<28> {
        TINSTANCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SW-DP Target instance\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tinstance](index.html) module"]
pub struct TINSTANCE_SPEC;
impl crate::RegisterSpec for TINSTANCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tinstance::R](R) reader structure"]
impl crate::Readable for TINSTANCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tinstance::W](W) writer structure"]
impl crate::Writable for TINSTANCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TINSTANCE to value 0xffff_ffff"]
impl crate::Resettable for TINSTANCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
