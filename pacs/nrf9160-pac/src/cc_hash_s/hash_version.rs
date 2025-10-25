#[doc = "Register `HASH_VERSION` reader"]
pub type R = crate::R<HashVersionSpec>;
#[doc = "Field `PATCH` reader - "]
pub type PatchR = crate::FieldReader;
#[doc = "Field `MINOR_VERSION_NUMBER` reader - Minor version number"]
pub type MinorVersionNumberR = crate::FieldReader;
#[doc = "Field `MAJOR_VERSION_NUMBER` reader - Major version number"]
pub type MajorVersionNumberR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn patch(&self) -> PatchR {
        PatchR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor version number"]
    #[inline(always)]
    pub fn minor_version_number(&self) -> MinorVersionNumberR {
        MinorVersionNumberR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major version number"]
    #[inline(always)]
    pub fn major_version_number(&self) -> MajorVersionNumberR {
        MajorVersionNumberR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "HASH engine HW version\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashVersionSpec;
impl crate::RegisterSpec for HashVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_version::R`](R) reader structure"]
impl crate::Readable for HashVersionSpec {}
#[doc = "`reset()` method sets HASH_VERSION to value 0"]
impl crate::Resettable for HashVersionSpec {}
