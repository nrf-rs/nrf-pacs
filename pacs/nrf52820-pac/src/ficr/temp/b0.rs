#[doc = "Register `B0` reader"]
pub type R = crate::R<B0Spec>;
#[doc = "Field `B` reader - B (y-intercept)"]
pub type BR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - B (y-intercept)"]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Y-intercept B0\n\nYou can [`read`](crate::Reg::read) this register and get [`b0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B0Spec;
impl crate::RegisterSpec for B0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b0::R`](R) reader structure"]
impl crate::Readable for B0Spec {}
#[doc = "`reset()` method sets B0 to value 0xffff_ffff"]
impl crate::Resettable for B0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
