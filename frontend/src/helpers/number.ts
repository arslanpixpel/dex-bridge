import { BigNumberish, ethers } from "ethers";

/**
 *
 * @param amount
 * @returns {weiAmount in string}
 */
export const toWei = (amount: string): string => ethers.utils.parseEther(amount).toString();

export const formatAmount = (decimalNumber: string) => Math.round(+decimalNumber * 1e4) / 1e4;

export const toFractionalAmount = (integerAmount: BigNumberish, decimals: number) => {
    const formatted = ethers.utils.formatUnits(integerAmount, decimals);
    const [whole, fractions] = formatted.split(".");

    if (fractions === "0") {
        return whole;
    }

    return formatted;
};
