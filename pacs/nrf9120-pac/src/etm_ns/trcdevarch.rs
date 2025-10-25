#[doc = "Register `TRCDEVARCH` reader"]
pub type R = crate::R<TrcdevarchSpec>;
#[doc = "Architecture ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Archid {
    #[doc = "18963: Component is an ETMv4 component"]
    Etmv42 = 18963,
}
impl From<Archid> for u16 {
    #[inline(always)]
    fn from(variant: Archid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Archid {
    type Ux = u16;
}
impl crate::IsEnum for Archid {}
#[doc = "Field `ARCHID` reader - Architecture ID"]
pub type ArchidR = crate::FieldReader<Archid>;
impl ArchidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Archid> {
        match self.bits {
            18963 => Some(Archid::Etmv42),
            _ => None,
        }
    }
    #[doc = "Component is an ETMv4 component"]
    #[inline(always)]
    pub fn is_etmv42(&self) -> bool {
        *self == Archid::Etmv42
    }
}
#[doc = "Architecture revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revision {
    #[doc = "2: Component is part of architecture 4.2"]
    V2 = 2,
}
impl From<Revision> for u8 {
    #[inline(always)]
    fn from(variant: Revision) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revision {
    type Ux = u8;
}
impl crate::IsEnum for Revision {}
#[doc = "Field `REVISION` reader - Architecture revision"]
pub type RevisionR = crate::FieldReader<Revision>;
impl RevisionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revision> {
        match self.bits {
            2 => Some(Revision::V2),
            _ => None,
        }
    }
    #[doc = "Component is part of architecture 4.2"]
    #[inline(always)]
    pub fn is_v2(&self) -> bool {
        *self == Revision::V2
    }
}
#[doc = "This register is implemented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Present {
    #[doc = "0: The register is not implemented."]
    Absent = 0,
    #[doc = "1: The register is implemented."]
    Present = 1,
}
impl From<Present> for bool {
    #[inline(always)]
    fn from(variant: Present) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRESENT` reader - This register is implemented"]
pub type PresentR = crate::BitReader<Present>;
impl PresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Present {
        match self.bits {
            false => Present::Absent,
            true => Present::Present,
        }
    }
    #[doc = "The register is not implemented."]
    #[inline(always)]
    pub fn is_absent(&self) -> bool {
        *self == Present::Absent
    }
    #[doc = "The register is implemented."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Present::Present
    }
}
#[doc = "Defines the architect of the component\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Architect {
    #[doc = "571: This peripheral was architected by Arm."]
    Arm = 571,
}
impl From<Architect> for u16 {
    #[inline(always)]
    fn from(variant: Architect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Architect {
    type Ux = u16;
}
impl crate::IsEnum for Architect {}
#[doc = "Field `ARCHITECT` reader - Defines the architect of the component"]
pub type ArchitectR = crate::FieldReader<Architect>;
impl ArchitectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Architect> {
        match self.bits {
            571 => Some(Architect::Arm),
            _ => None,
        }
    }
    #[doc = "This peripheral was architected by Arm."]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == Architect::Arm
    }
}
impl R {
    #[doc = "Bits 0:15 - Architecture ID"]
    #[inline(always)]
    pub fn archid(&self) -> ArchidR {
        ArchidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Architecture revision"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - This register is implemented"]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - Defines the architect of the component"]
    #[inline(always)]
    pub fn architect(&self) -> ArchitectR {
        ArchitectR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "The TRCDEVARCH identifies ETM-M33 as an ETMv4.2 component\n\nYou can [`read`](crate::Reg::read) this register and get [`trcdevarch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcdevarchSpec;
impl crate::RegisterSpec for TrcdevarchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcdevarch::R`](R) reader structure"]
impl crate::Readable for TrcdevarchSpec {}
#[doc = "`reset()` method sets TRCDEVARCH to value 0"]
impl crate::Resettable for TrcdevarchSpec {}
