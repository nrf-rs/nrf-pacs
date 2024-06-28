#[doc = "Register `RXSTATUS` reader"]
pub type R = crate::R<RxstatusSpec>;
#[doc = "Status of data in register RXDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxstatus {
    #[doc = "0: No data pending in register RXDATA"]
    NoDataPending = 0,
    #[doc = "1: Data pending in register RXDATA"]
    DataPending = 1,
}
impl From<Rxstatus> for bool {
    #[inline(always)]
    fn from(variant: Rxstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTATUS` reader - Status of data in register RXDATA"]
pub type RxstatusR = crate::BitReader<Rxstatus>;
impl RxstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxstatus {
        match self.bits {
            false => Rxstatus::NoDataPending,
            true => Rxstatus::DataPending,
        }
    }
    #[doc = "No data pending in register RXDATA"]
    #[inline(always)]
    pub fn is_no_data_pending(&self) -> bool {
        *self == Rxstatus::NoDataPending
    }
    #[doc = "Data pending in register RXDATA"]
    #[inline(always)]
    pub fn is_data_pending(&self) -> bool {
        *self == Rxstatus::DataPending
    }
}
impl R {
    #[doc = "Bit 0 - Status of data in register RXDATA"]
    #[inline(always)]
    pub fn rxstatus(&self) -> RxstatusR {
        RxstatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "This register shows a status that indicates if data sent from the debugger to the CPU has been read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxstatusSpec;
impl crate::RegisterSpec for RxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxstatus::R`](R) reader structure"]
impl crate::Readable for RxstatusSpec {}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RxstatusSpec {
    const RESET_VALUE: u32 = 0;
}
