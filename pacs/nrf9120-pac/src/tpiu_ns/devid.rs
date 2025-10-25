#[doc = "Register `DEVID` reader"]
pub type R = crate::R<DevidSpec>;
#[doc = "Field `MUXNUM` reader - Indicates the hidden level of input multiplexing. When non-zero, this value indicates the type of multiplexing on the input to the ATB. Currently only 0x00 is supported, that is, no multiplexing is present. This value helps detect the ATB structure."]
pub type MuxnumR = crate::FieldReader;
#[doc = "Indicates the relationship between atclk and traceclkin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkrelat {
    #[doc = "0: atclk and traceclkin are synchronous."]
    Synchronous = 0,
    #[doc = "1: atclk and traceclkin are asynchronous."]
    Asynchronous = 1,
}
impl From<Clkrelat> for bool {
    #[inline(always)]
    fn from(variant: Clkrelat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKRELAT` reader - Indicates the relationship between atclk and traceclkin."]
pub type ClkrelatR = crate::BitReader<Clkrelat>;
impl ClkrelatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkrelat {
        match self.bits {
            false => Clkrelat::Synchronous,
            true => Clkrelat::Asynchronous,
        }
    }
    #[doc = "atclk and traceclkin are synchronous."]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Clkrelat::Synchronous
    }
    #[doc = "atclk and traceclkin are asynchronous."]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Clkrelat::Asynchronous
    }
}
#[doc = "FIFO size in powers of 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fifosize {
    #[doc = "2: FIFO size of 4 entries, that is, 16 bytes."]
    Entries4 = 2,
}
impl From<Fifosize> for u8 {
    #[inline(always)]
    fn from(variant: Fifosize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fifosize {
    type Ux = u8;
}
impl crate::IsEnum for Fifosize {}
#[doc = "Field `FIFOSIZE` reader - FIFO size in powers of 2."]
pub type FifosizeR = crate::FieldReader<Fifosize>;
impl FifosizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fifosize> {
        match self.bits {
            2 => Some(Fifosize::Entries4),
            _ => None,
        }
    }
    #[doc = "FIFO size of 4 entries, that is, 16 bytes."]
    #[inline(always)]
    pub fn is_entries4(&self) -> bool {
        *self == Fifosize::Entries4
    }
}
#[doc = "Indicates whether trace clock plus data is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tclkdata {
    #[doc = "0: Trace clock and data is supported."]
    Supported = 0,
    #[doc = "1: Trace clock and data is not supported."]
    NotSupported = 1,
}
impl From<Tclkdata> for bool {
    #[inline(always)]
    fn from(variant: Tclkdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCLKDATA` reader - Indicates whether trace clock plus data is supported."]
pub type TclkdataR = crate::BitReader<Tclkdata>;
impl TclkdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tclkdata {
        match self.bits {
            false => Tclkdata::Supported,
            true => Tclkdata::NotSupported,
        }
    }
    #[doc = "Trace clock and data is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Tclkdata::Supported
    }
    #[doc = "Trace clock and data is not supported."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Tclkdata::NotSupported
    }
}
#[doc = "Indicates whether Serial Wire Output, Manchester encoded format, is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swoman {
    #[doc = "0: Serial Wire Output, Manchester encoded format, is not supported."]
    NotSupported = 0,
    #[doc = "1: Serial Wire Output, Manchester encoded format, is supported."]
    Supported = 1,
}
impl From<Swoman> for bool {
    #[inline(always)]
    fn from(variant: Swoman) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWOMAN` reader - Indicates whether Serial Wire Output, Manchester encoded format, is supported."]
pub type SwomanR = crate::BitReader<Swoman>;
impl SwomanR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swoman {
        match self.bits {
            false => Swoman::NotSupported,
            true => Swoman::Supported,
        }
    }
    #[doc = "Serial Wire Output, Manchester encoded format, is not supported."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Swoman::NotSupported
    }
    #[doc = "Serial Wire Output, Manchester encoded format, is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Swoman::Supported
    }
}
#[doc = "Indicates whether Serial Wire Output, UART or NRZ, is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swouartnrz {
    #[doc = "0: Serial Wire Output, UART or NRZ, is not supported."]
    NotSupported = 0,
    #[doc = "1: Serial Wire Output, UART or NRZ, is supported."]
    Supported = 1,
}
impl From<Swouartnrz> for bool {
    #[inline(always)]
    fn from(variant: Swouartnrz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWOUARTNRZ` reader - Indicates whether Serial Wire Output, UART or NRZ, is supported."]
pub type SwouartnrzR = crate::BitReader<Swouartnrz>;
impl SwouartnrzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swouartnrz {
        match self.bits {
            false => Swouartnrz::NotSupported,
            true => Swouartnrz::Supported,
        }
    }
    #[doc = "Serial Wire Output, UART or NRZ, is not supported."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Swouartnrz::NotSupported
    }
    #[doc = "Serial Wire Output, UART or NRZ, is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Swouartnrz::Supported
    }
}
impl R {
    #[doc = "Bits 0:4 - Indicates the hidden level of input multiplexing. When non-zero, this value indicates the type of multiplexing on the input to the ATB. Currently only 0x00 is supported, that is, no multiplexing is present. This value helps detect the ATB structure."]
    #[inline(always)]
    pub fn muxnum(&self) -> MuxnumR {
        MuxnumR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Indicates the relationship between atclk and traceclkin."]
    #[inline(always)]
    pub fn clkrelat(&self) -> ClkrelatR {
        ClkrelatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - FIFO size in powers of 2."]
    #[inline(always)]
    pub fn fifosize(&self) -> FifosizeR {
        FifosizeR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Indicates whether trace clock plus data is supported."]
    #[inline(always)]
    pub fn tclkdata(&self) -> TclkdataR {
        TclkdataR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates whether Serial Wire Output, Manchester encoded format, is supported."]
    #[inline(always)]
    pub fn swoman(&self) -> SwomanR {
        SwomanR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates whether Serial Wire Output, UART or NRZ, is supported."]
    #[inline(always)]
    pub fn swouartnrz(&self) -> SwouartnrzR {
        SwouartnrzR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Indicates the capabilities of the component.\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevidSpec;
impl crate::RegisterSpec for DevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devid::R`](R) reader structure"]
impl crate::Readable for DevidSpec {}
#[doc = "`reset()` method sets DEVID to value 0"]
impl crate::Resettable for DevidSpec {}
