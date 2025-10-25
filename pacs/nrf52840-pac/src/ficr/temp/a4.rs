#[doc = "Register `A4` reader"]
pub type R = crate::R<A4Spec>;
#[doc = "Field `A` reader - A (slope definition) register."]
pub type AR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - A (slope definition) register."]
    #[inline(always)]
    pub fn a(&self) -> AR {
        AR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Slope definition A4\n\nYou can [`read`](crate::Reg::read) this register and get [`a4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A4Spec;
impl crate::RegisterSpec for A4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a4::R`](R) reader structure"]
impl crate::Readable for A4Spec {}
#[doc = "`reset()` method sets A4 to value 0xffff_ffff"]
impl crate::Resettable for A4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
