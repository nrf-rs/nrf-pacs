#[doc = "Register `PACKETPTR` reader"]
pub type R = crate::R<PacketptrSpec>;
#[doc = "Register `PACKETPTR` writer"]
pub type W = crate::W<PacketptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Packet pointer. Decision point: START task.\n\nYou can [`read`](crate::Reg::read) this register and get [`packetptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packetptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PacketptrSpec;
impl crate::RegisterSpec for PacketptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`packetptr::R`](R) reader structure"]
impl crate::Readable for PacketptrSpec {}
#[doc = "`write(|w| ..)` method takes [`packetptr::W`](W) writer structure"]
impl crate::Writable for PacketptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PACKETPTR to value 0"]
impl crate::Resettable for PacketptrSpec {}
