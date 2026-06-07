#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `CR` reader - desc CR"]
pub type CrR = crate::BitReader;
#[doc = "Field `CR` writer - desc CR"]
pub type CrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG` reader - desc FLAG"]
pub type FlagR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CR"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FLAG"]
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("cr", &self.cr())
            .field("flag", &self.flag())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<'_, CrSpec> {
        CrW::new(self, 0)
    }
}
#[doc = "desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x01"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x01;
}
