#[doc = "Register `MAINREGSTATUS` reader"]
pub type R = crate::R<MainregstatusSpec>;
#[doc = "Main supply status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mainregstatus {
    #[doc = "0: Normal voltage mode. Voltage supplied on VDD."]
    Normal = 0,
    #[doc = "1: High voltage mode. Voltage supplied on VDDH."]
    High = 1,
}
impl From<Mainregstatus> for bool {
    #[inline(always)]
    fn from(variant: Mainregstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAINREGSTATUS` reader - Main supply status"]
pub type MainregstatusR = crate::BitReader<Mainregstatus>;
impl MainregstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mainregstatus {
        match self.bits {
            false => Mainregstatus::Normal,
            true => Mainregstatus::High,
        }
    }
    #[doc = "Normal voltage mode. Voltage supplied on VDD."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Mainregstatus::Normal
    }
    #[doc = "High voltage mode. Voltage supplied on VDDH."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Mainregstatus::High
    }
}
impl R {
    #[doc = "Bit 0 - Main supply status"]
    #[inline(always)]
    pub fn mainregstatus(&self) -> MainregstatusR {
        MainregstatusR::new((self.bits & 1) != 0)
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
