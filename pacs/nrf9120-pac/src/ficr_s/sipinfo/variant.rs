#[doc = "Register `VARIANT[%s]` reader"]
pub type R = crate::R<VariantSpec>;
#[doc = "Field `VARIANT` reader - "]
pub type VariantR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(self.bits)
    }
}
#[doc = "Description collection: SIP VARIANT, encoded in ASCII, ex SIAA, SIBA or SICA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`variant::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VariantSpec;
impl crate::RegisterSpec for VariantSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`variant::R`](R) reader structure"]
impl crate::Readable for VariantSpec {}
#[doc = "`reset()` method sets VARIANT[%s]
to value 0xff"]
impl crate::Resettable for VariantSpec {
    const RESET_VALUE: u8 = 0xff;
}
