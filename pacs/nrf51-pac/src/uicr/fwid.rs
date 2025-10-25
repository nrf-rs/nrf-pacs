#[doc = "Register `FWID` reader"]
pub type R = crate::R<FwidSpec>;
#[doc = "Field `FWID` reader - Identification number for the firmware loaded into the chip."]
pub type FwidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Identification number for the firmware loaded into the chip."]
    #[inline(always)]
    pub fn fwid(&self) -> FwidR {
        FwidR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Firmware ID.\n\nYou can [`read`](crate::Reg::read) this register and get [`fwid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwidSpec;
impl crate::RegisterSpec for FwidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwid::R`](R) reader structure"]
impl crate::Readable for FwidSpec {}
#[doc = "`reset()` method sets FWID to value 0xffff_ffff"]
impl crate::Resettable for FwidSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
