#[macro_export]
macro_rules! impl_values_collection_generator {
    ($ty:ident, $generator:ident, $default_len_range:expr $(,[$($params:path),*])?) => {
        pub struct $generator<V, L> {
            values: V,
            len: L,
        }

        impl<V: $crate::ValueGenerator, L> $generator<V, L> {
            pub fn values<Gen: $crate::ValueGenerator<Output = V::Output>>(
                self,
                values: Gen,
            ) -> $generator<Gen, L> {
                $generator {
                    values,
                    len: self.len,
                }
            }

            pub fn map_values<Gen: $crate::ValueGenerator<Output = V::Output>, F: Fn(V) -> Gen>(
                self,
                map: F,
            ) -> $generator<Gen, L> {
                $generator {
                    values: map(self.values),
                    len: self.len,
                }
            }

            pub fn len<Gen: $crate::ValueGenerator<Output = Len>, Len: Into<usize>>(
                self,
                len: Gen,
            ) -> $generator<V, Gen> {
                $generator {
                    values: self.values,
                    len,
                }
            }

            pub fn map_len<
                Gen: $crate::ValueGenerator<Output = Len>,
                F: Fn(L) -> Gen,
                Len: Into<usize>,
            >(
                self,
                map: F,
            ) -> $generator<V, Gen> {
                $generator {
                    values: self.values,
                    len: map(self.len),
                }
            }
        }

        impl<
                V: $crate::ValueGenerator,
                L: $crate::ValueGenerator<Output = Len>,
                Len: Into<usize>,
            > $crate::ValueGenerator for $generator<V, L>
        $( where V::Output: Sized $(+ $params)*, )?
        {
            type Output = $ty<V::Output>;

            fn generate<D: $crate::Driver>(&self, driver: &mut D) -> Option<Self::Output> {
                let len = $crate::ValueGenerator::generate(&self.len, driver)?.into();
                Some(
                    Iterator::filter_map(0..len, |_| {
                        $crate::ValueGenerator::generate(&self.values, driver)
                    })
                    .collect(),
                )
            }
        }

        impl<V: $crate::TypeGenerator $($( + $params)*)?,> $crate::TypeGenerator for $ty<V> {
            fn generate<D: $crate::Driver>(driver: &mut D) -> Option<Self> {
                let len = $crate::ValueGenerator::generate(&$default_len_range, driver)?;
                Some(Iterator::filter_map(0..len, |_| V::generate(driver)).collect())
            }
        }

        impl<V: $crate::TypeGenerator $($( + $params)*)?,> $crate::TypeGeneratorWithParams for $ty<V> {
            type Output =
                $generator<$crate::TypeValueGenerator<V>, core::ops::RangeInclusive<usize>>;

            fn gen_with() -> Self::Output {
                $generator {
                    values: Default::default(),
                    len: $default_len_range,
                }
            }
        }

        impl<V: $crate::ValueGenerator> $crate::ValueGenerator for $ty<V>
        $( where V::Output: Sized $(+ $params)* )?
        {
            type Output = $ty<V::Output>;

            fn generate<D: $crate::Driver>(&self, driver: &mut D) -> Option<Self::Output> {
                assert!(!self.is_empty());

                let len = $crate::ValueGenerator::generate(&$default_len_range, driver)?;
                let generators: $crate::alloc_generators::Vec<_> = self.iter().collect();
                let generators_len = 0..generators.len();

                Some(
                    Iterator::filter_map(0..len, |_| {
                        let index = $crate::ValueGenerator::generate(&generators_len, driver)?;
                        $crate::ValueGenerator::generate(generators[index], driver)
                    })
                    .collect(),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! impl_key_values_collection_generator {
    ($ty:ident, $generator:ident, $default_len_range:expr $(,[$($params:path),*])?) => {
        pub struct $generator<K, V, L> {
            keys: K,
            values: V,
            len: L,
        }

        impl<K: $crate::ValueGenerator, V: $crate::ValueGenerator, L> $generator<K, V, L> {
            pub fn keys<Gen: $crate::ValueGenerator<Output = K::Output>>(
                self,
                keys: Gen,
            ) -> $generator<Gen, V, L> {
                $generator {
                    keys,
                    values: self.values,
                    len: self.len,
                }
            }

            pub fn map_keys<Gen: $crate::ValueGenerator<Output = K::Output>, F: Fn(K) -> Gen>(
                self,
                map: F,
            ) -> $generator<Gen, V, L> {
                $generator {
                    keys: map(self.keys),
                    values: self.values,
                    len: self.len,
                }
            }

            pub fn values<Gen: $crate::ValueGenerator<Output = V::Output>>(
                self,
                values: Gen,
            ) -> $generator<K, Gen, L> {
                $generator {
                    keys: self.keys,
                    values,
                    len: self.len,
                }
            }

            pub fn map_values<Gen: $crate::ValueGenerator<Output = V::Output>, F: Fn(V) -> Gen>(
                self,
                map: F,
            ) -> $generator<K, Gen, L> {
                $generator {
                    keys: self.keys,
                    values: map(self.values),
                    len: self.len,
                }
            }

            pub fn len<Gen: $crate::ValueGenerator<Output = Len>, Len: Into<usize>>(
                self,
                len: Gen,
            ) -> $generator<K, V, Gen> {
                $generator {
                    keys: self.keys,
                    values: self.values,
                    len,
                }
            }

            pub fn map_len<
                Gen: $crate::ValueGenerator<Output = Len>,
                F: Fn(L) -> Gen,
                Len: Into<usize>,
            >(
                self,
                map: F,
            ) -> $generator<K, V, Gen> {
                $generator {
                    keys: self.keys,
                    values: self.values,
                    len: map(self.len),
                }
            }
        }

        impl<
                K: $crate::ValueGenerator,
                V: $crate::ValueGenerator,
                L: $crate::ValueGenerator<Output = Len>,
                Len: Into<usize>,
            > $crate::ValueGenerator for $generator<K, V, L>
        $( where K::Output: Sized $(+ $params)*, )?
        {
            type Output = $ty<K::Output, V::Output>;

            fn generate<D: $crate::Driver>(&self, driver: &mut D) -> Option<Self::Output> {
                use $crate::ValueGenerator;
                let len = ValueGenerator::generate(&self.len, driver)?.into();
                Some(
                    Iterator::filter_map(0..len, |_| {
                        Some((
                            ValueGenerator::generate(&self.keys, driver)?,
                            ValueGenerator::generate(&self.values, driver)?,
                        ))
                    })
                    .collect(),
                )
            }
        }

        impl<K: $crate::TypeGenerator $($( + $params)*)?, V: $crate::TypeGenerator> $crate::TypeGenerator
            for $ty<K, V>
        {
            fn generate<D: $crate::Driver>(driver: &mut D) -> Option<Self> {
                use $crate::ValueGenerator;
                let len = ValueGenerator::generate(&$default_len_range, driver)?;
                Some(Iterator::filter_map(0..len, |_| Some((K::generate(driver)?, V::generate(driver)?))).collect())
            }
        }

        impl<K: $crate::TypeGenerator $($( + $params)*)?, V: $crate::TypeGenerator> $crate::TypeGeneratorWithParams
            for $ty<K, V>
        {
            type Output = $generator<
                $crate::TypeValueGenerator<K>,
                $crate::TypeValueGenerator<V>,
                core::ops::RangeInclusive<usize>,
            >;

            fn gen_with() -> Self::Output {
                $generator {
                    keys: Default::default(),
                    values: Default::default(),
                    len: $default_len_range,
                }
            }
        }

        impl<V: $crate::ValueGenerator, K: $crate::ValueGenerator> $crate::ValueGenerator
            for $ty<K, V>
        $( where K::Output: Sized $(+ $params)* )?
        {
            type Output = $ty<K::Output, V::Output>;

            fn generate<D: $crate::Driver>(&self, driver: &mut D) -> Option<Self::Output> {
                use $crate::ValueGenerator;

                assert!(!self.is_empty());

                let len = ValueGenerator::generate(&$default_len_range, driver)?;
                let generators: $crate::alloc_generators::Vec<_> = self.iter().collect();
                let generators_len = 0..generators.len();

                Some(
                    Iterator::filter_map(0..len, |_| {
                        let index = ValueGenerator::generate(&generators_len, driver)?;
                        let (key, value) = generators[index];
                        Some((
                            ValueGenerator::generate(key, driver)?,
                            ValueGenerator::generate(value, driver)?,
                        ))
                    })
                    .collect(),
                )
            }
        }
    };
}
