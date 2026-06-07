#[doc = "Register `FCG1` reader"]
pub type R = crate::R<Fcg1Spec>;
#[doc = "Register `FCG1` writer"]
pub type W = crate::W<Fcg1Spec>;
#[doc = "Field `MCAN1` reader - desc MCAN1"]
pub type Mcan1R = crate::BitReader;
#[doc = "Field `MCAN1` writer - desc MCAN1"]
pub type Mcan1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN2` reader - desc MCAN2"]
pub type Mcan2R = crate::BitReader;
#[doc = "Field `MCAN2` writer - desc MCAN2"]
pub type Mcan2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI` reader - desc QSPI"]
pub type QspiR = crate::BitReader;
#[doc = "Field `QSPI` writer - desc QSPI"]
pub type QspiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - desc I2C1"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - desc I2C1"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - desc I2C2"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C2` writer - desc I2C2"]
pub type I2c2W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - desc MCAN1"]
    #[inline(always)]
    pub fn mcan1(&self) -> Mcan1R {
        Mcan1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc MCAN2"]
    #[inline(always)]
    pub fn mcan2(&self) -> Mcan2R {
        Mcan2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc QSPI"]
    #[inline(always)]
    pub fn qspi(&self) -> QspiR {
        QspiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 5) & 1) != 0)
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
        f.debug_struct("FCG1")
            .field("mcan1", &self.mcan1())
            .field("mcan2", &self.mcan2())
            .field("qspi", &self.qspi())
            .field("i2c1", &self.i2c1())
            .field("i2c2", &self.i2c2())
            .field("spi1", &self.spi1())
            .field("spi2", &self.spi2())
            .field("spi3", &self.spi3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc MCAN1"]
    #[inline(always)]
    pub fn mcan1(&mut self) -> Mcan1W<'_, Fcg1Spec> {
        Mcan1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc MCAN2"]
    #[inline(always)]
    pub fn mcan2(&mut self) -> Mcan2W<'_, Fcg1Spec> {
        Mcan2W::new(self, 1)
    }
    #[doc = "Bit 3 - desc QSPI"]
    #[inline(always)]
    pub fn qspi(&mut self) -> QspiW<'_, Fcg1Spec> {
        QspiW::new(self, 3)
    }
    #[doc = "Bit 4 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Fcg1Spec> {
        I2c1W::new(self, 4)
    }
    #[doc = "Bit 5 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<'_, Fcg1Spec> {
        I2c2W::new(self, 5)
    }
    #[doc = "Bit 16 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, Fcg1Spec> {
        Spi1W::new(self, 16)
    }
    #[doc = "Bit 17 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<'_, Fcg1Spec> {
        Spi2W::new(self, 17)
    }
    #[doc = "Bit 18 - desc SPI3"]
    #[inline(always)]
    pub fn spi3(&mut self) -> Spi3W<'_, Fcg1Spec> {
        Spi3W::new(self, 18)
    }
}
#[doc = "desc FCG1\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcg1Spec;
impl crate::RegisterSpec for Fcg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcg1::R`](R) reader structure"]
impl crate::Readable for Fcg1Spec {}
#[doc = "`write(|w| ..)` method takes [`fcg1::W`](W) writer structure"]
impl crate::Writable for Fcg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCG1 to value 0xffff_ffff"]
impl crate::Resettable for Fcg1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
