#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TxdataSpec>;
#[doc = "Field `TXDATA` writer - TX Data"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UBRXAT` writer - Unblock RX After Transmission"]
pub type UbrxatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIAT` writer - Set TXTRI After Transmisssion"]
pub type TxtriatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBREAK` writer - Transit Data as Break"]
pub type TxbreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISAT` writer - Clear TXEN After Transmission"]
pub type TxdisatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENAT` writer - Enable RXEN After Transmission"]
pub type RxenatW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<'_, TxdataSpec> {
        TxdataW::new(self, 0)
    }
    #[doc = "Bit 9 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat(&mut self) -> UbrxatW<'_, TxdataSpec> {
        UbrxatW::new(self, 9)
    }
    #[doc = "Bit 10 - Set TXTRI After Transmisssion"]
    #[inline(always)]
    pub fn txtriat(&mut self) -> TxtriatW<'_, TxdataSpec> {
        TxtriatW::new(self, 10)
    }
    #[doc = "Bit 11 - Transit Data as Break"]
    #[inline(always)]
    pub fn txbreak(&mut self) -> TxbreakW<'_, TxdataSpec> {
        TxbreakW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat(&mut self) -> TxdisatW<'_, TxdataSpec> {
        TxdisatW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable RXEN After Transmission"]
    #[inline(always)]
    pub fn rxenat(&mut self) -> RxenatW<'_, TxdataSpec> {
        RxenatW::new(self, 13)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdataSpec;
impl crate::RegisterSpec for TxdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TxdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TxdataSpec {}
