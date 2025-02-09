import { PublicKey } from "@solana/web3.js";
export * from "./accounts";
export * from "./errors";
export * from "./instructions";
export * from "./types";

const CONFIG_SEED = "config";
const OPERATOR_SEED = "operator";
const NCN_SEED = "ncn";

/**
 * Program address
 *
 * @category constants
 * @category generated
 */
export const PROGRAM_ADDRESS = "RestkWeAVL8fRGgzhfeoqFhsqKRchg6aa1XrcH96z4Q";

/**
 * Program public key
 *
 * @category constants
 * @category generated
 */
export const PROGRAM_ID = new PublicKey(PROGRAM_ADDRESS);

/**
 * Restaking Config public key
 */
export const findConfigPDA = () => {
  const seedBuffer = Buffer.from(CONFIG_SEED, "utf8");

  const [pda, bump] = PublicKey.findProgramAddressSync(
    [seedBuffer],
    PROGRAM_ID,
  );

  return { pda, bump, seeds: [seedBuffer] };
};

/**
 * Restaking Operator public key
 */
export const findOperatorPDA = (base: PublicKey) => {
  const seedBuffer = Buffer.from(OPERATOR_SEED, "utf8");
  const baseBuffer = base.toBuffer();

  const [pda, bump] = PublicKey.findProgramAddressSync(
    [seedBuffer, baseBuffer],
    PROGRAM_ID,
  );

  return { pda, bump, seeds: [seedBuffer] };
};

/**
 * Restaking NCN public key
 */
export const findNcnPDA = (base: PublicKey) => {
  const seedBuffer = Buffer.from(NCN_SEED, "utf8");
  const baseBuffer = base.toBuffer();

  const [pda, bump] = PublicKey.findProgramAddressSync(
    [seedBuffer, baseBuffer],
    PROGRAM_ID,
  );

  return { pda, bump, seeds: [seedBuffer] };
};

