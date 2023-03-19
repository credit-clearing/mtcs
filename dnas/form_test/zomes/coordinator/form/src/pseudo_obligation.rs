use hdk::prelude::*;
use form_integrity::*;
#[hdk_extern]
pub fn create_pseudo_obligation(
    pseudo_obligation: PseudoObligation,
) -> ExternResult<Record> {
    let pseudo_obligation_hash = create_entry(
        &EntryTypes::PseudoObligation(pseudo_obligation.clone()),
    )?;
    let record = get(pseudo_obligation_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created PseudoObligation"))
            ),
        )?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(
        my_agent_pub_key,
        pseudo_obligation_hash.clone(),
        LinkTypes::AllPseudoObligations,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_pseudo_obligation(
    pseudo_obligation_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get(pseudo_obligation_hash, GetOptions::default())
}

#[hdk_extern]
pub fn find_next_obligations(_: ()) -> ExternResult<String> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;

    let links = get_links(my_pub_key.clone(), LinkTypes::AllPseudoObligations, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();

    for r in records {
        // let creditor_key = r.entry().to_app_option::<PseudoObligation>()?.unwrap().creditor;
        let entry: PseudoObligation = r.entry()
                    .to_app_option()
                    .map_err(|err| wasm_error!(err))?
                    .ok_or(wasm_error!(WasmErrorInner::Guest(
                        "Could not deserialize element to Profile.".into(),
                    )))?;
        let creditor_key = entry.creditor;

        let links2 = get_links(creditor_key, LinkTypes::AllPseudoObligations, None)?;
        let get_input2: Vec<GetInput> = links2
            .into_iter()
            .map(|link| GetInput::new(
                ActionHash::from(link.target).into(),
                GetOptions::default(),
            ))
            .collect();
        let records2 = HDK.with(|hdk| hdk.borrow().get(get_input2))?;
        let records2: Vec<Record> = records2.into_iter().filter_map(|r| r).collect();

        for r2 in records2 {
            let entry2: PseudoObligation = r2.entry()
            .to_app_option()
            .map_err(|err| wasm_error!(err))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "Could not deserialize element to Profile.".into(),
            )))?;

            let mut newpath: Vec<ActionHash> = entry.trail.clone();
            newpath.extend(entry2.trail.clone());

            let pseudo_obligation_hash = create_entry(
                &EntryTypes::PseudoObligation(PseudoObligation {
                    amount: entry2.amount,
                    creditor: entry2.creditor.clone(),
                    trail: newpath,
                }),
            )?;
            let record = get(pseudo_obligation_hash.clone(), GetOptions::default())?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest(String::from("Could not find the newly created PseudoObligation"))
                    ),
                )?;
            create_link(
                my_pub_key.clone(),
                pseudo_obligation_hash.clone(),
                LinkTypes::AllPseudoObligations,
                (),
            )?;

            // let creditor_key = r2.entry().to_app_option::<PseudoObligation>()?.unwrap().creditor;

            let supercreditor_key = entry2.creditor.clone();
            if supercreditor_key == my_pub_key {
                return Ok("Found a match".to_string());
            }
        }
    }

    Ok("Added new pseudo obligations".to_string())
}