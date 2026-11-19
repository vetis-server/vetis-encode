use crate::EncodeHookConfig;
use cel::Context;
use vetis::{
    Request, Response, VetisFutureResult,
    hook::{Hook, HookConfig as _, HookPhase, HookPhases},
    matcher::Matcher,
};

/// Encode path
pub struct EncodeHook {
    _config: EncodeHookConfig,
    matcher: Option<Box<dyn Matcher>>,
}

unsafe impl Send for EncodeHook {}
unsafe impl Sync for EncodeHook {}

impl EncodeHook {
    /// Create a new encode hook with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The encode hook configuration
    ///
    /// # Returns
    ///
    /// * `EncodeHook` - The encode hook
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

    fn phases(&self) -> HookPhases {
        HookPhases::new(HookPhase::Rewrite)
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
