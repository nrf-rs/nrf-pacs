#[doc = "Register `CHACHA_KEY[%s]` writer"]
pub type W = crate::W<ChachaKeySpec>;
#[doc = "Field `VALUE` writer - CHACHA key value."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - CHACHA key value."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, ChachaKeySpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: CHACHA key value to use. The initial CHACHA_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_key::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaKeySpec;
impl crate::RegisterSpec for ChachaKeySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chacha_key::W`](W) writer structure"]
impl crate::Writable for ChachaKeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHACHA_KEY[%s] to value 0"]
impl crate::Resettable for ChachaKeySpec {}
