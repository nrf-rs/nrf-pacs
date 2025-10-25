#[doc = "Register `DEVARCH` reader"]
pub type R = crate::R<DevarchSpec>;
#[doc = "Field `Architecture` reader - Contains the CTI device architecture."]
pub type ArchitectureR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Contains the CTI device architecture."]
    #[inline(always)]
    pub fn architecture(&self) -> ArchitectureR {
        ArchitectureR::new((self.bits & 1) != 0)
    }
}
#[doc = "Device Architecture register\n\nYou can [`read`](crate::Reg::read) this register and get [`devarch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevarchSpec;
impl crate::RegisterSpec for DevarchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devarch::R`](R) reader structure"]
impl crate::Readable for DevarchSpec {}
#[doc = "`reset()` method sets DEVARCH to value 0x4770_1a14"]
impl crate::Resettable for DevarchSpec {
    const RESET_VALUE: u32 = 0x4770_1a14;
}
