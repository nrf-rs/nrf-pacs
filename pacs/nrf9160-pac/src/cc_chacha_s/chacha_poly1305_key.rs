#[doc = "Register `CHACHA_POLY1305_KEY[%s]` reader"]
pub type R = crate::R<ChachaPoly1305KeySpec>;
#[doc = "Field `VALUE` reader - Poly1305 key value."]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Poly1305 key value."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "Description collection: The auto-generated key to use in Poly1305 MAC calculation. The initial CHACHA_POLY1305_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_poly1305_key::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaPoly1305KeySpec;
impl crate::RegisterSpec for ChachaPoly1305KeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chacha_poly1305_key::R`](R) reader structure"]
impl crate::Readable for ChachaPoly1305KeySpec {}
#[doc = "`reset()` method sets CHACHA_POLY1305_KEY[%s] to value 0"]
impl crate::Resettable for ChachaPoly1305KeySpec {}
