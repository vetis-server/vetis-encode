use crate::HookConfig;
use cel::Context;
use vetis::{
    Request, Response, VetisFutureResult,
    hook::{Hook, HookConfig as _, HookPhase},
    matcher::Matcher,
};

/// Encode path
pub struct EncodeHook {
    matcher: Option<Box<dyn Matcher>>,
    config: HookConfig,
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
    pub fn new(config: HookConfig) -> EncodeHook {
        EncodeHook { matcher: None, config: config }
    }
}

impl Hook for EncodeHook {
    fn setup<'a>(&'a mut self) -> VetisFutureResult<'a, ()> {
        Box::pin(async move {
            let matcher = if let Some(matcher_config) = self
                .config
                .matcher()
            {
                Some(matcher_config.boxed_matcher())
            } else {
                None
            };
            self.matcher = matcher;
            Ok(())
        })
    }

    fn has_matches(&self, context: &Context) -> bool {
        if let Some(matcher) = &self.matcher { matcher.is_match(context) } else { true }
    }

    fn phase(&self) -> HookPhase {
        HookPhase::PostContent
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
