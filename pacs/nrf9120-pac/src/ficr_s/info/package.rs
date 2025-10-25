#[doc = "Register `PACKAGE` reader"]
pub type R = crate::R<PackageSpec>;
#[doc = "Package option\n\nValue on reset: 8192"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Package {
    #[doc = "8194: CExx or CFxx - 236 ball WLCSP"]
    Cecf = 8194,
}
impl From<Package> for u32 {
    #[inline(always)]
    fn from(variant: Package) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Package {
    type Ux = u32;
}
impl crate::IsEnum for Package {}
#[doc = "Field `PACKAGE` reader - Package option"]
pub type PackageR = crate::FieldReader<Package>;
impl PackageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Package> {
        match self.bits {
            8194 => Some(Package::Cecf),
            _ => None,
        }
    }
    #[doc = "CExx or CFxx - 236 ball WLCSP"]
    #[inline(always)]
    pub fn is_cecf(&self) -> bool {
        *self == Package::Cecf
    }
}
impl R {
    #[doc = "Bits 0:31 - Package option"]
    #[inline(always)]
    pub fn package(&self) -> PackageR {
        PackageR::new(self.bits)
    }
}
#[doc = "Package option\n\nYou can [`read`](crate::Reg::read) this register and get [`package::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PackageSpec;
impl crate::RegisterSpec for PackageSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`package::R`](R) reader structure"]
impl crate::Readable for PackageSpec {}
#[doc = "`reset()` method sets PACKAGE to value 0x2000"]
impl crate::Resettable for PackageSpec {
    const RESET_VALUE: u32 = 0x2000;
}
