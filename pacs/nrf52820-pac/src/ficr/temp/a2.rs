#[doc = "Register `A2` reader"]
pub type R = crate::R<A2Spec>;
#[doc = "Field `A` reader - A (slope definition) register."]
pub type AR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - A (slope definition) register."]
    #[inline(always)]
    pub fn a(&self) -> AR {
        AR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Slope definition A2\n\nYou can [`read`](crate::Reg::read) this register and get [`a2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A2Spec;
impl crate::RegisterSpec for A2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a2::R`](R) reader structure"]
impl crate::Readable for A2Spec {}
#[doc = "`reset()` method sets A2 to value 0xffff_ffff"]
impl crate::Resettable for A2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
