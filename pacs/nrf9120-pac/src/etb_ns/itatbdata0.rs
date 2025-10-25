#[doc = "Register `ITATBDATA0` reader"]
pub type R = crate::R<Itatbdata0Spec>;
#[doc = "Field `ATDATA_0` reader - Read the value of atdatas\\[0\\]."]
pub type Atdata0R = crate::BitReader;
#[doc = "Field `ATDATA_7` reader - Read the value of atdatas\\[7\\]."]
pub type Atdata7R = crate::BitReader;
#[doc = "Field `ATDATA_15` reader - Read the value of atdatas\\[15\\]."]
pub type Atdata15R = crate::BitReader;
#[doc = "Field `ATDATA_23` reader - Read the value of atdatas\\[23\\]."]
pub type Atdata23R = crate::BitReader;
#[doc = "Field `ATDATA_31` reader - Read the value of atdatas\\[31\\]."]
pub type Atdata31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read the value of atdatas\\[0\\]."]
    #[inline(always)]
    pub fn atdata_0(&self) -> Atdata0R {
        Atdata0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read the value of atdatas\\[7\\]."]
    #[inline(always)]
    pub fn atdata_7(&self) -> Atdata7R {
        Atdata7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read the value of atdatas\\[15\\]."]
    #[inline(always)]
    pub fn atdata_15(&self) -> Atdata15R {
        Atdata15R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read the value of atdatas\\[23\\]."]
    #[inline(always)]
    pub fn atdata_23(&self) -> Atdata23R {
        Atdata23R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read the value of atdatas\\[31\\]."]
    #[inline(always)]
    pub fn atdata_31(&self) -> Atdata31R {
        Atdata31R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Integration Test ATB Data Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbdata0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbdata0Spec;
impl crate::RegisterSpec for Itatbdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbdata0::R`](R) reader structure"]
impl crate::Readable for Itatbdata0Spec {}
#[doc = "`reset()` method sets ITATBDATA0 to value 0"]
impl crate::Resettable for Itatbdata0Spec {}
