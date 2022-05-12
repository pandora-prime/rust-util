// Small rust utility crates used across codebase by Pandora projects.
//
// Written in 2021-2022 by
//     Dr. Maxim Orlovsky <orlovsky@pandoraprime.ch>
//
// To the extent possible under law, the author(s) have dedicated all copyright and related and
// neighboring rights to this software to the public domain worldwide. This software is distributed
// without any warranty.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate clap;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

use colored::Colorize;
use serde::Serialize;

#[derive(Parser, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
pub enum Formatting {
    /// Print only data identifier strings (in Bech32m format), one per line
    #[display("id")]
    Id,

    /// Print a single entry per line formatted with a compact formatting
    /// option (type-specifc). This can be, for instance, `<txid>:<vout>`
    /// format for transaction outpoint, etc.
    #[display("compact")]
    Compact,

    /// Print tab-separated list of items
    #[display("tab")]
    Tab,

    /// Print comma-separated list of items
    #[display("csv")]
    Csv,

    /// Output data as formatted YAML
    #[display("yaml")]
    Yaml,

    /// Output data as JSON
    #[display("json")]
    Json,
}

impl FromStr for Formatting {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "id" => Formatting::Id,
            "compact" => Formatting::Compact,
            "tab" => Formatting::Tab,
            "csv" => Formatting::Csv,
            "yaml" => Formatting::Yaml,
            "json" => Formatting::Json,
            _ => Err("Unknown format name")?,
        })
    }
}

pub trait OutputCompact {
    fn output_compact(&self) -> String;
}

pub trait OutputFormat: OutputCompact + Serialize {
    fn output_print(&self, format: Formatting) {
        match format {
            Formatting::Id => println!("{}", self.output_id_string()),
            Formatting::Compact => println!("{}", self.output_compact()),
            Formatting::Tab => println!("{}", self.output_fields().join("\t")),
            Formatting::Csv => println!("{}", self.output_fields().join(",")),
            Formatting::Yaml => {
                println!("{}", serde_yaml::to_string(self).unwrap_or_default())
            }
            Formatting::Json => {
                println!("{}", serde_json::to_string(self).unwrap_or_default())
            }
        }
    }

    fn output_headers() -> Vec<String>;
    fn output_id_string(&self) -> String;
    fn output_fields(&self) -> Vec<String>;
}

#[doc(hidden)]
impl<T> OutputCompact for Vec<T>
where T: OutputCompact
{
    fn output_compact(&self) -> String { unreachable!() }
}

impl<T> OutputFormat for Vec<T>
where T: OutputFormat
{
    fn output_print(&self, format: Formatting) {
        if self.is_empty() {
            eprintln!("{}", "No items".red());
            return;
        }
        let headers = T::output_headers();
        if format == Formatting::Tab {
            println!("{}", headers.join("\t").bright_green())
        } else if format == Formatting::Csv {
            println!("{}", headers.join(","))
        }
        self.iter().for_each(|t| t.output_print(format));
    }

    #[doc(hidden)]
    fn output_id_string(&self) -> String { unreachable!() }

    #[doc(hidden)]
    fn output_headers() -> Vec<String> { unreachable!() }

    #[doc(hidden)]
    fn output_fields(&self) -> Vec<String> { unreachable!() }
}

#[doc(hidden)]
impl<T> OutputCompact for BTreeSet<T>
where T: OutputCompact
{
    fn output_compact(&self) -> String { unreachable!() }
}

impl<T> OutputFormat for BTreeSet<T>
where T: OutputFormat + Ord + Eq + Hash
{
    fn output_print(&self, format: Formatting) {
        if self.is_empty() {
            eprintln!("{}", "No items".red());
            return;
        }
        let headers = T::output_headers();
        if format == Formatting::Tab {
            println!("{}", headers.join("\t").bright_green())
        } else if format == Formatting::Csv {
            println!("{}", headers.join(","))
        }
        self.iter().for_each(|t| t.output_print(format));
    }

    #[doc(hidden)]
    fn output_id_string(&self) -> String { unreachable!() }

    #[doc(hidden)]
    fn output_headers() -> Vec<String> { unreachable!() }

    #[doc(hidden)]
    fn output_fields(&self) -> Vec<String> { unreachable!() }
}

#[doc(hidden)]
impl<T> OutputCompact for HashSet<T>
where T: OutputCompact
{
    fn output_compact(&self) -> String { unreachable!() }
}

