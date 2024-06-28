#[doc = "Register `TXSTATUS` reader"]
pub type R = crate::R<TxstatusSpec>;
#[doc = "Status of data in register TXDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txstatus {
    #[doc = "0: No data pending in register TXDATA"]
    NoDataPending = 0,
    #[doc = "1: Data pending in register TXDATA"]
    DataPending = 1,
}
impl From<Txstatus> for bool {
    #[inline(always)]
    fn from(variant: Txstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTATUS` reader - Status of data in register TXDATA"]
pub type TxstatusR = crate::BitReader<Txstatus>;
impl TxstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txstatus {
        match self.bits {
            false => Txstatus::NoDataPending,
            true => Txstatus::DataPending,
        }
    }
    #[doc = "No data pending in register TXDATA"]
    #[inline(always)]
    pub fn is_no_data_pending(&self) -> bool {
        *self == Txstatus::NoDataPending
    }
    #[doc = "Data pending in register TXDATA"]
    #[inline(always)]
    pub fn is_data_pending(&self) -> bool {
        *self == Txstatus::DataPending
    }
}
impl R {
    #[doc = "Bit 0 - Status of data in register TXDATA"]
    #[inline(always)]
    pub fn txstatus(&self) -> TxstatusR {
        TxstatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "This register shows a status that indicates if the data sent from the CPU to the debugger has been read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxstatusSpec;
impl crate::RegisterSpec for TxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txstatus::R`](R) reader structure"]
impl crate::Readable for TxstatusSpec {}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TxstatusSpec {
    const RESET_VALUE: u32 = 0;
}
