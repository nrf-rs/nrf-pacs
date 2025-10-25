#[doc = "Register `MAINREGSTATUS` reader"]
pub type R = crate::R<MainregstatusSpec>;
#[doc = "VREGH status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vregh {
    #[doc = "0: Normal voltage mode. Voltage supplied on VDD and VDDH."]
    Inactive = 0,
    #[doc = "1: High voltage mode. Voltage supplied on VDDH."]
    Active = 1,
}
impl From<Vregh> for bool {
    #[inline(always)]
    fn from(variant: Vregh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREGH` reader - VREGH status"]
pub type VreghR = crate::BitReader<Vregh>;
impl VreghR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vregh {
        match self.bits {
            false => Vregh::Inactive,
            true => Vregh::Active,
        }
    }
    #[doc = "Normal voltage mode. Voltage supplied on VDD and VDDH."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Vregh::Inactive
    }
    #[doc = "High voltage mode. Voltage supplied on VDDH."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Vregh::Active
    }
}
impl R {
    #[doc = "Bit 0 - VREGH status"]
    #[inline(always)]
    pub fn vregh(&self) -> VreghR {
        VreghR::new((self.bits & 1) != 0)
    }
}
#[doc = "Main supply status\n\nYou can [`read`](crate::Reg::read) this register and get [`mainregstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainregstatusSpec;
impl crate::RegisterSpec for MainregstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mainregstatus::R`](R) reader structure"]
impl crate::Readable for MainregstatusSpec {}
#[doc = "`reset()` method sets MAINREGSTATUS to value 0"]
impl crate::Resettable for MainregstatusSpec {}
