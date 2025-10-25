#[doc = "Register `PACKAGE` reader"]
pub type R = crate::R<PackageSpec>;
#[doc = "Package option\n\nValue on reset: 8192"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Package {
    #[doc = "8192: QFxx - 48-pin QFN"]
    Qf = 8192,
    #[doc = "8193: CHxx - 7x8 WLCSP 56 balls"]
    Ch = 8193,
    #[doc = "8194: CIxx - 7x8 WLCSP 56 balls"]
    Ci = 8194,
    #[doc = "8197: CKxx - 7x8 WLCSP 56 balls with backside coating for light protection"]
    Ck = 8197,
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
            8192 => Some(Package::Qf),
            8193 => Some(Package::Ch),
            8194 => Some(Package::Ci),
            8197 => Some(Package::Ck),
            4294967295 => Some(Package::Unspecified),
            _ => None,
        }
    }
    #[doc = "QFxx - 48-pin QFN"]
    #[inline(always)]
    pub fn is_qf(&self) -> bool {
        *self == Package::Qf
    }
    #[doc = "CHxx - 7x8 WLCSP 56 balls"]
    #[inline(always)]
    pub fn is_ch(&self) -> bool {
        *self == Package::Ch
    }
    #[doc = "CIxx - 7x8 WLCSP 56 balls"]
    #[inline(always)]
    pub fn is_ci(&self) -> bool {
        *self == Package::Ci
    }
    #[doc = "CKxx - 7x8 WLCSP 56 balls with backside coating for light protection"]
    #[inline(always)]
    pub fn is_ck(&self) -> bool {
        *self == Package::Ck
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
#[doc = "`reset()` method sets PACKAGE to value 0x2000"]
impl crate::Resettable for PackageSpec {
    const RESET_VALUE: u32 = 0x2000;
}
