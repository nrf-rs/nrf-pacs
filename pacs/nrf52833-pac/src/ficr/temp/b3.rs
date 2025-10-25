#[doc = "Register `B3` reader"]
pub type R = crate::R<B3Spec>;
#[doc = "Field `B` reader - B (y-intercept)"]
pub type BR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - B (y-intercept)"]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Y-intercept B3\n\nYou can [`read`](crate::Reg::read) this register and get [`b3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B3Spec;
impl crate::RegisterSpec for B3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b3::R`](R) reader structure"]
impl crate::Readable for B3Spec {}
#[doc = "`reset()` method sets B3 to value 0xffff_ffff"]
impl crate::Resettable for B3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
