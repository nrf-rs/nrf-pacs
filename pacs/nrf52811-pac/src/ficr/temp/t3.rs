#[doc = "Register `T3` reader"]
pub type R = crate::R<T3Spec>;
#[doc = "Field `T` reader - T (segment end) register"]
pub type TR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - T (segment end) register"]
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Segment end T3\n\nYou can [`read`](crate::Reg::read) this register and get [`t3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T3Spec;
impl crate::RegisterSpec for T3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t3::R`](R) reader structure"]
impl crate::Readable for T3Spec {}
#[doc = "`reset()` method sets T3 to value 0xffff_ffff"]
impl crate::Resettable for T3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
