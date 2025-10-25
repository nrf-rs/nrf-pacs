#[doc = "Register `CONFIGID` reader"]
pub type R = crate::R<ConfigidSpec>;
#[doc = "Field `HWID` reader - Identification number for the HW"]
pub type HwidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Identification number for the HW"]
    #[inline(always)]
    pub fn hwid(&self) -> HwidR {
        HwidR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Configuration identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`configid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigidSpec;
impl crate::RegisterSpec for ConfigidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`configid::R`](R) reader structure"]
impl crate::Readable for ConfigidSpec {}
#[doc = "`reset()` method sets CONFIGID to value 0xffff_ffff"]
impl crate::Resettable for ConfigidSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
