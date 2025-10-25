#[doc = "Register `N_NP_T0_T1_ADDR` reader"]
pub type R = crate::R<NNpT0T1AddrSpec>;
#[doc = "Register `N_NP_T0_T1_ADDR` writer"]
pub type W = crate::W<NNpT0T1AddrSpec>;
#[doc = "Field `N_VIRTUAL_ADDR` reader - Register N virtual register index. Default is R0."]
pub type NVirtualAddrR = crate::FieldReader;
#[doc = "Field `N_VIRTUAL_ADDR` writer - Register N virtual register index. Default is R0."]
pub type NVirtualAddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NP_VIRTUAL_ADDR` reader - Register Np virtual register index. Default is R1."]
pub type NpVirtualAddrR = crate::FieldReader;
#[doc = "Field `NP_VIRTUAL_ADDR` writer - Register Np virtual register index. Default is R1."]
pub type NpVirtualAddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `T0_VIRTUAL_ADDR` reader - Temporary register 0 virtual register index. Default is R30."]
pub type T0VirtualAddrR = crate::FieldReader;
#[doc = "Field `T0_VIRTUAL_ADDR` writer - Temporary register 0 virtual register index. Default is R30."]
pub type T0VirtualAddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `T1_VIRTUAL_ADDR` reader - Temporary register 1 virtual register index. Default is R31."]
pub type T1VirtualAddrR = crate::FieldReader;
#[doc = "Field `T1_VIRTUAL_ADDR` writer - Temporary register 1 virtual register index. Default is R31."]
pub type T1VirtualAddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Register N virtual register index. Default is R0."]
    #[inline(always)]
    pub fn n_virtual_addr(&self) -> NVirtualAddrR {
        NVirtualAddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Register Np virtual register index. Default is R1."]
    #[inline(always)]
    pub fn np_virtual_addr(&self) -> NpVirtualAddrR {
        NpVirtualAddrR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Temporary register 0 virtual register index. Default is R30."]
    #[inline(always)]
    pub fn t0_virtual_addr(&self) -> T0VirtualAddrR {
        T0VirtualAddrR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Temporary register 1 virtual register index. Default is R31."]
    #[inline(always)]
    pub fn t1_virtual_addr(&self) -> T1VirtualAddrR {
        T1VirtualAddrR::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Register N virtual register index. Default is R0."]
    #[inline(always)]
    pub fn n_virtual_addr(&mut self) -> NVirtualAddrW<'_, NNpT0T1AddrSpec> {
        NVirtualAddrW::new(self, 0)
    }
    #[doc = "Bits 5:9 - Register Np virtual register index. Default is R1."]
    #[inline(always)]
    pub fn np_virtual_addr(&mut self) -> NpVirtualAddrW<'_, NNpT0T1AddrSpec> {
        NpVirtualAddrW::new(self, 5)
    }
    #[doc = "Bits 10:14 - Temporary register 0 virtual register index. Default is R30."]
    #[inline(always)]
    pub fn t0_virtual_addr(&mut self) -> T0VirtualAddrW<'_, NNpT0T1AddrSpec> {
        T0VirtualAddrW::new(self, 10)
    }
    #[doc = "Bits 15:19 - Temporary register 1 virtual register index. Default is R31."]
    #[inline(always)]
    pub fn t1_virtual_addr(&mut self) -> T1VirtualAddrW<'_, NNpT0T1AddrSpec> {
        T1VirtualAddrW::new(self, 15)
    }
}
#[doc = "This register defines the N, Np, T0, and T1 virtual register index.\n\nYou can [`read`](crate::Reg::read) this register and get [`n_np_t0_t1_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_np_t0_t1_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NNpT0T1AddrSpec;
impl crate::RegisterSpec for NNpT0T1AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_np_t0_t1_addr::R`](R) reader structure"]
impl crate::Readable for NNpT0T1AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`n_np_t0_t1_addr::W`](W) writer structure"]
impl crate::Writable for NNpT0T1AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets N_NP_T0_T1_ADDR to value 0x000f_f820"]
impl crate::Resettable for NNpT0T1AddrSpec {
    const RESET_VALUE: u32 = 0x000f_f820;
}
