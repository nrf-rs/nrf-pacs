#[doc = "Register `ROSC1` reader"]
pub type R = crate::R<Rosc1Spec>;
#[doc = "Field `ROSC1` reader - Sample count for ring oscillator 1"]
pub type Rosc1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 1"]
    #[inline(always)]
    pub fn rosc1(&self) -> Rosc1R {
        Rosc1R::new(self.bits)
    }
}
#[doc = "Sample count for ring oscillator 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rosc1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rosc1Spec;
impl crate::RegisterSpec for Rosc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rosc1::R`](R) reader structure"]
impl crate::Readable for Rosc1Spec {}
#[doc = "`reset()` method sets ROSC1 to value 0xffff_ffff"]
impl crate::Resettable for Rosc1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
