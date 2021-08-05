import { AddressOrPair, ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
import { Hash } from "@polkadot/types/interfaces";
import { DevTestContext } from "./setup-dev-tests";
import { ParaTestContext } from "./setup-para-tests";

export const createBlockWithExtrinsic = async <
  Call extends SubmittableExtrinsic<ApiType>,
  ApiType extends ApiTypes
>(
  context: DevTestContext,
  sender: AddressOrPair,
  polkadotCall: Call
) => {
  // This should return a string, but is a bit complex to handle type properly so any will suffice
  const extrinsicHash = (await polkadotCall.signAndSend(sender)) as any;

  // We create the block which is containing the extrinsic
  const blockResult = await context.createBlock();

  findExtrinsicWithEvents(context, blockResult.block.hash, extrinsicHash)
};


export const findExtrinsicWithEvents = async(
  context: DevTestContext | ParaTestContext,
  blockHash: Hash,
  extrinsicHash: string
) => {
  // We retrieve the events for that block
  const allRecords = await context.polkadotApi.query.system.events.at(blockHash);

  // We retrieve the block (including the extrinsics)
  const blockData = await context.polkadotApi.rpc.chain.getBlock(blockHash);

  const extrinsicIndex = blockData.block.extrinsics.findIndex(
    (ext) => ext.hash.toHex() == extrinsicHash
  );
  if (extrinsicIndex < 0) {
    throw new Error(`Extrinsic ${extrinsicHash} is missing in the block ${blockHash}`);
  }
  const extrinsic = blockData.block.extrinsics[extrinsicIndex];

  // We retrieve the events associated with the extrinsic
  const events = allRecords
    .filter(
      ({ phase }) => phase.isApplyExtrinsic && phase.asApplyExtrinsic.toNumber() == extrinsicIndex
    )
    .map(({ event }) => event);

  return { extrinsic, events };
};
