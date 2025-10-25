#[doc = "Register `CRCSTATUS` reader"]
pub type R = crate::R<CrcstatusSpec>;
#[doc = "CRC status of packet received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcstatus {
    #[doc = "0: Packet received with CRC error"]
    Crcerror = 0,
    #[doc = "1: Packet received with CRC ok"]
    Crcok = 1,
}
impl From<Crcstatus> for bool {
    #[inline(always)]
    fn from(variant: Crcstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCSTATUS` reader - CRC status of packet received"]
pub type CrcstatusR = crate::BitReader<Crcstatus>;
impl CrcstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcstatus {
        match self.bits {
            false => Crcstatus::Crcerror,
            true => Crcstatus::Crcok,
        }
    }
    #[doc = "Packet received with CRC error"]
    #[inline(always)]
    pub fn is_crcerror(&self) -> bool {
        *self == Crcstatus::Crcerror
    }
    #[doc = "Packet received with CRC ok"]
    #[inline(always)]
    pub fn is_crcok(&self) -> bool {
        *self == Crcstatus::Crcok
    }
}
impl R {
    #[doc = "Bit 0 - CRC status of packet received"]
    #[inline(always)]
    pub fn crcstatus(&self) -> CrcstatusR {
        CrcstatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "CRC status\n\nYou can [`read`](crate::Reg::read) this register and get [`crcstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcstatusSpec;
impl crate::RegisterSpec for CrcstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcstatus::R`](R) reader structure"]
impl crate::Readable for CrcstatusSpec {}
#[doc = "`reset()` method sets CRCSTATUS to value 0"]
impl crate::Resettable for CrcstatusSpec {}
