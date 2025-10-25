#[doc = "Register `SFD` reader"]
pub type R = crate::R<SfdSpec>;
#[doc = "Register `SFD` writer"]
pub type W = crate::W<SfdSpec>;
#[doc = "Field `SFD` reader - IEEE 802.15.4 start of frame delimiter"]
pub type SfdR = crate::FieldReader;
#[doc = "Field `SFD` writer - IEEE 802.15.4 start of frame delimiter"]
pub type SfdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn sfd(&self) -> SfdR {
        SfdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SfdW<'_, SfdSpec> {
        SfdW::new(self, 0)
    }
}
#[doc = "IEEE 802.15.4 start of frame delimiter\n\nYou can [`read`](crate::Reg::read) this register and get [`sfd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfdSpec;
impl crate::RegisterSpec for SfdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfd::R`](R) reader structure"]
impl crate::Readable for SfdSpec {}
#[doc = "`write(|w| ..)` method takes [`sfd::W`](W) writer structure"]
impl crate::Writable for SfdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFD to value 0xa7"]
impl crate::Resettable for SfdSpec {
    const RESET_VALUE: u32 = 0xa7;
}
