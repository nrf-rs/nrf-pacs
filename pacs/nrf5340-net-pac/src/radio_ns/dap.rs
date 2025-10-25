#[doc = "Register `DAP[%s]` reader"]
pub type R = crate::R<DapSpec>;
#[doc = "Register `DAP[%s]` writer"]
pub type W = crate::W<DapSpec>;
#[doc = "Field `DAP` reader - Device address prefix n"]
pub type DapR = crate::FieldReader<u16>;
#[doc = "Field `DAP` writer - Device address prefix n"]
pub type DapW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Device address prefix n"]
    #[inline(always)]
    pub fn dap(&self) -> DapR {
        DapR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device address prefix n"]
    #[inline(always)]
    pub fn dap(&mut self) -> DapW<'_, DapSpec> {
        DapW::new(self, 0)
    }
}
#[doc = "Description collection: Device address prefix n\n\nYou can [`read`](crate::Reg::read) this register and get [`dap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DapSpec;
impl crate::RegisterSpec for DapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dap::R`](R) reader structure"]
impl crate::Readable for DapSpec {}
#[doc = "`write(|w| ..)` method takes [`dap::W`](W) writer structure"]
impl crate::Writable for DapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAP[%s] to value 0"]
impl crate::Resettable for DapSpec {}
