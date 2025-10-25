#[doc = "Register `EVENTS_TXPTRUPD` reader"]
pub type R = crate::R<EventsTxptrupdSpec>;
#[doc = "Register `EVENTS_TXPTRUPD` writer"]
pub type W = crate::W<EventsTxptrupdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txptrupd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txptrupd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxptrupdSpec;
impl crate::RegisterSpec for EventsTxptrupdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txptrupd::R`](R) reader structure"]
impl crate::Readable for EventsTxptrupdSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txptrupd::W`](W) writer structure"]
impl crate::Writable for EventsTxptrupdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXPTRUPD to value 0"]
impl crate::Resettable for EventsTxptrupdSpec {}
