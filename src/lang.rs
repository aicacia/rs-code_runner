use std::convert::TryFrom;

use super::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {
    C,
    Cpp,
    EcmaScript,
    Elixir,
    Go,
    Java,
    Lua,
    Python,
    Ruby,
    Rust,
}

impl TryFrom<String> for Lang {
    type Error = Error;

    #[inline(always)]
    fn try_from(lang: String) -> Result<Self, Self::Error> {
        TryFrom::try_from(lang.as_str())
    }
}

impl<'a> TryFrom<&'a str> for Lang {
    type Error = Error;

    #[inline]
    fn try_from(lang: &'a str) -> Result<Self, Self::Error> {
        let lang = lang.trim();

        match lang.to_lowercase().as_str() {
            "c" | "gcc" | "clang" => Ok(Lang::C),
            "cpp" | "c++" | "g++" | "gxx" | "clang++" | "c_cpp" => Ok(Lang::Cpp),
            "javascript" | "node" | "ecmascript" => Ok(Lang::EcmaScript),
            "elixir" => Ok(Lang::Elixir),
            "go" | "golang" => Ok(Lang::Go),
            "java" => Ok(Lang::Java),
            "lua" => Ok(Lang::Lua),
            "python" => Ok(Lang::Python),
            "ruby" => Ok(Lang::Ruby),
            "rust" => Ok(Lang::Rust),
            _ => Err(Error::NotSupported(lang.to_owned())),
        }
    }
}
