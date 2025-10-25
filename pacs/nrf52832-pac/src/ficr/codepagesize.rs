#[doc = "Register `CODEPAGESIZE` reader"]
pub type R = crate::R<CodepagesizeSpec>;
#[doc = "Field `CODEPAGESIZE` reader - Code memory page size"]
pub type CodepagesizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Code memory page size"]
    #[inline(always)]
    pub fn codepagesize(&self) -> CodepagesizeR {
        CodepagesizeR::new(self.bits)
    }
}
#[doc = "Code memory page size\n\nYou can [`read`](crate::Reg::read) this register and get [`codepagesize::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodepagesizeSpec;
impl crate::RegisterSpec for CodepagesizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codepagesize::R`](R) reader structure"]
impl crate::Readable for CodepagesizeSpec {}
#[doc = "`reset()` method sets CODEPAGESIZE to value 0xffff_ffff"]
impl crate::Resettable for CodepagesizeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
