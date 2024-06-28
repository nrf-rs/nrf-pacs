#[doc = "Register `ROSC2` reader"]
pub type R = crate::R<Rosc2Spec>;
#[doc = "Field `ROSC2` reader - Sample count for ring oscillator 2"]
pub type Rosc2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 2"]
    #[inline(always)]
    pub fn rosc2(&self) -> Rosc2R {
        Rosc2R::new(self.bits)
    }
}
#[doc = "Sample count for ring oscillator 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rosc2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rosc2Spec;
impl crate::RegisterSpec for Rosc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rosc2::R`](R) reader structure"]
impl crate::Readable for Rosc2Spec {}
#[doc = "`reset()` method sets ROSC2 to value 0xffff_ffff"]
impl crate::Resettable for Rosc2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
