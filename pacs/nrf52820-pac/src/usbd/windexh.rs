#[doc = "Register `WINDEXH` reader"]
pub type R = crate::R<WindexhSpec>;
#[doc = "Field `WINDEXH` reader - SETUP data, byte 5, MSB of wIndex"]
pub type WindexhR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 5, MSB of wIndex"]
    #[inline(always)]
    pub fn windexh(&self) -> WindexhR {
        WindexhR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 5, MSB of wIndex\n\nYou can [`read`](crate::Reg::read) this register and get [`windexh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WindexhSpec;
impl crate::RegisterSpec for WindexhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`windexh::R`](R) reader structure"]
impl crate::Readable for WindexhSpec {}
#[doc = "`reset()` method sets WINDEXH to value 0"]
impl crate::Resettable for WindexhSpec {}
