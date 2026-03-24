// SPDX-FileCopyrightText: Copyright (c) 2025-2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    ProviderDiscoverySpec, ProviderError, ProviderPlugin, RealDiscoveryContext, discover_with_spec,
};

pub struct OpenrouterProvider;

pub const SPEC: ProviderDiscoverySpec = ProviderDiscoverySpec {
    id: "openrouter",
    credential_env_vars: &["OPENROUTER_API_KEY"],
};

impl ProviderPlugin for OpenrouterProvider {
    fn id(&self) -> &'static str {
        SPEC.id
    }

    fn discover_existing(&self) -> Result<Option<crate::DiscoveredProvider>, ProviderError> {
        discover_with_spec(&SPEC, &RealDiscoveryContext)
    }

    fn credential_env_vars(&self) -> &'static [&'static str] {
        SPEC.credential_env_vars
    }
}

#[cfg(test)]
mod tests {
    use super::SPEC;
    use crate::discover_with_spec;
    use crate::test_helpers::MockDiscoveryContext;

    #[test]
    fn discovers_openrouter_env_credentials() {
        let ctx = MockDiscoveryContext::new().with_env("OPENROUTER_API_KEY", "sk-or-test-key");
        let discovered = discover_with_spec(&SPEC, &ctx)
            .expect("discovery")
            .expect("provider");
        assert_eq!(
            discovered.credentials.get("OPENROUTER_API_KEY"),
            Some(&"sk-or-test-key".to_string())
        );
    }
}
