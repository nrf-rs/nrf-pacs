#[doc = "Register `PIDR3` reader"]
pub struct R(crate::R<PIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMOD_A {
    #[doc = "0: Indicates that the customer has not modified this component."]
    UNMODIFIED = 0,
}
impl From<CMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMOD` reader - Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
pub struct CMOD_R(crate::FieldReader<u8, CMOD_A>);
impl CMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMOD_A> {
        match self.bits {
            0 => Some(CMOD_A::UNMODIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNMODIFIED`"]
    #[inline(always)]
    pub fn is_unmodified(&self) -> bool {
        **self == CMOD_A::UNMODIFIED
    }
}
impl core::ops::Deref for CMOD_R {
    type Target = crate::FieldReader<u8, CMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVAND_A {
    #[doc = "0: Indicates that there are no errata fixes to this component."]
    NOERRATA = 0,
}
impl From<REVAND_A> for u8 {
    #[inline(always)]
    fn from(variant: REVAND_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REVAND` reader - Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
pub struct REVAND_R(crate::FieldReader<u8, REVAND_A>);
impl REVAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REVAND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REVAND_A> {
        match self.bits {
            0 => Some(REVAND_A::NOERRATA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOERRATA`"]
    #[inline(always)]
    pub fn is_no_errata(&self) -> bool {
        **self == REVAND_A::NOERRATA
    }
}
impl core::ops::Deref for REVAND_R {
    type Target = crate::FieldReader<u8, REVAND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr3](index.html) module"]
pub struct PIDR3_SPEC;
impl crate::RegisterSpec for PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr3::R](R) reader structure"]
impl crate::Readable for PIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for PIDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
