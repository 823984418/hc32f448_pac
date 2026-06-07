#[doc = "Register `PMSR` reader"]
pub type R = crate::R<PmsrSpec>;
#[doc = "Register `PMSR` writer"]
pub type W = crate::W<PmsrSpec>;
#[doc = "Field `RVSL` reader - desc RVSL"]
pub type RvslR = crate::FieldReader;
#[doc = "Field `RVSL` writer - desc RVSL"]
pub type RvslW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CVSL` reader - desc CVSL"]
pub type CvslR = crate::FieldReader;
#[doc = "Field `CVSL` writer - desc CVSL"]
pub type CvslW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc RVSL"]
    #[inline(always)]
    pub fn rvsl(&self) -> RvslR {
        RvslR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc CVSL"]
    #[inline(always)]
    pub fn cvsl(&self) -> CvslR {
        CvslR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMSR")
            .field("rvsl", &self.rvsl())
            .field("cvsl", &self.cvsl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - desc RVSL"]
    #[inline(always)]
    pub fn rvsl(&mut self) -> RvslW<'_, PmsrSpec> {
        RvslW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc CVSL"]
    #[inline(always)]
    pub fn cvsl(&mut self) -> CvslW<'_, PmsrSpec> {
        CvslW::new(self, 16)
    }
}
#[doc = "desc PMSR\n\nYou can [`read`](crate::Reg::read) this register and get [`pmsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmsrSpec;
impl crate::RegisterSpec for PmsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmsr::R`](R) reader structure"]
impl crate::Readable for PmsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmsr::W`](W) writer structure"]
impl crate::Writable for PmsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMSR to value 0"]
impl crate::Resettable for PmsrSpec {}
