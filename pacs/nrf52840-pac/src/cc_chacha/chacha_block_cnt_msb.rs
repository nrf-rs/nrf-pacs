#[doc = "Register `CHACHA_BLOCK_CNT_MSB` reader"]
pub type R = crate::R<ChachaBlockCntMsbSpec>;
#[doc = "Register `CHACHA_BLOCK_CNT_MSB` writer"]
pub type W = crate::W<ChachaBlockCntMsbSpec>;
#[doc = "Field `VALUE` reader - This register holds the ChaCha block counter bits \\[63:32\\] and must be read and written during respectively suspend and resume operations."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - This register holds the ChaCha block counter bits \\[63:32\\] and must be read and written during respectively suspend and resume operations."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register holds the ChaCha block counter bits \\[63:32\\] and must be read and written during respectively suspend and resume operations."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register holds the ChaCha block counter bits \\[63:32\\] and must be read and written during respectively suspend and resume operations."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, ChachaBlockCntMsbSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Store the MSB value of the block counter, in order to support suspend/resume of operation\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_block_cnt_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_block_cnt_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaBlockCntMsbSpec;
impl crate::RegisterSpec for ChachaBlockCntMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chacha_block_cnt_msb::R`](R) reader structure"]
impl crate::Readable for ChachaBlockCntMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`chacha_block_cnt_msb::W`](W) writer structure"]
impl crate::Writable for ChachaBlockCntMsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHACHA_BLOCK_CNT_MSB to value 0"]
impl crate::Resettable for ChachaBlockCntMsbSpec {}
