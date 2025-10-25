#[doc = "Register `T4` reader"]
pub type R = crate::R<T4Spec>;
#[doc = "Field `T` reader - T (segment end) register"]
pub type TR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - T (segment end) register"]
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Segment end T4\n\nYou can [`read`](crate::Reg::read) this register and get [`t4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T4Spec;
impl crate::RegisterSpec for T4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t4::R`](R) reader structure"]
impl crate::Readable for T4Spec {}
#[doc = "`reset()` method sets T4 to value 0xffff_ffff"]
impl crate::Resettable for T4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
