#[doc = "Register `PERIPHID3` reader"]
pub type R = crate::R<Periphid3Spec>;
#[doc = "Field `CMOD` reader - Where the component is reusable IP, this value indicates if the customer has modified the behavior of the component. In most cases this field is zero."]
pub type CmodR = crate::FieldReader;
#[doc = "Field `REVAND` reader - This field indicates minor errata fixes specific to this design, for example metal fixes after implementation. In most cases this field is zero. It is recommended that component designers ensure this field can be changed by a metal fix if required, for example by driving it from registers that reset to zero."]
pub type RevandR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Where the component is reusable IP, this value indicates if the customer has modified the behavior of the component. In most cases this field is zero."]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This field indicates minor errata fixes specific to this design, for example metal fixes after implementation. In most cases this field is zero. It is recommended that component designers ensure this field can be changed by a metal fix if required, for example by driving it from registers that reset to zero."]
    #[inline(always)]
    pub fn revand(&self) -> RevandR {
        RevandR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid3Spec;
impl crate::RegisterSpec for Periphid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid3::R`](R) reader structure"]
impl crate::Readable for Periphid3Spec {}
#[doc = "`reset()` method sets PERIPHID3 to value 0"]
impl crate::Resettable for Periphid3Spec {}
