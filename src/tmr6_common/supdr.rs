#[doc = "Register `SUPDR` reader"]
pub type R = crate::R<SupdrSpec>;
#[doc = "Register `SUPDR` writer"]
pub type W = crate::W<SupdrSpec>;
#[doc = "Field `SUPD1` reader - desc SUPD1"]
pub type Supd1R = crate::BitReader;
#[doc = "Field `SUPD1` writer - desc SUPD1"]
pub type Supd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPD2` reader - desc SUPD2"]
pub type Supd2R = crate::BitReader;
#[doc = "Field `SUPD2` writer - desc SUPD2"]
pub type Supd2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SUPD1"]
    #[inline(always)]
    pub fn supd1(&self) -> Supd1R {
        Supd1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SUPD2"]
    #[inline(always)]
    pub fn supd2(&self) -> Supd2R {
        Supd2R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUPDR")
            .field("supd1", &self.supd1())
            .field("supd2", &self.supd2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc SUPD1"]
    #[inline(always)]
    pub fn supd1(&mut self) -> Supd1W<'_, SupdrSpec> {
        Supd1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc SUPD2"]
    #[inline(always)]
    pub fn supd2(&mut self) -> Supd2W<'_, SupdrSpec> {
        Supd2W::new(self, 1)
    }
}
#[doc = "desc SUPDR\n\nYou can [`read`](crate::Reg::read) this register and get [`supdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`supdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SupdrSpec;
impl crate::RegisterSpec for SupdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`supdr::R`](R) reader structure"]
impl crate::Readable for SupdrSpec {}
#[doc = "`write(|w| ..)` method takes [`supdr::W`](W) writer structure"]
impl crate::Writable for SupdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUPDR to value 0"]
impl crate::Resettable for SupdrSpec {}
