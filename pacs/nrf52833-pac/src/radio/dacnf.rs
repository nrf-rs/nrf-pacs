#[doc = "Register `DACNF` reader"]
pub type R = crate::R<DacnfSpec>;
#[doc = "Register `DACNF` writer"]
pub type W = crate::W<DacnfSpec>;
#[doc = "Enable or disable device address matching using device address 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena0 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Ena0> for bool {
    #[inline(always)]
    fn from(variant: Ena0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA0` reader - Enable or disable device address matching using device address 0"]
pub type Ena0R = crate::BitReader<Ena0>;
impl Ena0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena0 {
        match self.bits {
            false => Ena0::Disabled,
            true => Ena0::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena0::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena0::Enabled
    }
}
#[doc = "Field `ENA0` writer - Enable or disable device address matching using device address 0"]
pub type Ena0W<'a, REG> = crate::BitWriter<'a, REG, Ena0>;
impl<'a, REG> Ena0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena0::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena0::Enabled)
    }
}
#[doc = "Enable or disable device address matching using device address 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena1 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Ena1> for bool {
    #[inline(always)]
    fn from(variant: Ena1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA1` reader - Enable or disable device address matching using device address 1"]
pub type Ena1R = crate::BitReader<Ena1>;
impl Ena1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena1 {
        match self.bits {
            false => Ena1::Disabled,
            true => Ena1::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena1::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena1::Enabled
    }
}
#[doc = "Field `ENA1` writer - Enable or disable device address matching using device address 1"]
pub type Ena1W<'a, REG> = crate::BitWriter<'a, REG, Ena1>;
impl<'a, REG> Ena1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena1::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena1::Enabled)
    }
}
#[doc = "Enable or disable device address matching using device address 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena2 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Ena2> for bool {
    #[inline(always)]
    fn from(variant: Ena2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA2` reader - Enable or disable device address matching using device address 2"]
pub type Ena2R = crate::BitReader<Ena2>;
impl Ena2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena2 {
        match self.bits {
            false => Ena2::Disabled,
            true => Ena2::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena2::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena2::Enabled
    }
}
#[doc = "Field `ENA2` writer - Enable or disable device address matching using device address 2"]
pub type Ena2W<'a, REG> = crate::BitWriter<'a, REG, Ena2>;
impl<'a, REG> Ena2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena2::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena2::Enabled)
    }
}
#[doc = "Enable or disable device address matching using device address 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena3 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Ena3> for bool {
    #[inline(always)]
    fn from(variant: Ena3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA3` reader - Enable or disable device address matching using device address 3"]
pub type Ena3R = crate::BitReader<Ena3>;
impl Ena3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena3 {
        match self.bits {
            false => Ena3::Disabled,
            true => Ena3::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena3::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena3::Enabled
    }
}
#[doc = "Field `ENA3` writer - Enable or disable device address matching using device address 3"]
pub type Ena3W<'a, REG> = crate::BitWriter<'a, REG, Ena3>;
impl<'a, REG> Ena3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena3::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena3::Enabled)
    }
}
#[doc = "Enable or disable device address matching using device address 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena4 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Ena4> for bool {
    #[inline(always)]
    fn from(variant: Ena4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA4` reader - Enable or disable device address matching using device address 4"]
pub type Ena4R = crate::BitReader<Ena4>;
impl Ena4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena4 {
        match self.bits {
            false => Ena4::Disabled,
            true => Ena4::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena4::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena4::Enabled
    }
}
#[doc = "Field `ENA4` writer - Enable or disable device address matching using device address 4"]
pub type Ena4W<'a, REG> = crate::BitWriter<'a, REG, Ena4>;
impl<'a, REG> Ena4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena4::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena4::Enabled)
    }
}
#[doc = "Enable or disable device address matching using device address 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena5 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Ena5> for bool {
    #[inline(always)]
    fn from(variant: Ena5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA5` reader - Enable or disable device address matching using device address 5"]
pub type Ena5R = crate::BitReader<Ena5>;
impl Ena5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena5 {
        match self.bits {
            false => Ena5::Disabled,
            true => Ena5::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena5::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena5::Enabled
    }
}
#[doc = "Field `ENA5` writer - Enable or disable device address matching using device address 5"]
pub type Ena5W<'a, REG> = crate::BitWriter<'a, REG, Ena5>;
impl<'a, REG> Ena5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena5::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena5::Enabled)
    }
}
#[doc = "Enable or disable device address matching using device address 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena6 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Ena6> for bool {
    #[inline(always)]
    fn from(variant: Ena6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA6` reader - Enable or disable device address matching using device address 6"]
pub type Ena6R = crate::BitReader<Ena6>;
impl Ena6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena6 {
        match self.bits {
            false => Ena6::Disabled,
            true => Ena6::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena6::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena6::Enabled
    }
}
#[doc = "Field `ENA6` writer - Enable or disable device address matching using device address 6"]
pub type Ena6W<'a, REG> = crate::BitWriter<'a, REG, Ena6>;
impl<'a, REG> Ena6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena6::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena6::Enabled)
    }
}
#[doc = "Enable or disable device address matching using device address 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena7 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Ena7> for bool {
    #[inline(always)]
    fn from(variant: Ena7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA7` reader - Enable or disable device address matching using device address 7"]
pub type Ena7R = crate::BitReader<Ena7>;
impl Ena7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena7 {
        match self.bits {
            false => Ena7::Disabled,
            true => Ena7::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena7::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena7::Enabled
    }
}
#[doc = "Field `ENA7` writer - Enable or disable device address matching using device address 7"]
pub type Ena7W<'a, REG> = crate::BitWriter<'a, REG, Ena7>;
impl<'a, REG> Ena7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena7::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena7::Enabled)
    }
}
#[doc = "Field `TXADD0` reader - TxAdd for device address 0"]
pub type Txadd0R = crate::BitReader;
#[doc = "Field `TXADD0` writer - TxAdd for device address 0"]
pub type Txadd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXADD1` reader - TxAdd for device address 1"]
pub type Txadd1R = crate::BitReader;
#[doc = "Field `TXADD1` writer - TxAdd for device address 1"]
pub type Txadd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXADD2` reader - TxAdd for device address 2"]
pub type Txadd2R = crate::BitReader;
#[doc = "Field `TXADD2` writer - TxAdd for device address 2"]
pub type Txadd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXADD3` reader - TxAdd for device address 3"]
pub type Txadd3R = crate::BitReader;
#[doc = "Field `TXADD3` writer - TxAdd for device address 3"]
pub type Txadd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXADD4` reader - TxAdd for device address 4"]
pub type Txadd4R = crate::BitReader;
#[doc = "Field `TXADD4` writer - TxAdd for device address 4"]
pub type Txadd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXADD5` reader - TxAdd for device address 5"]
pub type Txadd5R = crate::BitReader;
#[doc = "Field `TXADD5` writer - TxAdd for device address 5"]
pub type Txadd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXADD6` reader - TxAdd for device address 6"]
pub type Txadd6R = crate::BitReader;
#[doc = "Field `TXADD6` writer - TxAdd for device address 6"]
pub type Txadd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXADD7` reader - TxAdd for device address 7"]
pub type Txadd7R = crate::BitReader;
#[doc = "Field `TXADD7` writer - TxAdd for device address 7"]
pub type Txadd7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub fn ena0(&self) -> Ena0R {
        Ena0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub fn ena1(&self) -> Ena1R {
        Ena1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub fn ena2(&self) -> Ena2R {
        Ena2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub fn ena3(&self) -> Ena3R {
        Ena3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub fn ena4(&self) -> Ena4R {
        Ena4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub fn ena5(&self) -> Ena5R {
        Ena5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub fn ena6(&self) -> Ena6R {
        Ena6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub fn ena7(&self) -> Ena7R {
        Ena7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TxAdd for device address 0"]
    #[inline(always)]
    pub fn txadd0(&self) -> Txadd0R {
        Txadd0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TxAdd for device address 1"]
    #[inline(always)]
    pub fn txadd1(&self) -> Txadd1R {
        Txadd1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TxAdd for device address 2"]
    #[inline(always)]
    pub fn txadd2(&self) -> Txadd2R {
        Txadd2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TxAdd for device address 3"]
    #[inline(always)]
    pub fn txadd3(&self) -> Txadd3R {
        Txadd3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TxAdd for device address 4"]
    #[inline(always)]
    pub fn txadd4(&self) -> Txadd4R {
        Txadd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TxAdd for device address 5"]
    #[inline(always)]
    pub fn txadd5(&self) -> Txadd5R {
        Txadd5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TxAdd for device address 6"]
    #[inline(always)]
    pub fn txadd6(&self) -> Txadd6R {
        Txadd6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TxAdd for device address 7"]
    #[inline(always)]
    pub fn txadd7(&self) -> Txadd7R {
        Txadd7R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub fn ena0(&mut self) -> Ena0W<'_, DacnfSpec> {
        Ena0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub fn ena1(&mut self) -> Ena1W<'_, DacnfSpec> {
        Ena1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub fn ena2(&mut self) -> Ena2W<'_, DacnfSpec> {
        Ena2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub fn ena3(&mut self) -> Ena3W<'_, DacnfSpec> {
        Ena3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub fn ena4(&mut self) -> Ena4W<'_, DacnfSpec> {
        Ena4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub fn ena5(&mut self) -> Ena5W<'_, DacnfSpec> {
        Ena5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub fn ena6(&mut self) -> Ena6W<'_, DacnfSpec> {
        Ena6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub fn ena7(&mut self) -> Ena7W<'_, DacnfSpec> {
        Ena7W::new(self, 7)
    }
    #[doc = "Bit 8 - TxAdd for device address 0"]
    #[inline(always)]
    pub fn txadd0(&mut self) -> Txadd0W<'_, DacnfSpec> {
        Txadd0W::new(self, 8)
    }
    #[doc = "Bit 9 - TxAdd for device address 1"]
    #[inline(always)]
    pub fn txadd1(&mut self) -> Txadd1W<'_, DacnfSpec> {
        Txadd1W::new(self, 9)
    }
    #[doc = "Bit 10 - TxAdd for device address 2"]
    #[inline(always)]
    pub fn txadd2(&mut self) -> Txadd2W<'_, DacnfSpec> {
        Txadd2W::new(self, 10)
    }
    #[doc = "Bit 11 - TxAdd for device address 3"]
    #[inline(always)]
    pub fn txadd3(&mut self) -> Txadd3W<'_, DacnfSpec> {
        Txadd3W::new(self, 11)
    }
    #[doc = "Bit 12 - TxAdd for device address 4"]
    #[inline(always)]
    pub fn txadd4(&mut self) -> Txadd4W<'_, DacnfSpec> {
        Txadd4W::new(self, 12)
    }
    #[doc = "Bit 13 - TxAdd for device address 5"]
    #[inline(always)]
    pub fn txadd5(&mut self) -> Txadd5W<'_, DacnfSpec> {
        Txadd5W::new(self, 13)
    }
    #[doc = "Bit 14 - TxAdd for device address 6"]
    #[inline(always)]
    pub fn txadd6(&mut self) -> Txadd6W<'_, DacnfSpec> {
        Txadd6W::new(self, 14)
    }
    #[doc = "Bit 15 - TxAdd for device address 7"]
    #[inline(always)]
    pub fn txadd7(&mut self) -> Txadd7W<'_, DacnfSpec> {
        Txadd7W::new(self, 15)
    }
}
#[doc = "Device address match configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dacnf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacnf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacnfSpec;
impl crate::RegisterSpec for DacnfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacnf::R`](R) reader structure"]
impl crate::Readable for DacnfSpec {}
#[doc = "`write(|w| ..)` method takes [`dacnf::W`](W) writer structure"]
impl crate::Writable for DacnfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DACNF to value 0"]
impl crate::Resettable for DacnfSpec {}
