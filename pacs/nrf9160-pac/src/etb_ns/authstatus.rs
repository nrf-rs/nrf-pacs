#[doc = "Register `AUTHSTATUS` reader"]
pub type R = crate::R<AuthstatusSpec>;
#[doc = "Field `NSID` reader - Indicates the security level for non-secure invasive debug"]
pub type NsidR = crate::FieldReader;
#[doc = "Field `NSNID` reader - Indicates the security level for non-secure non-invasive debug"]
pub type NsnidR = crate::FieldReader;
#[doc = "Field `SID` reader - Indicates the security level for secure invasive debug"]
pub type SidR = crate::FieldReader;
#[doc = "Field `SNID` reader - Indicates the security level for secure non-invasive debug"]
pub type SnidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Indicates the security level for non-secure invasive debug"]
    #[inline(always)]
    pub fn nsid(&self) -> NsidR {
        NsidR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Indicates the security level for non-secure non-invasive debug"]
    #[inline(always)]
    pub fn nsnid(&self) -> NsnidR {
        NsnidR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Indicates the security level for secure invasive debug"]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Indicates the security level for secure non-invasive debug"]
    #[inline(always)]
    pub fn snid(&self) -> SnidR {
        SnidR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Authentication Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`authstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuthstatusSpec;
impl crate::RegisterSpec for AuthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`authstatus::R`](R) reader structure"]
impl crate::Readable for AuthstatusSpec {}
#[doc = "`reset()` method sets AUTHSTATUS to value 0"]
impl crate::Resettable for AuthstatusSpec {}
