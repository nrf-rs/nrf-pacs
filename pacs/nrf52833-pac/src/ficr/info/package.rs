#[doc = "Register `PACKAGE` reader"]
pub type R = crate::R<PackageSpec>;
#[doc = "Package option\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Package {
    #[doc = "8199: QDxx - 5x5 40-pin QFN"]
    Qd = 8199,
    #[doc = "8196: QIxx - 7x7 73-pin aQFN"]
    Qi = 8196,
    #[doc = "8200: CJxx - 3.175 x 3.175 WLCSP"]
    Cj = 8200,
    #[doc = "4294967295: Unspecified"]
    Unspecified = 4294967295,
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
            8199 => Some(Package::Qd),
            8196 => Some(Package::Qi),
            8200 => Some(Package::Cj),
            4294967295 => Some(Package::Unspecified),
            _ => None,
        }
    }
    #[doc = "QDxx - 5x5 40-pin QFN"]
    #[inline(always)]
    pub fn is_qd(&self) -> bool {
        *self == Package::Qd
    }
    #[doc = "QIxx - 7x7 73-pin aQFN"]
    #[inline(always)]
    pub fn is_qi(&self) -> bool {
        *self == Package::Qi
    }
    #[doc = "CJxx - 3.175 x 3.175 WLCSP"]
    #[inline(always)]
    pub fn is_cj(&self) -> bool {
        *self == Package::Cj
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Package::Unspecified
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
#[doc = "`reset()` method sets PACKAGE to value 0xffff_ffff"]
impl crate::Resettable for PackageSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
