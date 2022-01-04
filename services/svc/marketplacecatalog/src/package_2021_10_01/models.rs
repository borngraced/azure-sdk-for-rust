#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ArtifactType {
    Template,
    Fragment,
    Custom,
    Metadata,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityEntity {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "isPIRequired")]
    pub is_pi_required: bool,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "planID", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meter: Option<serde_json::Value>,
    #[serde(rename = "pricingAudience")]
    pub pricing_audience: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terms: Vec<Term>,
    #[serde(rename = "piFilter", default, skip_serializing_if = "Option::is_none")]
    pub pi_filter: Option<serde_json::Value>,
    #[serde(rename = "isStopSell")]
    pub is_stop_sell: bool,
    #[serde(rename = "hasFreeTrials")]
    pub has_free_trials: bool,
    #[serde(rename = "assetBehaviors", default, skip_serializing_if = "Vec::is_empty")]
    pub asset_behaviors: Vec<String>,
    #[serde(rename = "consumptionUnitType", default, skip_serializing_if = "Option::is_none")]
    pub consumption_unit_type: Option<String>,
    #[serde(rename = "displayRank")]
    pub display_rank: i32,
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[serde(rename = "remediationRequired")]
    pub remediation_required: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remediations: Vec<Remediation>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i32>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i32>,
    #[serde(rename = "planAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub plan_availabilities: Vec<AvailabilityEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingPlan {
    #[serde(rename = "billingPeriod", default, skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CspState {
    OptIn,
    OptOut,
    Terminated,
    SelectiveOptIn,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogItem {
    pub language: String,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "hasStandardContractAmendments")]
    pub has_standard_contract_amendments: bool,
    #[serde(rename = "publisherMpnId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_mpn_id: Option<String>,
    #[serde(rename = "sellerId", default, skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[serde(rename = "partnerCenterId", default, skip_serializing_if = "Option::is_none")]
    pub partner_center_id: Option<String>,
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "legacyId")]
    pub legacy_id: String,
    #[serde(rename = "determinedStorefronts", default, skip_serializing_if = "Vec::is_empty")]
    pub determined_storefronts: Vec<Store>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "longSummary", default, skip_serializing_if = "Option::is_none")]
    pub long_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "offerType")]
    pub offer_type: serde_json::Value,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[serde(rename = "isPreview")]
    pub is_preview: bool,
    #[serde(rename = "isStopSell")]
    pub is_stop_sell: bool,
    #[serde(rename = "fulfillBeforeChargeEligible")]
    pub fulfill_before_charge_eligible: bool,
    #[serde(rename = "marketingMaterial", default, skip_serializing_if = "Option::is_none")]
    pub marketing_material: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub markets: Vec<String>,
    #[serde(rename = "isvContactDetails", default, skip_serializing_if = "Option::is_none")]
    pub isv_contact_details: Option<serde_json::Value>,
    #[serde(rename = "bigId")]
    pub big_id: String,
    #[serde(rename = "ocpSolutionId", default, skip_serializing_if = "Option::is_none")]
    pub ocp_solution_id: Option<String>,
    #[serde(rename = "legalTermsUri", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms_uri: Option<String>,
    #[serde(rename = "cspLegalTermsUri", default, skip_serializing_if = "Option::is_none")]
    pub csp_legal_terms_uri: Option<String>,
    #[serde(rename = "legalTermsType")]
    pub legal_terms_type: serde_json::Value,
    #[serde(rename = "privacyPolicyUri", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_uri: Option<String>,
    #[serde(rename = "helpLink", default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<String>,
    #[serde(rename = "supportUri", default, skip_serializing_if = "Option::is_none")]
    pub support_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
    #[serde(rename = "categoryIds", default, skip_serializing_if = "Vec::is_empty")]
    pub category_ids: Vec<String>,
    #[serde(rename = "marketCode", default, skip_serializing_if = "Option::is_none")]
    pub market_code: Option<String>,
    #[serde(rename = "marketStates", default, skip_serializing_if = "Vec::is_empty")]
    pub market_states: Vec<String>,
    #[serde(rename = "industryIds", default, skip_serializing_if = "Vec::is_empty")]
    pub industry_ids: Vec<String>,
    #[serde(rename = "cloudIndustryCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub cloud_industry_categories: Vec<String>,
    #[serde(rename = "primaryProduct", default, skip_serializing_if = "Option::is_none")]
    pub primary_product: Option<String>,
    #[serde(rename = "supportedProducts", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_products: Vec<String>,
    #[serde(rename = "applicableProducts", default, skip_serializing_if = "Vec::is_empty")]
    pub applicable_products: Vec<String>,
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub competencies: Vec<Competency>,
    #[serde(rename = "hasPrices", default, skip_serializing_if = "Option::is_none")]
    pub has_prices: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<serde_json::Value>,
    #[serde(rename = "marketPricingDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub market_pricing_details: Vec<MarketPricingDetailsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing: Option<serde_json::Value>,
    #[serde(rename = "solutionAreas", default, skip_serializing_if = "Vec::is_empty")]
    pub solution_areas: Vec<String>,
    #[serde(rename = "screenshotUris", default, skip_serializing_if = "Vec::is_empty")]
    pub screenshot_uris: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<LinkProperties>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<Filter>,
    #[serde(rename = "iconFileUris", default, skip_serializing_if = "Option::is_none")]
    pub icon_file_uris: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<Artifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<ImageGroup>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub videos: Vec<ProductVideo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plans: Vec<Plan>,
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "definitionTemplates", default, skip_serializing_if = "Option::is_none")]
    pub definition_templates: Option<serde_json::Value>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
    #[serde(rename = "restrictedAudience", default, skip_serializing_if = "Option::is_none")]
    pub restricted_audience: Option<serde_json::Value>,
    #[serde(rename = "isThirdParty")]
    pub is_third_party: bool,
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "hideKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub hide_keys: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    pub popularity: f64,
    #[serde(rename = "pricingDetailsUri", default, skip_serializing_if = "Option::is_none")]
    pub pricing_details_uri: Option<String>,
    #[serde(rename = "hasFreeTrials")]
    pub has_free_trials: bool,
    #[serde(rename = "isByol")]
    pub is_byol: bool,
    #[serde(rename = "isMacc")]
    pub is_macc: bool,
    #[serde(rename = "hasFreePlans")]
    pub has_free_plans: bool,
    #[serde(rename = "isQuantifiable")]
    pub is_quantifiable: bool,
    #[serde(rename = "altStackReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_stack_reference: Option<String>,
    #[serde(rename = "hasPaygPlans")]
    pub has_payg_plans: bool,
    #[serde(rename = "isReseller")]
    pub is_reseller: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    #[serde(rename = "isExcludedFromSearch")]
    pub is_excluded_from_search: bool,
    #[serde(rename = "applicableStoreFronts", default, skip_serializing_if = "Option::is_none")]
    pub applicable_store_fronts: Option<serde_json::Value>,
    #[serde(rename = "offerVersion", default, skip_serializing_if = "Option::is_none")]
    pub offer_version: Option<String>,
    #[serde(rename = "isMicrosoftProduct", default, skip_serializing_if = "Option::is_none")]
    pub is_microsoft_product: Option<bool>,
    #[serde(rename = "productOwnershipSellingMotion", default, skip_serializing_if = "Option::is_none")]
    pub product_ownership_selling_motion: Option<String>,
    #[serde(rename = "documentLinks", default, skip_serializing_if = "Vec::is_empty")]
    pub document_links: Vec<LinkProperties>,
    #[serde(rename = "offerEnvironment")]
    pub offer_environment: serde_json::Value,
    #[serde(rename = "linkedAddIns", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_add_ins: Vec<String>,
    #[serde(rename = "excludeFromBootstrap")]
    pub exclude_from_bootstrap: bool,
    #[serde(rename = "hydrationNotificationReceivedAt", default, skip_serializing_if = "Option::is_none")]
    pub hydration_notification_received_at: Option<String>,
    #[serde(rename = "bigCatLastModifiedDate", default, skip_serializing_if = "Option::is_none")]
    pub big_cat_last_modified_date: Option<String>,
    #[serde(rename = "disableSendEmailOnPurchase")]
    pub disable_send_email_on_purchase: bool,
    #[serde(rename = "hideFromSaasBlade")]
    pub hide_from_saas_blade: bool,
    #[serde(rename = "integratedWithMicrosoftGraphApi")]
    pub integrated_with_microsoft_graph_api: bool,
    #[serde(rename = "multiTenantAadAppId", default, skip_serializing_if = "Option::is_none")]
    pub multi_tenant_aad_app_id: Option<String>,
    #[serde(rename = "licenseManagementType", default, skip_serializing_if = "Option::is_none")]
    pub license_management_type: Option<String>,
    #[serde(rename = "licenseModel", default, skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "pbiServicePrincipalIds", default, skip_serializing_if = "Vec::is_empty")]
    pub pbi_service_principal_ids: Vec<String>,
    #[serde(rename = "isCoreVm", default, skip_serializing_if = "Option::is_none")]
    pub is_core_vm: Option<bool>,
    #[serde(rename = "m365CertificationInfo", default, skip_serializing_if = "Option::is_none")]
    pub m365_certification_info: Option<serde_json::Value>,
    #[serde(rename = "downloadLink", default, skip_serializing_if = "Option::is_none")]
    pub download_link: Option<String>,
    #[serde(rename = "downloadSampleLink", default, skip_serializing_if = "Option::is_none")]
    pub download_sample_link: Option<String>,
    #[serde(rename = "omexAssetId", default, skip_serializing_if = "Option::is_none")]
    pub omex_asset_id: Option<String>,
    #[serde(rename = "mixProductId", default, skip_serializing_if = "Option::is_none")]
    pub mix_product_id: Option<String>,
    #[serde(rename = "appFreeType", default, skip_serializing_if = "Option::is_none")]
    pub app_free_type: Option<String>,
    #[serde(rename = "storeFrontPricings", default, skip_serializing_if = "Option::is_none")]
    pub store_front_pricings: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CertificationType {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Competency {
    #[serde(rename = "competencyName", default, skip_serializing_if = "Option::is_none")]
    pub competency_name: Option<String>,
    #[serde(rename = "competencyLevel", default, skip_serializing_if = "Option::is_none")]
    pub competency_level: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CurrencyDecorator {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefinitionTemplates {
    #[serde(rename = "uiDefinitionFileUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_file_uri: Option<String>,
    #[serde(rename = "defaultDeploymentTemplateId", default, skip_serializing_if = "Option::is_none")]
    pub default_deployment_template_id: Option<String>,
    #[serde(rename = "deploymentTemplateFileUris", default, skip_serializing_if = "Option::is_none")]
    pub deployment_template_file_uris: Option<serde_json::Value>,
    #[serde(rename = "deploymentFragmentFileUris", default, skip_serializing_if = "Option::is_none")]
    pub deployment_fragment_file_uris: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Duration {
    #[serde(rename = "durationValue")]
    pub duration_value: i64,
    #[serde(rename = "durationUnit")]
    pub duration_unit: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnvironmentInfo {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IconKind {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Image>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncludedQuantityProperty {
    #[serde(flatten)]
    pub included_quantity_property2: IncludedQuantityProperty2,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncludedQuantityProperty2 {
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeadGeneration {
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LegalTermsType {
    None,
    #[serde(rename = "EA")]
    Ea,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct M365CertificationInfo {
    #[serde(rename = "m365CertificationType")]
    pub m365_certification_type: serde_json::Value,
    #[serde(rename = "m365CertificationDetailsUrl", default, skip_serializing_if = "Option::is_none")]
    pub m365_certification_details_url: Option<String>,
    #[serde(rename = "m365CertificationId", default, skip_serializing_if = "Option::is_none")]
    pub m365_certification_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketPricingDetailsItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing: Option<serde_json::Value>,
    #[serde(rename = "marketCode", default, skip_serializing_if = "Option::is_none")]
    pub market_code: Option<String>,
    #[serde(rename = "marketStates", default, skip_serializing_if = "Vec::is_empty")]
    pub market_states: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketingMaterial {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "learnUri", default, skip_serializing_if = "Option::is_none")]
    pub learn_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meter {
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "partNumber", default, skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "consumptionResourceId", default, skip_serializing_if = "Option::is_none")]
    pub consumption_resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "includedQuantityProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub included_quantity_properties: Vec<IncludedQuantityProperty>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferMetadata {
    #[serde(rename = "leadGeneration", default, skip_serializing_if = "Option::is_none")]
    pub lead_generation: Option<serde_json::Value>,
    #[serde(rename = "testDrive", default, skip_serializing_if = "Option::is_none")]
    pub test_drive: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OfferType {
    None,
    DevService,
    ManagedApplication,
    VirtualMachine,
    AzureApplication,
    Container,
    SaaS,
    SolutionTemplate,
    IotEdgeModules,
    ManagedServices,
    ContainerApps,
    VisualStudioExtension,
    DynamicsOps,
    #[serde(rename = "DynamicsCE")]
    DynamicsCe,
    #[serde(rename = "DynamicsBC")]
    DynamicsBc,
    #[serde(rename = "PowerBI")]
    PowerBi,
    ConsultingServices,
    CosellOnly,
    CoreVirtualMachine,
    #[serde(rename = "PowerBIVisuals")]
    PowerBiVisuals,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperatingSystem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PiFilter {
    #[serde(rename = "exclusionProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub exclusion_properties: Vec<String>,
    #[serde(rename = "inclusionProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub inclusion_properties: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageResultOfCatalogItem {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CatalogItem>,
    #[serde(rename = "nextPageLink", default, skip_serializing_if = "Option::is_none")]
    pub next_page_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "restrictedAudience", default, skip_serializing_if = "Option::is_none")]
    pub restricted_audience: Option<serde_json::Value>,
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "legacyPlanId", default, skip_serializing_if = "Option::is_none")]
    pub legacy_plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    #[serde(rename = "leadGeneration", default, skip_serializing_if = "Option::is_none")]
    pub lead_generation: Option<serde_json::Value>,
    #[serde(rename = "testDrive", default, skip_serializing_if = "Option::is_none")]
    pub test_drive: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<AvailabilityEntity>,
    #[serde(rename = "categoryIds", default, skip_serializing_if = "Vec::is_empty")]
    pub category_ids: Vec<String>,
    #[serde(rename = "conversionPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub conversion_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<serde_json::Value>,
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<Artifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "itemName", default, skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    #[serde(rename = "hasFreeTrials")]
    pub has_free_trials: bool,
    #[serde(rename = "isByol")]
    pub is_byol: bool,
    #[serde(rename = "isFree")]
    pub is_free: bool,
    #[serde(rename = "isPayg")]
    pub is_payg: bool,
    #[serde(rename = "isStopSell")]
    pub is_stop_sell: bool,
    #[serde(rename = "altStackReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_stack_reference: Option<String>,
    #[serde(rename = "stackType", default, skip_serializing_if = "Option::is_none")]
    pub stack_type: Option<String>,
    #[serde(rename = "cspState")]
    pub csp_state: serde_json::Value,
    #[serde(rename = "resourceProviderNamespace", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_namespace: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "minQuantity", default, skip_serializing_if = "Option::is_none")]
    pub min_quantity: Option<i32>,
    #[serde(rename = "maxQuantity", default, skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i32>,
    #[serde(rename = "isQuantifiable")]
    pub is_quantifiable: bool,
    #[serde(rename = "callToAction", default, skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<String>,
    #[serde(rename = "redirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(rename = "serviceIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub service_identifier: Option<String>,
    #[serde(rename = "vmSecurityType", default, skip_serializing_if = "Option::is_none")]
    pub vm_security_type: Option<serde_json::Value>,
    #[serde(rename = "displayRank", default, skip_serializing_if = "Option::is_none")]
    pub display_rank: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanPrice {
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "currencyDecorator")]
    pub currency_decorator: serde_json::Value,
    pub price: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreviewImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "imagePurpose", default, skip_serializing_if = "Option::is_none")]
    pub image_purpose: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "isPIRequired")]
    pub is_pi_required: bool,
    #[serde(rename = "listPrice")]
    pub list_price: f64,
    pub msrp: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pricing {
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "planPrices", default, skip_serializing_if = "Vec::is_empty")]
    pub plan_prices: Vec<PlanPrice>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PricingAudience {
    None,
    DirectCommercial,
    PartnerCommercial,
    Custom,
    IndirectCommercial,
    IndirectGov,
    DirectChk,
    DirectBlue,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PricingOptions {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductVideo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "videoPurpose", default, skip_serializing_if = "Option::is_none")]
    pub video_purpose: Option<String>,
    #[serde(rename = "previewImage", default, skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProrationPolicy {
    #[serde(rename = "minimumProratedUnits", default, skip_serializing_if = "Option::is_none")]
    pub minimum_prorated_units: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Remediation {
    #[serde(rename = "remediationId", default, skip_serializing_if = "Option::is_none")]
    pub remediation_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestrictedAudience {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tenants: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServiceDurationUnit {
    Days,
    Hours,
    Weeks,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Store {
    Appsource,
    #[serde(rename = "AMP")]
    Amp,
    Ibiza,
    Cosell,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StoreFrontOptions {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Term {
    #[serde(rename = "termDescriptionParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub term_description_parameters: Vec<TermDescriptionParameter>,
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
    #[serde(rename = "termUnits", default, skip_serializing_if = "Option::is_none")]
    pub term_units: Option<String>,
    #[serde(rename = "prorationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub proration_policy: Option<serde_json::Value>,
    #[serde(rename = "termDescription", default, skip_serializing_if = "Option::is_none")]
    pub term_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    #[serde(rename = "renewTermId", default, skip_serializing_if = "Option::is_none")]
    pub renew_term_id: Option<String>,
    #[serde(rename = "renewTermUnits", default, skip_serializing_if = "Option::is_none")]
    pub renew_term_units: Option<String>,
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TermDescriptionParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestDrive {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "userManual", default, skip_serializing_if = "Option::is_none")]
    pub user_manual: Option<String>,
    #[serde(rename = "testDriveDuration", default, skip_serializing_if = "Option::is_none")]
    pub test_drive_duration: Option<String>,
    #[serde(rename = "accessInformation", default, skip_serializing_if = "Option::is_none")]
    pub access_information: Option<String>,
    #[serde(rename = "orchestrationType", default, skip_serializing_if = "Option::is_none")]
    pub orchestration_type: Option<String>,
    #[serde(rename = "labId", default, skip_serializing_if = "Option::is_none")]
    pub lab_id: Option<String>,
    #[serde(rename = "demoId", default, skip_serializing_if = "Option::is_none")]
    pub demo_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<serde_json::Value>,
    #[serde(rename = "powerBiDashboardLink", default, skip_serializing_if = "Option::is_none")]
    pub power_bi_dashboard_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiPricing {
    #[serde(rename = "pricingOptions")]
    pub pricing_options: serde_json::Value,
    #[serde(rename = "hasPrices", default, skip_serializing_if = "Option::is_none")]
    pub has_prices: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VmSecurityType {
    None,
    Trusted,
    Confidential,
}
