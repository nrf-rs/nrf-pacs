#[doc = "Register `CONFIGID` reader"]
pub type R = crate::R<ConfigidSpec>;
#[doc = "Field `HWID` reader - Hardware Identification Number."]
pub type HwidR = crate::FieldReader<u16>;
#[doc = "Field `FWID` reader - Firmware Identification Number pre-loaded into the flash."]
pub type FwidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Hardware Identification Number."]
    #[inline(always)]
    pub fn hwid(&self) -> HwidR {
        HwidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Firmware Identification Number pre-loaded into the flash."]
    #[inline(always)]
    pub fn fwid(&self) -> FwidR {
        FwidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Configuration identifier.\n\nYou can [`read`](crate::Reg::read) this register and get [`configid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
