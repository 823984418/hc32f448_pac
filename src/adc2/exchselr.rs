#[doc = "Register `EXCHSELR` reader"]
pub type R = crate::R<ExchselrSpec>;
#[doc = "Register `EXCHSELR` writer"]
pub type W = crate::W<ExchselrSpec>;
#[doc = "Field `EXCHSEL` reader - desc EXCHSEL"]
pub type ExchselR = crate::BitReader;
#[doc = "Field `EXCHSEL` writer - desc EXCHSEL"]
pub type ExchselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EXCHSEL"]
    #[inline(always)]
    pub fn exchsel(&self) -> ExchselR {
        ExchselR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXCHSELR")
            .field("exchsel", &self.exchsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc EXCHSEL"]
    #[inline(always)]
    pub fn exchsel(&mut self) -> ExchselW<'_, ExchselrSpec> {
        ExchselW::new(self, 0)
    }
}
#[doc = "desc EXCHSELR\n\nYou can [`read`](crate::Reg::read) this register and get [`exchselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exchselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExchselrSpec;
impl crate::RegisterSpec for ExchselrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`exchselr::R`](R) reader structure"]
impl crate::Readable for ExchselrSpec {}
#[doc = "`write(|w| ..)` method takes [`exchselr::W`](W) writer structure"]
impl crate::Writable for ExchselrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXCHSELR to value 0"]
impl crate::Resettable for ExchselrSpec {}
