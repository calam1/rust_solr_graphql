use juniper::{FieldResult, RootNode, FieldError};
use juniper::{GraphQLObject};
use stellr::{SolrError, DirectSolrClient, SolrCloudMethods, SolrRequestBuilder, SolrRequest};
use serde::{Deserialize};
use std::borrow::Borrow;
pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn orderHistory(account_id: String, contact_id: String) -> FieldResult<Root> {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let future = stuff(account_id, contact_id);
        let result = rt.block_on(future);

        Ok(result.unwrap())
    }
}

pub struct MutationRoot;

#[juniper::object]

impl MutationRoot {
    fn create_order_history() -> FieldResult<Root> {
        Err(FieldError::from("not implemented"))
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

async fn stuff(account_id: String, contact_id: String) -> Result<Root, SolrError> {
    let solr_client = DirectSolrClient::new("https://solr.com:8983/solr")?;

    let pwd_option: Option<String> = Some(String::from("solr-pwd"));

    let mut owned_account_number_key_value = "accountNumber:".to_owned();
    owned_account_number_key_value.push_str(&*account_id.borrow());

    let mut owned_contact_id_key_value = "contactId:".to_owned();
    owned_contact_id_key_value.push_str(&*contact_id.borrow());

    let solr_request = solr_client
        .select("orders")?
        .rows(10)
        .q(&*owned_account_number_key_value)
        .fq(&*owned_contact_id_key_value)
        .fl("*")
        .wt("json")
        .basic_auth("solr-user", pwd_option);

    let result_struct = solr_request.call::<Root>().await?;
    println!("{:?}", result_struct);

    Ok(result_struct)
}

#[derive(GraphQLObject)]
#[graphql(description = "root of order history")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub response_header: ResponseHeader,
    pub response: Response,
}

#[derive(GraphQLObject)]
#[graphql(description = "order history respnse header")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseHeader {
    pub status: i32,
    #[serde(rename = "QTime")]
    pub qtime: i32,
    pub params: Params,
}

#[derive(GraphQLObject)]
#[graphql(description = "order history query params")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub q: String,
    pub fl: String,
    pub fq: String,
    pub rows: String,
    pub wt: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "order history response stats")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub num_found: i32,
    pub start: i32,
    pub max_score: f64,
    pub docs: Vec<Doc>,
}

#[derive(GraphQLObject)]
#[graphql(description = "order history details")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Doc {
    pub id: String,
    pub order_number: String,
    pub gcom_order_number: String,
    pub ordered_by: String,
    pub contact_id: String,
    pub account_number: String,
    pub sales_office: String,
    pub purchase_order_number: String,
    pub requisitioner_name: Option<String>,
    pub system_timestamp: String,
    pub order_channel: String,
    pub delivery_method: String,
    pub attention: Option<String>,
    pub ship_company_name1: String,
    pub ship_address1: String,
    pub ship_city: String,
    pub ship_region: String,
    pub ship_postal_code: String,
    pub ship_country: String,
    pub contact_phone_number: String,
    pub contact_fax_number: String,
    pub contact_email_address: String,
    pub currency: String,
    pub payment_method: String,
    pub customer_username: String,
    pub purchase_order_type: String,
    pub deleted_flag: String,
    pub order_line_number: String,
    pub item_number: String,
    pub quantity: i32,
    pub purchase_order_line_number: String,
    pub item_short_desc: String,
    pub brand_name: String,
    pub mfg_part_num: Option<String>,
    pub unspsc: String,
    pub order_header_flag: String,
    pub higher_level_line_item: String,
    pub orig_sls_ordline_itm: String,
    #[serde(rename = "_version_")]
    pub version: f64,
    pub doc_typ: String,
    pub order_created_date_time: String,
    pub extended_item_price: f64,
    pub unit_price: f64,
    pub order_subtotal: f64,
    pub order_tax: f64,
    pub order_freight: f64,
    pub order_total: f64,
    pub fuel_surcharge_freight: f64,
    pub quote_exp_dt: String,
    pub item_tax: f64,
    pub item_freight: f64,
    pub last_modified: String,
    pub freight_terms: Option<String>,
    pub line_freight_terms: Option<String>,
    #[serde(rename = "zDPPItemCond")]
    pub z_dppitem_cond: Option<String>,
    pub s_org: Option<String>,
    pub gsa_schedule: Option<String>,
    pub gsa_sched_desc: Option<String>,
}
