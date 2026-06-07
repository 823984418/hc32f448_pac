#[doc = "Register `SCER` reader"]
pub type R = crate::R<ScerSpec>;
#[doc = "Register `SCER` writer"]
pub type W = crate::W<ScerSpec>;
#[doc = "Field `EVTRS` reader - desc EVTRS"]
pub type EvtrsR = crate::FieldReader;
#[doc = "Field `EVTRS` writer - desc EVTRS"]
pub type EvtrsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCTS` reader - desc PCTS"]
pub type PctsR = crate::BitReader;
#[doc = "Field `PCTS` writer - desc PCTS"]
pub type PctsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc EVTRS"]
    #[inline(always)]
    pub fn evtrs(&self) -> EvtrsR {
        EvtrsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - desc PCTS"]
    #[inline(always)]
    pub fn pcts(&self) -> PctsR {
        PctsR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCER")
            .field("evtrs", &self.evtrs())
            .field("pcts", &self.pcts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - desc EVTRS"]
    #[inline(always)]
    pub fn evtrs(&mut self) -> EvtrsW<'_, ScerSpec> {
        EvtrsW::new(self, 0)
    }
    #[doc = "Bit 8 - desc PCTS"]
    #[inline(always)]
    pub fn pcts(&mut self) -> PctsW<'_, ScerSpec> {
        PctsW::new(self, 8)
    }
}
#[doc = "desc SCER\n\nYou can [`read`](crate::Reg::read) this register and get [`scer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScerSpec;
impl crate::RegisterSpec for ScerSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`scer::R`](R) reader structure"]
impl crate::Readable for ScerSpec {}
#[doc = "`write(|w| ..)` method takes [`scer::W`](W) writer structure"]
impl crate::Writable for ScerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCER to value 0"]
impl crate::Resettable for ScerSpec {}
