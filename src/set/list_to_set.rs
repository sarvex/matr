// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::marker::PhantomData;
use internal::*;

pub struct ListToSet<K: EqualityComparableKind, L: Expr<List<K>>> {
    k: PhantomData<K>,
    l: PhantomData<L>,
}

impl<K: EqualityComparableKind, L: Expr<List<K>>> Expr<Set<K>> for ListToSet<K, L> {
    type Eval = ListToSetValue<K, L>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ListToSetValue<K: EqualityComparableKind, L: Expr<List<K>>> {
        k: PhantomData<K>,
        l: PhantomData<L>,
    }

    impl<K: EqualityComparableKind, L: Expr<List<K>>> SetValue<K> for ListToSetValue<K, L> {
        type Impl = AsSet<K, <AsList<K, L> as ListTrait<K>>::Visit<Set<K>, ListToSetVisitor<K>>>;
    }

    pub struct ListToSetVisitor<K: EqualityComparableKind> {
        k: PhantomData<K>,
    }

    impl<K: EqualityComparableKind> ListVisitor<K, Set<K>> for ListToSetVisitor<K> {
        type VisitEmptyList = EmptySet<K>;
        type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = AddToSet<K, Elem, ListToSet<K, Tail>>;
    }
}
