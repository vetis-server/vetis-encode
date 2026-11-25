#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#[cfg(feature = "runtime-tokio")]
use crate::tokio::EncodeHook;
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use vetis::{
    VetisHook, VetisResult,
    errors::{ConfigError, VetisError},
    matcher::MatcherConfig,
};

#[cfg(feature = "runtime-tokio")]
/// Tokio runtime support module
pub mod tokio;

#[cfg(test)]
mod tests;

/// Builder for creating `HookConfigBuilder` instances.
pub struct HookConfigBuilder {
    matcher: Option<Box<dyn MatcherConfig>>,
    target: String,
}

impl HookConfigBuilder {
    /// Allow set a matcher
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn matcher<M>(mut self, matcher: M) -> Self
    where
        M: MatcherConfig + 'static,
    {
        self.matcher = Some(Box::new(matcher));
        self
    }

    /// Allow set the target of the proxy path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn target(mut self, target: &str) -> Self {
        self.target = target.to_string();
        self
    }

    /// Build the `ProxyHookConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<ProxyHookConfig>` - The `ProxyHookConfig` with the configured settings.
    pub fn build(self) -> VetisResult<HookConfig> {
        if self
            .target
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::Hook(
                "Target cannot be empty".to_compact_string(),
            )));
        }

        Ok(HookConfig {
            matcher: self
                .matcher
                .map_or(None, |val| Some(val)),
            target: self.target,
        })
    }
}

/// Configuration for a proxy path.
#[derive(Clone, Serialize, Deserialize)]
pub struct HookConfig {
    matcher: Option<Box<dyn MatcherConfig>>,
    target: String,
}

#[typetag::serde(name = "encode")]
impl vetis::hook::HookConfig for HookConfig {
    fn matcher(&self) -> Option<&Box<dyn MatcherConfig>> {
        self.matcher
            .as_ref()
    }

    fn boxed_clone(&self) -> Box<dyn vetis::hook::HookConfig> {
        Box::new(self.clone())
    }

    fn boxed_hook<'a>(&'a self) -> Box<dyn VetisHook> {
        Box::new(EncodeHook::new(self.clone()))
    }
}

impl HookConfig {
    /// Creates a new `HookConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `HookConfigBuilder` - The builder.
    pub fn builder() -> HookConfigBuilder {
        HookConfigBuilder { matcher: None, target: "http://localhost:8080".to_string() }
    }

    /// Returns the target of the proxy path.
    ///
    /// # Returns
    ///
    /// * `&str` - The target of the proxy path.
    pub fn target(&self) -> &str {
        &self.target
    }
}
