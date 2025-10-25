#[doc = "Register `SWITCHPATTERN` reader"]
pub type R = crate::R<SwitchpatternSpec>;
#[doc = "Register `SWITCHPATTERN` writer"]
pub type W = crate::W<SwitchpatternSpec>;
#[doc = "Field `SWITCHPATTERN` reader - Fill array of GPIO patterns for antenna control."]
pub type SwitchpatternR = crate::FieldReader;
#[doc = "Field `SWITCHPATTERN` writer - Fill array of GPIO patterns for antenna control."]
pub type SwitchpatternW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Fill array of GPIO patterns for antenna control."]
    #[inline(always)]
    pub fn switchpattern(&self) -> SwitchpatternR {
        SwitchpatternR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fill array of GPIO patterns for antenna control."]
    #[inline(always)]
    pub fn switchpattern(&mut self) -> SwitchpatternW<'_, SwitchpatternSpec> {
        SwitchpatternW::new(self, 0)
    }
}
#[doc = "GPIO patterns to be used for each antenna\n\nYou can [`read`](crate::Reg::read) this register and get [`switchpattern::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`switchpattern::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwitchpatternSpec;
impl crate::RegisterSpec for SwitchpatternSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`switchpattern::R`](R) reader structure"]
impl crate::Readable for SwitchpatternSpec {}
#[doc = "`write(|w| ..)` method takes [`switchpattern::W`](W) writer structure"]
impl crate::Writable for SwitchpatternSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWITCHPATTERN to value 0"]
impl crate::Resettable for SwitchpatternSpec {}
