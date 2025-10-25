#[doc = "Register `HASH_CONTROL` reader"]
pub type R = crate::R<HashControlSpec>;
#[doc = "Register `HASH_CONTROL` writer"]
pub type W = crate::W<HashControlSpec>;
#[doc = "Select HASH mode to execute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "1: Select SHA1 mode"]
    Sha1 = 1,
    #[doc = "2: Select SHA256 mode"]
    Sha256 = 2,
    #[doc = "10: Select SHA224 mode"]
    Sha224 = 10,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Select HASH mode to execute"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            1 => Some(Mode::Sha1),
            2 => Some(Mode::Sha256),
            10 => Some(Mode::Sha224),
            _ => None,
        }
    }
    #[doc = "Select SHA1 mode"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == Mode::Sha1
    }
    #[doc = "Select SHA256 mode"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == Mode::Sha256
    }
    #[doc = "Select SHA224 mode"]
    #[inline(always)]
    pub fn is_sha224(&self) -> bool {
        *self == Mode::Sha224
    }
}
#[doc = "Field `MODE` writer - Select HASH mode to execute"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select SHA1 mode"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Sha1)
    }
    #[doc = "Select SHA256 mode"]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Sha256)
    }
    #[doc = "Select SHA224 mode"]
    #[inline(always)]
    pub fn sha224(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Sha224)
    }
}
impl R {
    #[doc = "Bits 0:3 - Select HASH mode to execute"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select HASH mode to execute"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, HashControlSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Control the HASH engine behavior.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashControlSpec;
impl crate::RegisterSpec for HashControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_control::R`](R) reader structure"]
impl crate::Readable for HashControlSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_control::W`](W) writer structure"]
impl crate::Writable for HashControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_CONTROL to value 0"]
impl crate::Resettable for HashControlSpec {}
