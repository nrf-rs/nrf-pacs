#[doc = "Register `DEVTYPE` reader"]
pub type R = crate::R<DevtypeSpec>;
#[doc = "Field `MAJOR_TYPE` reader - Major classification grouping for this debug/trace component"]
pub type MajorTypeR = crate::FieldReader;
#[doc = "Field `SUB_TYPE` reader - Sub-classification within the major category"]
pub type SubTypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Major classification grouping for this debug/trace component"]
    #[inline(always)]
    pub fn major_type(&self) -> MajorTypeR {
        MajorTypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub-classification within the major category"]
    #[inline(always)]
    pub fn sub_type(&self) -> SubTypeR {
        SubTypeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Device Type Identifier Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevtypeSpec;
impl crate::RegisterSpec for DevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devtype::R`](R) reader structure"]
impl crate::Readable for DevtypeSpec {}
#[doc = "`reset()` method sets DEVTYPE to value 0x21"]
impl crate::Resettable for DevtypeSpec {
    const RESET_VALUE: u32 = 0x21;
}
