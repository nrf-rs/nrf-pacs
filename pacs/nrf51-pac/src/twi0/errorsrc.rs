#[doc = "Register `ERRORSRC` reader"]
pub type R = crate::R<ErrorsrcSpec>;
#[doc = "Register `ERRORSRC` writer"]
pub type W = crate::W<ErrorsrcSpec>;
#[doc = "Byte received in RXD register before read of the last received byte (data loss).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overrun {
    #[doc = "0: Error not present."]
    NotPresent = 0,
    #[doc = "1: Error present."]
    Present = 1,
}
impl From<Overrun> for bool {
    #[inline(always)]
    fn from(variant: Overrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` reader - Byte received in RXD register before read of the last received byte (data loss)."]
pub type OverrunR = crate::BitReader<Overrun>;
impl OverrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overrun {
        match self.bits {
            false => Overrun::NotPresent,
            true => Overrun::Present,
        }
    }
    #[doc = "Error not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Overrun::NotPresent
    }
    #[doc = "Error present."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Overrun::Present
    }
}
#[doc = "Byte received in RXD register before read of the last received byte (data loss).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverrunWO {
    #[doc = "1: Clear error on write."]
    Clear = 1,
}
impl From<OverrunWO> for bool {
    #[inline(always)]
    fn from(variant: OverrunWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` writer - Byte received in RXD register before read of the last received byte (data loss)."]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG, OverrunWO>;
impl<'a, REG> OverrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OverrunWO::Clear)
    }
}
#[doc = "NACK received after sending the address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anack {
    #[doc = "0: Error not present."]
    NotPresent = 0,
    #[doc = "1: Error present."]
    Present = 1,
}
impl From<Anack> for bool {
    #[inline(always)]
    fn from(variant: Anack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACK` reader - NACK received after sending the address."]
pub type AnackR = crate::BitReader<Anack>;
impl AnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anack {
        match self.bits {
            false => Anack::NotPresent,
            true => Anack::Present,
        }
    }
    #[doc = "Error not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Anack::NotPresent
    }
    #[doc = "Error present."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Anack::Present
    }
}
#[doc = "NACK received after sending the address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnackWO {
    #[doc = "1: Clear error on write."]
    Clear = 1,
}
impl From<AnackWO> for bool {
    #[inline(always)]
    fn from(variant: AnackWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACK` writer - NACK received after sending the address."]
pub type AnackW<'a, REG> = crate::BitWriter<'a, REG, AnackWO>;
impl<'a, REG> AnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AnackWO::Clear)
    }
}
#[doc = "NACK received after sending a data byte.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dnack {
    #[doc = "0: Error not present."]
    NotPresent = 0,
    #[doc = "1: Error present."]
    Present = 1,
}
impl From<Dnack> for bool {
    #[inline(always)]
    fn from(variant: Dnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNACK` reader - NACK received after sending a data byte."]
pub type DnackR = crate::BitReader<Dnack>;
impl DnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dnack {
        match self.bits {
            false => Dnack::NotPresent,
            true => Dnack::Present,
        }
    }
    #[doc = "Error not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Dnack::NotPresent
    }
    #[doc = "Error present."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Dnack::Present
    }
}
#[doc = "NACK received after sending a data byte.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DnackWO {
    #[doc = "1: Clear error on write."]
    Clear = 1,
}
impl From<DnackWO> for bool {
    #[inline(always)]
    fn from(variant: DnackWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNACK` writer - NACK received after sending a data byte."]
pub type DnackW<'a, REG> = crate::BitWriter<'a, REG, DnackWO>;
impl<'a, REG> DnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DnackWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Byte received in RXD register before read of the last received byte (data loss)."]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NACK received after sending the address."]
    #[inline(always)]
    pub fn anack(&self) -> AnackR {
        AnackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NACK received after sending a data byte."]
    #[inline(always)]
    pub fn dnack(&self) -> DnackR {
        DnackR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Byte received in RXD register before read of the last received byte (data loss)."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, ErrorsrcSpec> {
        OverrunW::new(self, 0)
    }
    #[doc = "Bit 1 - NACK received after sending the address."]
    #[inline(always)]
    pub fn anack(&mut self) -> AnackW<'_, ErrorsrcSpec> {
        AnackW::new(self, 1)
    }
    #[doc = "Bit 2 - NACK received after sending a data byte."]
    #[inline(always)]
    pub fn dnack(&mut self) -> DnackW<'_, ErrorsrcSpec> {
        DnackW::new(self, 2)
    }
}
#[doc = "Two-wire error source. Write error field to 1 to clear error.\n\nYou can [`read`](crate::Reg::read) this register and get [`errorsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorsrcSpec;
impl crate::RegisterSpec for ErrorsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorsrc::R`](R) reader structure"]
impl crate::Readable for ErrorsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`errorsrc::W`](W) writer structure"]
impl crate::Writable for ErrorsrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ErrorsrcSpec {}
