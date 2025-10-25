#[doc = "Register `DATAWHITEIV` reader"]
pub type R = crate::R<DatawhiteivSpec>;
#[doc = "Register `DATAWHITEIV` writer"]
pub type W = crate::W<DatawhiteivSpec>;
#[doc = "Field `DATAWHITEIV` reader - Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
pub type DatawhiteivR = crate::FieldReader;
#[doc = "Field `DATAWHITEIV` writer - Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
pub type DatawhiteivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
    #[inline(always)]
    pub fn datawhiteiv(&self) -> DatawhiteivR {
        DatawhiteivR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
    #[inline(always)]
    pub fn datawhiteiv(&mut self) -> DatawhiteivW<'_, DatawhiteivSpec> {
        DatawhiteivW::new(self, 0)
    }
}
#[doc = "Data whitening initial value\n\nYou can [`read`](crate::Reg::read) this register and get [`datawhiteiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datawhiteiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatawhiteivSpec;
impl crate::RegisterSpec for DatawhiteivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datawhiteiv::R`](R) reader structure"]
impl crate::Readable for DatawhiteivSpec {}
#[doc = "`write(|w| ..)` method takes [`datawhiteiv::W`](W) writer structure"]
impl crate::Writable for DatawhiteivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAWHITEIV to value 0x40"]
impl crate::Resettable for DatawhiteivSpec {
    const RESET_VALUE: u32 = 0x40;
}
