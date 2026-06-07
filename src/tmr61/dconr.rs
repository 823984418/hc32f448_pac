#[doc = "Register `DCONR` reader"]
pub type R = crate::R<DconrSpec>;
#[doc = "Register `DCONR` writer"]
pub type W = crate::W<DconrSpec>;
#[doc = "Field `DTCEN` reader - desc DTCEN"]
pub type DtcenR = crate::BitReader;
#[doc = "Field `DTCEN` writer - desc DTCEN"]
pub type DtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEPA` reader - desc SEPA"]
pub type SepaR = crate::BitReader;
#[doc = "Field `SEPA` writer - desc SEPA"]
pub type SepaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBENU` reader - desc DTBENU"]
pub type DtbenuR = crate::BitReader;
#[doc = "Field `DTBENU` writer - desc DTBENU"]
pub type DtbenuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBEND` reader - desc DTBEND"]
pub type DtbendR = crate::BitReader;
#[doc = "Field `DTBEND` writer - desc DTBEND"]
pub type DtbendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBTRU` reader - desc DTBTRU"]
pub type DtbtruR = crate::BitReader;
#[doc = "Field `DTBTRU` writer - desc DTBTRU"]
pub type DtbtruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBTRD` reader - desc DTBTRD"]
pub type DtbtrdR = crate::BitReader;
#[doc = "Field `DTBTRD` writer - desc DTBTRD"]
pub type DtbtrdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc DTCEN"]
    #[inline(always)]
    pub fn dtcen(&self) -> DtcenR {
        DtcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SEPA"]
    #[inline(always)]
    pub fn sepa(&self) -> SepaR {
        SepaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc DTBENU"]
    #[inline(always)]
    pub fn dtbenu(&self) -> DtbenuR {
        DtbenuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DTBEND"]
    #[inline(always)]
    pub fn dtbend(&self) -> DtbendR {
        DtbendR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DTBTRU"]
    #[inline(always)]
    pub fn dtbtru(&self) -> DtbtruR {
        DtbtruR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DTBTRD"]
    #[inline(always)]
    pub fn dtbtrd(&self) -> DtbtrdR {
        DtbtrdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCONR")
            .field("dtcen", &self.dtcen())
            .field("sepa", &self.sepa())
            .field("dtbenu", &self.dtbenu())
            .field("dtbend", &self.dtbend())
            .field("dtbtru", &self.dtbtru())
            .field("dtbtrd", &self.dtbtrd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc DTCEN"]
    #[inline(always)]
    pub fn dtcen(&mut self) -> DtcenW<'_, DconrSpec> {
        DtcenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SEPA"]
    #[inline(always)]
    pub fn sepa(&mut self) -> SepaW<'_, DconrSpec> {
        SepaW::new(self, 1)
    }
    #[doc = "Bit 4 - desc DTBENU"]
    #[inline(always)]
    pub fn dtbenu(&mut self) -> DtbenuW<'_, DconrSpec> {
        DtbenuW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DTBEND"]
    #[inline(always)]
    pub fn dtbend(&mut self) -> DtbendW<'_, DconrSpec> {
        DtbendW::new(self, 5)
    }
    #[doc = "Bit 6 - desc DTBTRU"]
    #[inline(always)]
    pub fn dtbtru(&mut self) -> DtbtruW<'_, DconrSpec> {
        DtbtruW::new(self, 6)
    }
    #[doc = "Bit 7 - desc DTBTRD"]
    #[inline(always)]
    pub fn dtbtrd(&mut self) -> DtbtrdW<'_, DconrSpec> {
        DtbtrdW::new(self, 7)
    }
}
#[doc = "desc DCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`dconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DconrSpec;
impl crate::RegisterSpec for DconrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dconr::R`](R) reader structure"]
impl crate::Readable for DconrSpec {}
#[doc = "`write(|w| ..)` method takes [`dconr::W`](W) writer structure"]
impl crate::Writable for DconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCONR to value 0"]
impl crate::Resettable for DconrSpec {}
