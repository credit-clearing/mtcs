use crate::notify;
use form_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn create_obligation(obligation: Obligation) -> ExternResult<Record> {
    let obligation_hash = create_entry(&EntryTypes::Obligation(obligation.clone()))?;
    create_link(
        obligation.debtor.clone(),
        obligation_hash.clone(),
        LinkTypes::DebtorToObligations,
        (),
    )?;
    create_link(
        obligation.creator.clone(),
        obligation_hash.clone(),
        LinkTypes::CreatorToObligations,
        (),
    )?;
    let record = get(obligation_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly created Obligation"))
    ))?;
    let path = Path::from("all_obligations");
    create_link(
        path.path_entry_hash()?,
        obligation_hash.clone(),
        LinkTypes::AllObligations,
        (),
    )?;

    notify(obligation.debtor, record.action_address().to_owned());

    Ok(record)
}
#[hdk_extern]
pub fn get_obligation(original_obligation_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_obligation_hash.clone(),
        LinkTypes::ObligationUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_obligation_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_obligation_hash.clone(),
    };
    get(latest_obligation_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateObligationInput {
    pub original_obligation_hash: ActionHash,
    pub previous_obligation_hash: ActionHash,
    pub updated_obligation: Obligation,
}
#[hdk_extern]
pub fn update_obligation(input: UpdateObligationInput) -> ExternResult<Record> {
    let updated_obligation_hash = update_entry(
        input.previous_obligation_hash.clone(),
        &input.updated_obligation,
    )?;
    create_link(
        input.original_obligation_hash.clone(),
        updated_obligation_hash.clone(),
        LinkTypes::ObligationUpdates,
        (),
    )?;
    let record =
        get(updated_obligation_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("Could not find the newly updated Obligation"))
        ))?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_obligation(original_obligation_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_obligation_hash)
}
#[hdk_extern]
pub fn get_obligations_for_debtor(debtor: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(debtor, LinkTypes::DebtorToObligations, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
#[hdk_extern]
pub fn get_obligations_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(creator, LinkTypes::CreatorToObligations, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
