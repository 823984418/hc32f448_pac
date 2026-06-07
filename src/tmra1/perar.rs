#[doc = "Register `PERAR` reader"]
pub type R = crate::R<PerarSpec>;
#[doc = "Register `PERAR` writer"]
pub type W = crate::W<PerarSpec>;
#[doc = "Field `PER` reader - desc PER"]
pub type PerR = crate::FieldReader<u32>;
#[doc = "Field `PER` writer - desc PER"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc PER"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERAR").field("per", &self.per()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - desc PER"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, PerarSpec> {
        PerW::new(self, 0)
    }
}
#[doc = "desc PERAR\n\nYou can [`read`](crate::Reg::read) this register and get [`perar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerarSpec;
impl crate::RegisterSpec for PerarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perar::R`](R) reader structure"]
impl crate::Readable for PerarSpec {}
#[doc = "`write(|w| ..)` method takes [`perar::W`](W) writer structure"]
impl crate::Writable for PerarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERAR to value 0xffff_ffff"]
impl crate::Resettable for PerarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
