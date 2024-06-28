#[doc = "Register `ROSC4` reader"]
pub type R = crate::R<Rosc4Spec>;
#[doc = "Field `ROSC4` reader - Sample count for ring oscillator 4"]
pub type Rosc4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 4"]
    #[inline(always)]
    pub fn rosc4(&self) -> Rosc4R {
        Rosc4R::new(self.bits)
    }
}
#[doc = "Sample count for ring oscillator 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rosc4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rosc4Spec;
impl crate::RegisterSpec for Rosc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rosc4::R`](R) reader structure"]
impl crate::Readable for Rosc4Spec {}
#[doc = "`reset()` method sets ROSC4 to value 0xffff_ffff"]
impl crate::Resettable for Rosc4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
