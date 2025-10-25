#[doc = "Register `VARIANT` reader"]
pub type R = crate::R<VariantSpec>;
#[doc = "Field `VARIANT` reader - For valid values see SoC revisions and variants."]
pub type VariantR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - For valid values see SoC revisions and variants."]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(self.bits)
    }
}
#[doc = "Build code, last two letters of Package Variant and first two characters of Build Code, encoded in ASCII.\n\nYou can [`read`](crate::Reg::read) this register and get [`variant::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VariantSpec;
impl crate::RegisterSpec for VariantSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`variant::R`](R) reader structure"]
impl crate::Readable for VariantSpec {}
#[doc = "`reset()` method sets VARIANT to value 0xffff_ffff"]
impl crate::Resettable for VariantSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
