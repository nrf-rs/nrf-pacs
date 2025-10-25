#[doc = "Register `MAXPACKETSIZE` reader"]
pub type R = crate::R<MaxpacketsizeSpec>;
#[doc = "Register `MAXPACKETSIZE` writer"]
pub type W = crate::W<MaxpacketsizeSpec>;
#[doc = "Field `MAXPACKETSIZE` reader - Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
pub type MaxpacketsizeR = crate::FieldReader;
#[doc = "Field `MAXPACKETSIZE` writer - Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
pub type MaxpacketsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
    #[inline(always)]
    pub fn maxpacketsize(&self) -> MaxpacketsizeR {
        MaxpacketsizeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
    #[inline(always)]
    pub fn maxpacketsize(&mut self) -> MaxpacketsizeW<'_, MaxpacketsizeSpec> {
        MaxpacketsizeW::new(self, 0)
    }
}
#[doc = "Length of keystream generated when MODE.LENGTH = Extended.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxpacketsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxpacketsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxpacketsizeSpec;
impl crate::RegisterSpec for MaxpacketsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxpacketsize::R`](R) reader structure"]
impl crate::Readable for MaxpacketsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`maxpacketsize::W`](W) writer structure"]
impl crate::Writable for MaxpacketsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAXPACKETSIZE to value 0xfb"]
impl crate::Resettable for MaxpacketsizeSpec {
    const RESET_VALUE: u32 = 0xfb;
}
