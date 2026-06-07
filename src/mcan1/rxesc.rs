#[doc = "Register `RXESC` reader"]
pub type R = crate::R<RxescSpec>;
#[doc = "Register `RXESC` writer"]
pub type W = crate::W<RxescSpec>;
#[doc = "Field `F0DS` reader - desc F0DS"]
pub type F0dsR = crate::FieldReader;
#[doc = "Field `F0DS` writer - desc F0DS"]
pub type F0dsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `F1DS` reader - desc F1DS"]
pub type F1dsR = crate::FieldReader;
#[doc = "Field `F1DS` writer - desc F1DS"]
pub type F1dsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RBDS` reader - desc RBDS"]
pub type RbdsR = crate::FieldReader;
#[doc = "Field `RBDS` writer - desc RBDS"]
pub type RbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc F0DS"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0dsR {
        F0dsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc F1DS"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1dsR {
        F1dsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc RBDS"]
    #[inline(always)]
    pub fn rbds(&self) -> RbdsR {
        RbdsR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXESC")
            .field("f0ds", &self.f0ds())
            .field("f1ds", &self.f1ds())
            .field("rbds", &self.rbds())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - desc F0DS"]
    #[inline(always)]
    pub fn f0ds(&mut self) -> F0dsW<'_, RxescSpec> {
        F0dsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc F1DS"]
    #[inline(always)]
    pub fn f1ds(&mut self) -> F1dsW<'_, RxescSpec> {
        F1dsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - desc RBDS"]
    #[inline(always)]
    pub fn rbds(&mut self) -> RbdsW<'_, RxescSpec> {
        RbdsW::new(self, 8)
    }
}
#[doc = "desc RXESC\n\nYou can [`read`](crate::Reg::read) this register and get [`rxesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxescSpec;
impl crate::RegisterSpec for RxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxesc::R`](R) reader structure"]
impl crate::Readable for RxescSpec {}
#[doc = "`write(|w| ..)` method takes [`rxesc::W`](W) writer structure"]
impl crate::Writable for RxescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RxescSpec {}
