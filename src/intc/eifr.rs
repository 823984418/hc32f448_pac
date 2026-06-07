#[doc = "Register `EIFR` reader"]
pub type R = crate::R<EifrSpec>;
#[doc = "Register `EIFR` writer"]
pub type W = crate::W<EifrSpec>;
#[doc = "Field `EIF0` reader - desc EIF0"]
pub type Eif0R = crate::BitReader;
#[doc = "Field `EIF0` writer - desc EIF0"]
pub type Eif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF1` reader - desc EIF1"]
pub type Eif1R = crate::BitReader;
#[doc = "Field `EIF1` writer - desc EIF1"]
pub type Eif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF2` reader - desc EIF2"]
pub type Eif2R = crate::BitReader;
#[doc = "Field `EIF2` writer - desc EIF2"]
pub type Eif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF3` reader - desc EIF3"]
pub type Eif3R = crate::BitReader;
#[doc = "Field `EIF3` writer - desc EIF3"]
pub type Eif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF4` reader - desc EIF4"]
pub type Eif4R = crate::BitReader;
#[doc = "Field `EIF4` writer - desc EIF4"]
pub type Eif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF5` reader - desc EIF5"]
pub type Eif5R = crate::BitReader;
#[doc = "Field `EIF5` writer - desc EIF5"]
pub type Eif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF6` reader - desc EIF6"]
pub type Eif6R = crate::BitReader;
#[doc = "Field `EIF6` writer - desc EIF6"]
pub type Eif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF7` reader - desc EIF7"]
pub type Eif7R = crate::BitReader;
#[doc = "Field `EIF7` writer - desc EIF7"]
pub type Eif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF8` reader - desc EIF8"]
pub type Eif8R = crate::BitReader;
#[doc = "Field `EIF8` writer - desc EIF8"]
pub type Eif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF9` reader - desc EIF9"]
pub type Eif9R = crate::BitReader;
#[doc = "Field `EIF9` writer - desc EIF9"]
pub type Eif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF10` reader - desc EIF10"]
pub type Eif10R = crate::BitReader;
#[doc = "Field `EIF10` writer - desc EIF10"]
pub type Eif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF11` reader - desc EIF11"]
pub type Eif11R = crate::BitReader;
#[doc = "Field `EIF11` writer - desc EIF11"]
pub type Eif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF12` reader - desc EIF12"]
pub type Eif12R = crate::BitReader;
#[doc = "Field `EIF12` writer - desc EIF12"]
pub type Eif12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF13` reader - desc EIF13"]
pub type Eif13R = crate::BitReader;
#[doc = "Field `EIF13` writer - desc EIF13"]
pub type Eif13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF14` reader - desc EIF14"]
pub type Eif14R = crate::BitReader;
#[doc = "Field `EIF14` writer - desc EIF14"]
pub type Eif14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF15` reader - desc EIF15"]
pub type Eif15R = crate::BitReader;
#[doc = "Field `EIF15` writer - desc EIF15"]
pub type Eif15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EIF0"]
    #[inline(always)]
    pub fn eif0(&self) -> Eif0R {
        Eif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EIF1"]
    #[inline(always)]
    pub fn eif1(&self) -> Eif1R {
        Eif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EIF2"]
    #[inline(always)]
    pub fn eif2(&self) -> Eif2R {
        Eif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc EIF3"]
    #[inline(always)]
    pub fn eif3(&self) -> Eif3R {
        Eif3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc EIF4"]
    #[inline(always)]
    pub fn eif4(&self) -> Eif4R {
        Eif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc EIF5"]
    #[inline(always)]
    pub fn eif5(&self) -> Eif5R {
        Eif5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EIF6"]
    #[inline(always)]
    pub fn eif6(&self) -> Eif6R {
        Eif6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc EIF7"]
    #[inline(always)]
    pub fn eif7(&self) -> Eif7R {
        Eif7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc EIF8"]
    #[inline(always)]
    pub fn eif8(&self) -> Eif8R {
        Eif8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EIF9"]
    #[inline(always)]
    pub fn eif9(&self) -> Eif9R {
        Eif9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc EIF10"]
    #[inline(always)]
    pub fn eif10(&self) -> Eif10R {
        Eif10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc EIF11"]
    #[inline(always)]
    pub fn eif11(&self) -> Eif11R {
        Eif11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EIF12"]
    #[inline(always)]
    pub fn eif12(&self) -> Eif12R {
        Eif12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc EIF13"]
    #[inline(always)]
    pub fn eif13(&self) -> Eif13R {
        Eif13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc EIF14"]
    #[inline(always)]
    pub fn eif14(&self) -> Eif14R {
        Eif14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EIF15"]
    #[inline(always)]
    pub fn eif15(&self) -> Eif15R {
        Eif15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EIFR")
            .field("eif0", &self.eif0())
            .field("eif1", &self.eif1())
            .field("eif2", &self.eif2())
            .field("eif3", &self.eif3())
            .field("eif4", &self.eif4())
            .field("eif5", &self.eif5())
            .field("eif6", &self.eif6())
            .field("eif7", &self.eif7())
            .field("eif8", &self.eif8())
            .field("eif9", &self.eif9())
            .field("eif10", &self.eif10())
            .field("eif11", &self.eif11())
            .field("eif12", &self.eif12())
            .field("eif13", &self.eif13())
            .field("eif14", &self.eif14())
            .field("eif15", &self.eif15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc EIF0"]
    #[inline(always)]
    pub fn eif0(&mut self) -> Eif0W<'_, EifrSpec> {
        Eif0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc EIF1"]
    #[inline(always)]
    pub fn eif1(&mut self) -> Eif1W<'_, EifrSpec> {
        Eif1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc EIF2"]
    #[inline(always)]
    pub fn eif2(&mut self) -> Eif2W<'_, EifrSpec> {
        Eif2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc EIF3"]
    #[inline(always)]
    pub fn eif3(&mut self) -> Eif3W<'_, EifrSpec> {
        Eif3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc EIF4"]
    #[inline(always)]
    pub fn eif4(&mut self) -> Eif4W<'_, EifrSpec> {
        Eif4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc EIF5"]
    #[inline(always)]
    pub fn eif5(&mut self) -> Eif5W<'_, EifrSpec> {
        Eif5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc EIF6"]
    #[inline(always)]
    pub fn eif6(&mut self) -> Eif6W<'_, EifrSpec> {
        Eif6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc EIF7"]
    #[inline(always)]
    pub fn eif7(&mut self) -> Eif7W<'_, EifrSpec> {
        Eif7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc EIF8"]
    #[inline(always)]
    pub fn eif8(&mut self) -> Eif8W<'_, EifrSpec> {
        Eif8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc EIF9"]
    #[inline(always)]
    pub fn eif9(&mut self) -> Eif9W<'_, EifrSpec> {
        Eif9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc EIF10"]
    #[inline(always)]
    pub fn eif10(&mut self) -> Eif10W<'_, EifrSpec> {
        Eif10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc EIF11"]
    #[inline(always)]
    pub fn eif11(&mut self) -> Eif11W<'_, EifrSpec> {
        Eif11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc EIF12"]
    #[inline(always)]
    pub fn eif12(&mut self) -> Eif12W<'_, EifrSpec> {
        Eif12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc EIF13"]
    #[inline(always)]
    pub fn eif13(&mut self) -> Eif13W<'_, EifrSpec> {
        Eif13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc EIF14"]
    #[inline(always)]
    pub fn eif14(&mut self) -> Eif14W<'_, EifrSpec> {
        Eif14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc EIF15"]
    #[inline(always)]
    pub fn eif15(&mut self) -> Eif15W<'_, EifrSpec> {
        Eif15W::new(self, 15)
    }
}
#[doc = "desc EIFR\n\nYou can [`read`](crate::Reg::read) this register and get [`eifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EifrSpec;
impl crate::RegisterSpec for EifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eifr::R`](R) reader structure"]
impl crate::Readable for EifrSpec {}
#[doc = "`write(|w| ..)` method takes [`eifr::W`](W) writer structure"]
impl crate::Writable for EifrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EIFR to value 0"]
impl crate::Resettable for EifrSpec {}
