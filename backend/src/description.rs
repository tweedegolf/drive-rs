use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CrateName(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CrateInfo {
    name: CrateName,
    version: Version,
    authors: Vec<String>,
    #[serde(with = "spdx_expression_serde")]
    license: spdx::Expression,
    repository: String,
    homepage: String,
    dependencies: Vec<(CrateName, VersionReq)>,
}

mod spdx_expression_serde {
    use spdx::Expression;

    pub fn serialize<S>(expr: &Expression, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(expr.as_ref())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Expression, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ExpressionVisitor)
    }

    struct ExpressionVisitor;
    impl<'de> serde::de::Visitor<'de> for ExpressionVisitor {
        type Value = Expression;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid SPDX expression")
        }

        fn visit_str<E>(self, value: &str) -> Result<Expression, E>
        where
            E: serde::de::Error,
        {
            Expression::parse(value).map_err(serde::de::Error::custom)
        }
    }
}
