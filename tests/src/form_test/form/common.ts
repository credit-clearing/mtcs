import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleObligation(cell: CallableCell, partialObligation = {}) {
    return {
        ...{
	  amount: 0.5,
          debtor: cell.cell_id[1],
	  attachment: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  datetime: 1674053334548000,
          creator: cell.cell_id[1],
        },
        ...partialObligation
    };
}

export async function createObligation(cell: CallableCell, obligation = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "form",
      fn_name: "create_obligation",
      payload: obligation || await sampleObligation(cell),
    });
}



export async function samplePseudoObligation(cell: CallableCell, partialPseudoObligation = {}) {
    return {
        ...{
	  amount: 0.5,
	  creditor: (await fakeAgentPubKey()),
	  trail: [(await fakeActionHash())],
        },
        ...partialPseudoObligation
    };
}

export async function createPseudoObligation(cell: CallableCell, pseudoObligation = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "form",
      fn_name: "create_pseudo_obligation",
      payload: pseudoObligation || await samplePseudoObligation(cell),
    });
}

