// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

/// Just like Clojure's Keyword. We'll need this as part of EDN parsing,
/// but it's also used for identification within Datomish, so we'll define
/// it here first.
/// Callers are expected to follow these rules:
/// http://www.clojure.org/reference/reader#_symbols
#[derive(Clone,Debug,PartialEq)]
pub struct Keyword {
    pub name: String,
    pub namespace: Option<String>,
}

impl Keyword {
    pub fn new(name: &str) -> Self {
        return Keyword { name: name.to_string(), namespace: None };
    }

    pub fn namespaced(name: &str, namespace: &str) -> Self {
        return Keyword { name: name.to_string(), namespace: Some(namespace.to_string()) };
    }
}

impl ToString for Keyword {
    /// Print the keyword in EDN format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate datomish;
    /// # use datomish::keyword::Keyword;
    /// # fn main() {
    ///   assert_eq!(":baz", Keyword::new("baz").to_string());
    ///   assert_eq!(":bar/baz", Keyword::namespaced("baz", "bar").to_string());
    /// # }
    /// ```
    fn to_string(&self) -> String {
        // Note that we don't currently do any escaping.
        if let Some(ref ns) = self.namespace {
            return format!(":{}/{}", ns, self.name);
        }
        return format!(":{}", self.name);
    }
}

