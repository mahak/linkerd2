use crate::index::{GATEWAY_API_GROUP, POLICY_API_GROUP};
use linkerd_policy_controller_core::routes::GroupKindName;
use linkerd_policy_controller_k8s_api::{gateway, policy as linkerd_k8s_api, Resource};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ResourceId {
    pub namespace: String,
    pub name: String,
}

impl ResourceId {
    pub fn new(namespace: String, name: String) -> Self {
        Self { namespace, name }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NamespaceGroupKindName {
    pub namespace: String,
    pub gkn: GroupKindName,
}

impl NamespaceGroupKindName {
    pub fn api_version(&self) -> anyhow::Result<Cow<'static, str>> {
        match (self.gkn.group.as_ref(), self.gkn.kind.as_ref()) {
            (POLICY_API_GROUP, "HTTPRoute") => Ok(linkerd_k8s_api::HttpRoute::api_version(&())),
            (POLICY_API_GROUP, "HTTPLocalRateLimitPolicy") => {
                Ok(linkerd_k8s_api::HttpLocalRateLimitPolicy::api_version(&()))
            }
            (POLICY_API_GROUP, "EgressNetwork") => {
                Ok(linkerd_k8s_api::EgressNetwork::api_version(&()))
            }
            (GATEWAY_API_GROUP, "HTTPRoute") => Ok(gateway::HTTPRoute::api_version(&())),
            (GATEWAY_API_GROUP, "GRPCRoute") => Ok(gateway::GRPCRoute::api_version(&())),
            (GATEWAY_API_GROUP, "TCPRoute") => Ok(gateway::TCPRoute::api_version(&())),
            (GATEWAY_API_GROUP, "TLSRoute") => Ok(gateway::TLSRoute::api_version(&())),
            (group, kind) => {
                anyhow::bail!("unknown group + kind combination: ({}, {})", group, kind)
            }
        }
    }

    pub fn is_a<K>(&self) -> bool
    where
        K: Resource<DynamicType = ()>,
    {
        self.gkn.group == K::group(&()) && self.gkn.kind == K::kind(&())
    }
}
