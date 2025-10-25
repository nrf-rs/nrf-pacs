#[doc = "Register `ERRORSRC` reader"]
pub type R = crate::R<ErrorsrcSpec>;
#[doc = "Register `ERRORSRC` writer"]
pub type W = crate::W<ErrorsrcSpec>;
#[doc = "Overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overrun {
    #[doc = "0: Error did not occur"]
    NotReceived = 0,
    #[doc = "1: Error occurred"]
    Received = 1,
}
impl From<Overrun> for bool {
    #[inline(always)]
    fn from(variant: Overrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` reader - Overrun error"]
pub type OverrunR = crate::BitReader<Overrun>;
impl OverrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overrun {
        match self.bits {
            false => Overrun::NotReceived,
            true => Overrun::Received,
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == Overrun::NotReceived
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == Overrun::Received
    }
}
#[doc = "Field `OVERRUN` writer - Overrun error"]
pub type OverrunW<'a, REG> = crate::BitWriter1C<'a, REG, Overrun>;
impl<'a, REG> OverrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_received(self) -> &'a mut crate::W<REG> {
        self.variant(Overrun::NotReceived)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn received(self) -> &'a mut crate::W<REG> {
        self.variant(Overrun::Received)
    }
}
#[doc = "NACK received after sending the address (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anack {
    #[doc = "0: Error did not occur"]
    NotReceived = 0,
    #[doc = "1: Error occurred"]
    Received = 1,
}
impl From<Anack> for bool {
    #[inline(always)]
    fn from(variant: Anack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACK` reader - NACK received after sending the address (write '1' to clear)"]
pub type AnackR = crate::BitReader<Anack>;
impl AnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anack {
        match self.bits {
            false => Anack::NotReceived,
            true => Anack::Received,
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == Anack::NotReceived
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == Anack::Received
    }
}
#[doc = "Field `ANACK` writer - NACK received after sending the address (write '1' to clear)"]
pub type AnackW<'a, REG> = crate::BitWriter1C<'a, REG, Anack>;
impl<'a, REG> AnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_received(self) -> &'a mut crate::W<REG> {
        self.variant(Anack::NotReceived)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn received(self) -> &'a mut crate::W<REG> {
        self.variant(Anack::Received)
    }
}
#[doc = "NACK received after sending a data byte (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dnack {
    #[doc = "0: Error did not occur"]
    NotReceived = 0,
    #[doc = "1: Error occurred"]
    Received = 1,
}
impl From<Dnack> for bool {
    #[inline(always)]
    fn from(variant: Dnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNACK` reader - NACK received after sending a data byte (write '1' to clear)"]
pub type DnackR = crate::BitReader<Dnack>;
impl DnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dnack {
        match self.bits {
            false => Dnack::NotReceived,
            true => Dnack::Received,
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == Dnack::NotReceived
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == Dnack::Received
    }
}
#[doc = "Field `DNACK` writer - NACK received after sending a data byte (write '1' to clear)"]
pub type DnackW<'a, REG> = crate::BitWriter1C<'a, REG, Dnack>;
impl<'a, REG> DnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_received(self) -> &'a mut crate::W<REG> {
        self.variant(Dnack::NotReceived)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn received(self) -> &'a mut crate::W<REG> {
        self.variant(Dnack::Received)
    }
}
impl R {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub fn anack(&self) -> AnackR {
        AnackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub fn dnack(&self) -> DnackR {
        DnackR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, ErrorsrcSpec> {
        OverrunW::new(self, 0)
    }
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub fn anack(&mut self) -> AnackW<'_, ErrorsrcSpec> {
        AnackW::new(self, 1)
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub fn dnack(&mut self) -> DnackW<'_, ErrorsrcSpec> {
        DnackW::new(self, 2)
    }
}
#[doc = "Error source\n\nYou can [`read`](crate::Reg::read) this register and get [`errorsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorsrcSpec;
impl crate::RegisterSpec for ErrorsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorsrc::R`](R) reader structure"]
impl crate::Readable for ErrorsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`errorsrc::W`](W) writer structure"]
impl crate::Writable for ErrorsrcSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ErrorsrcSpec {}
