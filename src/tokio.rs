use crate::EncodeHookConfig;
use cel::Context;
use vetis::{
    Request, Response, VetisFutureResult,
    hook::{Hook, HookConfig as _, HookPhase},
    matcher::Matcher,
};

/// Proxy path
pub struct EncodeHook {
    _config: EncodeHookConfig,
    matcher: Option<Box<dyn Matcher>>,
}

unsafe impl Send for EncodeHook {}
unsafe impl Sync for EncodeHook {}

impl EncodeHook {
    /// Create a new proxy path with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The proxy path configuration
    ///
    /// # Returns
    ///
    /// * `ProxyHook` - The proxy path
    pub fn new(config: EncodeHookConfig) -> EncodeHook {
        let matcher = if let Some(matcher_config) = config.matcher() {
            Some(matcher_config.boxed_matcher())
        } else {
            None
        };

        EncodeHook { matcher, _config: config }
    }
}

impl Hook for EncodeHook {
    fn has_matches(&self, context: &Context) -> bool {
        if let Some(matcher) = &self.matcher { matcher.is_match(context) } else { true }
    }

    fn phase(&self) -> &HookPhase {
        &HookPhase::Rewrite
    }

    fn matcher_mut(&mut self) -> Option<&mut Box<dyn Matcher>> {
        self.matcher
            .as_mut()
    }

    fn handle<'a>(
        &'a self,
        request: Request,
        _response: Option<Response>,
    ) -> VetisFutureResult<'a, (Request, Option<Response>)> {
        let future = async move { Ok((request, Some(Response::builder().empty()))) };
        Box::pin(future)
    }
}
