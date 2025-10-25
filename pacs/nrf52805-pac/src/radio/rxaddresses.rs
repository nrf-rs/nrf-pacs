#[doc = "Register `RXADDRESSES` reader"]
pub type R = crate::R<RxaddressesSpec>;
#[doc = "Register `RXADDRESSES` writer"]
pub type W = crate::W<RxaddressesSpec>;
#[doc = "Enable or disable reception on logical address 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addr0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Addr0> for bool {
    #[inline(always)]
    fn from(variant: Addr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR0` reader - Enable or disable reception on logical address 0."]
pub type Addr0R = crate::BitReader<Addr0>;
impl Addr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addr0 {
        match self.bits {
            false => Addr0::Disabled,
            true => Addr0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addr0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addr0::Enabled
    }
}
#[doc = "Field `ADDR0` writer - Enable or disable reception on logical address 0."]
pub type Addr0W<'a, REG> = crate::BitWriter<'a, REG, Addr0>;
impl<'a, REG> Addr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr0::Enabled)
    }
}
#[doc = "Enable or disable reception on logical address 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addr1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Addr1> for bool {
    #[inline(always)]
    fn from(variant: Addr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR1` reader - Enable or disable reception on logical address 1."]
pub type Addr1R = crate::BitReader<Addr1>;
impl Addr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addr1 {
        match self.bits {
            false => Addr1::Disabled,
            true => Addr1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addr1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addr1::Enabled
    }
}
#[doc = "Field `ADDR1` writer - Enable or disable reception on logical address 1."]
pub type Addr1W<'a, REG> = crate::BitWriter<'a, REG, Addr1>;
impl<'a, REG> Addr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr1::Enabled)
    }
}
#[doc = "Enable or disable reception on logical address 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addr2 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Addr2> for bool {
    #[inline(always)]
    fn from(variant: Addr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR2` reader - Enable or disable reception on logical address 2."]
pub type Addr2R = crate::BitReader<Addr2>;
impl Addr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addr2 {
        match self.bits {
            false => Addr2::Disabled,
            true => Addr2::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addr2::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addr2::Enabled
    }
}
#[doc = "Field `ADDR2` writer - Enable or disable reception on logical address 2."]
pub type Addr2W<'a, REG> = crate::BitWriter<'a, REG, Addr2>;
impl<'a, REG> Addr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr2::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr2::Enabled)
    }
}
#[doc = "Enable or disable reception on logical address 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addr3 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Addr3> for bool {
    #[inline(always)]
    fn from(variant: Addr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR3` reader - Enable or disable reception on logical address 3."]
pub type Addr3R = crate::BitReader<Addr3>;
impl Addr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addr3 {
        match self.bits {
            false => Addr3::Disabled,
            true => Addr3::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addr3::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addr3::Enabled
    }
}
#[doc = "Field `ADDR3` writer - Enable or disable reception on logical address 3."]
pub type Addr3W<'a, REG> = crate::BitWriter<'a, REG, Addr3>;
impl<'a, REG> Addr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr3::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr3::Enabled)
    }
}
#[doc = "Enable or disable reception on logical address 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addr4 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Addr4> for bool {
    #[inline(always)]
    fn from(variant: Addr4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR4` reader - Enable or disable reception on logical address 4."]
pub type Addr4R = crate::BitReader<Addr4>;
impl Addr4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addr4 {
        match self.bits {
            false => Addr4::Disabled,
            true => Addr4::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addr4::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addr4::Enabled
    }
}
#[doc = "Field `ADDR4` writer - Enable or disable reception on logical address 4."]
pub type Addr4W<'a, REG> = crate::BitWriter<'a, REG, Addr4>;
impl<'a, REG> Addr4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr4::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr4::Enabled)
    }
}
#[doc = "Enable or disable reception on logical address 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addr5 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Addr5> for bool {
    #[inline(always)]
    fn from(variant: Addr5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR5` reader - Enable or disable reception on logical address 5."]
pub type Addr5R = crate::BitReader<Addr5>;
impl Addr5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addr5 {
        match self.bits {
            false => Addr5::Disabled,
            true => Addr5::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addr5::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addr5::Enabled
    }
}
#[doc = "Field `ADDR5` writer - Enable or disable reception on logical address 5."]
pub type Addr5W<'a, REG> = crate::BitWriter<'a, REG, Addr5>;
impl<'a, REG> Addr5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr5::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr5::Enabled)
    }
}
#[doc = "Enable or disable reception on logical address 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addr6 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Addr6> for bool {
    #[inline(always)]
    fn from(variant: Addr6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR6` reader - Enable or disable reception on logical address 6."]
pub type Addr6R = crate::BitReader<Addr6>;
impl Addr6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addr6 {
        match self.bits {
            false => Addr6::Disabled,
            true => Addr6::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addr6::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addr6::Enabled
    }
}
#[doc = "Field `ADDR6` writer - Enable or disable reception on logical address 6."]
pub type Addr6W<'a, REG> = crate::BitWriter<'a, REG, Addr6>;
impl<'a, REG> Addr6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr6::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr6::Enabled)
    }
}
#[doc = "Enable or disable reception on logical address 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addr7 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Addr7> for bool {
    #[inline(always)]
    fn from(variant: Addr7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR7` reader - Enable or disable reception on logical address 7."]
pub type Addr7R = crate::BitReader<Addr7>;
impl Addr7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addr7 {
        match self.bits {
            false => Addr7::Disabled,
            true => Addr7::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addr7::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addr7::Enabled
    }
}
#[doc = "Field `ADDR7` writer - Enable or disable reception on logical address 7."]
pub type Addr7W<'a, REG> = crate::BitWriter<'a, REG, Addr7>;
impl<'a, REG> Addr7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr7::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addr7::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable reception on logical address 0."]
    #[inline(always)]
    pub fn addr0(&self) -> Addr0R {
        Addr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable reception on logical address 1."]
    #[inline(always)]
    pub fn addr1(&self) -> Addr1R {
        Addr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable reception on logical address 2."]
    #[inline(always)]
    pub fn addr2(&self) -> Addr2R {
        Addr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable reception on logical address 3."]
    #[inline(always)]
    pub fn addr3(&self) -> Addr3R {
        Addr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable reception on logical address 4."]
    #[inline(always)]
    pub fn addr4(&self) -> Addr4R {
        Addr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable reception on logical address 5."]
    #[inline(always)]
    pub fn addr5(&self) -> Addr5R {
        Addr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable reception on logical address 6."]
    #[inline(always)]
    pub fn addr6(&self) -> Addr6R {
        Addr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable reception on logical address 7."]
    #[inline(always)]
    pub fn addr7(&self) -> Addr7R {
        Addr7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable reception on logical address 0."]
    #[inline(always)]
    pub fn addr0(&mut self) -> Addr0W<'_, RxaddressesSpec> {
        Addr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable reception on logical address 1."]
    #[inline(always)]
    pub fn addr1(&mut self) -> Addr1W<'_, RxaddressesSpec> {
        Addr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable reception on logical address 2."]
    #[inline(always)]
    pub fn addr2(&mut self) -> Addr2W<'_, RxaddressesSpec> {
        Addr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable reception on logical address 3."]
    #[inline(always)]
    pub fn addr3(&mut self) -> Addr3W<'_, RxaddressesSpec> {
        Addr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable reception on logical address 4."]
    #[inline(always)]
    pub fn addr4(&mut self) -> Addr4W<'_, RxaddressesSpec> {
        Addr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable reception on logical address 5."]
    #[inline(always)]
    pub fn addr5(&mut self) -> Addr5W<'_, RxaddressesSpec> {
        Addr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable reception on logical address 6."]
    #[inline(always)]
    pub fn addr6(&mut self) -> Addr6W<'_, RxaddressesSpec> {
        Addr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable reception on logical address 7."]
    #[inline(always)]
    pub fn addr7(&mut self) -> Addr7W<'_, RxaddressesSpec> {
        Addr7W::new(self, 7)
    }
}
#[doc = "Receive address select\n\nYou can [`read`](crate::Reg::read) this register and get [`rxaddresses::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxaddresses::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxaddressesSpec;
impl crate::RegisterSpec for RxaddressesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxaddresses::R`](R) reader structure"]
impl crate::Readable for RxaddressesSpec {}
#[doc = "`write(|w| ..)` method takes [`rxaddresses::W`](W) writer structure"]
impl crate::Writable for RxaddressesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXADDRESSES to value 0"]
impl crate::Resettable for RxaddressesSpec {}
