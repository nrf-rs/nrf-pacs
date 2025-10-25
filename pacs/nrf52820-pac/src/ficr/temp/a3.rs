#[doc = "Register `A3` reader"]
pub type R = crate::R<A3Spec>;
#[doc = "Field `A` reader - A (slope definition) register."]
pub type AR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - A (slope definition) register."]
    #[inline(always)]
    pub fn a(&self) -> AR {
        AR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Slope definition A3\n\nYou can [`read`](crate::Reg::read) this register and get [`a3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A3Spec;
impl crate::RegisterSpec for A3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a3::R`](R) reader structure"]
impl crate::Readable for A3Spec {}
#[doc = "`reset()` method sets A3 to value 0xffff_ffff"]
impl crate::Resettable for A3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
