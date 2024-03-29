#[doc = "Register `DEST` reader"]
pub struct R(crate::R<DEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEST` writer"]
pub struct W(crate::W<DEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEST_SPEC>;
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
impl From<crate::W<DEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEST` reader - Secure APB destination address"]
pub type DEST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DEST` writer - Secure APB destination address"]
pub type DEST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Secure APB destination address"]
    #[inline(always)]
    pub fn dest(&self) -> DEST_R {
        DEST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure APB destination address"]
    #[inline(always)]
    pub fn dest(&mut self) -> DEST_W<0> {
        DEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address must match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dest](index.html) module"]
pub struct DEST_SPEC;
impl crate::RegisterSpec for DEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dest::R](R) reader structure"]
impl crate::Readable for DEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dest::W](W) writer structure"]
impl crate::Writable for DEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEST to value 0xffff_ffff"]
impl crate::Resettable for DEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
