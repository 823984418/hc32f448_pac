#[doc = "Register `FRST1` reader"]
pub type R = crate::R<Frst1Spec>;
#[doc = "Register `FRST1` writer"]
pub type W = crate::W<Frst1Spec>;
#[doc = "Field `QSPI` reader - desc QSPI"]
pub type QspiR = crate::BitReader;
#[doc = "Field `QSPI` writer - desc QSPI"]
pub type QspiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - desc SPI2"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI2` writer - desc SPI2"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` reader - desc SPI3"]
pub type Spi3R = crate::BitReader;
#[doc = "Field `SPI3` writer - desc SPI3"]
pub type Spi3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - desc QSPI"]
    #[inline(always)]
    pub fn qspi(&self) -> QspiR {
        QspiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc SPI3"]
    #[inline(always)]
    pub fn spi3(&self) -> Spi3R {
        Spi3R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRST1")
            .field("qspi", &self.qspi())
            .field("spi1", &self.spi1())
            .field("spi2", &self.spi2())
            .field("spi3", &self.spi3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - desc QSPI"]
    #[inline(always)]
    pub fn qspi(&mut self) -> QspiW<'_, Frst1Spec> {
        QspiW::new(self, 3)
    }
    #[doc = "Bit 16 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, Frst1Spec> {
        Spi1W::new(self, 16)
    }
    #[doc = "Bit 17 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<'_, Frst1Spec> {
        Spi2W::new(self, 17)
    }
    #[doc = "Bit 18 - desc SPI3"]
    #[inline(always)]
    pub fn spi3(&mut self) -> Spi3W<'_, Frst1Spec> {
        Spi3W::new(self, 18)
    }
}
#[doc = "desc FRST1\n\nYou can [`read`](crate::Reg::read) this register and get [`frst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frst1Spec;
impl crate::RegisterSpec for Frst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frst1::R`](R) reader structure"]
impl crate::Readable for Frst1Spec {}
#[doc = "`write(|w| ..)` method takes [`frst1::W`](W) writer structure"]
impl crate::Writable for Frst1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRST1 to value 0xffff_ffff"]
impl crate::Resettable for Frst1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
