#[doc = "Register `DOUT_FIFO_EMPTY` reader"]
pub type R = crate::R<DoutFifoEmptySpec>;
#[doc = "DOUT FIFO status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: DOUT FIFO is not empty, and more data will come"]
    NotEmpty = 0,
    #[doc = "1: DOUT FIFO is empty"]
    Empty = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - DOUT FIFO status"]
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
    #[doc = "DOUT FIFO is not empty, and more data will come"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Status::NotEmpty
    }
    #[doc = "DOUT FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Status::Empty
    }
}
impl R {
    #[doc = "Bit 0 - DOUT FIFO status"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Register indicating if DOUT FIFO is empty or if more data will come.\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_fifo_empty::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutFifoEmptySpec;
impl crate::RegisterSpec for DoutFifoEmptySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_fifo_empty::R`](R) reader structure"]
impl crate::Readable for DoutFifoEmptySpec {}
#[doc = "`reset()` method sets DOUT_FIFO_EMPTY to value 0x01"]
impl crate::Resettable for DoutFifoEmptySpec {
    const RESET_VALUE: u32 = 0x01;
}
