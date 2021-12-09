pub struct GetPullRequestsIdsForCommitIds;
pub mod get_pull_requests_ids_for_commit_ids {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetPullRequestsIdsForCommitIds";
    pub const QUERY : & str = "#query PullRequests($owner: String!, $repository: String!, $cursor: String) {\n#  ...GetRateLimit\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, after: $cursor, orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          body\n#          number\n#          baseRefName\n#          headRefName\n#          title,\n#          mergedAt,\n#          updatedAt,\n#          author {\n#            login\n#          }\n#          closingIssuesReferences(first: 100) {\n#            totalCount\n#            edges {\n#              node {\n#                number\n#                title\n#              }\n#            }\n#          }\n#          mergeCommit {\n#            oid\n#          }\n#        }\n#      }\n#    }\n#  }\n#}\n\n#query PullRequestIDs($owner: String!, $repository: String!, $cursor: String) {\n#  ...GetRateLimit\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, after: $cursor, orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          id\n#        }\n#      }\n#    }\n#  }\n#}\n\n\nquery GetPullRequestsIdsForCommitIds($commit_ids: [ID!]!) {\n  ...RateLimit\n  nodes(ids: $commit_ids) {\n    __typename\n    ... on Commit {\n      id\n      oid\n      associatedPullRequests(first: 100) {\n        nodes {\n          id\n        }\n      }\n    }\n  }\n}\n\n\nquery GetPullRequestsById($pr_ids: [ID!]!) {\n  ...RateLimit\n  nodes(ids: $pr_ids) {\n    __typename\n    ...on PullRequest {\n      id\n      number\n      state\n      body\n      baseRefName\n      headRefName\n      title,\n      mergedAt,\n      updatedAt,\n      author {\n        __typename\n        login\n      }\n      closingIssuesReferences(first: 100) {\n        totalCount\n        edges {\n          node {\n            number\n            title\n          }\n        }\n      }\n      mergeCommit {\n        oid\n      }\n    }\n  }\n}\n\nquery GetCommitOnBranchHead($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n        }\n      }\n    }\n  }\n}\n\nquery GetCommitHistory($owner: String!, $repository: String!, $oid: GitObjectID!, $cursor: String) {\n  ...RateLimit\n  repository(owner: $owner, name: $repository) {\n    object(oid: $oid) {\n      __typename\n      ... on Commit {\n        oid\n        history(first: 100, after: $cursor) {\n          totalCount\n          edges {\n            cursor\n            node {\n              id\n              oid\n              committedDate\n              message\n              parents(first: 10) {\n                edges {\n                  node {\n                    oid\n                  }\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n\nfragment RateLimit on Query {\n  rateLimit {\n    limit\n    cost\n    remaining\n    resetAt\n  }\n}\n\n#fragment GetPullRequests on Commit {\n#  associatedPullRequests(first: 10) {\n#    nodes {\n#      id\n#      number\n#      mergeCommit {\n#        oid\n#      }\n#    }\n##    edges {\n##      node {\n##        id\n##      }\n##    }\n#  }\n#}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type GitObjectID = crate::github_collect::GitObjectID;
    type DateTime = crate::github_collect::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub commit_ids: Vec<ID>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct RateLimit {
        #[serde(rename = "rateLimit")]
        pub rate_limit: Option<RateLimitRateLimit>,
    }
    #[derive(Deserialize, Debug)]
    pub struct RateLimitRateLimit {
        pub limit: Int,
        pub cost: Int,
        pub remaining: Int,
        #[serde(rename = "resetAt")]
        pub reset_at: DateTime,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(flatten)]
        pub rate_limit: RateLimit,
        pub nodes: Vec<Option<GetPullRequestsIdsForCommitIdsNodes>>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum GetPullRequestsIdsForCommitIdsNodes {
        AddedToProjectEvent,
        App,
        AssignedEvent,
        AutoMergeDisabledEvent,
        AutoMergeEnabledEvent,
        AutoRebaseEnabledEvent,
        AutoSquashEnabledEvent,
        AutomaticBaseChangeFailedEvent,
        AutomaticBaseChangeSucceededEvent,
        BaseRefChangedEvent,
        BaseRefDeletedEvent,
        BaseRefForcePushedEvent,
        Blob,
        Bot,
        BranchProtectionRule,
        CWE,
        CheckRun,
        CheckSuite,
        ClosedEvent,
        CodeOfConduct,
        CommentDeletedEvent,
        Commit(GetPullRequestsIdsForCommitIdsNodesOnCommit),
        CommitComment,
        CommitCommentThread,
        ConnectedEvent,
        ConvertToDraftEvent,
        ConvertedNoteToIssueEvent,
        CrossReferencedEvent,
        DemilestonedEvent,
        DependencyGraphManifest,
        DeployKey,
        DeployedEvent,
        Deployment,
        DeploymentEnvironmentChangedEvent,
        DeploymentReview,
        DeploymentStatus,
        DisconnectedEvent,
        Discussion,
        DiscussionCategory,
        DiscussionComment,
        Enterprise,
        EnterpriseAdministratorInvitation,
        EnterpriseIdentityProvider,
        EnterpriseRepositoryInfo,
        EnterpriseServerInstallation,
        EnterpriseServerUserAccount,
        EnterpriseServerUserAccountEmail,
        EnterpriseServerUserAccountsUpload,
        EnterpriseUserAccount,
        Environment,
        ExternalIdentity,
        Gist,
        GistComment,
        HeadRefDeletedEvent,
        HeadRefForcePushedEvent,
        HeadRefRestoredEvent,
        IpAllowListEntry,
        Issue,
        IssueComment,
        Label,
        LabeledEvent,
        Language,
        License,
        LockedEvent,
        Mannequin,
        MarkedAsDuplicateEvent,
        MarketplaceCategory,
        MarketplaceListing,
        MembersCanDeleteReposClearAuditEntry,
        MembersCanDeleteReposDisableAuditEntry,
        MembersCanDeleteReposEnableAuditEntry,
        MentionedEvent,
        MergedEvent,
        Milestone,
        MilestonedEvent,
        MovedColumnsInProjectEvent,
        OauthApplicationCreateAuditEntry,
        OrgAddBillingManagerAuditEntry,
        OrgAddMemberAuditEntry,
        OrgBlockUserAuditEntry,
        OrgConfigDisableCollaboratorsOnlyAuditEntry,
        OrgConfigEnableCollaboratorsOnlyAuditEntry,
        OrgCreateAuditEntry,
        OrgDisableOauthAppRestrictionsAuditEntry,
        OrgDisableSamlAuditEntry,
        OrgDisableTwoFactorRequirementAuditEntry,
        OrgEnableOauthAppRestrictionsAuditEntry,
        OrgEnableSamlAuditEntry,
        OrgEnableTwoFactorRequirementAuditEntry,
        OrgInviteMemberAuditEntry,
        OrgInviteToBusinessAuditEntry,
        OrgOauthAppAccessApprovedAuditEntry,
        OrgOauthAppAccessDeniedAuditEntry,
        OrgOauthAppAccessRequestedAuditEntry,
        OrgRemoveBillingManagerAuditEntry,
        OrgRemoveMemberAuditEntry,
        OrgRemoveOutsideCollaboratorAuditEntry,
        OrgRestoreMemberAuditEntry,
        OrgUnblockUserAuditEntry,
        OrgUpdateDefaultRepositoryPermissionAuditEntry,
        OrgUpdateMemberAuditEntry,
        OrgUpdateMemberRepositoryCreationPermissionAuditEntry,
        OrgUpdateMemberRepositoryInvitationPermissionAuditEntry,
        Organization,
        OrganizationIdentityProvider,
        OrganizationInvitation,
        Package,
        PackageFile,
        PackageTag,
        PackageVersion,
        PinnedDiscussion,
        PinnedEvent,
        PinnedIssue,
        PrivateRepositoryForkingDisableAuditEntry,
        PrivateRepositoryForkingEnableAuditEntry,
        Project,
        ProjectCard,
        ProjectColumn,
        PublicKey,
        PullRequest,
        PullRequestCommit,
        PullRequestCommitCommentThread,
        PullRequestReview,
        PullRequestReviewComment,
        PullRequestReviewThread,
        Push,
        PushAllowance,
        Reaction,
        ReadyForReviewEvent,
        Ref,
        ReferencedEvent,
        Release,
        ReleaseAsset,
        RemovedFromProjectEvent,
        RenamedTitleEvent,
        ReopenedEvent,
        RepoAccessAuditEntry,
        RepoAddMemberAuditEntry,
        RepoAddTopicAuditEntry,
        RepoArchivedAuditEntry,
        RepoChangeMergeSettingAuditEntry,
        RepoConfigDisableAnonymousGitAccessAuditEntry,
        RepoConfigDisableCollaboratorsOnlyAuditEntry,
        RepoConfigDisableContributorsOnlyAuditEntry,
        RepoConfigDisableSockpuppetDisallowedAuditEntry,
        RepoConfigEnableAnonymousGitAccessAuditEntry,
        RepoConfigEnableCollaboratorsOnlyAuditEntry,
        RepoConfigEnableContributorsOnlyAuditEntry,
        RepoConfigEnableSockpuppetDisallowedAuditEntry,
        RepoConfigLockAnonymousGitAccessAuditEntry,
        RepoConfigUnlockAnonymousGitAccessAuditEntry,
        RepoCreateAuditEntry,
        RepoDestroyAuditEntry,
        RepoRemoveMemberAuditEntry,
        RepoRemoveTopicAuditEntry,
        Repository,
        RepositoryInvitation,
        RepositoryTopic,
        RepositoryVisibilityChangeDisableAuditEntry,
        RepositoryVisibilityChangeEnableAuditEntry,
        RepositoryVulnerabilityAlert,
        ReviewDismissalAllowance,
        ReviewDismissedEvent,
        ReviewRequest,
        ReviewRequestRemovedEvent,
        ReviewRequestedEvent,
        SavedReply,
        SecurityAdvisory,
        SponsorsActivity,
        SponsorsListing,
        SponsorsTier,
        Sponsorship,
        SponsorshipNewsletter,
        Status,
        StatusCheckRollup,
        StatusContext,
        SubscribedEvent,
        Tag,
        Team,
        TeamAddMemberAuditEntry,
        TeamAddRepositoryAuditEntry,
        TeamChangeParentTeamAuditEntry,
        TeamDiscussion,
        TeamDiscussionComment,
        TeamRemoveMemberAuditEntry,
        TeamRemoveRepositoryAuditEntry,
        Topic,
        TransferredEvent,
        Tree,
        UnassignedEvent,
        UnlabeledEvent,
        UnlockedEvent,
        UnmarkedAsDuplicateEvent,
        UnpinnedEvent,
        UnsubscribedEvent,
        User,
        UserBlockedEvent,
        UserContentEdit,
        UserStatus,
        VerifiableDomain,
        Workflow,
        WorkflowRun,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsIdsForCommitIdsNodesOnCommit {
        pub id: ID,
        pub oid: GitObjectID,
        #[serde(rename = "associatedPullRequests")]
        pub associated_pull_requests:
            Option<GetPullRequestsIdsForCommitIdsNodesOnCommitAssociatedPullRequests>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsIdsForCommitIdsNodesOnCommitAssociatedPullRequests {
        pub nodes: Option<
            Vec<Option<GetPullRequestsIdsForCommitIdsNodesOnCommitAssociatedPullRequestsNodes>>,
        >,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsIdsForCommitIdsNodesOnCommitAssociatedPullRequestsNodes {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for GetPullRequestsIdsForCommitIds {
    type Variables = get_pull_requests_ids_for_commit_ids::Variables;
    type ResponseData = get_pull_requests_ids_for_commit_ids::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_pull_requests_ids_for_commit_ids::QUERY,
            operation_name: get_pull_requests_ids_for_commit_ids::OPERATION_NAME,
        }
    }
}
pub struct GetPullRequestsById;
pub mod get_pull_requests_by_id {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetPullRequestsById";
    pub const QUERY : & str = "#query PullRequests($owner: String!, $repository: String!, $cursor: String) {\n#  ...GetRateLimit\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, after: $cursor, orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          body\n#          number\n#          baseRefName\n#          headRefName\n#          title,\n#          mergedAt,\n#          updatedAt,\n#          author {\n#            login\n#          }\n#          closingIssuesReferences(first: 100) {\n#            totalCount\n#            edges {\n#              node {\n#                number\n#                title\n#              }\n#            }\n#          }\n#          mergeCommit {\n#            oid\n#          }\n#        }\n#      }\n#    }\n#  }\n#}\n\n#query PullRequestIDs($owner: String!, $repository: String!, $cursor: String) {\n#  ...GetRateLimit\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, after: $cursor, orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          id\n#        }\n#      }\n#    }\n#  }\n#}\n\n\nquery GetPullRequestsIdsForCommitIds($commit_ids: [ID!]!) {\n  ...RateLimit\n  nodes(ids: $commit_ids) {\n    __typename\n    ... on Commit {\n      id\n      oid\n      associatedPullRequests(first: 100) {\n        nodes {\n          id\n        }\n      }\n    }\n  }\n}\n\n\nquery GetPullRequestsById($pr_ids: [ID!]!) {\n  ...RateLimit\n  nodes(ids: $pr_ids) {\n    __typename\n    ...on PullRequest {\n      id\n      number\n      state\n      body\n      baseRefName\n      headRefName\n      title,\n      mergedAt,\n      updatedAt,\n      author {\n        __typename\n        login\n      }\n      closingIssuesReferences(first: 100) {\n        totalCount\n        edges {\n          node {\n            number\n            title\n          }\n        }\n      }\n      mergeCommit {\n        oid\n      }\n    }\n  }\n}\n\nquery GetCommitOnBranchHead($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n        }\n      }\n    }\n  }\n}\n\nquery GetCommitHistory($owner: String!, $repository: String!, $oid: GitObjectID!, $cursor: String) {\n  ...RateLimit\n  repository(owner: $owner, name: $repository) {\n    object(oid: $oid) {\n      __typename\n      ... on Commit {\n        oid\n        history(first: 100, after: $cursor) {\n          totalCount\n          edges {\n            cursor\n            node {\n              id\n              oid\n              committedDate\n              message\n              parents(first: 10) {\n                edges {\n                  node {\n                    oid\n                  }\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n\nfragment RateLimit on Query {\n  rateLimit {\n    limit\n    cost\n    remaining\n    resetAt\n  }\n}\n\n#fragment GetPullRequests on Commit {\n#  associatedPullRequests(first: 10) {\n#    nodes {\n#      id\n#      number\n#      mergeCommit {\n#        oid\n#      }\n#    }\n##    edges {\n##      node {\n##        id\n##      }\n##    }\n#  }\n#}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type DateTime = crate::github_collect::DateTime;
    type GitObjectID = crate::github_collect::GitObjectID;
    #[derive(Debug)]
    pub enum PullRequestState {
        CLOSED,
        MERGED,
        OPEN,
        Other(String),
    }
    impl ::serde::Serialize for PullRequestState {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                PullRequestState::CLOSED => "CLOSED",
                PullRequestState::MERGED => "MERGED",
                PullRequestState::OPEN => "OPEN",
                PullRequestState::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PullRequestState {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CLOSED" => Ok(PullRequestState::CLOSED),
                "MERGED" => Ok(PullRequestState::MERGED),
                "OPEN" => Ok(PullRequestState::OPEN),
                _ => Ok(PullRequestState::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub pr_ids: Vec<ID>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct RateLimit {
        #[serde(rename = "rateLimit")]
        pub rate_limit: Option<RateLimitRateLimit>,
    }
    #[derive(Deserialize, Debug)]
    pub struct RateLimitRateLimit {
        pub limit: Int,
        pub cost: Int,
        pub remaining: Int,
        #[serde(rename = "resetAt")]
        pub reset_at: DateTime,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(flatten)]
        pub rate_limit: RateLimit,
        pub nodes: Vec<Option<GetPullRequestsByIdNodes>>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum GetPullRequestsByIdNodes {
        AddedToProjectEvent,
        App,
        AssignedEvent,
        AutoMergeDisabledEvent,
        AutoMergeEnabledEvent,
        AutoRebaseEnabledEvent,
        AutoSquashEnabledEvent,
        AutomaticBaseChangeFailedEvent,
        AutomaticBaseChangeSucceededEvent,
        BaseRefChangedEvent,
        BaseRefDeletedEvent,
        BaseRefForcePushedEvent,
        Blob,
        Bot,
        BranchProtectionRule,
        CWE,
        CheckRun,
        CheckSuite,
        ClosedEvent,
        CodeOfConduct,
        CommentDeletedEvent,
        Commit,
        CommitComment,
        CommitCommentThread,
        ConnectedEvent,
        ConvertToDraftEvent,
        ConvertedNoteToIssueEvent,
        CrossReferencedEvent,
        DemilestonedEvent,
        DependencyGraphManifest,
        DeployKey,
        DeployedEvent,
        Deployment,
        DeploymentEnvironmentChangedEvent,
        DeploymentReview,
        DeploymentStatus,
        DisconnectedEvent,
        Discussion,
        DiscussionCategory,
        DiscussionComment,
        Enterprise,
        EnterpriseAdministratorInvitation,
        EnterpriseIdentityProvider,
        EnterpriseRepositoryInfo,
        EnterpriseServerInstallation,
        EnterpriseServerUserAccount,
        EnterpriseServerUserAccountEmail,
        EnterpriseServerUserAccountsUpload,
        EnterpriseUserAccount,
        Environment,
        ExternalIdentity,
        Gist,
        GistComment,
        HeadRefDeletedEvent,
        HeadRefForcePushedEvent,
        HeadRefRestoredEvent,
        IpAllowListEntry,
        Issue,
        IssueComment,
        Label,
        LabeledEvent,
        Language,
        License,
        LockedEvent,
        Mannequin,
        MarkedAsDuplicateEvent,
        MarketplaceCategory,
        MarketplaceListing,
        MembersCanDeleteReposClearAuditEntry,
        MembersCanDeleteReposDisableAuditEntry,
        MembersCanDeleteReposEnableAuditEntry,
        MentionedEvent,
        MergedEvent,
        Milestone,
        MilestonedEvent,
        MovedColumnsInProjectEvent,
        OauthApplicationCreateAuditEntry,
        OrgAddBillingManagerAuditEntry,
        OrgAddMemberAuditEntry,
        OrgBlockUserAuditEntry,
        OrgConfigDisableCollaboratorsOnlyAuditEntry,
        OrgConfigEnableCollaboratorsOnlyAuditEntry,
        OrgCreateAuditEntry,
        OrgDisableOauthAppRestrictionsAuditEntry,
        OrgDisableSamlAuditEntry,
        OrgDisableTwoFactorRequirementAuditEntry,
        OrgEnableOauthAppRestrictionsAuditEntry,
        OrgEnableSamlAuditEntry,
        OrgEnableTwoFactorRequirementAuditEntry,
        OrgInviteMemberAuditEntry,
        OrgInviteToBusinessAuditEntry,
        OrgOauthAppAccessApprovedAuditEntry,
        OrgOauthAppAccessDeniedAuditEntry,
        OrgOauthAppAccessRequestedAuditEntry,
        OrgRemoveBillingManagerAuditEntry,
        OrgRemoveMemberAuditEntry,
        OrgRemoveOutsideCollaboratorAuditEntry,
        OrgRestoreMemberAuditEntry,
        OrgUnblockUserAuditEntry,
        OrgUpdateDefaultRepositoryPermissionAuditEntry,
        OrgUpdateMemberAuditEntry,
        OrgUpdateMemberRepositoryCreationPermissionAuditEntry,
        OrgUpdateMemberRepositoryInvitationPermissionAuditEntry,
        Organization,
        OrganizationIdentityProvider,
        OrganizationInvitation,
        Package,
        PackageFile,
        PackageTag,
        PackageVersion,
        PinnedDiscussion,
        PinnedEvent,
        PinnedIssue,
        PrivateRepositoryForkingDisableAuditEntry,
        PrivateRepositoryForkingEnableAuditEntry,
        Project,
        ProjectCard,
        ProjectColumn,
        PublicKey,
        PullRequest(GetPullRequestsByIdNodesOnPullRequest),
        PullRequestCommit,
        PullRequestCommitCommentThread,
        PullRequestReview,
        PullRequestReviewComment,
        PullRequestReviewThread,
        Push,
        PushAllowance,
        Reaction,
        ReadyForReviewEvent,
        Ref,
        ReferencedEvent,
        Release,
        ReleaseAsset,
        RemovedFromProjectEvent,
        RenamedTitleEvent,
        ReopenedEvent,
        RepoAccessAuditEntry,
        RepoAddMemberAuditEntry,
        RepoAddTopicAuditEntry,
        RepoArchivedAuditEntry,
        RepoChangeMergeSettingAuditEntry,
        RepoConfigDisableAnonymousGitAccessAuditEntry,
        RepoConfigDisableCollaboratorsOnlyAuditEntry,
        RepoConfigDisableContributorsOnlyAuditEntry,
        RepoConfigDisableSockpuppetDisallowedAuditEntry,
        RepoConfigEnableAnonymousGitAccessAuditEntry,
        RepoConfigEnableCollaboratorsOnlyAuditEntry,
        RepoConfigEnableContributorsOnlyAuditEntry,
        RepoConfigEnableSockpuppetDisallowedAuditEntry,
        RepoConfigLockAnonymousGitAccessAuditEntry,
        RepoConfigUnlockAnonymousGitAccessAuditEntry,
        RepoCreateAuditEntry,
        RepoDestroyAuditEntry,
        RepoRemoveMemberAuditEntry,
        RepoRemoveTopicAuditEntry,
        Repository,
        RepositoryInvitation,
        RepositoryTopic,
        RepositoryVisibilityChangeDisableAuditEntry,
        RepositoryVisibilityChangeEnableAuditEntry,
        RepositoryVulnerabilityAlert,
        ReviewDismissalAllowance,
        ReviewDismissedEvent,
        ReviewRequest,
        ReviewRequestRemovedEvent,
        ReviewRequestedEvent,
        SavedReply,
        SecurityAdvisory,
        SponsorsActivity,
        SponsorsListing,
        SponsorsTier,
        Sponsorship,
        SponsorshipNewsletter,
        Status,
        StatusCheckRollup,
        StatusContext,
        SubscribedEvent,
        Tag,
        Team,
        TeamAddMemberAuditEntry,
        TeamAddRepositoryAuditEntry,
        TeamChangeParentTeamAuditEntry,
        TeamDiscussion,
        TeamDiscussionComment,
        TeamRemoveMemberAuditEntry,
        TeamRemoveRepositoryAuditEntry,
        Topic,
        TransferredEvent,
        Tree,
        UnassignedEvent,
        UnlabeledEvent,
        UnlockedEvent,
        UnmarkedAsDuplicateEvent,
        UnpinnedEvent,
        UnsubscribedEvent,
        User,
        UserBlockedEvent,
        UserContentEdit,
        UserStatus,
        VerifiableDomain,
        Workflow,
        WorkflowRun,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsByIdNodesOnPullRequest {
        pub id: ID,
        pub number: Int,
        pub state: PullRequestState,
        pub body: String,
        #[serde(rename = "baseRefName")]
        pub base_ref_name: String,
        #[serde(rename = "headRefName")]
        pub head_ref_name: String,
        pub title: String,
        #[serde(rename = "mergedAt")]
        pub merged_at: Option<DateTime>,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        pub author: Option<GetPullRequestsByIdNodesOnPullRequestAuthor>,
        #[serde(rename = "closingIssuesReferences")]
        pub closing_issues_references:
            Option<GetPullRequestsByIdNodesOnPullRequestClosingIssuesReferences>,
        #[serde(rename = "mergeCommit")]
        pub merge_commit: Option<GetPullRequestsByIdNodesOnPullRequestMergeCommit>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsByIdNodesOnPullRequestAuthor {
        pub login: String,
        #[serde(flatten)]
        pub on: GetPullRequestsByIdNodesOnPullRequestAuthorOn,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum GetPullRequestsByIdNodesOnPullRequestAuthorOn {
        Bot,
        EnterpriseUserAccount,
        Mannequin,
        Organization,
        User,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsByIdNodesOnPullRequestClosingIssuesReferences {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
        pub edges:
            Option<Vec<Option<GetPullRequestsByIdNodesOnPullRequestClosingIssuesReferencesEdges>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsByIdNodesOnPullRequestClosingIssuesReferencesEdges {
        pub node: Option<GetPullRequestsByIdNodesOnPullRequestClosingIssuesReferencesEdgesNode>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsByIdNodesOnPullRequestClosingIssuesReferencesEdgesNode {
        pub number: Int,
        pub title: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsByIdNodesOnPullRequestMergeCommit {
        pub oid: GitObjectID,
    }
}
impl graphql_client::GraphQLQuery for GetPullRequestsById {
    type Variables = get_pull_requests_by_id::Variables;
    type ResponseData = get_pull_requests_by_id::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_pull_requests_by_id::QUERY,
            operation_name: get_pull_requests_by_id::OPERATION_NAME,
        }
    }
}
pub struct GetCommitOnBranchHead;
pub mod get_commit_on_branch_head {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetCommitOnBranchHead";
    pub const QUERY : & str = "#query PullRequests($owner: String!, $repository: String!, $cursor: String) {\n#  ...GetRateLimit\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, after: $cursor, orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          body\n#          number\n#          baseRefName\n#          headRefName\n#          title,\n#          mergedAt,\n#          updatedAt,\n#          author {\n#            login\n#          }\n#          closingIssuesReferences(first: 100) {\n#            totalCount\n#            edges {\n#              node {\n#                number\n#                title\n#              }\n#            }\n#          }\n#          mergeCommit {\n#            oid\n#          }\n#        }\n#      }\n#    }\n#  }\n#}\n\n#query PullRequestIDs($owner: String!, $repository: String!, $cursor: String) {\n#  ...GetRateLimit\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, after: $cursor, orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          id\n#        }\n#      }\n#    }\n#  }\n#}\n\n\nquery GetPullRequestsIdsForCommitIds($commit_ids: [ID!]!) {\n  ...RateLimit\n  nodes(ids: $commit_ids) {\n    __typename\n    ... on Commit {\n      id\n      oid\n      associatedPullRequests(first: 100) {\n        nodes {\n          id\n        }\n      }\n    }\n  }\n}\n\n\nquery GetPullRequestsById($pr_ids: [ID!]!) {\n  ...RateLimit\n  nodes(ids: $pr_ids) {\n    __typename\n    ...on PullRequest {\n      id\n      number\n      state\n      body\n      baseRefName\n      headRefName\n      title,\n      mergedAt,\n      updatedAt,\n      author {\n        __typename\n        login\n      }\n      closingIssuesReferences(first: 100) {\n        totalCount\n        edges {\n          node {\n            number\n            title\n          }\n        }\n      }\n      mergeCommit {\n        oid\n      }\n    }\n  }\n}\n\nquery GetCommitOnBranchHead($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n        }\n      }\n    }\n  }\n}\n\nquery GetCommitHistory($owner: String!, $repository: String!, $oid: GitObjectID!, $cursor: String) {\n  ...RateLimit\n  repository(owner: $owner, name: $repository) {\n    object(oid: $oid) {\n      __typename\n      ... on Commit {\n        oid\n        history(first: 100, after: $cursor) {\n          totalCount\n          edges {\n            cursor\n            node {\n              id\n              oid\n              committedDate\n              message\n              parents(first: 10) {\n                edges {\n                  node {\n                    oid\n                  }\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n\nfragment RateLimit on Query {\n  rateLimit {\n    limit\n    cost\n    remaining\n    resetAt\n  }\n}\n\n#fragment GetPullRequests on Commit {\n#  associatedPullRequests(first: 10) {\n#    nodes {\n#      id\n#      number\n#      mergeCommit {\n#        oid\n#      }\n#    }\n##    edges {\n##      node {\n##        id\n##      }\n##    }\n#  }\n#}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type GitObjectID = crate::github_collect::GitObjectID;
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repository: String,
        pub branch: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub repository: Option<GetCommitOnBranchHeadRepository>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitOnBranchHeadRepository {
        #[serde(rename = "ref")]
        pub ref_: Option<GetCommitOnBranchHeadRepositoryRef>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitOnBranchHeadRepositoryRef {
        pub name: String,
        pub target: Option<GetCommitOnBranchHeadRepositoryRefTarget>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum GetCommitOnBranchHeadRepositoryRefTarget {
        Blob,
        Commit(GetCommitOnBranchHeadRepositoryRefTargetOnCommit),
        Tag,
        Tree,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitOnBranchHeadRepositoryRefTargetOnCommit {
        pub oid: GitObjectID,
    }
}
impl graphql_client::GraphQLQuery for GetCommitOnBranchHead {
    type Variables = get_commit_on_branch_head::Variables;
    type ResponseData = get_commit_on_branch_head::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_commit_on_branch_head::QUERY,
            operation_name: get_commit_on_branch_head::OPERATION_NAME,
        }
    }
}
pub struct GetCommitHistory;
pub mod get_commit_history {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetCommitHistory";
    pub const QUERY : & str = "#query PullRequests($owner: String!, $repository: String!, $cursor: String) {\n#  ...GetRateLimit\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, after: $cursor, orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          body\n#          number\n#          baseRefName\n#          headRefName\n#          title,\n#          mergedAt,\n#          updatedAt,\n#          author {\n#            login\n#          }\n#          closingIssuesReferences(first: 100) {\n#            totalCount\n#            edges {\n#              node {\n#                number\n#                title\n#              }\n#            }\n#          }\n#          mergeCommit {\n#            oid\n#          }\n#        }\n#      }\n#    }\n#  }\n#}\n\n#query PullRequestIDs($owner: String!, $repository: String!, $cursor: String) {\n#  ...GetRateLimit\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, after: $cursor, orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          id\n#        }\n#      }\n#    }\n#  }\n#}\n\n\nquery GetPullRequestsIdsForCommitIds($commit_ids: [ID!]!) {\n  ...RateLimit\n  nodes(ids: $commit_ids) {\n    __typename\n    ... on Commit {\n      id\n      oid\n      associatedPullRequests(first: 100) {\n        nodes {\n          id\n        }\n      }\n    }\n  }\n}\n\n\nquery GetPullRequestsById($pr_ids: [ID!]!) {\n  ...RateLimit\n  nodes(ids: $pr_ids) {\n    __typename\n    ...on PullRequest {\n      id\n      number\n      state\n      body\n      baseRefName\n      headRefName\n      title,\n      mergedAt,\n      updatedAt,\n      author {\n        __typename\n        login\n      }\n      closingIssuesReferences(first: 100) {\n        totalCount\n        edges {\n          node {\n            number\n            title\n          }\n        }\n      }\n      mergeCommit {\n        oid\n      }\n    }\n  }\n}\n\nquery GetCommitOnBranchHead($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n        }\n      }\n    }\n  }\n}\n\nquery GetCommitHistory($owner: String!, $repository: String!, $oid: GitObjectID!, $cursor: String) {\n  ...RateLimit\n  repository(owner: $owner, name: $repository) {\n    object(oid: $oid) {\n      __typename\n      ... on Commit {\n        oid\n        history(first: 100, after: $cursor) {\n          totalCount\n          edges {\n            cursor\n            node {\n              id\n              oid\n              committedDate\n              message\n              parents(first: 10) {\n                edges {\n                  node {\n                    oid\n                  }\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n\nfragment RateLimit on Query {\n  rateLimit {\n    limit\n    cost\n    remaining\n    resetAt\n  }\n}\n\n#fragment GetPullRequests on Commit {\n#  associatedPullRequests(first: 10) {\n#    nodes {\n#      id\n#      number\n#      mergeCommit {\n#        oid\n#      }\n#    }\n##    edges {\n##      node {\n##        id\n##      }\n##    }\n#  }\n#}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type GitObjectID = crate::github_collect::GitObjectID;
    type DateTime = crate::github_collect::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repository: String,
        pub oid: GitObjectID,
        pub cursor: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct RateLimit {
        #[serde(rename = "rateLimit")]
        pub rate_limit: Option<RateLimitRateLimit>,
    }
    #[derive(Deserialize, Debug)]
    pub struct RateLimitRateLimit {
        pub limit: Int,
        pub cost: Int,
        pub remaining: Int,
        #[serde(rename = "resetAt")]
        pub reset_at: DateTime,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(flatten)]
        pub rate_limit: RateLimit,
        pub repository: Option<GetCommitHistoryRepository>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitHistoryRepository {
        pub object: Option<GetCommitHistoryRepositoryObject>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum GetCommitHistoryRepositoryObject {
        Blob,
        Commit(GetCommitHistoryRepositoryObjectOnCommit),
        Tag,
        Tree,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitHistoryRepositoryObjectOnCommit {
        pub oid: GitObjectID,
        pub history: GetCommitHistoryRepositoryObjectOnCommitHistory,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitHistoryRepositoryObjectOnCommitHistory {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
        pub edges: Option<Vec<Option<GetCommitHistoryRepositoryObjectOnCommitHistoryEdges>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitHistoryRepositoryObjectOnCommitHistoryEdges {
        pub cursor: String,
        pub node: Option<GetCommitHistoryRepositoryObjectOnCommitHistoryEdgesNode>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitHistoryRepositoryObjectOnCommitHistoryEdgesNode {
        pub id: ID,
        pub oid: GitObjectID,
        #[serde(rename = "committedDate")]
        pub committed_date: DateTime,
        pub message: String,
        pub parents: GetCommitHistoryRepositoryObjectOnCommitHistoryEdgesNodeParents,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitHistoryRepositoryObjectOnCommitHistoryEdgesNodeParents {
        pub edges: Option<
            Vec<Option<GetCommitHistoryRepositoryObjectOnCommitHistoryEdgesNodeParentsEdges>>,
        >,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitHistoryRepositoryObjectOnCommitHistoryEdgesNodeParentsEdges {
        pub node: Option<GetCommitHistoryRepositoryObjectOnCommitHistoryEdgesNodeParentsEdgesNode>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetCommitHistoryRepositoryObjectOnCommitHistoryEdgesNodeParentsEdgesNode {
        pub oid: GitObjectID,
    }
}
impl graphql_client::GraphQLQuery for GetCommitHistory {
    type Variables = get_commit_history::Variables;
    type ResponseData = get_commit_history::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_commit_history::QUERY,
            operation_name: get_commit_history::OPERATION_NAME,
        }
    }
}
