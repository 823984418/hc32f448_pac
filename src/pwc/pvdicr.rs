#[doc = "Register `PVDICR` reader"]
pub type R = crate::R<PvdicrSpec>;
#[doc = "Register `PVDICR` writer"]
pub type W = crate::W<PvdicrSpec>;
#[doc = "Field `PVD1EDGS` reader - desc PVD1EDGS"]
pub type Pvd1edgsR = crate::FieldReader;
#[doc = "Field `PVD1EDGS` writer - desc PVD1EDGS"]
pub type Pvd1edgsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PVD2EDGS` reader - desc PVD2EDGS"]
pub type Pvd2edgsR = crate::FieldReader;
#[doc = "Field `PVD2EDGS` writer - desc PVD2EDGS"]
pub type Pvd2edgsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 1:2 - desc PVD1EDGS"]
    #[inline(always)]
    pub fn pvd1edgs(&self) -> Pvd1edgsR {
        Pvd1edgsR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 5:6 - desc PVD2EDGS"]
    #[inline(always)]
    pub fn pvd2edgs(&self) -> Pvd2edgsR {
        Pvd2edgsR::new((self.bits >> 5) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVDICR")
            .field("pvd1edgs", &self.pvd1edgs())
            .field("pvd2edgs", &self.pvd2edgs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:2 - desc PVD1EDGS"]
    #[inline(always)]
    pub fn pvd1edgs(&mut self) -> Pvd1edgsW<'_, PvdicrSpec> {
        Pvd1edgsW::new(self, 1)
    }
    #[doc = "Bits 5:6 - desc PVD2EDGS"]
    #[inline(always)]
    pub fn pvd2edgs(&mut self) -> Pvd2edgsW<'_, PvdicrSpec> {
        Pvd2edgsW::new(self, 5)
    }
}
#[doc = "desc PVDICR\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvdicrSpec;
impl crate::RegisterSpec for PvdicrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pvdicr::R`](R) reader structure"]
impl crate::Readable for PvdicrSpec {}
#[doc = "`write(|w| ..)` method takes [`pvdicr::W`](W) writer structure"]
impl crate::Writable for PvdicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVDICR to value 0"]
impl crate::Resettable for PvdicrSpec {}
