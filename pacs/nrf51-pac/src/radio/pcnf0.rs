#[doc = "Register `PCNF0` reader"]
pub type R = crate::R<Pcnf0Spec>;
#[doc = "Register `PCNF0` writer"]
pub type W = crate::W<Pcnf0Spec>;
#[doc = "Field `LFLEN` reader - Length of length field in number of bits. Decision point: START task."]
pub type LflenR = crate::FieldReader;
#[doc = "Field `LFLEN` writer - Length of length field in number of bits. Decision point: START task."]
pub type LflenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `S0LEN` reader - Length of S0 field in number of bytes. Decision point: START task."]
pub type S0lenR = crate::BitReader;
#[doc = "Field `S0LEN` writer - Length of S0 field in number of bytes. Decision point: START task."]
pub type S0lenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1LEN` reader - Length of S1 field in number of bits. Decision point: START task."]
pub type S1lenR = crate::FieldReader;
#[doc = "Field `S1LEN` writer - Length of S1 field in number of bits. Decision point: START task."]
pub type S1lenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Length of length field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn lflen(&self) -> LflenR {
        LflenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Length of S0 field in number of bytes. Decision point: START task."]
    #[inline(always)]
    pub fn s0len(&self) -> S0lenR {
        S0lenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Length of S1 field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn s1len(&self) -> S1lenR {
        S1lenR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Length of length field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn lflen(&mut self) -> LflenW<'_, Pcnf0Spec> {
        LflenW::new(self, 0)
    }
    #[doc = "Bit 8 - Length of S0 field in number of bytes. Decision point: START task."]
    #[inline(always)]
    pub fn s0len(&mut self) -> S0lenW<'_, Pcnf0Spec> {
        S0lenW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Length of S1 field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn s1len(&mut self) -> S1lenW<'_, Pcnf0Spec> {
        S1lenW::new(self, 16)
    }
}
#[doc = "Packet configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcnf0Spec;
impl crate::RegisterSpec for Pcnf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnf0::R`](R) reader structure"]
impl crate::Readable for Pcnf0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcnf0::W`](W) writer structure"]
impl crate::Writable for Pcnf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNF0 to value 0"]
impl crate::Resettable for Pcnf0Spec {}
