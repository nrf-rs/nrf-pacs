#[doc = "Register `DPMDUR` reader"]
pub type R = crate::R<DpmdurSpec>;
#[doc = "Register `DPMDUR` writer"]
pub type W = crate::W<DpmdurSpec>;
#[doc = "Field `ENTER` reader - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
pub type EnterR = crate::FieldReader<u16>;
#[doc = "Field `ENTER` writer - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
pub type EnterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EXIT` reader - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
pub type ExitR = crate::FieldReader<u16>;
#[doc = "Field `EXIT` writer - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
pub type ExitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn enter(&self) -> EnterR {
        EnterR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn exit(&self) -> ExitR {
        ExitR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn enter(&mut self) -> EnterW<'_, DpmdurSpec> {
        EnterW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn exit(&mut self) -> ExitW<'_, DpmdurSpec> {
        ExitW::new(self, 16)
    }
}
#[doc = "Set the duration required to enter/exit deep power-down mode (DPM).\n\nYou can [`read`](crate::Reg::read) this register and get [`dpmdur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpmdur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpmdurSpec;
impl crate::RegisterSpec for DpmdurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpmdur::R`](R) reader structure"]
impl crate::Readable for DpmdurSpec {}
#[doc = "`write(|w| ..)` method takes [`dpmdur::W`](W) writer structure"]
impl crate::Writable for DpmdurSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPMDUR to value 0xffff_ffff"]
impl crate::Resettable for DpmdurSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