impl<T> OutputFormat for HashSet<T>
where T: OutputFormat + Eq + Hash
{
    fn output_print(&self, format: Formatting) {
        if self.is_empty() {
            eprintln!("{}", "No items".red());
            return;
        }
        let headers = T::output_headers();
        if format == Formatting::Tab {
            println!("{}", headers.join("\t").bright_green())
        } else if format == Formatting::Csv {
            println!("{}", headers.join(","))
        }
        self.iter().for_each(|t| t.output_print(format));
    }

    #[doc(hidden)]
    fn output_id_string(&self) -> String { unreachable!() }

    #[doc(hidden)]
    fn output_headers() -> Vec<String> { unreachable!() }

    #[doc(hidden)]
    fn output_fields(&self) -> Vec<String> { unreachable!() }
}

impl<K, V> OutputCompact for HashMap<K, V>
where
    K: Display,
    V: OutputCompact,
{
    fn output_compact(&self) -> String { unimplemented!() }
}

impl<K, V> OutputFormat for HashMap<K, V>
where
    K: Clone + Display + std::hash::Hash + Eq + Serialize,
    V: OutputFormat + Serialize,
{
    fn output_print(&self, format: Formatting) {
        if self.is_empty() {
            eprintln!("{}", "No items".red());
            return;
        }
        let headers = Self::output_headers();
        if format == Formatting::Tab {
            println!("{}", headers.join("\t").bright_green())
        } else if format == Formatting::Csv {
            println!("{}", headers.join(","))
        }

        match format {
            Formatting::Yaml => {
                println!("{}", serde_yaml::to_string(self).unwrap_or_default())
            }

            Formatting::Json => {
                println!("{}", serde_json::to_string(self).unwrap_or_default())
            }

            _ => self.iter().for_each(|(id, rec)| match format {
                Formatting::Id => println!("{}", id),
                Formatting::Compact => {
                    println!("{}#{}", rec.output_compact(), id)
                }
                Formatting::Tab => {
                    println!("{}\t{}", id, rec.output_fields().join("\t"))
                }
                Formatting::Csv => {
                    println!("{},{}", id, rec.output_fields().join(","))
                }
                _ => unreachable!(),
            }),
        }
    }

    fn output_headers() -> Vec<String> {
        let mut vec = vec![s!("ID")];
        vec.extend(V::output_headers());
        vec
    }

    #[doc(hidden)]
    fn output_id_string(&self) -> String { unreachable!() }

    #[doc(hidden)]
    fn output_fields(&self) -> Vec<String> { unreachable!() }
}

impl<K, V> OutputCompact for BTreeMap<K, Vec<V>>
where
    K: Display,
    V: OutputCompact,
{
    fn output_compact(&self) -> String { unimplemented!() }
}

impl<K, V> OutputFormat for BTreeMap<K, Vec<V>>
where
    K: Clone + Display + Ord + Serialize,
    V: OutputFormat + Ord + Serialize,
{
    fn output_print(&self, format: Formatting) {
        if self.values().all(Vec::is_empty) {
            eprintln!("{}", "No items".red());
            return;
        }
        let headers = Self::output_headers();
        if format == Formatting::Tab {
            println!("{}", headers.join("\t").bright_green())
        } else if format == Formatting::Csv {
            println!("{}", headers.join(","))
        }

        match format {
            Formatting::Yaml => {
                println!("{}", serde_yaml::to_string(self).unwrap_or_default())
            }

            Formatting::Json => {
                println!("{}", serde_json::to_string(self).unwrap_or_default())
            }

            _ => self.iter().for_each(|(id, details)| {
                let id = id.to_string().as_str().bright_white();
                details.iter().for_each(|rec| match format {
                    Formatting::Id => println!("{}", id),
                    Formatting::Compact => {
                        println!("{}#{}", rec.output_compact(), id)
                    }
                    Formatting::Tab => {
                        println!("{}\t{}", id, rec.output_fields().join("\t"))
                    }
                    Formatting::Csv => {
                        println!("{},{}", id, rec.output_fields().join(","))
                    }
                    _ => unreachable!(),
                })
            }),
        }
    }

    fn output_headers() -> Vec<String> {
        let mut vec = vec![s!("ID")];
        vec.extend(V::output_headers());
        vec
    }

    #[doc(hidden)]
    fn output_id_string(&self) -> String { unreachable!() }

    #[doc(hidden)]
    fn output_fields(&self) -> Vec<String> { unreachable!() }
}
