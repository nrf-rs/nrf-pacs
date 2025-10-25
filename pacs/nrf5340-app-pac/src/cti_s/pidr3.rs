#[doc = "Register `PIDR3` reader"]
pub type R = crate::R<Pidr3Spec>;
#[doc = "Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmod {
    #[doc = "0: Indicates that the customer has not modified this component."]
    Unmodified = 0,
}
impl From<Cmod> for u8 {
    #[inline(always)]
    fn from(variant: Cmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmod {
    type Ux = u8;
}
impl crate::IsEnum for Cmod {}
#[doc = "Field `CMOD` reader - Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
pub type CmodR = crate::FieldReader<Cmod>;
impl CmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmod> {
        match self.bits {
            0 => Some(Cmod::Unmodified),
            _ => None,
        }
    }
    #[doc = "Indicates that the customer has not modified this component."]
    #[inline(always)]
    pub fn is_unmodified(&self) -> bool {
        *self == Cmod::Unmodified
    }
}
#[doc = "Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revand {
    #[doc = "0: Indicates that there are no errata fixes to this component."]
    NoErrata = 0,
}
impl From<Revand> for u8 {
    #[inline(always)]
    fn from(variant: Revand) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revand {
    type Ux = u8;
}
impl crate::IsEnum for Revand {}
#[doc = "Field `REVAND` reader - Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
pub type RevandR = crate::FieldReader<Revand>;
impl RevandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revand> {
        match self.bits {
            0 => Some(Revand::NoErrata),
            _ => None,
        }
    }
    #[doc = "Indicates that there are no errata fixes to this component."]
    #[inline(always)]
    pub fn is_no_errata(&self) -> bool {
        *self == Revand::NoErrata
    }
}
impl R {
    #[doc = "Bits 0:3 - Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
    #[inline(always)]
    pub fn revand(&self) -> RevandR {
        RevandR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr3Spec;
impl crate::RegisterSpec for Pidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr3::R`](R) reader structure"]
impl crate::Readable for Pidr3Spec {}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for Pidr3Spec {}
