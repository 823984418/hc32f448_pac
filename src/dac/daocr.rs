#[doc = "Register `DAOCR` reader"]
pub type R = crate::R<DaocrSpec>;
#[doc = "Register `DAOCR` writer"]
pub type W = crate::W<DaocrSpec>;
#[doc = "Field `DAODIS1` reader - desc DAODIS1"]
pub type Daodis1R = crate::BitReader;
#[doc = "Field `DAODIS1` writer - desc DAODIS1"]
pub type Daodis1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAODIS2` reader - desc DAODIS2"]
pub type Daodis2R = crate::BitReader;
#[doc = "Field `DAODIS2` writer - desc DAODIS2"]
pub type Daodis2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - desc DAODIS1"]
    #[inline(always)]
    pub fn daodis1(&self) -> Daodis1R {
        Daodis1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DAODIS2"]
    #[inline(always)]
    pub fn daodis2(&self) -> Daodis2R {
        Daodis2R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAOCR")
            .field("daodis1", &self.daodis1())
            .field("daodis2", &self.daodis2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 14 - desc DAODIS1"]
    #[inline(always)]
    pub fn daodis1(&mut self) -> Daodis1W<'_, DaocrSpec> {
        Daodis1W::new(self, 14)
    }
    #[doc = "Bit 15 - desc DAODIS2"]
    #[inline(always)]
    pub fn daodis2(&mut self) -> Daodis2W<'_, DaocrSpec> {
        Daodis2W::new(self, 15)
    }
}
#[doc = "desc DAOCR\n\nYou can [`read`](crate::Reg::read) this register and get [`daocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaocrSpec;
impl crate::RegisterSpec for DaocrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`daocr::R`](R) reader structure"]
impl crate::Readable for DaocrSpec {}
#[doc = "`write(|w| ..)` method takes [`daocr::W`](W) writer structure"]
impl crate::Writable for DaocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAOCR to value 0"]
impl crate::Resettable for DaocrSpec {}
