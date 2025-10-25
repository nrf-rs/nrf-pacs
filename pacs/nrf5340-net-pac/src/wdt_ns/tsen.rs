#[doc = "Register `TSEN` writer"]
pub type W = crate::W<TsenSpec>;
#[doc = "Allow stopping WDT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Tsen {
    #[doc = "1850885685: Value to allow stopping WDT"]
    Enable = 1850885685,
}
impl From<Tsen> for u32 {
    #[inline(always)]
    fn from(variant: Tsen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsen {
    type Ux = u32;
}
impl crate::IsEnum for Tsen {}
#[doc = "Field `TSEN` writer - Allow stopping WDT"]
pub type TsenW<'a, REG> = crate::FieldWriter<'a, REG, 32, Tsen>;
impl<'a, REG> TsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Value to allow stopping WDT"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tsen::Enable)
    }
}
impl W {
    #[doc = "Bits 0:31 - Allow stopping WDT"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<'_, TsenSpec> {
        TsenW::new(self, 0)
    }
}
#[doc = "Task stop enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsenSpec;
impl crate::RegisterSpec for TsenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tsen::W`](W) writer structure"]
impl crate::Writable for TsenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSEN to value 0"]
impl crate::Resettable for TsenSpec {}
