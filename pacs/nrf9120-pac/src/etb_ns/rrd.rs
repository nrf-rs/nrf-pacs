#[doc = "Register `RRD` reader"]
pub type R = crate::R<RrdSpec>;
#[doc = "Field `RAM_READ_DATA` reader - Data read from the ETB Trace RAM."]
pub type RamReadDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data read from the ETB Trace RAM."]
    #[inline(always)]
    pub fn ram_read_data(&self) -> RamReadDataR {
        RamReadDataR::new(self.bits)
    }
}
#[doc = "ETB RAM Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrdSpec;
impl crate::RegisterSpec for RrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrd::R`](R) reader structure"]
impl crate::Readable for RrdSpec {}
#[doc = "`reset()` method sets RRD to value 0"]
impl crate::Resettable for RrdSpec {}
