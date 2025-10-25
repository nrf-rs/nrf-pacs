#[doc = "Register `WINDEXL` reader"]
pub type R = crate::R<WindexlSpec>;
#[doc = "Field `WINDEXL` reader - SETUP data, byte 4, LSB of wIndex"]
pub type WindexlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 4, LSB of wIndex"]
    #[inline(always)]
    pub fn windexl(&self) -> WindexlR {
        WindexlR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 4, LSB of wIndex\n\nYou can [`read`](crate::Reg::read) this register and get [`windexl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WindexlSpec;
impl crate::RegisterSpec for WindexlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`windexl::R`](R) reader structure"]
impl crate::Readable for WindexlSpec {}
#[doc = "`reset()` method sets WINDEXL to value 0"]
impl crate::Resettable for WindexlSpec {}
