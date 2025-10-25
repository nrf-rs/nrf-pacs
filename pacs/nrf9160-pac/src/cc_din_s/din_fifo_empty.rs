#[doc = "Register `DIN_FIFO_EMPTY` reader"]
pub type R = crate::R<DinFifoEmptySpec>;
#[doc = "DIN FIFO status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: DIN FIFO is not empty"]
    NotEmpty = 0,
    #[doc = "1: DIN FIFO is empty, and more data can be accepted"]
    Empty = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - DIN FIFO status"]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::NotEmpty,
            true => Status::Empty,
        }
    }
    #[doc = "DIN FIFO is not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Status::NotEmpty
    }
    #[doc = "DIN FIFO is empty, and more data can be accepted"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Status::Empty
    }
}
impl R {
    #[doc = "Bit 0 - DIN FIFO status"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Register indicating if DIN FIFO is empty and if more data can be accepted.\n\nYou can [`read`](crate::Reg::read) this register and get [`din_fifo_empty::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinFifoEmptySpec;
impl crate::RegisterSpec for DinFifoEmptySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din_fifo_empty::R`](R) reader structure"]
impl crate::Readable for DinFifoEmptySpec {}
#[doc = "`reset()` method sets DIN_FIFO_EMPTY to value 0x01"]
impl crate::Resettable for DinFifoEmptySpec {
    const RESET_VALUE: u32 = 0x01;
}
