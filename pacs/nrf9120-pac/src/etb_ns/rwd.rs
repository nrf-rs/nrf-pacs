#[doc = "Register `RWD` reader"]
pub type R = crate::R<RwdSpec>;
#[doc = "Register `RWD` writer"]
pub type W = crate::W<RwdSpec>;
#[doc = "Field `RAM_WRITE_DATA` reader - Data written to the ETB Trace RAM. When trace capture is disabled, the contents of this register are placed into the ETB Trace RAM when this register is written to. Writing to this register increments the RAM Write Pointer Register. If trace capture is enabled, and this register is accessed, then a read from this register outputs 0xFFFFFFFF. Reads of this register never increment the RAM Write Pointer Register. A constant stream of 1s being output corresponds to a synchronization output from the ETB. If a write access is attempted, the data is not written into Trace RAM."]
pub type RamWriteDataR = crate::FieldReader<u32>;
#[doc = "Field `RAM_WRITE_DATA` writer - Data written to the ETB Trace RAM. When trace capture is disabled, the contents of this register are placed into the ETB Trace RAM when this register is written to. Writing to this register increments the RAM Write Pointer Register. If trace capture is enabled, and this register is accessed, then a read from this register outputs 0xFFFFFFFF. Reads of this register never increment the RAM Write Pointer Register. A constant stream of 1s being output corresponds to a synchronization output from the ETB. If a write access is attempted, the data is not written into Trace RAM."]
pub type RamWriteDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data written to the ETB Trace RAM. When trace capture is disabled, the contents of this register are placed into the ETB Trace RAM when this register is written to. Writing to this register increments the RAM Write Pointer Register. If trace capture is enabled, and this register is accessed, then a read from this register outputs 0xFFFFFFFF. Reads of this register never increment the RAM Write Pointer Register. A constant stream of 1s being output corresponds to a synchronization output from the ETB. If a write access is attempted, the data is not written into Trace RAM."]
    #[inline(always)]
    pub fn ram_write_data(&self) -> RamWriteDataR {
        RamWriteDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data written to the ETB Trace RAM. When trace capture is disabled, the contents of this register are placed into the ETB Trace RAM when this register is written to. Writing to this register increments the RAM Write Pointer Register. If trace capture is enabled, and this register is accessed, then a read from this register outputs 0xFFFFFFFF. Reads of this register never increment the RAM Write Pointer Register. A constant stream of 1s being output corresponds to a synchronization output from the ETB. If a write access is attempted, the data is not written into Trace RAM."]
    #[inline(always)]
    pub fn ram_write_data(&mut self) -> RamWriteDataW<'_, RwdSpec> {
        RamWriteDataW::new(self, 0)
    }
}
#[doc = "ETB RAM Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RwdSpec;
impl crate::RegisterSpec for RwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwd::R`](R) reader structure"]
impl crate::Readable for RwdSpec {}
#[doc = "`write(|w| ..)` method takes [`rwd::W`](W) writer structure"]
impl crate::Writable for RwdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RWD to value 0"]
impl crate::Resettable for RwdSpec {}
