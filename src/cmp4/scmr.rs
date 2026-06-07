#[doc = "Register `SCMR` reader"]
pub type R = crate::R<ScmrSpec>;
#[doc = "Register `SCMR` writer"]
pub type W = crate::W<ScmrSpec>;
#[doc = "Field `RVST` reader - desc RVST"]
pub type RvstR = crate::FieldReader;
#[doc = "Field `RVST` writer - desc RVST"]
pub type RvstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CVST` reader - desc CVST"]
pub type CvstR = crate::FieldReader;
#[doc = "Field `CVST` writer - desc CVST"]
pub type CvstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc RVST"]
    #[inline(always)]
    pub fn rvst(&self) -> RvstR {
        RvstR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc CVST"]
    #[inline(always)]
    pub fn cvst(&self) -> CvstR {
        CvstR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCMR")
            .field("rvst", &self.rvst())
            .field("cvst", &self.cvst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - desc RVST"]
    #[inline(always)]
    pub fn rvst(&mut self) -> RvstW<'_, ScmrSpec> {
        RvstW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc CVST"]
    #[inline(always)]
    pub fn cvst(&mut self) -> CvstW<'_, ScmrSpec> {
        CvstW::new(self, 16)
    }
}
#[doc = "desc SCMR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmrSpec;
impl crate::RegisterSpec for ScmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmr::R`](R) reader structure"]
impl crate::Readable for ScmrSpec {}
#[doc = "`write(|w| ..)` method takes [`scmr::W`](W) writer structure"]
impl crate::Writable for ScmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMR to value 0"]
impl crate::Resettable for ScmrSpec {}
