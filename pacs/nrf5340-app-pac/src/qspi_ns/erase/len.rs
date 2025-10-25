#[doc = "Register `LEN` reader"]
pub type R = crate::R<LenSpec>;
#[doc = "Register `LEN` writer"]
pub type W = crate::W<LenSpec>;
#[doc = "LEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Len {
    #[doc = "0: Erase 4 kB block (flash command 0x20)"]
    _4kb = 0,
    #[doc = "1: Erase 64 kB block (flash command 0xD8)"]
    _64kb = 1,
    #[doc = "2: Erase all (flash command 0xC7)"]
    All = 2,
}
impl From<Len> for u8 {
    #[inline(always)]
    fn from(variant: Len) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Len {
    type Ux = u8;
}
impl crate::IsEnum for Len {}
#[doc = "Field `LEN` reader - LEN"]
pub type LenR = crate::FieldReader<Len>;
impl LenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Len> {
        match self.bits {
            0 => Some(Len::_4kb),
            1 => Some(Len::_64kb),
            2 => Some(Len::All),
            _ => None,
        }
    }
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Len::_4kb
    }
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Len::_64kb
    }
    #[doc = "Erase all (flash command 0xC7)"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Len::All
    }
}
#[doc = "Field `LEN` writer - LEN"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Len>;
impl<'a, REG> LenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Len::_4kb)
    }
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Len::_64kb)
    }
    #[doc = "Erase all (flash command 0xC7)"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Len::All)
    }
}
impl R {
    #[doc = "Bits 0:1 - LEN"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LEN"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<'_, LenSpec> {
        LenW::new(self, 0)
    }
}
#[doc = "Size of block to be erased.\n\nYou can [`read`](crate::Reg::read) this register and get [`len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LenSpec;
impl crate::RegisterSpec for LenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`len::R`](R) reader structure"]
impl crate::Readable for LenSpec {}
#[doc = "`write(|w| ..)` method takes [`len::W`](W) writer structure"]
impl crate::Writable for LenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEN to value 0"]
impl crate::Resettable for LenSpec {}
