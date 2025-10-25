#[doc = "Register `TRCEVENTCTL0R` reader"]
pub type R = crate::R<Trceventctl0rSpec>;
#[doc = "Register `TRCEVENTCTL0R` writer"]
pub type W = crate::W<Trceventctl0rSpec>;
#[doc = "Field `EVENT` reader - Select which event should generate trace elements."]
pub type EventR = crate::FieldReader;
#[doc = "Field `EVENT` writer - Select which event should generate trace elements."]
pub type EventW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Select which event should generate trace elements."]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Select which event should generate trace elements."]
    #[inline(always)]
    pub fn event(&mut self) -> EventW<'_, Trceventctl0rSpec> {
        EventW::new(self, 0)
    }
}
#[doc = "Controls the tracing of arbitrary events. If the selected event occurs a trace element is generated in the trace stream according to the settings in TRCEVENTCTL1R.DATAEN and TRCEVENTCTL1R.INSTEN.\n\nYou can [`read`](crate::Reg::read) this register and get [`trceventctl0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trceventctl0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Trceventctl0rSpec;
impl crate::RegisterSpec for Trceventctl0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trceventctl0r::R`](R) reader structure"]
impl crate::Readable for Trceventctl0rSpec {}
#[doc = "`write(|w| ..)` method takes [`trceventctl0r::W`](W) writer structure"]
impl crate::Writable for Trceventctl0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCEVENTCTL0R to value 0"]
impl crate::Resettable for Trceventctl0rSpec {}
