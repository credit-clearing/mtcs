use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct PseudoObligation {
    pub amount: f32,
    pub creditor: AgentPubKey,
    pub trail: Vec<ActionHash>,
}
pub fn validate_create_pseudo_obligation(
    _action: EntryCreationAction,
    _pseudo_obligation: PseudoObligation,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_pseudo_obligation(
    _action: Update,
    _pseudo_obligation: PseudoObligation,
    _original_action: EntryCreationAction,
    _original_pseudo_obligation: PseudoObligation,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Pseudo Obligations cannot be updated"),
        ),
    )
}
pub fn validate_delete_pseudo_obligation(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_pseudo_obligation: PseudoObligation,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Pseudo Obligations cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_pseudo_obligations(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _pseudo_obligation: crate::PseudoObligation = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_pseudo_obligations(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllPseudoObligations links cannot be deleted"),
        ),
    )
}
