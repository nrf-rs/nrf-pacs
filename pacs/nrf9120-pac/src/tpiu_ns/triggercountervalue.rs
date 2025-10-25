#[doc = "Register `TRIGGERCOUNTERVALUE` reader"]
pub type R = crate::R<TriggercountervalueSpec>;
#[doc = "Register `TRIGGERCOUNTERVALUE` writer"]
pub type W = crate::W<TriggercountervalueSpec>;
#[doc = "Field `TrigCount` reader - 8-bit counter value for the number of words to be output from the formatter before a trigger is inserted."]
pub type TrigCountR = crate::FieldReader;
#[doc = "Field `TrigCount` writer - 8-bit counter value for the number of words to be output from the formatter before a trigger is inserted."]
pub type TrigCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit counter value for the number of words to be output from the formatter before a trigger is inserted."]
    #[inline(always)]
    pub fn trig_count(&self) -> TrigCountR {
        TrigCountR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit counter value for the number of words to be output from the formatter before a trigger is inserted."]
    #[inline(always)]
    pub fn trig_count(&mut self) -> TrigCountW<'_, TriggercountervalueSpec> {
        TrigCountW::new(self, 0)
    }
}
#[doc = "The Trigger_counter_value register enables delaying the indication of triggers to any external connected trace capture or storage devices.\n\nYou can [`read`](crate::Reg::read) this register and get [`triggercountervalue::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`triggercountervalue::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggercountervalueSpec;
impl crate::RegisterSpec for TriggercountervalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`triggercountervalue::R`](R) reader structure"]
impl crate::Readable for TriggercountervalueSpec {}
#[doc = "`write(|w| ..)` method takes [`triggercountervalue::W`](W) writer structure"]
impl crate::Writable for TriggercountervalueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGGERCOUNTERVALUE to value 0"]
impl crate::Resettable for TriggercountervalueSpec {}
