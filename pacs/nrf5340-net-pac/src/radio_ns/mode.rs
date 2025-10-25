#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: 1 Mbps Nordic proprietary radio mode"]
    Nrf1mbit = 0,
    #[doc = "1: 2 Mbps Nordic proprietary radio mode"]
    Nrf2mbit = 1,
    #[doc = "3: 1 Mbps BLE"]
    Ble1mbit = 3,
    #[doc = "4: 2 Mbps BLE"]
    Ble2mbit = 4,
    #[doc = "5: Long Range 125 kbps TX, 125 kbps and 500 kbps RX"]
    BleLr125kbit = 5,
    #[doc = "6: Long Range 500 kbps TX, 125 kbps and 500 kbps RX"]
    BleLr500kbit = 6,
    #[doc = "15: IEEE 802.15.4-2006 250 kbps"]
    Ieee802154_250kbit = 15,
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
#[doc = "Field `MODE` reader - Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Nrf1mbit),
            1 => Some(Mode::Nrf2mbit),
            3 => Some(Mode::Ble1mbit),
            4 => Some(Mode::Ble2mbit),
            5 => Some(Mode::BleLr125kbit),
            6 => Some(Mode::BleLr500kbit),
            15 => Some(Mode::Ieee802154_250kbit),
            _ => None,
        }
    }
    #[doc = "1 Mbps Nordic proprietary radio mode"]
    #[inline(always)]
    pub fn is_nrf_1mbit(&self) -> bool {
        *self == Mode::Nrf1mbit
    }
    #[doc = "2 Mbps Nordic proprietary radio mode"]
    #[inline(always)]
    pub fn is_nrf_2mbit(&self) -> bool {
        *self == Mode::Nrf2mbit
    }
    #[doc = "1 Mbps BLE"]
    #[inline(always)]
    pub fn is_ble_1mbit(&self) -> bool {
        *self == Mode::Ble1mbit
    }
    #[doc = "2 Mbps BLE"]
    #[inline(always)]
    pub fn is_ble_2mbit(&self) -> bool {
        *self == Mode::Ble2mbit
    }
    #[doc = "Long Range 125 kbps TX, 125 kbps and 500 kbps RX"]
    #[inline(always)]
    pub fn is_ble_lr125kbit(&self) -> bool {
        *self == Mode::BleLr125kbit
    }
    #[doc = "Long Range 500 kbps TX, 125 kbps and 500 kbps RX"]
    #[inline(always)]
    pub fn is_ble_lr500kbit(&self) -> bool {
        *self == Mode::BleLr500kbit
    }
    #[doc = "IEEE 802.15.4-2006 250 kbps"]
    #[inline(always)]
    pub fn is_ieee802154_250kbit(&self) -> bool {
        *self == Mode::Ieee802154_250kbit
    }
}
#[doc = "Field `MODE` writer - Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 Mbps Nordic proprietary radio mode"]
    #[inline(always)]
    pub fn nrf_1mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Nrf1mbit)
    }
    #[doc = "2 Mbps Nordic proprietary radio mode"]
    #[inline(always)]
    pub fn nrf_2mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Nrf2mbit)
    }
    #[doc = "1 Mbps BLE"]
    #[inline(always)]
    pub fn ble_1mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ble1mbit)
    }
    #[doc = "2 Mbps BLE"]
    #[inline(always)]
    pub fn ble_2mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ble2mbit)
    }
    #[doc = "Long Range 125 kbps TX, 125 kbps and 500 kbps RX"]
    #[inline(always)]
    pub fn ble_lr125kbit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::BleLr125kbit)
    }
    #[doc = "Long Range 500 kbps TX, 125 kbps and 500 kbps RX"]
    #[inline(always)]
    pub fn ble_lr500kbit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::BleLr500kbit)
    }
    #[doc = "IEEE 802.15.4-2006 250 kbps"]
    #[inline(always)]
    pub fn ieee802154_250kbit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ieee802154_250kbit)
    }
}
impl R {
    #[doc = "Bits 0:3 - Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ModeSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Data rate and modulation\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {}
