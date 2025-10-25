#[doc = "Register `CODEPAGESIZE` reader"]
pub type R = crate::R<CodepagesizeSpec>;
#[doc = "Code memory page size in bytes\n\nValue on reset: 2048"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Codepagesize {
    #[doc = "2048: 2 kByte"]
    K2048 = 2048,
}
impl From<Codepagesize> for u32 {
    #[inline(always)]
    fn from(variant: Codepagesize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Codepagesize {
    type Ux = u32;
}
impl crate::IsEnum for Codepagesize {}
#[doc = "Field `CODEPAGESIZE` reader - Code memory page size in bytes"]
pub type CodepagesizeR = crate::FieldReader<Codepagesize>;
impl CodepagesizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Codepagesize> {
        match self.bits {
            2048 => Some(Codepagesize::K2048),
            _ => None,
        }
    }
    #[doc = "2 kByte"]
    #[inline(always)]
    pub fn is_k2048(&self) -> bool {
        *self == Codepagesize::K2048
    }
}
impl R {
    #[doc = "Bits 0:31 - Code memory page size in bytes"]
    #[inline(always)]
    pub fn codepagesize(&self) -> CodepagesizeR {
        CodepagesizeR::new(self.bits)
    }
}
#[doc = "Code memory page size in bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`codepagesize::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodepagesizeSpec;
impl crate::RegisterSpec for CodepagesizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codepagesize::R`](R) reader structure"]
impl crate::Readable for CodepagesizeSpec {}
#[doc = "`reset()` method sets CODEPAGESIZE to value 0x0800"]
impl crate::Resettable for CodepagesizeSpec {
    const RESET_VALUE: u32 = 0x0800;
}
