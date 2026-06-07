#[doc = "Register `PRAMLPC` reader"]
pub type R = crate::R<PramlpcSpec>;
#[doc = "Register `PRAMLPC` writer"]
pub type W = crate::W<PramlpcSpec>;
#[doc = "Field `PRAMPDC0` reader - desc PRAMPDC0"]
pub type Prampdc0R = crate::BitReader;
#[doc = "Field `PRAMPDC0` writer - desc PRAMPDC0"]
pub type Prampdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRAMPDC2` reader - desc PRAMPDC2"]
pub type Prampdc2R = crate::BitReader;
#[doc = "Field `PRAMPDC2` writer - desc PRAMPDC2"]
pub type Prampdc2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PRAMPDC0"]
    #[inline(always)]
    pub fn prampdc0(&self) -> Prampdc0R {
        Prampdc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc PRAMPDC2"]
    #[inline(always)]
    pub fn prampdc2(&self) -> Prampdc2R {
        Prampdc2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRAMLPC")
            .field("prampdc0", &self.prampdc0())
            .field("prampdc2", &self.prampdc2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc PRAMPDC0"]
    #[inline(always)]
    pub fn prampdc0(&mut self) -> Prampdc0W<'_, PramlpcSpec> {
        Prampdc0W::new(self, 0)
    }
    #[doc = "Bit 2 - desc PRAMPDC2"]
    #[inline(always)]
    pub fn prampdc2(&mut self) -> Prampdc2W<'_, PramlpcSpec> {
        Prampdc2W::new(self, 2)
    }
}
#[doc = "desc PRAMLPC\n\nYou can [`read`](crate::Reg::read) this register and get [`pramlpc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pramlpc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PramlpcSpec;
impl crate::RegisterSpec for PramlpcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pramlpc::R`](R) reader structure"]
impl crate::Readable for PramlpcSpec {}
#[doc = "`write(|w| ..)` method takes [`pramlpc::W`](W) writer structure"]
impl crate::Writable for PramlpcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRAMLPC to value 0"]
impl crate::Resettable for PramlpcSpec {}
