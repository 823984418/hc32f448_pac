#[doc = "Register `STFLR` reader"]
pub type R = crate::R<StflrSpec>;
#[doc = "Register `STFLR` writer"]
pub type W = crate::W<StflrSpec>;
#[doc = "Field `CMPF1` reader - desc CMPF1"]
pub type Cmpf1R = crate::BitReader;
#[doc = "Field `CMPF1` writer - desc CMPF1"]
pub type Cmpf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF2` reader - desc CMPF2"]
pub type Cmpf2R = crate::BitReader;
#[doc = "Field `CMPF2` writer - desc CMPF2"]
pub type Cmpf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF3` reader - desc CMPF3"]
pub type Cmpf3R = crate::BitReader;
#[doc = "Field `CMPF3` writer - desc CMPF3"]
pub type Cmpf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF4` reader - desc CMPF4"]
pub type Cmpf4R = crate::BitReader;
#[doc = "Field `CMPF4` writer - desc CMPF4"]
pub type Cmpf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF5` reader - desc CMPF5"]
pub type Cmpf5R = crate::BitReader;
#[doc = "Field `CMPF5` writer - desc CMPF5"]
pub type Cmpf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF6` reader - desc CMPF6"]
pub type Cmpf6R = crate::BitReader;
#[doc = "Field `CMPF6` writer - desc CMPF6"]
pub type Cmpf6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF7` reader - desc CMPF7"]
pub type Cmpf7R = crate::BitReader;
#[doc = "Field `CMPF7` writer - desc CMPF7"]
pub type Cmpf7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF8` reader - desc CMPF8"]
pub type Cmpf8R = crate::BitReader;
#[doc = "Field `CMPF8` writer - desc CMPF8"]
pub type Cmpf8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPF1` reader - desc ICPF1"]
pub type Icpf1R = crate::BitReader;
#[doc = "Field `ICPF1` writer - desc ICPF1"]
pub type Icpf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPF2` reader - desc ICPF2"]
pub type Icpf2R = crate::BitReader;
#[doc = "Field `ICPF2` writer - desc ICPF2"]
pub type Icpf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPF3` reader - desc ICPF3"]
pub type Icpf3R = crate::BitReader;
#[doc = "Field `ICPF3` writer - desc ICPF3"]
pub type Icpf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPF4` reader - desc ICPF4"]
pub type Icpf4R = crate::BitReader;
#[doc = "Field `ICPF4` writer - desc ICPF4"]
pub type Icpf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPF5` reader - desc ICPF5"]
pub type Icpf5R = crate::BitReader;
#[doc = "Field `ICPF5` writer - desc ICPF5"]
pub type Icpf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPF6` reader - desc ICPF6"]
pub type Icpf6R = crate::BitReader;
#[doc = "Field `ICPF6` writer - desc ICPF6"]
pub type Icpf6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPF7` reader - desc ICPF7"]
pub type Icpf7R = crate::BitReader;
#[doc = "Field `ICPF7` writer - desc ICPF7"]
pub type Icpf7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPF8` reader - desc ICPF8"]
pub type Icpf8R = crate::BitReader;
#[doc = "Field `ICPF8` writer - desc ICPF8"]
pub type Icpf8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CMPF1"]
    #[inline(always)]
    pub fn cmpf1(&self) -> Cmpf1R {
        Cmpf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CMPF2"]
    #[inline(always)]
    pub fn cmpf2(&self) -> Cmpf2R {
        Cmpf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMPF3"]
    #[inline(always)]
    pub fn cmpf3(&self) -> Cmpf3R {
        Cmpf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CMPF4"]
    #[inline(always)]
    pub fn cmpf4(&self) -> Cmpf4R {
        Cmpf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CMPF5"]
    #[inline(always)]
    pub fn cmpf5(&self) -> Cmpf5R {
        Cmpf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CMPF6"]
    #[inline(always)]
    pub fn cmpf6(&self) -> Cmpf6R {
        Cmpf6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CMPF7"]
    #[inline(always)]
    pub fn cmpf7(&self) -> Cmpf7R {
        Cmpf7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CMPF8"]
    #[inline(always)]
    pub fn cmpf8(&self) -> Cmpf8R {
        Cmpf8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ICPF1"]
    #[inline(always)]
    pub fn icpf1(&self) -> Icpf1R {
        Icpf1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ICPF2"]
    #[inline(always)]
    pub fn icpf2(&self) -> Icpf2R {
        Icpf2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ICPF3"]
    #[inline(always)]
    pub fn icpf3(&self) -> Icpf3R {
        Icpf3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc ICPF4"]
    #[inline(always)]
    pub fn icpf4(&self) -> Icpf4R {
        Icpf4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc ICPF5"]
    #[inline(always)]
    pub fn icpf5(&self) -> Icpf5R {
        Icpf5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc ICPF6"]
    #[inline(always)]
    pub fn icpf6(&self) -> Icpf6R {
        Icpf6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ICPF7"]
    #[inline(always)]
    pub fn icpf7(&self) -> Icpf7R {
        Icpf7R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ICPF8"]
    #[inline(always)]
    pub fn icpf8(&self) -> Icpf8R {
        Icpf8R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STFLR")
            .field("cmpf1", &self.cmpf1())
            .field("cmpf2", &self.cmpf2())
            .field("cmpf3", &self.cmpf3())
            .field("cmpf4", &self.cmpf4())
            .field("cmpf5", &self.cmpf5())
            .field("cmpf6", &self.cmpf6())
            .field("cmpf7", &self.cmpf7())
            .field("cmpf8", &self.cmpf8())
            .field("icpf1", &self.icpf1())
            .field("icpf2", &self.icpf2())
            .field("icpf3", &self.icpf3())
            .field("icpf4", &self.icpf4())
            .field("icpf5", &self.icpf5())
            .field("icpf6", &self.icpf6())
            .field("icpf7", &self.icpf7())
            .field("icpf8", &self.icpf8())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc CMPF1"]
    #[inline(always)]
    pub fn cmpf1(&mut self) -> Cmpf1W<'_, StflrSpec> {
        Cmpf1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc CMPF2"]
    #[inline(always)]
    pub fn cmpf2(&mut self) -> Cmpf2W<'_, StflrSpec> {
        Cmpf2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMPF3"]
    #[inline(always)]
    pub fn cmpf3(&mut self) -> Cmpf3W<'_, StflrSpec> {
        Cmpf3W::new(self, 2)
    }
    #[doc = "Bit 3 - desc CMPF4"]
    #[inline(always)]
    pub fn cmpf4(&mut self) -> Cmpf4W<'_, StflrSpec> {
        Cmpf4W::new(self, 3)
    }
    #[doc = "Bit 4 - desc CMPF5"]
    #[inline(always)]
    pub fn cmpf5(&mut self) -> Cmpf5W<'_, StflrSpec> {
        Cmpf5W::new(self, 4)
    }
    #[doc = "Bit 5 - desc CMPF6"]
    #[inline(always)]
    pub fn cmpf6(&mut self) -> Cmpf6W<'_, StflrSpec> {
        Cmpf6W::new(self, 5)
    }
    #[doc = "Bit 6 - desc CMPF7"]
    #[inline(always)]
    pub fn cmpf7(&mut self) -> Cmpf7W<'_, StflrSpec> {
        Cmpf7W::new(self, 6)
    }
    #[doc = "Bit 7 - desc CMPF8"]
    #[inline(always)]
    pub fn cmpf8(&mut self) -> Cmpf8W<'_, StflrSpec> {
        Cmpf8W::new(self, 7)
    }
    #[doc = "Bit 8 - desc ICPF1"]
    #[inline(always)]
    pub fn icpf1(&mut self) -> Icpf1W<'_, StflrSpec> {
        Icpf1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc ICPF2"]
    #[inline(always)]
    pub fn icpf2(&mut self) -> Icpf2W<'_, StflrSpec> {
        Icpf2W::new(self, 9)
    }
    #[doc = "Bit 10 - desc ICPF3"]
    #[inline(always)]
    pub fn icpf3(&mut self) -> Icpf3W<'_, StflrSpec> {
        Icpf3W::new(self, 10)
    }
    #[doc = "Bit 11 - desc ICPF4"]
    #[inline(always)]
    pub fn icpf4(&mut self) -> Icpf4W<'_, StflrSpec> {
        Icpf4W::new(self, 11)
    }
    #[doc = "Bit 12 - desc ICPF5"]
    #[inline(always)]
    pub fn icpf5(&mut self) -> Icpf5W<'_, StflrSpec> {
        Icpf5W::new(self, 12)
    }
    #[doc = "Bit 13 - desc ICPF6"]
    #[inline(always)]
    pub fn icpf6(&mut self) -> Icpf6W<'_, StflrSpec> {
        Icpf6W::new(self, 13)
    }
    #[doc = "Bit 14 - desc ICPF7"]
    #[inline(always)]
    pub fn icpf7(&mut self) -> Icpf7W<'_, StflrSpec> {
        Icpf7W::new(self, 14)
    }
    #[doc = "Bit 15 - desc ICPF8"]
    #[inline(always)]
    pub fn icpf8(&mut self) -> Icpf8W<'_, StflrSpec> {
        Icpf8W::new(self, 15)
    }
}
#[doc = "desc STFLR\n\nYou can [`read`](crate::Reg::read) this register and get [`stflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StflrSpec;
impl crate::RegisterSpec for StflrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`stflr::R`](R) reader structure"]
impl crate::Readable for StflrSpec {}
#[doc = "`write(|w| ..)` method takes [`stflr::W`](W) writer structure"]
impl crate::Writable for StflrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STFLR to value 0"]
impl crate::Resettable for StflrSpec {}
