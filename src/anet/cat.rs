gw2rs_id_u64!(CatId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Cat {
    pub id: CatId,
    pub hint: String,
}
